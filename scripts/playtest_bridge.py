#!/usr/bin/env python3
"""Read one local, opt-in playtest journal; queue diagnostic requests only.

Examples (run from the repository root):
  python3 scripts/playtest_bridge.py status /tmp/won-playtest
  python3 scripts/playtest_bridge.py tail /tmp/won-playtest --lines 30
  python3 scripts/playtest_bridge.py summary /tmp/won-playtest
  python3 scripts/playtest_bridge.py request /tmp/won-playtest screenshot

`status --json` exposes current.json plus journal health, `tail --json` emits
JSONL, and `summary --json` emits a structured report. Incomplete trailing lines
are ignored and reported. A missing replay E record means live or unclean, not
completed. Hash verification is performed by the separate playtest_replay binary.
There is no network, shell execution, game command injection, or process control.
"""
from __future__ import annotations

import argparse
from collections import Counter
import json
import os
from pathlib import Path
import sys
import tempfile
import uuid


ACTIONS = ("snapshot", "screenshot", "checkpoint")


def json_text(value):
    return json.dumps(value, ensure_ascii=False, sort_keys=True)


def complete_lines(path):
    """Snapshot an append-only file, excluding even a partial UTF-8 record."""
    raw = path.read_bytes()
    end = raw.rfind(b"\n") + 1
    return raw[:end].decode("utf-8").splitlines(), end < len(raw)


def journal(directory):
    lines, partial = complete_lines(directory / "events.jsonl")
    records = []
    for index, line in enumerate(lines, 1):
        try:
            record = json.loads(line)
        except json.JSONDecodeError as exc:
            raise ValueError(f"events.jsonl line {index}: {exc}") from exc
        if not isinstance(record, dict):
            raise ValueError(f"events.jsonl line {index} is not a JSON object")
        records.append(record)
    return records, partial


def replay_metadata(directory):
    """Inspect framing only; never imply that Python verified simulation hashes."""
    lines, partial = complete_lines(directory / "replay.tsv")
    if not lines:
        return {"complete": False, "partial_trailing_line": partial,
                "commands": 0, "checkpoints": 0, "hashes_verified": False,
                "status": "missing complete header"}
    header = lines[0].split("\t")
    if len(header) != 3 or header[0] != "# won-playtest-v1" or header[1] not in ("Local", "Remote"):
        raise ValueError("replay.tsv has an unsupported or malformed header")
    result = {"scenario": header[1], "seed": int(header[2]), "commands": 0,
              "checkpoints": 0, "complete": False, "partial_trailing_line": partial,
              "hashes_verified": False, "latest_checkpoint": None}
    ended = False
    last_checkpoint_time = None
    previous_time = None
    for index, line in enumerate(lines[1:], 2):
        fields = line.split("\t")
        if ended:
            raise ValueError(f"replay.tsv line {index}: record after session end")
        if len(fields) < 2:
            raise ValueError(f"replay.tsv line {index}: missing simulation time")
        at = int(fields[1])
        if at < 0 or (previous_time is not None and at < previous_time):
            raise ValueError(f"replay.tsv line {index}: simulation time goes backwards")
        previous_time = at
        if fields[0] == "C" and len(fields) >= 3:
            body = fields[2:]
            valid = (len(body) == 3 and body[0] == "commission" and all(v.isdecimal() for v in body[1:]))
            valid |= len(body) == 2 and body[0] == "revoke" and body[1].isdecimal()
            valid |= (len(body) == 3 and body[0] == "unit" and body[1] in ("Move", "Attack", "Hold", "Withdraw") and body[2].isdecimal())
            valid |= body == ["observe"]
            if not valid:
                raise ValueError(f"replay.tsv line {index}: invalid command")
            result["commands"] += 1
            last_checkpoint_time = None
        elif fields[0] == "H" and len(fields) == 4:
            if any(len(value) != 16 or any(c not in "0123456789abcdefABCDEF" for c in value) for value in fields[2:]):
                raise ValueError(f"replay.tsv line {index}: invalid hash")
            result["checkpoints"] += 1
            last_checkpoint_time = at
            result["latest_checkpoint"] = {"sim_ms": at, "state_hash": fields[2], "event_hash": fields[3]}
        elif fields[0] == "E" and len(fields) == 2 and last_checkpoint_time == at:
            ended = True
        else:
            raise ValueError(f"replay.tsv line {index}: invalid record or uncertified session end")
    result["complete"] = ended and not partial
    result["status"] = "clean end recorded" if result["complete"] else "live or unclean prefix"
    return result


def current_state(directory):
    path = directory / "current.json"
    if not path.exists():
        return None
    state = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(state, dict):
        raise ValueError("current.json is not a JSON object")
    return state


def event_kind(record):
    return str(record.get("kind", "unknown")).lower().replace("_", "")


def value_text(value):
    return json_text(value) if isinstance(value, (dict, list)) else str(value)


def ui_text(state):
    if not state:
        return "Current UI state: not yet available"
    return (f"sim_ms={state.get('sim_ms', '?')}, frame={state.get('frame', '?')}, "
            f"view={state.get('view_mode', '?')}, paused={state.get('paused', '?')}, speed={state.get('speed', '?')}\n"
            f"Selection: {state.get('selection', '?')}; context={state.get('verb', '?')}; "
            f"draft={state.get('draft', '?')}; tab={state.get('tab', '?')}\n"
            f"Notice: {state.get('notice', '')}")


def status(directory):
    records, partial = journal(directory)
    state = current_state(directory)
    return {"session_dir": str(directory), "current": state,
            "events": len(records), "events_partial_trailing_line": partial,
            "last_event": records[-1] if records else None,
            "replay": replay_metadata(directory),
            "pending_request": (directory / "request.txt").exists()}


def short_event(record):
    prefix = f"#{record.get('seq', '?')} sim={record.get('sim_ms', '?')} frame={record.get('frame', '?')} {record.get('kind', '?')}"
    fields = " ".join(f"{key}={value_text(value)}" for key, value in record.items()
                      if key not in ("seq", "kind", "wall_elapsed_ms", "sim_ms", "frame"))
    return f"{prefix} {fields}".rstrip()


def structured_field(record, field, expected_type):
    value = record.get(field)
    if value is None:
        return None
    if isinstance(value, str):
        try:
            value = json.loads(value)
        except json.JSONDecodeError as exc:
            raise ValueError(f"event #{record.get('seq', '?')} has invalid JSON in {field}") from exc
    if not isinstance(value, expected_type):
        raise ValueError(f"event #{record.get('seq', '?')} has an unexpected {field} type")
    return value


def transitions(records, fields):
    changes = []
    previous = None
    for record in records:
        if event_kind(record) != "uistate":
            continue
        # Journal fields are strings: the recorder puts the UI object in
        # `state`. Keep compatibility with early flat UIState records too.
        state = {**record, **(structured_field(record, "state", dict) or {})}
        values = {key: state[key] for key in fields if key in state}
        if values and values != previous:
            changes.append({"seq": record.get("seq"), "sim_ms": record.get("sim_ms"), **values})
            previous = values
    return changes


def input_counts(records):
    counts = Counter()
    for record in records:
        if not any(field in record for field in ("keys_pressed", "keys_released", "buttons", "wheel")):
            counts[str(record.get("key", record.get("button", "unknown"))) + ":" + str(record.get("phase", "?"))] += 1
            continue
        # Vec<String>'s debug representation is a JSON array for these ASCII
        # key/button names. Parse it as data; never evaluate journal content.
        for field, phase in (("keys_pressed", "press"), ("keys_released", "release")):
            for key in structured_field(record, field, list) or []:
                counts[f"{key}:{phase}"] += 1
        for button in structured_field(record, "buttons", list) or []:
            name, separator, phase = str(button).rpartition(":")
            if not separator:
                counts[f"{button}:unknown"] += 1
            else:
                counts[f"{name}:{dict(pressed='press', released='release').get(phase, phase)}"] += 1
        wheel = record.get("wheel")
        if wheel is not None:
            if isinstance(wheel, str):
                # Rust's '(x, y)' tuple format contains numeric values only.
                wheel = wheel.strip().removeprefix("(").removesuffix(")").split(",")
            if not isinstance(wheel, (list, tuple)) or len(wheel) != 2:
                raise ValueError(f"event #{record.get('seq', '?')} has invalid wheel data")
            try:
                x, y = (float(value) for value in wheel)
            except (TypeError, ValueError) as exc:
                raise ValueError(f"event #{record.get('seq', '?')} has invalid wheel data") from exc
            if x:
                counts["wheel:right" if x > 0 else "wheel:left"] += 1
            if y:
                counts["wheel:up" if y > 0 else "wheel:down"] += 1
    return dict(counts)


def summary(directory):
    records, partial = journal(directory)
    kinds = Counter(str(record.get("kind", "unknown")) for record in records)
    inputs = [r for r in records if event_kind(r) == "input"]
    widgets = [r for r in records if event_kind(r) == "widget"]
    commands = [r for r in records if event_kind(r) == "command"]
    rejections = [r for r in commands if r.get("accepted") in (False, "false") or r.get("outcome_status") == "error"]
    return {"session_dir": str(directory), "current": current_state(directory),
            "replay": replay_metadata(directory), "events": len(records),
            "events_partial_trailing_line": partial, "event_kinds": dict(kinds),
            "selections": transitions(records, ("selection", "verb", "draft", "tab")),
            "speed_changes": transitions(records, ("paused", "speed")),
            "inputs": inputs, "widgets": widgets, "commands": commands,
            "rejections": rejections,
            "input_counts": input_counts(inputs),
            "widget_counts": dict(Counter(str(r.get("label", "unknown")) for r in widgets))}


def report_text(report, limit):
    replay = report["replay"]
    lines = [f"Playtest: {report['session_dir']}",
             f"Replay: {replay['status']}; hashes not checked (use playtest_replay)",
             f"Events: {report['events']}; partial event tail={report['events_partial_trailing_line']}; partial replay tail={replay['partial_trailing_line']}",
             ui_text(report["current"]),
             f"Commands: {len(report['commands'])} journaled, {replay['commands']} replay inputs, {len(report['rejections'])} rejected",
             f"Input counts: {json_text(report['input_counts'])}",
             f"Widget counts: {json_text(report['widget_counts'])}"]
    for title, field in (("Selection and context changes", "selections"),
                         ("Pause and speed changes", "speed_changes"),
                         ("Recent raw inputs", "inputs"),
                         ("Recent widget attempts (enabled/help state included)", "widgets"),
                         ("Commands and outcomes", "commands"),
                         ("Rejected commands", "rejections")):
        entries = report[field]
        lines.append(f"\n{title} ({len(entries)}):")
        if not entries:
            lines.append("  none recorded")
        if len(entries) > limit:
            lines.append(f"  showing last {limit}")
        for entry in entries[-limit:]:
            lines.append("  " + (short_event(entry) if "kind" in entry else json_text(entry)))
    state = report["current"] or {}
    if state.get("logger_error"):
        lines.append(f"\nLogger error: {state['logger_error']}")
    return "\n".join(lines)


def request(directory, action):
    if action not in ACTIONS:
        raise ValueError(f"diagnostic action must be one of {', '.join(ACTIONS)}")
    # Require an initialized journal, so a mistyped path cannot silently create
    # a diagnostic queue in an unrelated directory.
    metadata = replay_metadata(directory)
    if "scenario" not in metadata:
        raise ValueError("session replay header is not complete yet")
    if metadata["complete"]:
        raise ValueError("session already has a clean end; diagnostic requests need a running client")
    request_id = uuid.uuid4().hex
    destination = directory / "request.txt"
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(mode="w", encoding="ascii", dir=directory,
                                         prefix=f".request-{request_id}-", suffix=".tmp", delete=False) as output:
            temporary = Path(output.name)
            output.write(f"{request_id} {action}\n")
            output.flush()
            os.fsync(output.fileno())
        # A hard link publishes the complete file atomically without replacing
        # another caller's outstanding request (or following a symlink).
        try:
            os.link(temporary, destination)
        except FileExistsError as exc:
            raise ValueError("request.txt already exists; wait for the client to consume the pending request") from exc
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
    return {"id": request_id, "action": action, "status": "queued",
            "request_path": str(destination)}


def positive_number(text):
    value = int(text)
    if value < 1:
        raise argparse.ArgumentTypeError("must be at least 1")
    return value


def parser():
    result = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = result.add_subparsers(dest="command", required=True)
    for name, help_text in (("status", "Show current UI state, replay framing, and logger health"),
                            ("tail", "Read recent complete events (no persistent listener)"),
                            ("summary", "Summarize selection, speed, input, widgets, commands, and rejections"),
                            ("request", "Queue snapshot, screenshot, or checkpoint only")):
        command_parser = commands.add_parser(name, help=help_text)
        command_parser.add_argument("session_dir", type=Path)
        command_parser.add_argument("--json", action="store_true", help="Print machine-readable JSON (JSONL for tail)")
        if name == "tail":
            command_parser.add_argument("--lines", "-n", type=positive_number, default=20)
        if name == "summary":
            command_parser.add_argument("--limit", type=positive_number, default=20, help="Maximum entries per human report section")
        if name == "request":
            command_parser.add_argument("action", choices=ACTIONS)
    return result


def main(argv=None):
    args = parser().parse_args(argv)
    directory = args.session_dir.expanduser().resolve()
    try:
        if not directory.is_dir():
            raise ValueError(f"session directory does not exist: {directory}")
        if args.command == "status":
            result = status(directory)
            if args.json:
                print(json_text(result))
            else:
                print(f"Playtest: {directory}\n{ui_text(result['current'])}")
                print(f"Replay: {result['replay']['status']}; hashes not checked (use playtest_replay)")
                print(f"Events: {result['events']}; partial event tail={result['events_partial_trailing_line']}; pending request={result['pending_request']}")
                print(f"Logger error: {(result['current'] or {}).get('logger_error') or 'none reported'}")
        elif args.command == "tail":
            records, partial = journal(directory)
            for record in records[-args.lines:]:
                print(json_text(record) if args.json else short_event(record))
            if partial:
                print("events.jsonl: ignored incomplete trailing record", file=sys.stderr)
        elif args.command == "summary":
            result = summary(directory)
            print(json_text(result) if args.json else report_text(result, args.limit))
        else:
            result = request(directory, args.action)
            print(json_text(result) if args.json else f"Queued {result['action']} request {result['id']} at {result['request_path']}")
    except (OSError, ValueError, UnicodeError) as exc:
        print(json_text({"status": "error", "error": str(exc)}), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
