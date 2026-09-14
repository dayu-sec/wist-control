# wist-control

Control-domain model: agent control plane and center governance.

[![crates.io](https://img.shields.io/crates/v/wist-control.svg)](https://crates.io/crates/wist-control)
[![docs.rs](https://img.shields.io/docsrs/wist-control/latest.svg)](https://docs.rs/wist-control)
[![Downloads](https://img.shields.io/crates/d/wist-control.svg)](https://crates.io/crates/wist-control)
[![CI](https://github.com/dayu-sec/wist-control/actions/workflows/ci.yml/badge.svg)](https://github.com/dayu-sec/wist-control/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/dayu-sec/wist-control/branch/main/graph/badge.svg)](https://codecov.io/gh/dayu-sec/wist-control)
[![dependency status](https://deps.rs/repo/github/dayu-sec/wist-control/status.svg)](https://deps.rs/repo/github/dayu-sec/wist-control)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`wist-control` is the reusable **Control** domain model in the `wist` system. It captures the agent
control plane and center governance objects — actors, capabilities, agent command/enrollment/identity
lifecycle, gateway identity/management, and center administration — as plain `serde` structs.

## Modules

| Module                          | Purpose                                                              |
| ------------------------------- | -------------------------------------------------------------------- |
| `actors`                        | Control-plane actors (admin operator, agentd, public install client). |
| `agent`                         | Agent command, enrollment, identity, registry, and status.           |
| `agent_app`                     | Agent-facing control application messages.                           |
| `gateway`                       | Gateway identity, management, security, and supervision.             |
| `gateway_app`                   | Gateway-facing control application messages.                         |
| `insight_center`                | Center governance (platform releases).                               |
| `insight_center_app`            | Center admin-facing interface messages.                              |
| `caps` / `protocol` / `storage` | Capabilities, wire protocol, and storage model.                      |
| `types`                         | Shared primitives re-exported from `wist-shared`.                    |

## Related crates

- [`wist-shared`](../wist-shared) — shared primitives (`DateTime`, `Secret`, `Int`, …).
- [`wist-contracts`](../wist-contracts) — versioned edge/center contract objects.

## License

[Apache-2.0](LICENSE)
