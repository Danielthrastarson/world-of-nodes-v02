# F11C — physical raid and protection

F11C passes at the registered graph-local mechanism scope. F11A's previously open protective-benefit gate is now satisfied: voluntary assembly and physical contact prevent a real stock loss in a controlled case, with civilian costs recorded. Neither conclusion establishes that military service generally improves welfare.

Core source: `631c76e864dae6c77288901f027372e72add99ff`. [Registered laws and amendments](../../docs/RAID_EXPERIMENT.md), [panel provenance](matrix/provenance.json), [paired effects](analysis/paired.csv), [full numeric summary](analysis/summary.json), [contact reconstruction](trace-audit/checks.json).

## Experiment and physical model

The final panel contains **1,440 runs**:12 cases ×4 arms ×30 paired seeds, each ending at24h. Arms are raid/no defense, raid/voluntary defense, peace/no defense, peace/voluntary defense. Fixed starting cohorts distinguish the six prospective defenders, three raiders and additional civilians/dependent. No person is spawned by mobilization. Exact per-case configurations, initial inventories and disclosed route memories are in each `parameters.txt`.

The base world has200 communal food quanta, six prospective defenders at muster and three opposing Persons two nominal travel-hours from the stock. Each Person begins with24 nutrition-hours,4 fatigue-hours and6 personal food quanta. Hunger/congestion/household variants are documented initial conditions; paired arms retain identical physical inputs. A locally observed service request offers12h voluntary service; one local Protect order is issued at1h05m to members actually present. Raiders begin their seeded objective at2h. They know their objective and remembered edges, not hidden live resources or enemy positions. Opposing identities are controlled threat affiliations, recognized only in local contact.

Contact uses existing nodes, not invented continuous geometry. Each capable Person can perform one timed outgoing strike; several real opponents can target one Person. Base work15min, one injury per completed valid strike. Simultaneous due strikes use the same pre-damage state. Injury1 halves existing movement/work/contact progress; injury2 incapacitates; injury3 kills through ordinary Person death. There is no execution of incapacitated targets, healing, UnitHP, morale score or random casualty percentage. Healthy and wounded people retain ordinary hunger, fatigue, care and service-expiry constraints.

Theft is5min base work per quantum, limited by actual carrying space and a controlled10-quantum objective per raider. Start and completion require physical stock access without a capable opposing defender. Goods change custody while retaining Commons title and original age. Stolen food can be physically eaten; it does not disappear on settlement or become lawful private property. There is no conquest or automatic recovery from dead owners.

## Results

All quantities below are per run. Primary theft, hit, death, care and harvest counts were identical across the30 seeds in each arm. These repeats verify paired execution, not broad statistical robustness across threats or combat laws. Some queue outcomes vary with seed.

| Case | Food stolen, no defense → defense | Assembled | Defender deaths | Raider deaths |
|---|---:|---:|---:|---:|
| Local |30 →0|4|0|1|
| Under-strength |30 →30|1|1|0|
| Over-mobilization |30 →0|6|0|1|
| Late arrival |30 →30|4|0|0|
| Bottleneck |30 →0|4|0|2|
| Food-constrained |30 →30|0|0|0|
| Civilian congestion |24 →24|4|1|0|
| Paths disabled |30 →0|4|0|1|
| Paths enabled |30 →0|4|0|1|
| Constructed crossing |30 →0|4|0|2|
| Household |30 →0|4|0|1|
| Farming |30 →0|4|0|1|

Local causal trace:

- At0h, six Persons independently volunteer; four physically join the local muster at1min. Administrative demand does not create extra soldiers.
- At1h05m, those four receive Protect locally and subsequently traverse the existing edge.
- At4h, three raiders arrive at the defended stock. Actual Persons begin contact.
- First strikes finish at4h15m; final damaging contact occurs at4h45m. Twelve completed strikes produce bodily consequences.
- One defender remains incapacitated; two raiders remain incapacitated; one raider dies. No theft completes.
- Without defense, the same raid steals30 quanta. At24h the target stock contains134 quanta without defense and164 with defense:30 additional quanta remain there.

This is a real protective effect, not a safety score. It is not injury-free protection. The entire preserved trace appears in [causal-chains.json](trace-audit/causal-chains.json).

## Costs and failed protection

**Household:** care transfer falls6 →0 quanta; dependent starvation exposure rises10 →16 person-hours. Household separation rises21.983 →22.900 hours. This occurs even though theft is prevented. The peaceful mobilization control reproduces the care loss, identifying a cost of service rather than a consequence requiring combat. Care transfer is an existing atomic action, so there is no invented nonzero feeding-work duration; resource loss and separation are measured directly.

**Farming:** prospective defenders' farming time falls6.667 →2.222 person-hours. Their food consumption rises64 →68 quanta. Stock remaining rises142 →168, a26-quantum difference despite30 thefts prevented. Neither arm harvests within24h: this establishes foregone work, not a measured harvest reduction in this panel. The earlier F11A96 →0 production result remains unchanged. Peaceful mobilization produces the same farm-work reduction.

**Over-mobilization:** six members prevent exactly the same30 stolen quanta as four. Military-duty time rises29.733 →45.733 person-hours; military queue exposure rises0 →4 hours. There is no additional measured protection.

**Under-strength:** one defender dies after contact, while theft remains30. More stock remains than in its control because subsequent consumption changes; that is not evidence of theft prevention. Remaining inventory alone would misclassify this failure.

**Congestion:** four assemble, but common capacity prevents simultaneous protective presence. The first defender meets three opponents and dies; the remaining members do not prevent the24-quanta theft. Military queue exposure is12 hours. Additional civilians' mean queue exposure rises0.066 →10.304 person-hours. No starvation-hours occur in either congestion arm during this24h window; do not import the earlier F11B starvation effect into this experiment.

**Late/hungry:** distance produces zero contact and zero theft prevention despite four assembled members. With food-constrained prospective defenders, zero assemble. Neither failure is repaired.

In the local case, the fixed prospective-defender cohort consumes72 quanta in both arms, and no crop output is lost within the window. Thirty additional food quanta remain in the target stock, but defense causes permanent injury and death elsewhere. Food, labor, bodily injury and dependent deprivation are different quantities; there is no justified scalar showing that preserved food outweighs every cost.

The role-tagged military-food counter is zero in these initial-reserve/12h-service fixtures; later meals occur outside active service. The fixed-cohort consumption columns and per-person files prevent that timing fact from being mistaken for free soldier nourishment. Lower total consumption after combat is also partly explained by raider death/incapacity; it is not improved nourishment.

## Geography and scope limits

Paths advance first contact4.0 →3.8h through existing traversal physics, without changing theft prevention. Capacity-one approach traffic spreads the bottleneck contacts through8.278h, changing casualties: zero surviving incapacitated defenders instead of one, and two raider deaths instead of one. This is sequential arrival through an edge, not a fabricated narrow-front combat bonus.

The crossing case physically consumes one wood and one construction-hour using existing construction actions before autonomy resumes. Both paired arms receive the same disclosed preparation. It demonstrates shared infrastructure carrying a raid and defense; it does not demonstrate autonomous crossing investment or isolate crossing construction against a separate closed-edge arm.

The study does not validate tactics, weapon reach, medical realism, long-term recovery, casualty fairness under every target-selection convention, optimal force size or net social welfare. No healing means injury persists. Twenty-four hours cannot establish long-term household or productive consequences. Incapacitated members remain Persons and organizational members until normal termination; membership is not combat capability.

## Validation and preservation

- **284 tests pass**, including17 F11C tests; formatting and all-target Clippy with warnings denied pass.
- **169 historical executions** reproduce physical outcomes, authoritative state/event hashes and raw traces with combat disabled.
- **11,261 prior research files** remain byte-identical.
- All1,440 final runs pass accounting and record zero FIFO overtakes.
- Independent reconstruction checks all48 seed-zero arm traces: strike/theft locality, elapsed work and defenders' prior assembly.
- Static checks cover simultaneous hits, last-quantum contention, preserved food age/title, contact interruption, service expiry, injury-aware travel/work, incapacity, newborn body state, normal death and observed grief.
- Splitting time advancement preserves physical and event hashes. Reversed evaluation preserves physical hashes and semantic events; existing diagnostic service/formation log ordering changes event IDs/causal pointers. That pre-existing diagnostic distinction was not rewritten.

[Final replay validation](regression-final/validation.json) and [validation manifest](validation.json) identify the source and exceptions. Optional `combat: None` changes debug configuration schema, not old authoritative behavior. Nondeterministic wall runtime and the earlier documented debug-derived signature are excluded from historical equality.

Development failures remain preserved: invalid notice-expiry setup, the mistaken combined event-order assertion, and the raider eating-access mismatch that caused expired-sleep polling. Only the fixture deadline and proven new access inconsistency were corrected. Combat coefficients and survival priorities were not calibrated to victories. Initial partial `smoke/` is invalidated; `smoke-corrected/` is preliminary. Only `matrix/` supplies the final1,440-run evidence.

## Gate decision and reproduction

F11C's ten mechanism gates pass. F11A's open protective-benefit requirement closes at its stated controlled scope: voluntary mobilization → real assembly → physical defensive contact → measurable reduction in harm, with opportunity costs retained. General economic desirability remains unproven.

No compulsory service, requisition, taxation, conquest, morale system, weapons chain or new player interface is adopted. The default configuration remains combat-disabled. No further milestone is implemented in this increment.

From source `631c76e`, using new output directories:

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
python3 scripts/run_combat.py NEW_PANEL_DIRECTORY 30
python3 scripts/analyze_combat.py NEW_PANEL_DIRECTORY NEW_ANALYSIS_DIRECTORY
python3 scripts/trace_combat.py NEW_PANEL_DIRECTORY NEW_TRACE_DIRECTORY
python3 scripts/verify_combat.py NEW_REPLAY_DIRECTORY
```

The trace-audit script is included in the subsequent evidence commit; it changes no simulation code. Each panel subprocess has a300s harness timeout, not an in-world rule. The kernel, personal policy, institutional permissions and research injection remain distinct.
