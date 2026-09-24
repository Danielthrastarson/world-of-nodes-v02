#!/usr/bin/env python3
"""Exercise the local journal through actual X11 window input and read requests.

Run after building both client binaries, on an isolated X display:
  xvfb-run -a -s '-screen 0 1600x1000x24' python3 scripts/playtest_journal_smoke.py /tmp/new-smoke-dir

The output directory must not exist. This harness never sends gameplay input
through the journal protocol and never edits simulation state. All gameplay
input is delivered by XTest to the client window. Python stdlib, X11 and XTest
are the only dependencies. Existing research captures are not modified.
"""
import argparse
import ctypes as c
import json
import os
from pathlib import Path
import re
import subprocess
import time


ROOT = Path(__file__).resolve().parents[1]


def wait_for(predicate, label, timeout=10):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        result = predicate()
        if result:
            return result
        time.sleep(0.05)
    raise AssertionError(f"Timed out waiting for {label}")


def read_json(path):
    try:
        return json.loads(path.read_text())
    except FileNotFoundError:
        return None


class ClientMessageData(c.Union):
    _fields_ = [("b", c.c_char * 20), ("s", c.c_short * 10), ("l", c.c_long * 5)]


class ClientMessage(c.Structure):
    _fields_ = [
        ("type", c.c_int), ("serial", c.c_ulong), ("send_event", c.c_int),
        ("display", c.c_void_p), ("window", c.c_ulong),
        ("message_type", c.c_ulong), ("format", c.c_int),
        ("data", ClientMessageData),
    ]


class XEvent(c.Union):
    _fields_ = [("client", ClientMessage), ("pad", c.c_long * 24)]


class WindowAttributes(c.Structure):
    _fields_ = [
        ("x", c.c_int), ("y", c.c_int), ("width", c.c_int), ("height", c.c_int),
        ("border_width", c.c_int), ("depth", c.c_int), ("visual", c.c_void_p),
        ("root", c.c_ulong), ("window_class", c.c_int), ("bit_gravity", c.c_int),
        ("win_gravity", c.c_int), ("backing_store", c.c_int),
        ("backing_planes", c.c_ulong), ("backing_pixel", c.c_ulong),
        ("save_under", c.c_int), ("colormap", c.c_ulong), ("map_installed", c.c_int),
        ("map_state", c.c_int), ("all_event_masks", c.c_long),
        ("your_event_mask", c.c_long), ("do_not_propagate_mask", c.c_long),
        ("override_redirect", c.c_int), ("screen", c.c_void_p),
    ]


class WindowInput:
    def __init__(self):
        self.x = c.CDLL("libX11.so.6")
        self.t = c.CDLL("libXtst.so.6")
        bindings = {
            "XOpenDisplay": ([c.c_char_p], c.c_void_p),
            "XDefaultRootWindow": ([c.c_void_p], c.c_ulong),
            "XQueryTree": ([c.c_void_p, c.c_ulong, c.POINTER(c.c_ulong),
                            c.POINTER(c.c_ulong), c.POINTER(c.POINTER(c.c_ulong)),
                            c.POINTER(c.c_uint)], c.c_int),
            "XFetchName": ([c.c_void_p, c.c_ulong, c.POINTER(c.c_void_p)], c.c_int),
            "XFree": ([c.c_void_p], c.c_int),
            "XStringToKeysym": ([c.c_char_p], c.c_ulong),
            "XKeysymToKeycode": ([c.c_void_p, c.c_ulong], c.c_uint),
            "XSetInputFocus": ([c.c_void_p, c.c_ulong, c.c_int, c.c_ulong], c.c_int),
            "XGetWindowAttributes": ([c.c_void_p, c.c_ulong,
                                      c.POINTER(WindowAttributes)], c.c_int),
            "XRaiseWindow": ([c.c_void_p, c.c_ulong], c.c_int),
            "XInternAtom": ([c.c_void_p, c.c_char_p, c.c_int], c.c_ulong),
            "XSendEvent": ([c.c_void_p, c.c_ulong, c.c_int, c.c_long,
                            c.POINTER(XEvent)], c.c_int),
            "XFlush": ([c.c_void_p], c.c_int),
            "XCloseDisplay": ([c.c_void_p], c.c_int),
        }
        for name, (args, result) in bindings.items():
            fn = getattr(self.x, name)
            fn.argtypes, fn.restype = args, result
        self.t.XTestFakeKeyEvent.argtypes = [c.c_void_p, c.c_uint, c.c_int, c.c_ulong]
        self.t.XTestFakeButtonEvent.argtypes = [c.c_void_p, c.c_uint, c.c_int, c.c_ulong]
        self.t.XTestFakeMotionEvent.argtypes = [c.c_void_p, c.c_int, c.c_int, c.c_int, c.c_ulong]
        self.display = self.x.XOpenDisplay(None)
        assert self.display, "X display unavailable; run this harness inside xvfb-run"
        self.root = self.x.XDefaultRootWindow(self.display)
        self.events = []

    def find_client(self):
        pending = [self.root]
        while pending:
            window = pending.pop()
            name = c.c_void_p()
            if self.x.XFetchName(self.display, window, c.byref(name)) and name.value:
                title = c.string_at(name.value).decode(errors="replace")
                self.x.XFree(name)
                if title.startswith("World of Nodes"):
                    return window
            root, parent, count = c.c_ulong(), c.c_ulong(), c.c_uint()
            children = c.POINTER(c.c_ulong)()
            if self.x.XQueryTree(self.display, window, c.byref(root), c.byref(parent),
                                 c.byref(children), c.byref(count)):
                pending.extend(children[i] for i in range(count.value))
                if children:
                    self.x.XFree(children)
        return None

    def focus(self, window):
        # The title exists before the native window is mapped. Asking X11 to
        # focus an unmapped window raises BadMatch, so wait on server state.
        def viewable():
            attributes = WindowAttributes()
            return (self.x.XGetWindowAttributes(self.display, window, c.byref(attributes))
                    and attributes.map_state == 2)  # IsViewable
        wait_for(viewable, "viewable client window")
        self.x.XRaiseWindow(self.display, window)
        self.x.XSetInputFocus(self.display, window, 2, 0)
        self.x.XFlush(self.display)
        time.sleep(0.25)

    def key(self, name):
        code = self.x.XKeysymToKeycode(self.display, self.x.XStringToKeysym(name.encode()))
        assert code, name
        for state in (1, 0):
            self.t.XTestFakeKeyEvent(self.display, code, state, 0)
            self.x.XFlush(self.display)
            time.sleep(0.09)
        self.events.append({"key": name})

    def move(self, x, y):
        self.t.XTestFakeMotionEvent(self.display, -1, x, y, 0)
        self.x.XFlush(self.display)
        time.sleep(0.10)

    def click(self, x, y, button=1):
        self.move(x, y)
        for state in (1, 0):
            self.t.XTestFakeButtonEvent(self.display, button, state, 0)
            self.x.XFlush(self.display)
            time.sleep(0.09)
        self.events.append({"click": [x, y, button]})

    def close_window(self, window):
        event = XEvent()
        event.client.type = 33  # ClientMessage
        event.client.send_event = 1
        event.client.display = self.display
        event.client.window = window
        event.client.message_type = self.x.XInternAtom(self.display, b"WM_PROTOCOLS", 0)
        event.client.format = 32
        event.client.data.l[0] = self.x.XInternAtom(self.display, b"WM_DELETE_WINDOW", 0)
        event.client.data.l[1] = 0  # CurrentTime
        assert self.x.XSendEvent(self.display, window, 0, 0, c.byref(event))
        self.x.XFlush(self.display)
        self.events.append({"window": "WM_DELETE_WINDOW"})

    def close(self):
        self.x.XCloseDisplay(self.display)


def evidence(input_device, path):
    previous = path.stat().st_mtime_ns if path.exists() else None
    input_device.key("F12")
    wait_for(lambda: path.exists() and path.stat().st_mtime_ns != previous, "F12 evidence")
    content = path.read_text()
    assert "replay_equal=true" in content, content
    return content


def evidence_field(content, name):
    match = re.search(rf"^{re.escape(name)}=(.*)$", content, re.M)
    assert match, f"Missing evidence field {name}"
    return match.group(1)


def exercise_paused_controls(input_device):
    # Help and unavailable actions cannot dispatch commands.
    input_device.key("F1")
    for key in ("w", "1", "Return"):
        input_device.key(key)
    input_device.key("Escape")
    input_device.key("e")
    input_device.key("1")
    input_device.key("q")

    # Both attempts are at time zero. Choosing this same node for Withdraw is
    # permitted by the draft UI and rejected by authoritative Session dispatch.
    input_device.click(382, 349)  # Unit 0
    input_device.key("w")
    input_device.key("4")
    input_device.click(395, 435)  # Hearth, the commander's current node
    input_device.key("Return")
    input_device.key("w")
    input_device.key("3")  # Hold here
    input_device.key("Return")

    # Speed changes while paused, selection, camera and debug remain UI-only.
    input_device.click(860, 53)  # 4x
    input_device.click(910, 53)  # 16x
    input_device.click(800, 53)  # 1x
    input_device.click(795, 435)
    input_device.click(600, 500, 4)  # wheel zoom
    input_device.key("Right")
    input_device.key("Down")
    input_device.key("F2")


def journal_events(directory, final=False):
    path = directory / "events.jsonl"
    if not path.exists():
        return []
    raw = path.read_text()
    if final:
        assert raw.endswith("\n"), "Normal exit left a partial JSONL record"
    elif not raw.endswith("\n"):
        raw = raw.rsplit("\n", 1)[0] if "\n" in raw else ""
    return [json.loads(line) for line in raw.splitlines()]


def request(directory, contents, accepted=True):
    previous_seq = int(journal_events(directory)[-1]["seq"])
    pending = directory / "request.txt"
    assert not pending.exists(), "Previous request was not consumed"
    temporary = directory / "harness-request.tmp"
    temporary.write_bytes(contents.encode())
    temporary.replace(pending)
    wanted = "request_completed" if accepted else "request_rejected"

    def completion():
        return next((event for event in journal_events(directory)
                     if int(event["seq"]) > previous_seq and event["kind"] == wanted), None)

    response = wait_for(completion, f"{wanted}: {contents[:60]}")
    assert not pending.exists(), "Acknowledged request is still pending"
    if accepted:
        request_id, action = contents.split()
        assert response["request_id"] == request_id, response
        assert response["action"] == action, response
        path = Path(response["path"])
        if not path.is_absolute():
            path = directory / path
        assert path.resolve().parent == directory.resolve(), response
        assert path.is_file(), response
        return path
    return response


def command_rows(directory):
    return [line.split("\t") for line in (directory / "replay.tsv").read_text().splitlines()
            if line.startswith("C\t")]


def check_read_requests(directory):
    before = read_json(request(directory, "before checkpoint"))
    assert int(before["sim_ms"]) == 0, before
    assert int(before["command_count"]) == 2, before
    snapshot = read_json(request(directory, "debug-view snapshot"))
    assert snapshot["knowledge_mode"] == "Play", snapshot
    assert snapshot["view_mode"] == "Debug", snapshot
    assert snapshot["paused"] in (True, "true"), snapshot
    assert {"nodes", "orders", "reports", "matters"} <= snapshot["known"].keys(), snapshot
    screenshot = request(directory, "own-window screenshot")
    assert screenshot.read_bytes().startswith(b"\x89PNG\r\n\x1a\n"), screenshot

    # Unsupported gameplay commands, malformed syntax, path-like IDs, excessive
    # requests and repeated IDs all receive rejection without dispatching input.
    rejections = [
        request(directory, "attempt-key key Space", accepted=False),
        request(directory, "attempt-order commission", accepted=False),
        request(directory, "../escape screenshot", accepted=False),
        request(directory, "missing-action", accepted=False),
        request(directory, "x" * 257, accepted=False),
        request(directory, "debug-view snapshot", accepted=False),
    ]
    assert all(event.get("reason") for event in rejections), rejections

    # Read through more than two publication intervals. A reader never retries
    # malformed JSON: atomic replacement must expose a complete document.
    deadline = time.monotonic() + 1.2
    frames = set()
    while time.monotonic() < deadline:
        status = read_json(directory / "current.json")
        assert status and status["knowledge_mode"] == "Play", status
        assert status["view_mode"] == "Debug", status
        assert status["paused"] in (True, "true"), status
        assert int(status["sim_ms"]) == 0, status
        assert not status["logger_error"], status
        frames.add(int(status["frame"]))
        time.sleep(0.02)
    assert len(frames) >= 2, f"current.json did not refresh: frames={frames}"

    after = read_json(request(directory, "after checkpoint"))
    for field in ("sim_ms", "state_hash", "event_hash", "command_count"):
        assert before[field] == after[field], (field, before, after)
    assert command_rows(directory) == [
        ["C", "0", "unit", "Withdraw", "1"],
        ["C", "0", "unit", "Hold", "1"],
    ], command_rows(directory)
    return {"requests_rejected": len(rejections), "status_frames_observed": len(frames)}


def run_case(args, device, enabled):
    case = args.output / ("journal-on" if enabled else "journal-off")
    case.mkdir()
    journal = case / "session"
    evidence_path = case / "evidence.txt"
    command = [str(args.client.resolve()), "--evidence", str(evidence_path)]
    if enabled:
        command += ["--journal", str(journal)]
    protocol = {}
    with (case / "client.log").open("w") as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        try:
            window = wait_for(device.find_client, "client window")
            device.focus(window)
            assert process.poll() is None, (case / "client.log").read_text()
            if enabled:
                wait_for(lambda: read_json(journal / "current.json"), "initial current.json")
            exercise_paused_controls(device)
            if enabled:
                protocol = check_read_requests(journal)
                live = subprocess.run([str(args.replay.resolve()), str(journal)],
                                      text=True, capture_output=True, timeout=30)
                (case / "live-replay.stdout.json").write_text(live.stdout)
                assert live.returncode == 2, (live.stdout, live.stderr)
                live_result = json.loads(live.stdout)
                assert live_result["status"] == "verified_prefix", live_result
                assert live_result["complete"] is False, live_result
                assert live_result["commands"] == 2, live_result
                assert live_result["rejected_commands"] == 1, live_result
            device.key("F2")  # Return to Play before the final checkpoint.
            if not enabled:
                content = evidence(device, evidence_path)
                assert content.count("CommandRecord {") == 2, content
                assert "withdrawal needs a different known destination" in content, content
                assert "verb: Hold" in content, content
                result = {"sim_ms": int(evidence_field(content, "sim_ms")),
                          "state_hash": evidence_field(content, "state_hash"),
                          "event_hash": evidence_field(content, "event_hash"),
                          "commands": content.count("CommandRecord {")}
            device.close_window(window)
            assert process.wait(timeout=15) == 0, (case / "client.log").read_text()
        finally:
            # Cleanup is only for a failed assertion or timeout. A passing case
            # must have used the native window-close protocol above.
            if process.poll() is None:
                process.terminate()
                process.wait(timeout=10)
    if not enabled:
        return result

    assert not evidence_path.exists(), "Journal-on smoke must not depend on F12 exports"
    status = read_json(journal / "status.json")
    assert status["running"] in (False, "false"), status
    assert status["clean_exit"] in (True, "true"), status
    events = journal_events(journal, final=True)
    assert [int(event["seq"]) for event in events] == list(range(1, len(events) + 1))
    assert events[-1]["kind"] == "session_end", events[-1]
    attempts = [event for event in events if event["kind"] == "command"]
    assert [event["accepted"] for event in attempts] == ["false", "true"], attempts
    assert "withdrawal needs a different known destination" in attempts[0]["error"], attempts
    lines = (journal / "replay.tsv").read_text().splitlines()
    assert lines[-1] == "E\t0", lines[-2:]
    assert lines[-2].startswith("H\t0\t"), lines[-2:]
    replay = subprocess.run([str(args.replay.resolve()), str(journal)], text=True,
                            capture_output=True, timeout=30)
    (case / "replay.stdout.json").write_text(replay.stdout)
    (case / "replay.stderr.txt").write_text(replay.stderr)
    assert replay.returncode == 0, (replay.stdout, replay.stderr)
    result = json.loads(replay.stdout)
    assert result["status"] == "verified_complete", result
    assert result["complete"] is True, result
    assert result["rejected_commands"] == 1, result
    final = read_json(journal / "checkpoint.json")
    for field in ("sim_ms", "state_hash", "event_hash"):
        assert str(final[field]) == str(result[field]), (final, result)
    result.update(protocol)
    result["event_rows"] = len(events)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--client", type=Path, default=ROOT / "client/target/release/world-of-nodes-client")
    parser.add_argument("--replay", type=Path, default=ROOT / "client/target/release/playtest_replay")
    args = parser.parse_args()
    args.output = args.output.resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    assert args.client.is_file(), f"Missing client binary: {args.client}"
    assert args.replay.is_file(), f"Missing replay binary: {args.replay}"
    device = WindowInput()
    try:
        off = run_case(args, device, enabled=False)
        on = run_case(args, device, enabled=True)
        assert off["sim_ms"] == on["sim_ms"] == 0, (off, on)
        assert off["state_hash"] == on["state_hash"], (off, on)
        assert off["event_hash"] == on["event_hash"], (off, on)
        assert off["commands"] == on["commands"] == 2, (off, on)
        result = {"result": "PASS", "display": os.environ.get("DISPLAY"),
                  "journal_off": off, "journal_on": on,
                  "inputs": device.events}
        (args.output / "result.json").write_text(json.dumps(result, indent=2) + "\n")
        print(f"PASS: actual window input, read-only requests, native close, replay, and paired hashes ({args.output})")
    finally:
        device.close()


if __name__ == "__main__":
    main()
