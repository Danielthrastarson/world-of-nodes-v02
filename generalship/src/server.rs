use crate::{bot::Bot, model::*, sim::Game};
use axum::{
    Json, Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{StatusCode, header},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use futures_util::{SinkExt, StreamExt};
use rand::RngCore;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub struct Room {
    pub bot: Option<Bot>,
    pub game: Game,
    pub tokens: [String; 2],
    pub invite: Option<String>,
    pub active: [bool; 2],
    pub sequences: [u64; 2],
    pub rematch: [bool; 2],
    pub generation: u64,
    pub created: Instant,
    pub written: bool,
}
impl Room {
    fn enable_bot(&mut self) {
        self.invite = None;
        self.bot = Some(Bot::default());
        let _ = self.game.input(1, Input::Connection { connected: true });
        let _ = self.game.input(1, Input::Ready);
    }
    fn reset(&mut self) {
        self.game = Game::new(self.game.seed, self.game.config.clone());
        for side in 0..2 {
            let _ = self.game.input(
                side,
                Input::Connection {
                    connected: self.active[side],
                },
            );
        }
        if self.bot.is_some() {
            self.enable_bot();
        }
        self.rematch = [false; 2];
        self.generation += 1;
        self.written = false;
    }
}
#[derive(Clone)]
pub struct App {
    pub rooms: Arc<Mutex<BTreeMap<String, Arc<Mutex<Room>>>>>,
    pub host_code: Arc<String>,
    pub boot: String,
    pub data_dir: PathBuf,
}
fn token() -> String {
    let mut bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn equal(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes().zip(b.bytes()).fold(0u8, |v, (x, y)| v | (x ^ y)) == 0
}
pub fn app(host_code: String, data_dir: PathBuf) -> App {
    App {
        rooms: Default::default(),
        host_code: Arc::new(host_code),
        boot: token(),
        data_dir,
    }
}
pub fn router(state: App) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/app.js", get(js))
        .route("/controls.mjs", get(controls))
        .route("/motion.mjs", get(motion))
        .route("/style.css", get(css))
        .route(
            "/health",
            get(|| async { Json(json!({"ok":true,"version":"g0.3"})) }),
        )
        .route("/api/create", post(create))
        .route("/api/join", post(join))
        .route("/ws", get(ws))
        .layer(axum::extract::DefaultBodyLimit::max(16384))
        .with_state(state)
}
async fn index() -> impl IntoResponse {
    (
        [
            (header::CACHE_CONTROL, "no-store"),
            (header::REFERRER_POLICY, "no-referrer"),
        ],
        Html(include_str!("../web/index.html")),
    )
}
async fn js() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        include_str!("../web/app.js"),
    )
}
async fn css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css")],
        include_str!("../web/style.css"),
    )
}
async fn controls() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        include_str!("../web/controls.mjs"),
    )
}
async fn motion() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/javascript"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        include_str!("../web/motion.mjs"),
    )
}
#[derive(Deserialize)]
struct Create {
    host_code: String,
    #[serde(default)]
    seed: u64,
    #[serde(default)]
    bot: bool,
}
async fn create(State(app): State<App>, Json(req): Json<Create>) -> Response {
    if !equal(&req.host_code, &app.host_code) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"Host code not accepted"})),
        )
            .into_response();
    }
    let mut rooms = app.rooms.lock().await;
    let mut expired = vec![];
    for (id, r) in rooms.iter() {
        let room = r.lock().await;
        if room.created.elapsed() > Duration::from_secs(3600)
            && room.active == [false; 2]
            && room.game.status != Status::Running
        {
            expired.push(id.clone());
        }
    }
    for id in expired {
        rooms.remove(&id);
    }
    if rooms.len() >= 4 {
        return (StatusCode::TOO_MANY_REQUESTS,Json(json!({"error":"Four test rooms already exist; reuse a room or restart the service"}))).into_response();
    }
    let room = token();
    let tokens = [token(), token()];
    let invite = token();
    let reply = json!({"room":room,"token":tokens[0],"invite":if req.bot {None} else {Some(format!("{room}.{invite}"))},"side":0,"boot":app.boot,"opponent":if req.bot {"bot"} else {"human"}});
    let mut new_room = Room {
        bot: None,
        game: Game::new(req.seed, Config::default()),
        tokens,
        invite: Some(invite),
        active: [false; 2],
        sequences: [0; 2],
        rematch: [false; 2],
        generation: 0,
        created: Instant::now(),
        written: false,
    };
    if req.bot {
        new_room.enable_bot();
    }
    rooms.insert(room, Arc::new(Mutex::new(new_room)));
    Json(reply).into_response()
}
#[derive(Deserialize)]
struct Join {
    invite: String,
}
async fn join(State(app): State<App>, Json(req): Json<Join>) -> Response {
    let Some((id, secret)) = req.invite.split_once('.') else {
        return (StatusCode::BAD_REQUEST, "Invalid invite").into_response();
    };
    let r = app.rooms.lock().await.get(id).cloned();
    let Some(r) = r else {
        return (StatusCode::NOT_FOUND, "Match no longer exists").into_response();
    };
    let mut r = r.lock().await;
    if !r.invite.as_ref().is_some_and(|s| equal(s, secret)) {
        return (
            StatusCode::UNAUTHORIZED,
            "Invite has expired or was already used",
        )
            .into_response();
    }
    r.invite = None;
    Json(json!({"room":id,"token":r.tokens[1],"side":1,"boot":app.boot})).into_response()
}
async fn ws(State(app): State<App>, upgrade: WebSocketUpgrade) -> Response {
    upgrade
        .max_message_size(16384)
        .max_frame_size(16384)
        .on_upgrade(move |socket| connection(socket, app))
}
#[derive(Deserialize)]
struct Auth {
    room: String,
    token: String,
    boot: Option<String>,
}
async fn send(socket: &mut WebSocket, v: Value) -> bool {
    matches!(
        tokio::time::timeout(
            Duration::from_secs(5),
            socket.send(Message::Text(v.to_string().into()))
        )
        .await,
        Ok(Ok(()))
    )
}
async fn connection(mut socket: WebSocket, app: App) {
    let Some(Ok(Message::Text(raw))) = tokio::time::timeout(Duration::from_secs(5), socket.recv())
        .await
        .ok()
        .flatten()
    else {
        return;
    };
    let Ok(auth) = serde_json::from_str::<Auth>(&raw) else {
        return;
    };
    let room = app.rooms.lock().await.get(&auth.room).cloned();
    let Some(room) = room else {
        send(&mut socket,json!({"type":"gone","reason":if auth.boot.as_ref().is_some_and(|s|s!=&app.boot){"Server restarted; the match was aborted without a winner"}else{"Match no longer exists"}})).await;
        return;
    };
    let seat;
    {
        let mut r = room.lock().await;
        let Some(s) = (0..2).find(|&s| equal(&auth.token, &r.tokens[s])) else {
            send(
                &mut socket,
                json!({"type":"error","message":"Invalid seat credential"}),
            )
            .await;
            return;
        };
        seat = s;
        if seat == 1 && r.bot.is_some() {
            send(
                &mut socket,
                json!({"type":"error","message":"This seat belongs to the practice bot"}),
            )
            .await;
            return;
        }
        if r.active[seat] {
            send(
                &mut socket,
                json!({"type":"error","message":"This seat is already connected in another tab"}),
            )
            .await;
            return;
        }
        r.active[seat] = true;
        let _ = r.game.input(seat, Input::Connection { connected: true });
    }
    let (mut tx, mut rx) = socket.split();
    let mut snapshots = tokio::time::interval(Duration::from_millis(100));
    snapshots.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut window = Instant::now();
    let mut count = 0u32;
    let mut heartbeat = Instant::now();
    loop {
        let message = tokio::select! {
            _=snapshots.tick()=>{
                if heartbeat.elapsed()>Duration::from_secs(25){break;}
                let r=room.lock().await;
                json!({"type":"view","boot":app.boot,"generation":r.generation,"last_sequence":r.sequences[seat],"rematch":r.rematch,"opponent":if r.bot.is_some(){"bot"}else{"human"},"can_add_bot":seat==0&&r.invite.is_some()&&r.game.status==Status::Lobby,"view":r.game.view(seat)})
            },
            incoming=rx.next()=>{
                let Some(Ok(incoming))=incoming else{break};heartbeat=Instant::now();
                if let Message::Text(raw)=incoming{
                    if window.elapsed()>Duration::from_secs(1){window=Instant::now();count=0;}count+=1;if count>30{break;}
                    let Ok(v)=serde_json::from_str::<Value>(&raw)else{break};
                    if v["type"]=="ping"{continue;}
                    let mut r=room.lock().await;
                    if v["type"]=="add_bot"{
                        if seat!=0||r.game.status!=Status::Lobby||r.invite.is_none()||r.active[1]{json!({"type":"error","message":"A bot can only fill an unclaimed opponent seat before a match"})}
                        else{r.enable_bot();let _=r.game.input(0,Input::Ready);json!({"type":"receipt","message":"Practice started against the bot"})}
                    }else if v["type"]=="rematch"{
                        if !matches!(r.game.status,Status::Finished{..}){json!({"type":"error","message":"Finish the current match first"})}
                        else{r.rematch[seat]=true;if r.bot.is_some(){r.rematch[1]=true;}
                        if r.rematch==[true;2]{r.reset();}json!({"type":"receipt","message":"Rematch requested"})}
                    }else if v["type"]=="replay"{
                        if matches!(r.game.status,Status::Finished{..}){json!({"type":"replay","replay":r.game.replay()})}else{json!({"type":"error","message":"Omniscient replay is available only after the match ends"})}
                    }else{
                        let seq=v["sequence"].as_u64().unwrap_or(0);
                        if seq==0||seq<=r.sequences[seat]{json!({"type":"receipt","sequence":seq,"message":"Duplicate ignored"})}
                        else if v["generation"].as_u64()!=Some(r.generation){json!({"type":"error","message":"Stale match generation"})}
                        else{r.sequences[seat]=seq;match serde_json::from_value::<Input>(v["input"].clone()){
                            Ok(Input::Connection{..})=>json!({"type":"error","message":"Connection events are server-owned"}),
                            Ok(input)=>match r.game.input(seat,input){Ok(())=>json!({"type":"receipt","sequence":seq,"message":"Input accepted by server; remote orders still require delivery"}),Err(e)=>json!({"type":"error","sequence":seq,"message":e})},
                            Err(_)=>json!({"type":"error","message":"Invalid command"}),
                        }}
                    }
                }else if matches!(incoming,Message::Close(_)){break;}else{continue;}
            }
        };
        if !matches!(
            tokio::time::timeout(
                Duration::from_secs(5),
                tx.send(Message::Text(message.to_string().into()))
            )
            .await,
            Ok(Ok(()))
        ) {
            break;
        }
    }
    let mut r = room.lock().await;
    r.active[seat] = false;
    let _ = r.game.input(seat, Input::Connection { connected: false });
}
pub async fn ticker(app: App) {
    let mut ticks = tokio::time::interval(Duration::from_millis(DT_MS));
    ticks.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    loop {
        ticks.tick().await;
        let rooms: Vec<_> = app
            .rooms
            .lock()
            .await
            .iter()
            .map(|(id, r)| (id.clone(), r.clone()))
            .collect();
        for (id, room) in rooms {
            let start = Instant::now();
            let mut r = room.lock().await;
            if r.bot.is_some() && r.game.status == Status::Running && r.game.tick.is_multiple_of(30)
            {
                let snapshot = r.game.view(1);
                let inputs = r.bot.as_mut().unwrap().decide(snapshot);
                for input in inputs {
                    // Same validation, physical delivery, costs, and replay recording as a human.
                    let _ = r.game.input(1, input);
                }
            }
            r.game.step();
            if matches!(r.game.status, Status::Finished { .. }) && !r.written {
                let path = app.data_dir.join(format!("{id}-{}.json", r.generation));
                let data = serde_json::to_vec(&r.game.replay()).unwrap();
                r.written = true;
                tokio::spawn(async move {
                    if let Err(e) = tokio::fs::write(path, data).await {
                        eprintln!("Replay write failed: {e}");
                    }
                });
            }
            if start.elapsed() > Duration::from_millis(100) {
                eprintln!("Slow simulation tick: {} ms", start.elapsed().as_millis());
            }
        }
    }
}
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if let Some(i) = args.iter().position(|s| s == "--replay") {
        let path = args.get(i + 1).ok_or("--replay requires a file")?;
        let r: Replay = serde_json::from_slice(&std::fs::read(path)?)?;
        let g = Game::replay_run(&r)?;
        let hash = g.hash();
        println!("Replay tick {}: {hash}", g.tick);
        if hash != r.hash {
            return Err("Replay hash mismatch".into());
        }
        return Ok(());
    }
    let host = std::env::var("G0_BIND").unwrap_or("127.0.0.1".into());
    let port = std::env::var("PORT").unwrap_or("8080".into());
    let code = std::env::var("G0_HOST_CODE").map_err(
        |_| "Set G0_HOST_CODE to a private match-creation code (at least 12 characters)",
    )?;
    if code.len() < 12 {
        return Err("G0_HOST_CODE must have at least 12 characters".into());
    }
    let data = PathBuf::from(std::env::var("G0_DATA_DIR").unwrap_or("generalship/runtime".into()));
    tokio::fs::create_dir_all(&data).await?;
    let app = app(code, data);
    tokio::spawn(ticker(app.clone()));
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    println!(
        "Generalship G0 listening at http://{host}:{port}; share the browser-generated invite, not the host code."
    );
    axum::serve(listener, router(app))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
