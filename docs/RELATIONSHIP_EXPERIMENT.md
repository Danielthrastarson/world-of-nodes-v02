# B2 preregistration — compressed interpersonal history

Baseline a72e269. Preserve B1 and older results. Optional social configuration is absent in every historical fixture. No physical/institutional change; no new transfer rights. All people have identical provisional appraisal rules. No tuning after outcomes.

## State and evidence

Sparse directed records keyed by (observer, other): trust [-100,100], attachment/grievance/obligation [0,100], last_update. No loyalty/friendship number. Initial explicit kin and caregiver links establish records (not all household-member pairs); attachment20, other dimensions0. Support can establish a directed recipient→giver record. No continuous relationship decay.

Person-level anger, fear and grief are pulses: add bounded intensity at an appraisal, linear lazy decay of one integer intensity unit per simulated hour, cap100. This is a transparent experimental timescale, not validated psychology. Read-only access never mutates or refreshes state. Persistent grievance does not automatically decay with anger.

Observable event types are typed evidence, not betrayal/loyalty commands:
- Completed personal care transfer: the recipient and locally present people with a care expectation toward that actor can see the actual transfer and local physiology. No remote witnesses. Positive appraisal uses the physical quantity sacrificed and an explicitly imperfect local scarcity proxy: multiplier3 for giver nutrition≤6h,2 for≤12h,1 otherwise. Strength=q×multiplier; trust and obligation rise by strength, attachment by ceil(strength/2). This measures visible sacrifice, not exact counterfactual future survival.
- Food used for oneself/another while an expected-care recipient is severely hungry: expectation requires an explicit caregiver obligation toward the witness. The witness must actually be present, alive, and at≤6h nutrition. Self-preservation at actor nutrition≤6h is evidence against an available alternative. Above that threshold, the witness may infer that the visible food could have helped them. This inference can be wrong; it is not access to the actor's plan or private motive.
- Perceived unmet expectation with an alternative: loss = min(100,q×(1+max(trust,0)/20)); trust falls, grievance rises by loss, anger by2×loss. Outcome matters through the witness's severe need. Without an available alternative, disappointment reduces trust by1, with no grievance/anger.
- A physically rejected local care-transfer attempt is visible evidence of an unsuccessful attempt. Conservatively infer inability, not deliberate harm, regardless of whether the actor might have other hidden options. This permits mistaken appraisal without omniscience. No reward for endlessly making impossible offers.
- Death learned at physical colocation: once per known person, grief increases by attachment/2. Distant death does nothing; arriving later can reveal it. Dead people never appraise. No inheritance or claims on property.
- Own danger transition: nutrition0 and continuous starvation≥18h produces fear12. Deduplicate while the same danger continues; recovery permits a future new episode. It does not change physiology. Recent fear may persist after nourishment.

Local evidence delivery uses existing kin/care/social adjacency and current colocation. Sparse records are not recomputed as all-to-all pairs. Per-person observations may inspect that person's existing sparse contacts; appraisals occur only on discrete evidence/threshold transitions, not every tick. No message/report institution is introduced.

## Choice integration

First obtain the existing B1 feasible care candidate through the common Person capability stage. Keep all prior self-eating, critical-survival/sleep/atomic-work/job gates. B2 cannot create an otherwise infeasible candidate or a new ownership right.

For that candidate, compare small inspectable terms:

support pressure = floor((12h - observed dependent nutrition)/2h)
                 + trust/6 + attachment/10 + obligation/6

sacrifice/reluctance = candidate food quantity (or planned gift, at most6)
                    + ceil(believed travel hours)
                    + grievance/6 + anger(now)/6 + fear(now)/6 + grief(now)/6

Integer division; negative trust keeps its sign. Support must exceed sacrifice/reluctance. Otherwise the normal existing policy continues. The decomposition is logged; no history label such as 'betrayed' enters selection. These coefficients are a registered behavioral hypothesis, not a correction to B1 or a fitted psychological model. Do not amplify them to ensure visible drama. Affects initially influence discretionary care only; no combat, revenge or general productivity debuff.

## Experiments

Use seeds0–29 for paired runs, with identical initial psychology. Preserve all physical prelude events, appraisal evidence and later decisions.

Controlled histories use real kernel commands (not autonomous-history claims): six personal food quanta given with giver nutrition7h versus18h; a rejected transfer with no food; six quanta self-consumed while caregiver nutrition18h and dependent0h; same self-consumption remote; symmetric rights but asymmetric histories; an attached third person dying locally versus remotely; a self-starvation danger episode; no-history and non-relation controls. These episodes establish appraisal correctness, not the prevalence of spontaneous sacrifice/refusal.

Critical matched-state experiment: transplant ONLY the resulting social memories into copies of one explicitly registered later physical state. Later actor1 has18h nutrition and six personal quanta; person0 has0h and is a seeded care dependent. All parties are colocated, with identical physical state, spatial/care memory, RNG, rights and obligations across history arms. Use the same authoritative clock and retain the actual earlier episode/appraisal transcript as provenance. This is a research-only conditional-state control, not a production reset/teleportation mechanism. Select and execute through Person without forced help/refusal. Compare support/refusal/failed-attempt/unknown/no-history; delayed checkpoints test lazy affect fading. Log exact input equality excluding social history.

Also run B1 colocated, farming, distant-farming and construction fixtures under B2 absent versus present (30 paired seeds each) to measure autonomous interaction, allocation, work, food and starvation. This separates seeded-history capability from naturally occurring events. If negative appraisals/costly loyalty are rare, report that.

Required controls/tests: directionality, sparse storage bound, local event delivery, remote mutation independence, known-vs-unknown death, death learned on later visit, no repeated grief/fear from identical observations, exact lazy decay, distinct relationship/affect persistence, bodily/accounting invariants, identical history→identical choice, and each affect's observable influence on eligible choices. Do not claim every dimension is independently identified by a joint-history experiment; use small isolated state controls where necessary.

## Evidence and gates

Metrics: observed evidence/appraisals, dimension deltas, affect episodes/duration (analytic pulse integral or duration, not tick updates), attempted/selected/refused care, gift quantities, social care travel, per-person activity, starvation, deaths, later action differences and causal traces. Repeated seeds with identical outcomes are reproducibility, not population inference.

Full Rust tests/fmt/clippy; representative historical F0–F10 panel plus all B1 fixtures at seed0, with exact old events/hashes/physical outputs. Old parameters may add only optional social: None. SHA256 every old research file. B2 passes only if real observed history changes a later autonomous decision while locality, sparsity and physical laws hold. B3 and F11 remain blocked; no additional institution is selected.
