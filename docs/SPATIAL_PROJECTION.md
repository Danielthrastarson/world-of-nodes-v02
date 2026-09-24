# V0 / V1 — spatial projection and visual grammar

The owner paused simulation expansion after personally finding the P4 client too
visually unreadable to evaluate deeper gameplay. This increment changes only the
projection of the existing scenario and its permitted snapshots. The graph is
still authoritative. Human readability is a review gate, not a test assertion.

## Boundary

`World → ruler knowledge → Snapshot → spatial/glyph drawing → screen`.

`scene.rs`, `spatial.rs` and `glyphs.rs` receive an owned Snapshot, never World or
Session. The existing Play snapshot excludes unknown remote contents. Debug uses
the existing explicitly distinct truth snapshot. Input still passes through the
existing typed command handler and its separate Play snapshot.

No kernel, policy, physics, rights, scenario quantities, clock, command authority
or dependencies change. Curves, clearing shapes, within-node packing and screen
distances are display geometry. They cannot create routes, contact, area,
productive land, capacity, housing, ownership or travel shortcuts.

## Visual contract

| Existing state | Projection | Limits |
|---|---|---|
| Locally known public institutional place | Compact roof/place symbol and clearing | A place symbol, not a newly simulated house or housing capacity. Only known contents support it. |
| Charted location with no observed contents | Quiet hollow chart mark | Unknown does not mean empty. No inferred fields, stock, population or land use. |
| Known cultivation unit | Bounded arable patch; actual crop stages add soil/rows/ripe color | Available plots remain uncultivated. Remembered crops do not grow just because display time passes. |
| Food/wood at a physical location | Distinct stock glyph with quantity-dependent fill | Symbol scale is logarithmic and saturates at 256; exact quanta remain in inspection. Shapes do not create containers. |
| Finite wood deposit | Small canopy/resource-zone symbol | This represents known wood, not modeled trees, a renewable forest or a new harvesting rule. |
| Known route | Curved route with observed path-wear width | Curves preserve authoritative endpoints; curvature and screen length do not change travel. Unknown path stage is not filled from truth. |
| Crossing site/project | Gap, banks and partial timbers; completed crossing spans it | Schematic obstruction, not a simulated river. Partial timbers encode committed material, not percent labor complete. Only completion renders a full deck. |
| Actual visible/remembered Person | Small head/body figure | Exactly one figure per supplied Person record. No population-count fill, fake traffic or aggregate army. |
| Observed military membership | Same Person with military cloth/shield; local group standard | Unit grouping is display packing, not continuous formation geometry. Separate route progress groups remain separate. |
| Ruler / steward / recognized hostile | Gold identity / office badge / red hostile accent | Only explicitly available role/hostility evidence permits the mark. No private role inference. |
| Existing physical cart | Small wheeled silhouette at its known position | No cart is spawned for the illustration; the default P4 fixture has none. Owner/user rights remain unchanged. |
| Dated memory / report | Faded outlines / paper provenance mark | Exact observation age/source stays in inspection. No remote live tracking or new acknowledgment. |

The kernel has no modeled hydrology, continuous terrain, forest extent or housing
stock. These are not invented to fill the landscape. A courier marker is also
deferred: the current public Person snapshot does not reliably identify a
message-carrying role. Report panels remain the source of that permitted evidence.

## View and interaction

- Default camera frames the existing kingdom around Hearth at a more legible scale.
- Existing LMB selection, RMB context/cancel, pan/zoom, pause/speed and Q/W/E/R
  meanings remain. Visible unit labels now have their own matching click rectangle.
- F9 hides/restores panels and labels; `--map-only` starts in that view. It is a
  presentation toggle, not a knowledge mode. A persistent ruler-knowledge or red
  DEBUG watermark remains even with panels hidden.
- Selection, hover target geometry, the journal and frame interpolation remain
  separate from authoritative action execution.

## Scope and review

V0 and V1 are ready for visual review, not certified intuitive. Normal Play cannot
show East fields' remote food/crops/raiders from chart knowledge alone. The wider
Debug picture must not be presented as the normal player's knowledge.

The renderer is still schematic. At very high population density its within-node
packing can cover adjacent place symbols; the 1,000-Person stress test measures
drawing cost, not layout quality or fully simulated kingdom performance. The
crossing drawing is oriented for this fixture's near-horizontal edge. Arbitrary
crossing orientation, label collision handling, wider screen-size/accessibility
testing and richer public role metadata remain presentation debt.

V2 mouse-first expansion, V3 further knowledge visualization and V4 atmosphere
remain NOT STARTED. No lighting, torches, smoke, weather, sound, new economy or
military system is needed to claim this bounded implementation complete.

See [captures, validation and remaining limits](../research/spatial-projection/REPORT.md).
