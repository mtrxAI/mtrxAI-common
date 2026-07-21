# Shared UI kit

Canonical copies of dashboard tokens, formatters, and the logo for **mtrxAI-peer** and **mtrxAI-server**.

| File | Purpose |
|------|---------|
| `tokens.css` | CSS variables (`--color-bg`, etc.) |
| `formatters.js` | `fmtGpu` / `locationLine` helpers |
| `logo.svg` | Brand mark |

## How peer / server consume these

Embedded UIs use `include_str!` / `include_bytes!` on **local** copies under each crate (`peer/ui/…`, `server/ui/…`). Cross-crate `include_str!` into `mtrxAI-common/ui` is fragile in Docker builds that only copy one repo.

**Practical sync:**

1. Edit files here when changing shared look/feel.
2. Copy `logo.svg` into `mtrxAI-peer/peer/ui/assets/logo.svg` and `mtrxAI-server/server/ui/assets/logo.svg`.
3. Keep `:root` tokens in `peer/ui/v2/index.html` and `server/ui/v1/index.html` aligned with `tokens.css` (or paste the block when updating).
4. Keep Alpine `fmtGpu` / `locationLine` methods aligned with `formatters.js`.

HTML files note this with a short comment at the top of each index.
