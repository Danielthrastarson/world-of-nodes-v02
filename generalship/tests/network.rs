use futures_util::{SinkExt, StreamExt};
use generalship_g0::{model::Status, server};
use serde_json::{Value, json};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;
async fn post(addr: std::net::SocketAddr, path: &str, value: Value) -> (u16, Value) {
    let mut tcp = TcpStream::connect(addr).await.unwrap();
    let body = value.to_string();
    tcp.write_all(format!("POST {path} HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
    let mut out = vec![];
    tcp.read_to_end(&mut out).await.unwrap();
    let raw = String::from_utf8(out).unwrap();
    let (h, b) = raw.split_once("\r\n\r\n").unwrap();
    let code = h.split_whitespace().nth(1).unwrap().parse().unwrap();
    (
        code,
        serde_json::from_str(b).unwrap_or(Value::String(b.into())),
    )
}
async fn receive(socket: &mut Socket, kind: &str) -> Value {
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if let Some(Ok(Message::Text(text))) = socket.next().await {
                let v: Value = serde_json::from_str(&text).unwrap();
                if v["type"] == kind {
                    return v;
                }
            } else {
                panic!("socket closed before {kind}");
            }
        }
    })
    .await
    .unwrap()
}
async fn write(socket: &mut Socket, v: Value) {
    socket
        .send(Message::Text(v.to_string().into()))
        .await
        .unwrap();
}
async fn auth(addr: std::net::SocketAddr, seat: &Value) -> Socket {
    let (mut s, _) = connect_async(format!("ws://{addr}/ws")).await.unwrap();
    write(
        &mut s,
        json!({"room":seat["room"],"token":seat["token"],"boot":seat["boot"]}),
    )
    .await;
    s
}
#[tokio::test]
async fn two_seats_invite_auth_latency_reconnect_finish_and_rematch() {
    let app = server::app("test-only-host-code".into(), std::env::temp_dir());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = tokio::spawn(axum::serve(listener, server::router(app.clone())).into_future());
    let ticker = tokio::spawn(server::ticker(app.clone()));
    assert_eq!(
        post(addr, "/api/create", json!({"host_code":"incorrect"}))
            .await
            .0,
        401
    );
    let (code, a) = post(
        addr,
        "/api/create",
        json!({"host_code":"test-only-host-code","seed":7}),
    )
    .await;
    assert_eq!(code, 200);
    let (code, b) = post(addr, "/api/join", json!({"invite":a["invite"]})).await;
    assert_eq!(code, 200);
    assert_eq!(
        post(addr, "/api/join", json!({"invite":a["invite"]}))
            .await
            .0,
        401
    );
    let mut bad = auth(addr, &json!({"room":a["room"],"token":"bad"})).await;
    assert!(
        receive(&mut bad, "error").await["message"]
            .as_str()
            .unwrap()
            .contains("credential")
    );
    let mut west = auth(addr, &a).await;
    let mut east = auth(addr, &b).await;
    let w = receive(&mut west, "view").await;
    let e = receive(&mut east, "view").await;
    assert!(
        w["view"]["people"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["side"] == 0)
    );
    assert!(
        e["view"]["people"]
            .as_array()
            .unwrap()
            .iter()
            .all(|p| p["side"] == 1)
    );
    assert!(w["view"].get("missions").is_none());
    assert!(w["view"].get("tokens").is_none());
    for s in [&mut west, &mut east] {
        write(
            s,
            json!({"sequence":1,"generation":0,"input":{"type":"ready"}}),
        )
        .await;
        receive(s, "receipt").await;
    }
    let target = json!({"type":"move_general","id":0,"target":{"x":17000,"y":28000}});
    // 75ms outbound plus 75ms inbound delay models 150ms application round-trip latency.
    tokio::time::sleep(std::time::Duration::from_millis(75)).await;
    write(
        &mut west,
        json!({"sequence":2,"generation":0,"input":target}),
    )
    .await;
    let receipt = receive(&mut west, "receipt").await;
    tokio::time::sleep(std::time::Duration::from_millis(75)).await;
    assert_eq!(receipt["sequence"], 2);
    write(
        &mut west,
        json!({"sequence":2,"generation":0,"input":target}),
    )
    .await;
    assert_eq!(
        receive(&mut west, "receipt").await["message"],
        "Duplicate ignored"
    );
    write(&mut west,json!({"sequence":3,"generation":0,"input":{"type":"move_general","id":48,"target":{"x":17000,"y":28000}}})).await;
    receive(&mut west, "error").await;
    write(&mut west, json!({"type":"replay"})).await;
    receive(&mut west, "error").await;
    let room = app.rooms.lock().await[a["room"].as_str().unwrap()].clone();
    assert_eq!(
        room.lock()
            .await
            .game
            .inputs
            .iter()
            .filter(|r| matches!(r.input, generalship_g0::model::Input::MoveGeneral { .. }))
            .count(),
        1
    );
    west.close(None).await.unwrap();
    drop(west);
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    assert!(!room.lock().await.game.connected[0]);
    let mut west = auth(addr, &a).await;
    assert_eq!(receive(&mut west, "view").await["view"]["side"], 0);
    assert_eq!(room.lock().await.game.status, Status::Running);
    write(
        &mut west,
        json!({"sequence":4,"generation":0,"input":{"type":"surrender"}}),
    )
    .await;
    receive(&mut west, "receipt").await;
    assert!(matches!(
        room.lock().await.game.status,
        Status::Finished {
            winner: Some(1),
            ..
        }
    ));
    write(&mut west, json!({"type":"replay"})).await;
    let replay = receive(&mut west, "replay").await;
    assert!(replay["replay"]["hash"].is_string());
    for s in [&mut west, &mut east] {
        write(s, json!({"type":"rematch"})).await;
        receive(s, "receipt").await;
    }
    assert_eq!(room.lock().await.game.status, Status::Lobby);
    assert_eq!(room.lock().await.generation, 1);
    write(
        &mut west,
        json!({"sequence":5,"generation":0,"input":{"type":"ready"}}),
    )
    .await;
    assert!(
        receive(&mut west, "error").await["message"]
            .as_str()
            .unwrap()
            .contains("generation")
    );
    // A fresh process has no room state and explicitly distinguishes the old boot identity.
    let new = server::app("test-only-host-code".into(), std::env::temp_dir());
    let listener2 = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr2 = listener2.local_addr().unwrap();
    let svc2 = tokio::spawn(axum::serve(listener2, server::router(new)).into_future());
    let mut stale = auth(addr2, &a).await;
    assert!(
        receive(&mut stale, "gone").await["reason"]
            .as_str()
            .unwrap()
            .contains("restarted")
    );
    svc.abort();
    svc2.abort();
    ticker.abort();
}

/// Actual wall-clock soak; run explicitly with --ignored --nocapture.
#[tokio::test]
#[ignore]
async fn twenty_minute_two_socket_clock_soak() {
    let app = server::app("soak-test-host-code".into(), std::env::temp_dir());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let svc = tokio::spawn(axum::serve(listener, server::router(app.clone())).into_future());
    let ticker = tokio::spawn(server::ticker(app.clone()));
    let (_, a) = post(
        addr,
        "/api/create",
        json!({"host_code":"soak-test-host-code"}),
    )
    .await;
    let (_, b) = post(addr, "/api/join", json!({"invite":a["invite"]})).await;
    let mut west = auth(addr, &a).await;
    let mut east = auth(addr, &b).await;
    receive(&mut west, "view").await;
    receive(&mut east, "view").await;
    let start = std::time::Instant::now();
    for s in [&mut west, &mut east] {
        write(
            s,
            json!({"sequence":1,"generation":0,"input":{"type":"ready"}}),
        )
        .await;
        receive(s, "receipt").await;
    }
    async fn observe(mut socket: Socket) -> (u64, u64) {
        let mut heartbeat = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut count = 0;
        let mut last = 0;
        loop {
            tokio::select! {
                _=heartbeat.tick()=>write(&mut socket,json!({"type":"ping"})).await,
                frame=socket.next()=>{
                    let Some(Ok(Message::Text(text)))=frame else {panic!("socket dropped during soak")};
                    let v:Value=serde_json::from_str(&text).unwrap();
                    if v["type"]!="view" {continue;}
                    let tick=v["view"]["tick"].as_u64().unwrap();assert!(tick>=last);last=tick;count+=1;
                    if tick.is_multiple_of(600) {println!("soak tick {tick}");}
                    if v["view"]["status"]["state"]=="finished" {return (tick,count);}
                }
            }
        }
    }
    let results = tokio::time::timeout(std::time::Duration::from_secs(1206), async {
        tokio::join!(observe(west), observe(east))
    })
    .await
    .unwrap();
    let elapsed = start.elapsed().as_secs_f64();
    assert_eq!(results.0.0, 12000);
    assert_eq!(results.1.0, 12000);
    assert!(
        (1199.0..1205.0).contains(&elapsed),
        "clock drift: {elapsed}"
    );
    assert!(
        results.0.1 >= 11950 && results.1.1 >= 11950,
        "snapshots missing: {results:?}"
    );
    println!(
        "SOAK PASSED: {elapsed:.3}s, West {} snapshots, East {} snapshots",
        results.0.1, results.1.1
    );
    let room = app.rooms.lock().await[a["room"].as_str().unwrap()].clone();
    room.lock().await.game.audit().unwrap();
    svc.abort();
    ticker.abort();
}
