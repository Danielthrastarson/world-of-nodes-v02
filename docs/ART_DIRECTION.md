# World of Nodes — proposed art direction

W1 is a single review scene, not an approved world-wide art style. Macroquad,
Rust, the observation boundary and the headless simulation remain unchanged.
Do not extend this treatment to the normal client before owner review.

## Direction: an inhabited illustrated landscape

Continuous top-down 2.5D, orthographic projection, restrained medieval materials.
The camera looks at a landscape, not a diagram. Silhouette, ground contact,
overlap, sunlight and material contrast supply depth without a 3D asset pipeline.

Muted moss, olive, straw and clay dominate daytime. Water is desaturated blue-green,
with dark banks and sparse glints. Buildings use pale plaster, dark timber and
weathered russet roofs. Near-black blue holds the night; small amber flame cores
and broad soft light pools provide warm contrast. Avoid neon outlines, glossy
plastic, noisy ground detail, uniform tiled lawns and a brown wash over everything.

The hierarchy is: settlement/river/crossing → roads/working land → people/stock →
small material details. Empty land is useful negative space. UI occupies very
little of the picture; detailed inspection remains a separate product concern.

## Visual vocabulary

| Meaning | Proposed shape/material | Source and constraint |
|---|---|---|
| Institutional settlement | One asymmetric roofed hall/place silhouette, compact earth yard | Known local board/institution. A location glyph, not a claim of simulated housing capacity or extra buildings |
| Cultivation unit | Irregular soil quadrilateral, furrows and state-dependent crop rows | One known field record. Empty, growing and mature remain distinct; no fictional crop yield |
| Finite wood zone | Grouped overlapping tree canopies with dark trunks, physical cut-wood pile | Known nonempty wood deposit. Trees represent the zone, not additional harvestable entities; pile count is a bounded quantity glyph |
| Food stock | Small tied ochre sacks, each grouped within its source stock location | Known food batch/quantity, not decorative surplus |
| Person | Tiny head/body silhouette; cloth mark distinguishes known role | One existing Person, with existing identity. No crowd filler |
| Unit | Real member silhouettes with subtle matching tabards/shields | Known unit membership; no abstract army body or invented formation |
| Cart | Wheels, axle and wooden bed | Actual known cart, same ownership/custody rules |
| Connection | Uneven earth road with soft shoulders and restrained wheel wear | Existing route, actual known path stage/capacity. No lines connecting circular nodes |
| Crossing | Timber deck, beams and rails over river; incomplete state visibly incomplete | Actual crossing snapshot and construction/completion state |
| Remembered object | Desaturated translucent silhouette | Dated evidence. Night never reveals unknown objects |
| Graph node/edge ID | Diagnostic ring/label | Explicit debug overlay only; absent from art/normal presentation |

Scale is intentionally illustrative. Building symbols are enlarged enough to
recognize; people are small, not head-sized map pins. At close zoom show limbs,
roof seams and crops; at middle zoom preserve silhouettes; far away use individual
small marks and area masses. Never merge Persons into simulated population totals.

## Roads and geography

Road drawing and traveling Persons share W0 RouteGeometry. Roads terminate at
places and disappear beneath yards rather than ending at exposed graph markers.
Do not add apparent traversable links. The W1 river is authored presentation-only
geography placed beneath the existing constrained crossing edge; the existing
upper detour runs around its headwaters. It neither cuts an existing route nor
creates a new one. Its shoreline is not collision, fertility or water supply.

This river is an explicit art-fixture initial condition. W0's current general
scenario does not establish hydraulic geography. General river generation or
world-wide inferred geography is out of scope.

## Light, fire and time

W1 samples authoritative time for a smooth day → dusk → night treatment. Light
changes only appearance, never observation range, work, danger or heat.

Two small stationary fire/torch props may be authored as **presentation-only
light-study state** beside the institutional hall. They are not carried by
Persons, do not imply fuel consumption, ignition, available equipment or a new
fire system, and cannot be selected or commanded. They exist only in this named
art fixture. Their placement, activation and animation are deterministic and
listed in the scene manifest. They disappear when their anchoring institution is
not in the supplied observation. No arbitrary living Person becomes a torchbearer.

This exception is for selecting a visual direction. Before shipping such lights
in normal gameplay, their presence must be tied to an explicitly authorized
physical/presentation source model. No burning house, battle smoke, weather or
unmodeled danger is drawn. No expensive shadow system is introduced.

## Knowledge and capture honesty

The isolated renderer receives the existing RenderSnapshot. It cannot query
World or dispatch commands. The capture harness may request Debug truth for an
explicitly labeled research art view. Full landscape shots must never masquerade
as ruler knowledge. A separate ruler-view screenshot checks that hidden remote
contents remain absent. Graph diagnostics require a separate explicit flag.

The scene uses the existing Local scenario and a typed crossing commission.
Buildings, fields, stock, Persons, carts and crossing progress are snapshot-driven.
Recorded motion advances the real clock; interpolated poses do not feed back.
Research captions identify full-state art study versus ruler knowledge.

## Review question and stop

Deliver 3–5 screenshots spanning overview, settlement, close detail, knowledge
view and night, plus one short accelerated day-to-night clip. Assess:

- Does it read as a small lived-in medieval place without node IDs?
- Are river, bridge, paths, fields and working/resource zones distinguishable?
- Are people the right size and density for the world we actually simulate?
- Is the night readable, with localized warm contrast rather than global orange?
- Is this the desired degree of illustration versus abstraction?

Human judgment chooses the direction. Screenshot generation does not pass that
art gate. Stop before adding renderer features, spreading the treatment across
the map, adding simulation systems or beginning another art iteration.

## E0 alternative: embodied miniature study

The owner subsequently authorized a bounded 3D embodiment experiment. It tests
simple low-poly people/buildings in the same muted material vocabulary, with an
elevated strategic camera and a following ruler camera. This does not overwrite
W1 or adopt a production 3D art pipeline. See [E0](EMBODIED_WORLD.md). Both lighting
props and terrain embedding remain explicitly labelled presentation studies;
actual Persons, crops, carts and crossings retain their simulation identities.
