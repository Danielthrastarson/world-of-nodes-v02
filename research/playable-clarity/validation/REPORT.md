# P4.1 headless regression boundary

Baseline: clean **`0f8c18bc9b16f47dbf75276f577e20f850518f36`**, before the
presentation-clarity changes. This validation changes no root Rust or client
source. Its scope is unchanged authoritative behavior and preservation of prior
research, not a new simulation experiment or a human usability claim.

| Check | Result |
|---|---|
| Root source and dependencies | **PASS:** all 113 files under `src/`, `Cargo.toml` and `Cargo.lock` remain byte-identical; no additional root source files |
| Full headless suite | **PASS:** 356 library tests and 13 integration tests; binary/doc tests also pass |
| Playable scenario regression | **PASS:** 48 runs, with 48 additional exact dated-input replays; 193 artifact comparisons |
| F13B historical command chain | **PASS:** all 600 runs and 3,000 native arm artifacts, plus the aggregate CSV, match exactly |
| Prior research | **PASS:** all 21,439 previously existing research files remain unchanged, including P0–P4 evidence |

The 48 scenario runs repeat the committed three seeds, two initial histories and
eight input transcripts from `research/playable/probes/scenarios-3seeds`.
All typed transcripts, hourly timelines, event streams and final-state diagnostics
match. The final CSV matches in every ordered field except the two explicitly
registered wall-time values, `run_wall_ns` and `replay_wall_ns`. State hashes,
event hashes and replay outcomes are included in the exact comparison.

Rejected Remote military inputs remain rejected inputs; they are not relabeled
executed orders. The pre-existing militia remains an explicit initial condition.
No claim is made about an improved autonomous policy, successful military
protection in every arm, or improved welfare.

F13B comparison has no semantic exceptions, including its timing, parameters,
physical outcomes, institution diagnostics, causal traces, state hashes and event
hashes. These runs are historical replays, not 600 new independent observations.

## Existing F11C exception remains unchanged

The baseline already contains the contact-clock correction reviewed in
`research/playable/regression/REPORT.md`. Its **60 of 1,440 strict F11C failures**
remain preserved, along with the original 217-control verifier's failure archive.
They are not new P4.1 failures and are not renamed PASS. The earlier exact
169-control result and bounded causal review likewise remain historical evidence.

The complete F11C panel was not redundantly rerun here: all authoritative root
source and dependencies are byte-identical to the accepted baseline, and fresh
headless tests, playable transcripts and full F13B replays validate the boundary.
This is an explicit scope decision, not a claim that the old F11C hashes match.

## Provenance and retention

`baseline.json` registers the revision and counts. `root-source-baseline.json`
and `prior-research-sha256.json` contain SHA-256 manifests. `check_integrity.py`
can recheck the boundary after further client-only edits; its dated outputs
record both file hashes and the root-source Git diff against the baseline.

`replay.py` records the actual executable hashes and comparison rules, runs into
fresh temporary directories, and retains complete failed output if any check
fails. Successful duplicate raw output is discarded only after comparison;
`*-retention.json` keeps every generated file's fingerprint. The original raw
research remains the reference. No prior artifact was overwritten.

Reproduce into a new validation directory or preserve these outputs before a new
run; the verifier refuses to overwrite its evidence:

```sh
cargo test
cargo build --release --bin playable_probe --bin command_chain_probe
python3 research/playable-clarity/validation/replay.py scenarios
python3 research/playable-clarity/validation/replay.py commands
python3 research/playable-clarity/validation/check_integrity.py integrity-final.json
```

The tests establish an unchanged headless foundation. Actual window interaction,
command discoverability, legibility and human comprehension require the separate
client evidence and are not established by these replay results.
