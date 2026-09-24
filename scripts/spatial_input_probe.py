#!/usr/bin/env python3
"""Actual mouse/key smoke for V0/V1 projected hit geometry and F9 map-only view.

Creates its own isolated Xvfb display. Never addresses a human playtest window.
No F12 dependency: native close flushes the opt-in journal, then replay verifies it.
"""

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

from playtest_journal_smoke import (
    ROOT, WindowInput, command_rows, journal_events, read_json, request, wait_for,
)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--inside-xvfb", action="store_true", help=argparse.SUPPRESS)
    parser.add_argument("--client", type=Path, default=ROOT / "client/target/release/world-of-nodes-client")
    parser.add_argument("--replay", type=Path, default=ROOT / "client/target/release/playtest_replay")
    args = parser.parse_args()
    if not args.inside_xvfb:
        env = dict(os.environ)
        env.pop("WAYLAND_DISPLAY", None)
        env["LIBGL_ALWAYS_SOFTWARE"] = "1"
        command = ["xvfb-run", "-a", "-s", "-screen 0 1440x900x24", sys.executable,
                   str(Path(__file__).resolve()), *sys.argv[1:], "--inside-xvfb"]
        return subprocess.run(command, env=env, check=False).returncode
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    session = out / "session"
    stages = []
    device = WindowInput()
    process = None
    result = {"result": "RUNNING", "isolated_display": os.environ.get("DISPLAY"), "stages": stages}

    def stage(name, predicate):
        def ready():
            state = read_json(session / "current.json")
            return state if state and predicate(state) else None
        current = wait_for(ready, name)
        assert current["paused"] is True and int(current["sim_ms"]) == 0, current
        assert current["knowledge_mode"] == "Play" and current["view_mode"] == "Play", current
        assert not current["logger_error"], current
        stages.append({"name": name, "state": current})
        (out / f"{name}.json").write_text(json.dumps(current, indent=2) + "\n")
        return current

    try:
        command = [str(args.client.resolve()), "--journal", str(session)]
        result["command"] = command
        with (out / "client.log").open("w") as log:
            process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
            window = wait_for(device.find_client, "client window")
            device.focus(window)
            stage("initial", lambda s: s["map_only"] is False)

            device.click(199, 495)  # Unit 0's projected label/standard, not a Person.
            stage("unit-selected", lambda s: s["selection"] == "Some(Unit(0))")
            request(session, "unit-selected screenshot")
            for key in ("w", "3", "Return"):
                device.key(key)
            stage("hold-issued", lambda s: "UNIT ORDER RECEIVED" in s["notice"])
            assert command_rows(session) == [["C", "0", "unit", "Hold", "1"]], command_rows(session)

            device.click(320, 493)  # Hearth: screen projection of authoritative node 1.
            stage("settlement-selected", lambda s: s["selection"] == "Some(Node(1))")
            device.key("w")
            device.key("1")
            device.click(599, 526)  # Curved route's crossing glyph, not the old straight-edge midpoint.
            stage("crossing-confirmation", lambda s: "Confirm(Commission" in s["draft"])
            request(session, "crossing-confirmation screenshot")
            device.key("Return")
            stage("commission-issued", lambda s: "ORDER 0 ISSUED" in s["notice"])
            expected = [["C", "0", "unit", "Hold", "1"], ["C", "0", "commission", "0", "0"]]
            assert command_rows(session) == expected, command_rows(session)

            before = read_json(request(session, "before-view checkpoint"))
            device.key("F9")
            stage("map-only", lambda s: s["map_only"] is True)
            request(session, "map-only screenshot")
            device.key("F9")
            stage("interface-restored", lambda s: s["map_only"] is False)
            camera_before = read_json(session / "current.json")["camera"]
            device.click(600, 500, 4)  # Wheel zoom.
            device.key("Right")
            device.key("Down")
            stage("camera-changed", lambda s: s["camera"] != camera_before)
            after = read_json(request(session, "after-view checkpoint"))
            for field in ("sim_ms", "state_hash", "event_hash", "command_count"):
                assert before[field] == after[field], (field, before, after)
            assert command_rows(session) == expected, command_rows(session)
            result["view_changes_preserved_world"] = True
            device.close_window(window)
            assert process.wait(timeout=15) == 0, (out / "client.log").read_text()
        status = read_json(session / "status.json")
        assert status["clean_exit"] is True and status["running"] is False, status
        events = journal_events(session, final=True)
        assert events[-1]["kind"] == "session_end", events[-1]
        commands = [e for e in events if e["kind"] == "command"]
        assert len(commands) == 2 and all(e["accepted"] == "true" for e in commands), commands
        replay = subprocess.run([str(args.replay.resolve()), str(session)], capture_output=True,
                                text=True, timeout=30, check=False)
        (out / "replay.json").write_text(replay.stdout)
        (out / "replay.stderr").write_text(replay.stderr)
        assert replay.returncode == 0, (replay.stdout, replay.stderr)
        result["replay"] = json.loads(replay.stdout)
        assert result["replay"]["status"] == "verified_complete", result["replay"]
        assert result["replay"]["commands"] == 2 and result["replay"]["rejected_commands"] == 0, result["replay"]
        result["result"] = "PASS"
        print(f"PASS: projected hits, Hold + commission, F9, camera, native close and replay ({out})")
        return 0
    except Exception as error:
        result["result"] = "FAIL"
        result["error"] = str(error)
        raise
    finally:
        if process and process.poll() is None:
            process.terminate()
            process.wait(timeout=10)
        result["inputs"] = device.events
        (out / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        device.close()


if __name__ == "__main__":
    raise SystemExit(main())
