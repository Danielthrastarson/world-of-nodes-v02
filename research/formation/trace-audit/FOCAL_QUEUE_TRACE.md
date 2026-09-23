# Congested soldier3, seed0

Absolute clock, not hours since command. The order was issued at1h. These are observed events from the preserved final run.

```text
0.016666667h  event=43  UnitAssembled  unit=0; no remote orders received
1.000000000h  event=635  UnitOrderReceived  unit=0; serial=0; target=1; no route/stock information supplied
1.016666667h  event=639  UnitActionState  WITH_UNIT; unit order 0: execute remembered movement undertaking
1.016666667h  event=650  Interrupted  queue for route; was=MilitaryDuty { unit: 0 }
1.016666667h  event=651  Queued  edge=0 target=1 remembered_food=false
8.016666667h  event=4223  UnitActionState  PAUSED_EAT; locally accessible food; exact ownership/ration ceiling
8.016666667h  event=4231  Interrupted  eat accessible food; was=Queued { edge: 0, to: 1, joined: 3660000 }
8.016666667h  event=4232  Consumed  batch=4 q=6 owner=Agent(3) nutrition_ms=64740000
8.033333333h  event=4239  UnitActionState  WITH_UNIT; unit order 0: execute remembered movement undertaking
8.033333333h  event=4245  Queued  edge=0 target=1 remembered_food=false
12.016666667h  event=6326  Interrupted  voluntary service ended; was=Queued { edge: 0, to: 1, joined: 28920000 }
12.016666667h  event=6327  UnitServiceEnded  unit=0; reason=TERM_COMPLETED; roster retained
```

The order persists across eating, but existing queue cancellation/reentry loses the old FIFO position. Expiry cancels the pending traversal. This trace supports an interaction between shared congestion, survival/action arbitration and finite commitment duration; it does not justify tuning any of them.
