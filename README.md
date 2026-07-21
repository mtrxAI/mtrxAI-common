# mtrxAI common

Shared crates used across the mtrxAI sibling repos:

| Crate | Consumers | Role |
|-------|-----------|------|
| `mtrxai-attestation` | peer, server | Build attestation / peer auth crypto |
| `mtrxai-tee-attestation` | (optional) | GPU TEE attestation (not wired into prod images yet) |
| `mtrxai-protocol` | peer, server | Lobby WS `ProtocolMessage`, peer/GPU DTOs, ranking helpers, swarm presence |
| `mtrxai-icell-api` | peer, icell | Inference-cell `/mtrxai/v1` paths and admin DTOs |
| `mtrxai-auth` | peer, server, icell | Constant-time token comparison helpers |

Static UI kit (CSS tokens, logos, small JS helpers): [`ui/`](ui/).

## Build

```bash
cargo test
```

Peer and server path-depend on crates under this workspace. Clone as a sibling of `mtrxAI-peer` / `mtrxAI-server` / `mtrxAI-icell`.
