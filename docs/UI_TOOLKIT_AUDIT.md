# UI toolkit audit

The shared `src/ui.rs` boundary owns the semantics that must stay consistent
across the cabinet: logical viewport conversion, checked pointer conversion,
touch-target expansion, neighbour-aware collision auditing, readable text
size, toolkit-backed text measurement, and toolkit-backed text drawing.

Per-game UI modules keep small local drawing helpers for their established
visual language. Those helpers delegate text measurement and drawing through
`src/ui.rs`, while click routing delegates hit testing through the same
touch-aware boundary. Their custom rectangles and palette choices are visual
composition, not alternate input semantics.

The cabinet, responsive cabinet, and result surfaces intentionally retain
their bespoke panels, bevels, and brass/cream palette so the title-screen
identity remains coherent. New shared widgets should use the toolkit's
`button_on_release`, `Pointer::released_on`, surface, plaque, and text helpers;
local helpers should only remain when a surface needs a distinct treatment or
an existing action-map contract.
