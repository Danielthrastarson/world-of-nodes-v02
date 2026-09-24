//! Reconstruct one opt-in playtest from its local replay.tsv using the real Session.
//! No window, wall clock, UI automation, network, or additional dependencies.
use std::{env, fs, path::Path, process};
use world_of_nodes::provisioning::playable::{MilitaryOrder, PlayerCommand, ScenarioKind, Session};

const HELP: &str = "Usage: playtest_replay SESSION_DIR\n\
Reads SESSION_DIR/replay.tsv and verifies every recorded hash checkpoint.\n\
Outputs one JSON result, including rejected inputs and final hashes.\n\
Exit 0: verified clean completion; 2: verified prefix (live/unclean/partial);\n\
1: read, format, simulation, or checkpoint failure.\n\
A partial trailing line is ignored and never reported as complete.\n\
Format: # won-playtest-v1<TAB>Local|Remote<TAB>seed; C inputs; H hashes;\n\
E<TAB>sim_ms explicitly closes a session after its final H checkpoint.";

#[derive(Debug)]
struct Rejection {
    command_index: usize,
    sim_ms: i64,
    reason: String,
}

struct ReplayResult {
    session: Session,
    checkpoints: usize,
    complete: bool,
    partial_trailing_line: bool,
    rejections: Vec<Rejection>,
}

fn number<T: std::str::FromStr>(value: &str, field: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("invalid {field}: {value:?}"))
}

fn command(fields: &[&str]) -> Result<PlayerCommand, String> {
    match fields {
        ["commission", office, project] => Ok(PlayerCommand::Commission {
            office: number(office, "office")?,
            project: number(project, "project")?,
        }),
        ["revoke", id] => Ok(PlayerCommand::Revoke {
            command: number(id, "command id")?,
        }),
        ["unit", verb, target] => Ok(PlayerCommand::Unit {
            verb: match *verb {
                "Move" => MilitaryOrder::Move,
                "Attack" => MilitaryOrder::Attack,
                "Hold" => MilitaryOrder::Hold,
                "Withdraw" => MilitaryOrder::Withdraw,
                _ => return Err(format!("invalid unit verb: {verb:?}")),
            },
            target: number(target, "target")?,
        }),
        ["observe"] => Ok(PlayerCommand::ObserveReport),
        _ => Err("unknown command or wrong field count".into()),
    }
}

fn hash(value: &str) -> Result<u64, String> {
    if value.len() != 16 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("expected 16 hex hash digits, got {value:?}"));
    }
    u64::from_str_radix(value, 16).map_err(|_| "invalid hash".into())
}

fn replay(bytes: &[u8]) -> Result<ReplayResult, String> {
    // Read a fixed byte snapshot. An append may leave an incomplete UTF-8
    // sequence too, so strip the unfinished record before UTF-8 decoding.
    let complete_bytes = bytes.iter().rposition(|&b| b == b'\n').map_or(0, |p| p + 1);
    let partial_trailing_line = complete_bytes < bytes.len();
    let source = std::str::from_utf8(&bytes[..complete_bytes])
        .map_err(|e| format!("replay is not UTF-8: {e}"))?;
    let mut lines = source.lines();
    let header: Vec<_> = lines
        .next()
        .ok_or("missing complete replay header")?
        .split('\t')
        .collect();
    let ["# won-playtest-v1", scenario, seed] = header.as_slice() else {
        return Err("invalid replay header or unsupported version".into());
    };
    let scenario = match *scenario {
        "Local" => ScenarioKind::Local,
        "Remote" => ScenarioKind::Remote,
        _ => return Err(format!("unknown scenario: {scenario:?}")),
    };
    let mut session = Session::new(number(seed, "seed")?, scenario)?;
    let mut checkpoints = 0;
    let mut rejections = Vec::new();
    let mut ended = false;
    let mut last_was_checkpoint = false;
    for (index, line) in lines.enumerate() {
        let line_number = index + 2;
        let parse_line = |session: &mut Session,
                          checkpoints: &mut usize,
                          rejections: &mut Vec<Rejection>,
                          ended: &mut bool,
                          last_was_checkpoint: &mut bool|
         -> Result<(), String> {
            if *ended {
                return Err("record after session end".into());
            }
            let fields: Vec<_> = line.split('\t').collect();
            let at = number::<i64>(fields.get(1).ok_or("missing simulation time")?, "sim_ms")?;
            match fields.first().copied() {
                Some("C") => {
                    let input = command(&fields[2..])?;
                    session.advance_to(at)?;
                    // Rejections are real inputs too: dispatch records them and
                    // may affect the same state as in the original session.
                    if let Err(reason) = session.dispatch(input) {
                        rejections.push(Rejection {
                            command_index: session.command_log().len(),
                            sim_ms: at,
                            reason,
                        });
                    }
                    *last_was_checkpoint = false;
                }
                Some("H") if fields.len() == 4 => {
                    let expected = (hash(fields[2])?, hash(fields[3])?);
                    session.advance_to(at)?;
                    let actual = (session.world().state_hash(), session.world().event_hash());
                    if expected != actual {
                        return Err(format!(
                            "checkpoint mismatch at {at}: expected state={:016x} event={:016x}; actual state={:016x} event={:016x}",
                            expected.0, expected.1, actual.0, actual.1
                        ));
                    }
                    *checkpoints += 1;
                    *last_was_checkpoint = true;
                }
                Some("E") if fields.len() == 2 => {
                    if !*last_was_checkpoint || session.now() != at {
                        return Err(
                            "session end must immediately follow a checkpoint at the same time"
                                .into(),
                        );
                    }
                    *ended = true;
                }
                _ => return Err("unknown record or wrong field count".into()),
            }
            Ok(())
        };
        parse_line(
            &mut session,
            &mut checkpoints,
            &mut rejections,
            &mut ended,
            &mut last_was_checkpoint,
        )
        .map_err(|e| format!("line {line_number}: {e}"))?;
    }
    Ok(ReplayResult {
        session,
        checkpoints,
        complete: ended && !partial_trailing_line,
        partial_trailing_line,
        rejections,
    })
}

fn json_string(value: &str) -> String {
    let mut output = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            c if c <= '\u{1f}' => output.push_str(&format!("\\u{:04x}", c as u32)),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

impl ReplayResult {
    fn json(&self) -> String {
        let rejections = self
            .rejections
            .iter()
            .map(|r| {
                format!(
                    "{{\"command_index\":{},\"sim_ms\":{},\"reason\":{}}}",
                    r.command_index,
                    r.sim_ms,
                    json_string(&r.reason)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema_version\":1,\"status\":\"{}\",\"complete\":{},\"partial_trailing_line\":{},\"scenario\":\"{:?}\",\"seed\":{},\"sim_ms\":{},\"commands\":{},\"rejected_commands\":{},\"checkpoints_verified\":{},\"state_hash\":\"{:016x}\",\"event_hash\":\"{:016x}\",\"rejections\":[{}]}}",
            if self.complete {
                "verified_complete"
            } else {
                "verified_prefix"
            },
            self.complete,
            self.partial_trailing_line,
            self.session.scenario(),
            self.session.seed(),
            self.session.now(),
            self.session.command_log().len(),
            self.rejections.len(),
            self.checkpoints,
            self.session.world().state_hash(),
            self.session.world().event_hash(),
            rejections
        )
    }
}

fn main() {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!("{HELP}");
        return;
    }
    let result = if args.len() != 1 {
        Err(HELP.to_string())
    } else {
        let path = Path::new(&args[0]).join("replay.tsv");
        fs::read(&path)
            .map_err(|e| format!("{}: {e}", path.display()))
            .and_then(|bytes| replay(&bytes))
    };
    match result {
        Ok(result) => {
            println!("{}", result.json());
            if !result.complete {
                process::exit(2);
            }
        }
        Err(error) => {
            println!(
                "{{\"schema_version\":1,\"status\":\"error\",\"complete\":false,\"error\":{}}}",
                json_string(&error)
            );
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world_of_nodes::provisioning::MINUTE;

    fn checkpoint(source: &mut String, session: &Session) {
        source.push_str(&format!(
            "H\t{}\t{:016x}\t{:016x}\n",
            session.now(),
            session.world().state_hash(),
            session.world().event_hash()
        ));
    }

    fn parity(scenario: ScenarioKind, seed: u64) {
        let mut original = Session::new(seed, scenario).unwrap();
        let mut source = format!("# won-playtest-v1\t{scenario:?}\t{seed}\n");
        checkpoint(&mut source, &original);
        for (offset, text) in [
            (0, "commission\t999\t0"),
            (0, "commission\t0\t0"),
            (0, "observe"),
            (17, "unit\tHold\t0"),
            (2 * MINUTE, "revoke\t0"),
            (2 * MINUTE, "unit\tMove\t999"),
            (3 * MINUTE, "unit\tAttack\t4"),
            (4 * MINUTE, "unit\tWithdraw\t1"),
        ] {
            let initial = if scenario == ScenarioKind::Remote {
                15 * MINUTE
            } else {
                0
            };
            original.advance_to(initial + offset).unwrap();
            source.push_str(&format!("C\t{}\t{text}\n", original.now()));
            let _ = original.dispatch(command(&text.split('\t').collect::<Vec<_>>()).unwrap());
            checkpoint(&mut source, &original);
        }
        original.advance_to(original.now() + 83).unwrap();
        checkpoint(&mut source, &original);
        source.push_str(&format!("E\t{}\n", original.now()));
        let result = replay(source.as_bytes()).unwrap();
        assert!(result.complete);
        assert!(!result.partial_trailing_line);
        assert_eq!(result.checkpoints, 10);
        assert_eq!(original.command_log(), result.session.command_log());
        assert_eq!(
            original.world().state_hash(),
            result.session.world().state_hash()
        );
        assert_eq!(
            original.world().event_hash(),
            result.session.world().event_hash()
        );
        assert!(!result.rejections.is_empty());
    }

    #[test]
    fn local_exact_inputs_hashes_and_rejections() {
        parity(ScenarioKind::Local, 17);
    }
    #[test]
    fn remote_exact_inputs_hashes_and_rejections() {
        parity(ScenarioKind::Remote, 23);
    }
    #[test]
    fn live_and_partial_prefix_are_not_complete() {
        let mut source = "# won-playtest-v1\tLocal\t17\n".to_owned();
        let original = Session::new(17, ScenarioKind::Local).unwrap();
        checkpoint(&mut source, &original);
        assert!(!replay(source.as_bytes()).unwrap().complete);
        let mut bytes = source.into_bytes();
        bytes.extend_from_slice(b"C\t0\tobserve\xff");
        let result = replay(&bytes).unwrap();
        assert!(result.partial_trailing_line);
        assert!(!result.complete);
        assert!(result.session.command_log().is_empty());
    }
    #[test]
    fn malformed_mismatched_and_uncertified_end_are_rejected() {
        for suffix in [
            "H\t0\t0000000000000000\t0000000000000000\n",
            "C\t0\tunit\tTeleport\t1\n",
            "C\t0\tobserve\textra\n",
            "C\t-1\tobserve\n",
            "E\t0\n",
        ] {
            assert!(replay(format!("# won-playtest-v1\tLocal\t17\n{suffix}").as_bytes()).is_err());
        }
        let original = Session::new(17, ScenarioKind::Local).unwrap();
        let mut source = "# won-playtest-v1\tLocal\t17\n".to_owned();
        checkpoint(&mut source, &original);
        source.push_str("E\t0\nC\t0\tobserve\n");
        assert!(replay(source.as_bytes()).is_err());
    }
    #[test]
    fn json_escapes_control_characters_and_quotes() {
        assert_eq!(
            json_string("\"\\\n\r\t\u{0} café"),
            "\"\\\"\\\\\\n\\r\\t\\u0000 café\""
        );
    }
}
