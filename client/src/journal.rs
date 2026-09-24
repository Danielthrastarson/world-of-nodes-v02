//! Opt-in, local evidence for a play session. This module never changes the game.
//!
//! A fresh private directory holds an append-only event journal and canonical
//! replay. The only incoming requests ask the client to export its current view,
//! capture its own window, or record a checkpoint; they cannot dispatch inputs.
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;
use world_of_nodes::provisioning::playable::session::CommandRecord;
use world_of_nodes::provisioning::playable::{MilitaryOrder, PlayerCommand, ScenarioKind};

const MAX_REQUEST_BYTES: u64 = 256;
const PUBLISHED_NAMES: &[&str] = &[
    "current.json",
    "snapshot.json",
    "status.json",
    "checkpoint.json",
];

pub struct Journal {
    directory: PathBuf,
    events: File,
    replay: File,
    started: Instant,
    sequence: u64,
    frame: u64,
    sim_ms: i64,
    commands_written: usize,
    publication: u64,
    request_ids: BTreeSet<String>,
    error: Option<String>,
    finished: bool,
}

impl Journal {
    /// Create a new session directory. An existing path is always an error.
    pub fn open(path: &Path, seed: u64, scenario: ScenarioKind) -> io::Result<Self> {
        let mut builder = fs::DirBuilder::new();
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(path)?;
        let events = new_file(&path.join("events.jsonl"))?;
        let mut replay = new_file(&path.join("replay.tsv"))?;
        let scenario = match scenario {
            ScenarioKind::Local => "Local",
            ScenarioKind::Remote => "Remote",
        };
        writeln!(replay, "# won-playtest-v1\t{scenario}\t{seed}")?;
        let mut journal = Self {
            directory: path.to_path_buf(),
            events,
            replay,
            started: Instant::now(),
            sequence: 0,
            frame: 0,
            sim_ms: 0,
            commands_written: 0,
            publication: 0,
            request_ids: BTreeSet::new(),
            error: None,
            finished: false,
        };
        journal.event(
            "session_start",
            &[("seed", seed.to_string()), ("scenario", scenario.into())],
        );
        if let Some(error) = journal.error() {
            return Err(io::Error::other(error.to_owned()));
        }
        Ok(journal)
    }

    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// The first failure is retained for the client to display. After it, all
    /// writes and requests stop so an incomplete journal cannot look current.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Let a caller's diagnostic export failure stop the journal through the
    /// same visible error path as an append or publication failure.
    pub fn mark_error(&mut self, operation: &str, error: io::Error) {
        self.fail(operation, error);
    }

    pub fn begin_frame(&mut self, frame: u64, sim_ms: i64) {
        self.frame = frame;
        self.sim_ms = sim_ms;
    }

    /// Every value is a JSON string, including counters and timestamps. Callers
    /// cannot replace the reserved envelope fields with supplied fields.
    pub fn event(&mut self, kind: &str, fields: &[(&str, String)]) {
        if !self.writable() {
            return;
        }
        let next_sequence = self.sequence + 1;
        let mut line = format!(
            "{{\"seq\":\"{next_sequence}\",\"kind\":{},\"wall_elapsed_ms\":\"{}\",\"sim_ms\":\"{}\",\"frame\":\"{}\"",
            json_string(kind),
            self.started.elapsed().as_millis(),
            self.sim_ms,
            self.frame
        );
        let mut keys = BTreeSet::from(["seq", "kind", "wall_elapsed_ms", "sim_ms", "frame"]);
        for (key, value) in fields {
            if keys.insert(key) {
                line.push(',');
                line.push_str(&json_string(key));
                line.push(':');
                line.push_str(&json_string(value));
            }
        }
        line.push_str("}\n");
        match self.events.write_all(line.as_bytes()) {
            Ok(()) => self.sequence = next_sequence,
            Err(error) => self.fail("append events.jsonl", error),
        }
    }

    /// Append only the unseen suffix of Session::command_log(). Failed attempts
    /// are replay inputs too; outcomes stay in JSONL rather than affect replay.
    pub fn record_commands(&mut self, commands: &[CommandRecord]) {
        if !self.writable() {
            return;
        }
        if commands.len() < self.commands_written {
            self.fail(
                "record commands",
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "command log shrank during session",
                ),
            );
            return;
        }
        for record in &commands[self.commands_written..] {
            let (kind, command) = command_text(&record.command);
            let line = format!("C\t{}\t{command}\n", record.at);
            if let Err(error) = self.replay.write_all(line.as_bytes()) {
                self.fail("append replay command", error);
                return;
            }
            self.commands_written += 1;
            let mut fields = vec![
                ("command_index", self.commands_written.to_string()),
                ("command_type", kind.to_owned()),
                ("command", command),
                ("attempt_sim_ms", record.at.to_string()),
                ("accepted", record.outcome.is_ok().to_string()),
            ];
            match &record.outcome {
                Ok(result) => fields.push(("result", format!("{result:?}"))),
                Err(error) => fields.push(("error", error.clone())),
            }
            self.event("command", &fields);
            if !self.writable() {
                return;
            }
        }
    }

    /// Append a replay checkpoint. These opaque hashes reveal no private state.
    pub fn checkpoint(&mut self, sim_ms: i64, state_hash: u64, event_hash: u64) {
        if !self.writable() {
            return;
        }
        let line = format!("H\t{sim_ms}\t{state_hash:016x}\t{event_hash:016x}\n");
        if let Err(error) = self.replay.write_all(line.as_bytes()) {
            self.fail("append replay checkpoint", error);
            return;
        }
        self.sim_ms = sim_ms;
        self.event(
            "checkpoint",
            &[
                ("state_hash", format!("{state_hash:016x}")),
                ("event_hash", format!("{event_hash:016x}")),
            ],
        );
    }

    /// Publish one of the fixed status files by atomic rename. The client is
    /// responsible for passing only its public, knowledge-bounded snapshot.
    pub fn publish(&mut self, name: &str, contents: &str) {
        if !self.writable() {
            return;
        }
        if !PUBLISHED_NAMES.contains(&name) {
            self.fail(
                "publish status",
                io::Error::new(io::ErrorKind::InvalidInput, "unrecognized status filename"),
            );
            return;
        }
        self.publication += 1;
        let temporary = self
            .directory
            .join(format!(".{name}.{}.tmp", self.publication));
        let result = (|| -> io::Result<()> {
            let mut file = new_file(&temporary)?;
            file.write_all(contents.as_bytes())?;
            file.flush()?;
            drop(file);
            fs::rename(&temporary, self.directory.join(name))
        })();
        if let Err(error) = result {
            let _ = fs::remove_file(temporary);
            self.fail("publish status", error);
        }
    }

    /// Consume one bounded, local `id action` request. Rejected requests are
    /// acknowledged in JSONL; only the three read-only actions are returned.
    pub fn poll_request(&mut self) -> Option<(String, String)> {
        if !self.writable() {
            return None;
        }
        let path = self.directory.join("request.txt");
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return None,
            Err(error) => {
                self.fail("inspect request", error);
                return None;
            }
        };
        if !metadata.is_file() {
            self.reject_request("request must be a regular file", None);
            if let Err(error) = fs::remove_file(path) {
                self.fail("remove invalid request", error);
            }
            return None;
        }
        let result = (|| -> io::Result<Vec<u8>> {
            let file = File::open(&path)?;
            let mut bytes = Vec::new();
            file.take(MAX_REQUEST_BYTES + 1).read_to_end(&mut bytes)?;
            fs::remove_file(&path)?;
            Ok(bytes)
        })();
        let bytes = match result {
            Ok(bytes) => bytes,
            Err(error) => {
                self.fail("consume request", error);
                return None;
            }
        };
        if bytes.len() > MAX_REQUEST_BYTES as usize {
            self.reject_request("request exceeds 256 bytes", None);
            return None;
        }
        let Ok(request) = std::str::from_utf8(&bytes) else {
            self.reject_request("request is not UTF-8", None);
            return None;
        };
        let mut words = request.split_whitespace();
        let Some(id) = words.next().filter(|id| valid_id(id)) else {
            self.reject_request("invalid request id", None);
            return None;
        };
        let Some(action) = words.next() else {
            self.reject_request("missing request action", Some(id));
            return None;
        };
        if words.next().is_some() || !matches!(action, "snapshot" | "screenshot" | "checkpoint") {
            self.reject_request(
                "action must be snapshot, screenshot, or checkpoint",
                Some(id),
            );
            return None;
        }
        if !self.request_ids.insert(id.to_owned()) {
            self.reject_request("duplicate request id", Some(id));
            return None;
        }
        self.event(
            "request_accepted",
            &[("request_id", id.to_owned()), ("action", action.to_owned())],
        );
        self.writable().then(|| (id.to_owned(), action.to_owned()))
    }

    /// Mark a normal exit after the caller has recorded its final checkpoint.
    /// No Drop implementation invents completion after a crash or early abort.
    pub fn finish(&mut self, sim_ms: i64) {
        if !self.writable() {
            return;
        }
        self.sim_ms = sim_ms;
        self.event("session_end", &[("clean_exit", "true".into())]);
        if !self.writable() {
            return;
        }
        let result = (|| -> io::Result<()> {
            self.events.sync_all()?;
            writeln!(self.replay, "E\t{sim_ms}")?;
            self.replay.sync_all()
        })();
        match result {
            Ok(()) => {
                // A complete status is the final publication, after both logs
                // have been synced. Leave the previous incomplete status in
                // place if any earlier write or sync failed.
                self.publish("status.json", "{\"running\":false,\"clean_exit\":true}\n");
                if self.error.is_none() {
                    self.finished = true;
                }
            }
            Err(error) => self.fail("finish journal", error),
        }
    }

    fn writable(&self) -> bool {
        self.error.is_none() && !self.finished
    }

    fn fail(&mut self, operation: &str, error: io::Error) {
        if self.error.is_none() {
            self.error = Some(format!("{operation}: {error}"));
        }
    }

    fn reject_request(&mut self, reason: &str, id: Option<&str>) {
        let mut fields = vec![("reason", reason.to_owned())];
        if let Some(id) = id {
            fields.push(("request_id", id.to_owned()));
        }
        self.event("request_rejected", &fields);
    }
}

fn new_file(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn command_text(command: &PlayerCommand) -> (&'static str, String) {
    match command {
        PlayerCommand::Commission { office, project } => {
            ("commission", format!("commission\t{office}\t{project}"))
        }
        PlayerCommand::Revoke { command } => ("revoke", format!("revoke\t{command}")),
        PlayerCommand::Unit { verb, target } => {
            let verb = match verb {
                MilitaryOrder::Move => "Move",
                MilitaryOrder::Attack => "Attack",
                MilitaryOrder::Hold => "Hold",
                MilitaryOrder::Withdraw => "Withdraw",
            };
            ("unit", format!("unit\t{verb}\t{target}"))
        }
        PlayerCommand::ObserveReport => ("observe", "observe".into()),
    }
}

/// Quote a complete JSON string without any serialization dependency.
pub fn json_string(value: &str) -> String {
    use std::fmt::Write as _;
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('"');
    for character in value.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            '\u{08}' => quoted.push_str("\\b"),
            '\u{0c}' => quoted.push_str("\\f"),
            character if character <= '\u{1f}' => {
                let _ = write!(quoted, "\\u{:04x}", character as u32);
            }
            character => quoted.push(character),
        }
    }
    quoted.push('"');
    quoted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use world_of_nodes::provisioning::playable::Session;

    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);
    impl TestDirectory {
        fn new() -> Self {
            Self(std::env::temp_dir().join(format!(
                "won-journal-{}-{}",
                std::process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            )))
        }
    }
    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn json_quotes_controls_and_preserves_unicode() {
        assert_eq!(
            json_string("a\"\\\n\r\t\u{8}\u{c}\0\u{1f} café 🗺"),
            "\"a\\\"\\\\\\n\\r\\t\\b\\f\\u0000\\u001f café 🗺\""
        );
    }

    #[test]
    fn requires_fresh_directory_and_private_permissions() {
        let directory = TestDirectory::new();
        let journal = Journal::open(&directory.0, 19, ScenarioKind::Remote).unwrap();
        assert_eq!(journal.directory(), directory.0);
        assert!(Journal::open(&directory.0, 20, ScenarioKind::Local).is_err());
        assert_eq!(
            fs::read_to_string(directory.0.join("replay.tsv")).unwrap(),
            "# won-playtest-v1\tRemote\t19\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&directory.0).unwrap().permissions().mode() & 0o777,
                0o700
            );
            assert_eq!(
                fs::metadata(directory.0.join("events.jsonl"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
    }

    #[test]
    fn event_sequence_and_frame_are_strings_and_cannot_be_replaced() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 0, ScenarioKind::Local).unwrap();
        journal.begin_frame(12, 340);
        journal.event(
            "selection",
            &[("target", "a\"b".into()), ("seq", "bad".into())],
        );
        journal.event("paused", &[]);
        let text = fs::read_to_string(directory.0.join("events.jsonl")).unwrap();
        let lines: Vec<_> = text.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[1].starts_with("{\"seq\":\"2\",\"kind\":\"selection\""));
        assert!(lines[1].contains("\"sim_ms\":\"340\",\"frame\":\"12\""));
        assert!(lines[1].contains("\"target\":\"a\\\"b\""));
        assert!(!text.contains("bad"));
        assert!(lines[2].starts_with("{\"seq\":\"3\""));
    }

    #[test]
    fn request_protocol_is_bounded_and_read_only() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 0, ScenarioKind::Local).unwrap();
        let request = directory.0.join("request.txt");
        assert_eq!(journal.poll_request(), None);
        for contents in [
            "id move",
            "../escape screenshot",
            "id snapshot extra",
            "id",
            "é snapshot",
        ] {
            fs::write(&request, contents).unwrap();
            assert_eq!(journal.poll_request(), None);
            assert!(!request.exists());
        }
        fs::write(&request, "x".repeat(257)).unwrap();
        assert_eq!(journal.poll_request(), None);
        fs::write(&request, [0xff]).unwrap();
        assert_eq!(journal.poll_request(), None);
        for (index, action) in ["snapshot", "screenshot", "checkpoint"].iter().enumerate() {
            let id = format!("Request_{index}-A");
            fs::write(&request, format!("{id} {action}\n")).unwrap();
            assert_eq!(
                journal.poll_request(),
                Some((id.clone(), action.to_string()))
            );
            fs::write(&request, format!("{id} {action}")).unwrap();
            assert_eq!(journal.poll_request(), None);
        }
        assert!(journal.error().is_none());
        let events = fs::read_to_string(directory.0.join("events.jsonl")).unwrap();
        assert_eq!(events.matches("\"kind\":\"request_accepted\"").count(), 3);
        assert_eq!(events.matches("\"kind\":\"request_rejected\"").count(), 10);
    }

    #[test]
    fn canonical_transcript_records_each_attempt_once_and_clean_end() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 7, ScenarioKind::Local).unwrap();
        let mut session = Session::new(7, ScenarioKind::Local).unwrap();
        let commands = [
            PlayerCommand::ObserveReport,
            PlayerCommand::Commission {
                office: 999,
                project: 998,
            },
            PlayerCommand::Revoke { command: 997 },
            PlayerCommand::Unit {
                verb: MilitaryOrder::Move,
                target: 996,
            },
            PlayerCommand::Unit {
                verb: MilitaryOrder::Attack,
                target: 995,
            },
            PlayerCommand::Unit {
                verb: MilitaryOrder::Hold,
                target: 994,
            },
            PlayerCommand::Unit {
                verb: MilitaryOrder::Withdraw,
                target: 993,
            },
        ];
        for command in commands {
            let _ = session.dispatch(command);
            journal.record_commands(session.command_log());
            journal.record_commands(session.command_log());
        }
        journal.checkpoint(0, 0xab, 0xcd);
        journal.finish(0);
        journal.finish(0);
        assert_eq!(
            fs::read_to_string(directory.0.join("replay.tsv")).unwrap(),
            concat!(
                "# won-playtest-v1\tLocal\t7\n",
                "C\t0\tobserve\n",
                "C\t0\tcommission\t999\t998\n",
                "C\t0\trevoke\t997\n",
                "C\t0\tunit\tMove\t996\n",
                "C\t0\tunit\tAttack\t995\n",
                "C\t0\tunit\tHold\t994\n",
                "C\t0\tunit\tWithdraw\t993\n",
                "H\t0\t00000000000000ab\t00000000000000cd\n",
                "E\t0\n"
            )
        );
        let events = fs::read_to_string(directory.0.join("events.jsonl")).unwrap();
        assert_eq!(events.matches("\"kind\":\"command\"").count(), 7);
        assert!(events.contains("\"accepted\":\"false\""));
        assert!(events.contains("\"error\":"));
        assert!(journal.error().is_none());
    }

    #[test]
    fn publications_replace_atomically_and_failures_stop_future_writes() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 0, ScenarioKind::Local).unwrap();
        journal.publish("current.json", "{\"version\":\"1\"}\n");
        journal.publish("current.json", "{\"version\":\"2\"}\n");
        assert_eq!(
            fs::read_to_string(directory.0.join("current.json")).unwrap(),
            "{\"version\":\"2\"}\n"
        );
        let before = fs::read_to_string(directory.0.join("events.jsonl")).unwrap();
        // A directory at the destination reliably makes rename fail, even when
        // tests run with permissions that bypass normal read-only file modes.
        fs::create_dir(directory.0.join("status.json")).unwrap();
        journal.publish("status.json", "{}");
        let first_error = journal.error().unwrap().to_owned();
        journal.event("should_not_write", &[]);
        journal.publish("../escape.json", "{}");
        journal.checkpoint(0, 0, 0);
        journal.finish(0);
        assert_eq!(journal.error(), Some(first_error.as_str()));
        assert_eq!(
            fs::read_to_string(directory.0.join("events.jsonl")).unwrap(),
            before
        );
        assert!(
            !fs::read_to_string(directory.0.join("replay.tsv"))
                .unwrap()
                .contains("H\t")
        );
        assert!(!directory.0.join(".status.json.3.tmp").exists());
    }

    #[cfg(unix)]
    #[test]
    fn failed_finish_cannot_publish_clean_status_or_replay_end() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 0, ScenarioKind::Local).unwrap();
        journal.publish("status.json", "{\"running\":true,\"clean_exit\":false}\n");
        journal.checkpoint(0, 1, 2);
        // A special device accepts event writes but rejects sync_all, exercising
        // an actual I/O failure after session_end rather than an input error.
        journal.events = OpenOptions::new().write(true).open("/dev/null").unwrap();
        journal.finish(0);
        assert!(journal.error().is_some());
        assert_eq!(
            fs::read_to_string(directory.0.join("status.json")).unwrap(),
            "{\"running\":true,\"clean_exit\":false}\n"
        );
        assert!(
            !fs::read_to_string(directory.0.join("replay.tsv"))
                .unwrap()
                .contains("E\t")
        );
    }

    #[test]
    fn caller_export_failure_stops_clean_completion() {
        let directory = TestDirectory::new();
        let mut journal = Journal::open(&directory.0, 0, ScenarioKind::Local).unwrap();
        journal.publish("status.json", "{\"running\":true,\"clean_exit\":false}\n");
        journal.mark_error("save screenshot", io::Error::other("export failed"));
        journal.finish(0);
        assert_eq!(journal.error(), Some("save screenshot: export failed"));
        assert!(
            fs::read_to_string(directory.0.join("status.json"))
                .unwrap()
                .contains("\"clean_exit\":false")
        );
        assert!(
            !fs::read_to_string(directory.0.join("replay.tsv"))
                .unwrap()
                .contains("E\t")
        );
    }
}
