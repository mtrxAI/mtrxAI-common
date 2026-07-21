# Auth token matrix

Shared helper crate: [`mtrxai-auth`](mtrxai-auth/) (`constant_time_eq`, `verify_bearer_token`, `verify_header_token`).

| Surface | Env var | Header | Notes |
|---------|---------|--------|-------|
| Peer local LLM proxy | `MTRXAI_PROXY_TOKEN` | `Authorization: Bearer …` | Optional; if unset, proxy is open on loopback-oriented deploys |
| Peer UI API | (none / proxy token exempt) | `/api/client/*` and `/api/peer/*` skip proxy token | UI is local-operator facing |
| Server admin | `MTRXAI_ADMIN_KEY` | `x-admin-key` | Constant-time compare via `mtrxai-auth` |
| Icell admin (pull/load/…) | `CELL_ADMIN_TOKEN` | `Authorization: Bearer …` | Empty token = open (dev); set `MTRXAI_ICELL_REQUIRE_ADMIN=1` to fail closed |

These remain **distinct secrets** (different trust domains). Do not unify env names across peer/server/icell.
