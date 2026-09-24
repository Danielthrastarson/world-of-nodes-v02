#!/usr/bin/env python3
"""Capture V0/V1 presentation evidence without touching a human playtest window.

Every executable receives its own Xvfb display. The authoritative fixture options
are identical across frozen/current pairs; only current map-only cases hide UI.
Screenshots require human review: matching hashes establish mechanical stability,
not legibility. This script creates a fresh output directory and preserves failures.
"""

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time


ROOT = Path(__file__).resolve().parents[1]
CASES = (
    ("local-initial-play", (), False),
    ("local-commissioned-4h-play", ("--commission", "--hour", "4"), False),
    ("local-commissioned-4h-debug", ("--commission", "--hour", "4", "--debug"), False),
    ("remote-initial-play", ("--remote",), False),
    ("map-only-local-initial-play", (), True),
    ("map-only-local-commissioned-4h-debug", ("--commission", "--hour", "4", "--debug"), True),
    ("local-commissioned-24h-play", ("--commission", "--hour", "24"), False),
    ("map-only-local-commissioned-24h-debug", ("--commission", "--hour", "24", "--debug"), True),
)


def utc_now():
    return datetime.now(timezone.utc).isoformat()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def read_evidence(path):
    raw = path.read_text()
    values = dict(line.split("=", 1) for line in raw.splitlines() if "=" in line)
    for key in ("scenario", "seed", "sim_ms", "state_hash", "event_hash", "replay_equal"):
        if key not in values:
            raise ValueError(f"{path}: missing {key}")
    start = raw.index("commands=") + len("commands=")
    end = raw.find("\nui_selection=", start)
    values["commands"] = raw[start:end if end >= 0 else None].rstrip()
    values["known_orders"] = [line for line in raw.splitlines() if line.startswith("known_order=")]
    values["known_matters"] = [line for line in raw.splitlines() if line.startswith("known_matter=")]
    return values


def isolated_run(executable, arguments, directory, timeout):
    directory.mkdir()
    command = ["xvfb-run", "-a", "-s", "-screen 0 1440x900x24", str(executable), *arguments]
    started = time.monotonic()
    result = {"command": command, "started_at": utc_now(), "isolated_display": True}
    # Do not inherit a caller-selected graphics backend/display. xvfb-run sets DISPLAY.
    env = dict(os.environ)
    env.pop("WAYLAND_DISPLAY", None)
    env["LIBGL_ALWAYS_SOFTWARE"] = "1"
    try:
        with (directory / "client.log").open("w") as log:
            process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=log,
                                       stderr=subprocess.STDOUT, start_new_session=True)
            try:
                result["exit_code"] = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                result["timed_out"] = True
                # Only this probe's isolated process group, never a human window.
                os.killpg(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
                raise RuntimeError(f"client exceeded {timeout}s; see {directory / 'client.log'}") from None
        if result["exit_code"]:
            raise RuntimeError(f"client exited {result['exit_code']}; see {directory / 'client.log'}")
    finally:
        result["elapsed_seconds"] = time.monotonic() - started
        write_json(directory / "invocation.json", result)
    return result


def capture(executable, options, map_only, directory, seed, frames, timeout):
    arguments = [*options, "--seed", str(seed), "--frames", str(frames),
                 "--capture", str(directory / "world.png"),
                 "--evidence", str(directory / "evidence.txt")]
    if map_only:
        arguments.append("--map-only")
    result = isolated_run(executable, arguments, directory, timeout)
    png = directory / "world.png"
    if not png.is_file() or png.read_bytes()[:8] != b"\x89PNG\r\n\x1a\n":
        raise RuntimeError(f"missing or invalid PNG: {png}")
    result["evidence"] = read_evidence(directory / "evidence.txt")
    result["screenshot_sha256"] = digest(png)
    if result["evidence"]["replay_equal"] != "true":
        raise RuntimeError(f"independent replay failed: {directory}")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="new evidence directory; existing paths are refused")
    parser.add_argument("--before", type=Path, default=ROOT / "client/target/p42-frozen-243d07b/world-of-nodes-client")
    parser.add_argument("--after", type=Path, default=ROOT / "client/target/release/world-of-nodes-client")
    parser.add_argument("--render-probe", type=Path, help="optional existing 1,000-Person render-probe executable")
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--frames", type=int, default=5)
    parser.add_argument("--timeout", type=float, default=120)
    args = parser.parse_args()
    if args.frames < 3:
        parser.error("--frames must be at least 3 for the client capture lifecycle")
    if shutil.which("xvfb-run") is None:
        parser.error("xvfb-run is required; this probe never uses a human display")
    paths = [args.before, args.after, *([args.render_probe] if args.render_probe else [])]
    for path in paths:
        if not path.is_file() or not os.access(path, os.X_OK):
            parser.error(f"executable missing: {path}")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    report = {
        "schema": 1,
        "started_at": utc_now(),
        "purpose": "presentation capture with paired physical/event hash and independent replay checks",
        "visual_gate": "PENDING_HUMAN_REVIEW",
        "seed": args.seed,
        "frames": args.frames,
        "rendering_environment": "isolated Xvfb; software GL; no human-window input",
        "source_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "source_worktree": subprocess.check_output(["git", "status", "--short"], cwd=ROOT, text=True),
        "binaries": {str(path.resolve()): digest(path) for path in paths},
        "cases": [],
        "result": "RUNNING",
    }
    write_json(output / "summary.json", report)
    try:
        for name, options, map_only in CASES:
            case = {"name": name, "fixture_options": options, "new_map_only": map_only,
                    "frozen_map_only": False}
            report["cases"].append(case)
            case_dir = output / name
            case_dir.mkdir()
            before = capture(args.before.resolve(), options, False, case_dir / "before", args.seed, args.frames, args.timeout)
            after = capture(args.after.resolve(), options, map_only, case_dir / "after", args.seed, args.frames, args.timeout)
            compared = ("scenario", "seed", "sim_ms", "state_hash", "event_hash", "commands",
                        "view_mode", "visible_or_remembered_people", "known_orders", "known_matters")
            case["comparisons"] = {key: before["evidence"].get(key) == after["evidence"].get(key)
                                   for key in compared}
            case["before"] = before
            case["after"] = after
            case["result"] = "PASS" if all(case["comparisons"].values()) else "FAIL"
            write_json(output / "summary.json", report)
            if case["result"] != "PASS":
                mismatches = [key for key, value in case["comparisons"].items() if not value]
                raise RuntimeError(f"{name}: mismatch in {', '.join(mismatches)}")
            print(f"{name}: physical hashes, commands, knowledge summary and independent replays PASS", flush=True)
        if args.render_probe:
            probe_dir = output / "render-load"
            report["render_load"] = isolated_run(args.render_probe.resolve(), [str(probe_dir)], probe_dir, args.timeout)
            values = dict(line.split("=", 1) for line in (probe_dir / "1000-persons.txt").read_text().splitlines())
            report["render_load"]["measurements"] = values
            if values.get("actual_persons") != "1000" or values.get("state_unchanged") != "true":
                raise RuntimeError("1,000-Person render probe did not preserve authoritative state")
        report["result"] = "PASS_MECHANICAL_COMPARISONS"
    except Exception as error:
        report["result"] = "FAIL"
        report["error"] = str(error)
        raise
    finally:
        report["finished_at"] = utc_now()
        write_json(output / "summary.json", report)
    print(f"Evidence: {output}; visual legibility requires review, not hash equality.")


if __name__ == "__main__":
    main()
