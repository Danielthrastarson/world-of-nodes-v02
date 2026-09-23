# F12 — local reports and embodied information transport

**PASSED at the registered mechanism scope.** Immutable local evidence can cross space through a real Person, preserve its observation age, and change another Person's existing decisions. This establishes neither autonomous messenger recruitment nor ruler command authority. Historical defaults remain unchanged.

Implementation checkpoint: `4c84081dd62b837c2798d72398b32ad3491da465`, based on clean `2b00dd9`. [Specification](../../docs/INFORMATION_EXPERIMENT.md), [architecture](../../docs/INFORMATION_ARCHITECTURE.md), [execution plan](../../docs/INFORMATION_EXECUTION_PLAN.md), [raw panel/provenance](matrix/provenance.json), [regression validation](regression/validation.json), [reviewed results](analysis-reviewed/SUMMARY.md), [causal trace excerpts](analysis-reviewed/TRACE_EXCERPTS.md). The [reviewed analysis](analysis-reviewed/summary.json) records its own analyzer hash; no physical panel was rerun to change an interpretation.

## What changed

A Person may capture its current local NodeView as an immutable report and carry a copy in personal memory. Both sender and recipient must be alive, capable and present at the same physical node for handoff. Receipt records original author/time, immediate sender and receipt time. Newer report evidence can update spatial beliefs; older/equal reports do not overwrite newer direct knowledge. Direct observation clears active imported-belief provenance without deleting historical receipts.

A controlled initial delivery undertaking uses the common Person selector, personal route knowledge, ordinary travel, FIFO, physiology and survival priorities. It does not install a courier brain, track a remote recipient, send acknowledgements or convey authority. Oral handoff uses the existing instantaneous local institutional-action convention. Accepting truthful reports is an explicit experimental assumption; credibility/deception and B2 trust integration remain unimplemented.

Two defects were fixed before the final panel: a late barter arbitration step could overwrite the selected handoff, and manually cancelling an unavailable report undertaking could panic. Failed tests and intermediate logs remain in [development evidence](DEVELOPMENT.md). Neither repair changes information-disabled historical behavior.

## Experiments and scope

840 final runs: 14 fixtures × 2 arms × 30 paired seeds, ending at 24 simulated hours. Eleven live fixtures compare an explicitly assigned courier purpose with unassigned civilian behavior. Both hold the recipient's deliberation until 6h while physiology continues, then release normal autonomy. This difference includes transport effort and displaced behavior, not information alone. Three matched fixtures share the same real one-hour journey and differ only in legal local handoff. Immediate comparisons use identical personal RNG and physical state.

The worlds contain 2–10 Persons, fixed initial resources and one captured report. Repeated seeds measure this controlled setup, not independent worlds or demographic generality. No coefficients or fixtures were retuned after observing failure.

| Live fixture | Received /30 | Mean observation age at receipt | Explanation |
|---|---:|---:|---|
| Short route |30|2h|Courier physically completes the route and delivers.|
| Long route |0|undefined|Travel and recovery delay arrival; recipient has left when cancellation occurs.|
| Congestion |0|undefined|Earlier ordinary traffic delays courier; recipient has left.|
| Unknown route |30|10.161h|Carrier cannot plan the route and stays at source; recipient eventually reaches carrier. All reports are already superseded by direct evidence.|
| Fatigue |13|9h|Recovery delays movement; 13 local encounters occur and 17 miss the recipient. Received reports do not update newer direct beliefs.|
| Initially hungry |30|2.017h|Carried legal food relieves hunger before ordinary travel; this is not a no-food survival control.|
| Absent recipient |0|undefined|Fixed rendezvous absence cancels; no remote tracking.|
| Courier death |0|undefined|Ordinary death cancels all 30 undertakings; no delivery or remote knowledge inheritance.|
| Source depleted |30|2h|All 30 faithfully deliver belief in 8 available quanta after another Person has picked them up.|
| Established path enabled |30|1.8h|Existing path mechanics reduce travel time without changing capacity.|
| Same path control, effect disabled |30|2h|Same report and undertaking under ordinary traversal cost.|

Across 330 assigned live runs, 193 handoffs occur; only 150 update spatial beliefs. There are 107 recipient-absence cancellations and 30 death cancellations. A completed handoff is not necessarily useful new information or a successful courier journey.

## Information versus mechanics versus subsequent change

- **Matched fresh:** 30/30 immediate typed intentions change; 17/30 change immediate destination. The report is accurate at receipt: 8 legally available food quanta. In the preserved seed-zero trace another Person eats them one minute later, before the recipient arrives. This is world change after receipt.
- **Matched depleted:** 30/30 typed intentions change; 17/30 change immediate destination. The report says 8 legally available quanta, but the true value is already 0 at receipt in all 30. The goods still physically exist in another Person's custody/ownership. A resulting failed food trip is stale information, not wrong transfer arithmetic.
- **Matched spoiled:** 13/30 immediate intentions/destinations change. All 30 correctly infer zero edible quantity from the original expiry. The report also communicates source-node geography; reduced redundant exploration changes later movement. No food is made fresh by receipt.

Fresh/depleted arms have identical selected aggregate physical metrics at 24h, **not identical spatial execution**. In the seed-zero trace the recipient visits 0→1→0→2 after handoff versus 0→2→0→1 without it: equal travel time, different route order and final node. In 13 of the 30 immediate comparisons, the destination remains node 1 while purpose changes from exploration to food seeking. The spoiled report reduces recipient travel by 2h in all 30 pairs; even the 17 initially unchanged choices later avoid exploring the already reported node. The reviewed analysis separates typed-intention change, immediate destination change and aggregate outcomes.

Live research truth checkpoints are sampled at the first minute boundary at/after receipt, explicitly separate from Person knowledge. In this panel the sampled lag is 0. Original observation time remains unchanged. Positive believed availability with true legal availability 0 occurs in 30 depleted live receipts and 30 matched depleted receipts. Lack of a receipt has undefined age/error, not a zero-delay success.

## Physical costs and negative findings

Congested courier assignment increases total starvation exposure by 0.6 person-hours per run and courier queue time by 7.583h relative to the unassigned control, without any report delivered. Short-route assignment increases courier travel by 4.667h and courier consumption by 6 quanta; recipient consumption falls by 1.7 quanta on average. These are 24h consequences of the entire intervention, not a money value or pure communication cost. Long-route assignment adds 19 courier travel-hours and reduces its consumption by 6 quanta, without delivery.

The unknown-route treatment produces no difference in measured physical aggregates: handoff occurs only after the recipient arrives independently. More handoffs therefore do not establish better navigation, better nourishment or successful dispatch. No arm changes death counts relative to its control within 24h. This horizon does not establish long-run welfare or sustainability.

Full [per-arm statistics](analysis-reviewed/summary.json) and [paired data](analysis-reviewed/paired.csv) include nutrition, consumption, starvation, death, travel, queueing, sleep, cancellations and retained report copies. [Truth comparisons](analysis-reviewed/truth-comparisons.csv) retain the original and receipt timestamps. No scalar efficiency score is inferred.

## Validation and computational limits

- [304 tests pass](full-tests-final.log), including 20 focused information contracts; formatting and Clippy with warnings denied pass.
- 217 historical executions exactly reproduce: 169 earlier regression controls plus 48 F11C seed-zero arms. Outcomes, authoritative hashes and referenced traces match under the existing documented verifier exceptions (wall runtime and old debug-derived signatures). This is representative regression coverage, not a rerun of every historical seed or every enabled subsystem configuration.
- All 15,388 previous research files retain their SHA256 hashes. Original artifacts remain authoritative; successful replay copies are digested rather than duplicated.
- All 840 final runs pass kernel audits and report zero FIFO overtakes.
- The probe reports 27.206s summed per-run runtime on this host, maximum 167ms/run; those figures include research instrumentation and concurrent execution. Peak fixture population is 10; retained copies are at most 2 per run. This is not a scaling benchmark. Report forgetting, large-population memory, existing observation/combat scans and deliberation frequency remain explicit performance debt.

## Gate and next dependency

F12A local evidence/provenance, F12B embodied transport and F12C receipt-to-decision gates pass. Frozen-personal-input and irregular-time-partition tests pass; no hidden state is added to Person reasoning. Bodies, goods, ownership, institutions, starvation and default policy remain preserved.

Continue to an F13A **research-only integration readiness control** using an existing local steward commission. This is safe without inventing sovereign command powers. Full F13 remains open: the existing research unit-order API has no issuer authority check, and appointment currently conveys an office assignment rather than obedience to remote orders. Evidence transport must not silently establish those rights.

## Reproduce

From implementation checkpoint `4c84081`:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_information.py NEW_PANEL 30
python3 scripts/analyze_information.py NEW_PANEL NEW_ANALYSIS
python3 scripts/verify_information.py NEW_REGRESSION
```

Use the reviewed analyzer from the subsequent evidence commit for clarified route-versus-aggregate labels; its hash is stored in the reviewed summary. Every output directory is new. Development smoke runs are retained separately and excluded from the 840 final runs.
