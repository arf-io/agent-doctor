# agent-doctor

AI agent environment readiness and diagnostics.

## Release Info

| Field | Value |
|---|---|
| Release wave | later first tranche |
| Planned release date | 2026-07-06 |
| Owner vehicle | ARF.io |
| License | MIT |
| Usefulness | 4 / 5 |
| Maintenance | Medium |
| Release shape | CLI |

## Why

AI agent setups fail in boring ways: missing binaries, bad config, unreachable proxy ports, absent database settings, and broken runner auth. `agent-doctor` is the standalone diagnostic surface for those checks.

## Install

```sh
cargo install --path .
```

## Use

```sh
agent-doctor
```

Current status: release scaffold. It compiles independently and records the extraction map for Engrave readiness checks.

## Part Of Engrave

This tool is extracted from Engrave's doctor, readiness, health, and runner probe work. It stays standalone by default, with optional Engrave adapters reserved for future integration.

## Optional Integration

The `engrave-integration` feature is reserved for future optional adapters. The default build remains standalone.

## License

MIT. The small-tool plan uses MIT for focused developer CLIs where adoption and embedding matter more than control.

## Hire ARF.io

ARF.io builds governed AI automation for small teams: agent workflow audits, AI coding setup, secure local LLM/CLI integration, custom internal tools, reverse engineering, and production hardening.
