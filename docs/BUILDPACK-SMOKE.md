# Buildpack Smoke Test

Validates that the [octopilot/rust](https://github.com/octopilot/buildpacks) buildpack can build RERP's general-ledger service.

## Prerequisites

- **rust 0.1.5+** — Required for RERP (fixes `rm: cannot remove target/` in some environments). Publish via octopilot buildpacks release. For local dogfooding before 0.1.5 is published, create a builder with `uri` pointing to your local octopilot/buildpacks/rust directory.
- **pack** — [Install Pack](https://buildpacks.io/docs/tools/pack/)
- **rerp ci patch-brrtrouter** — Run before pack build (replaces BRRTRouter/Lifeguard path deps with git)

## Local Build

```bash
# 1. Patch Cargo.toml (BRRTRouter/Lifeguard from git)
tooling/.venv/bin/rerp ci patch-brrtrouter

# 2. Create builder (one-time)
pack builder create rerp-builder --config .github/buildpack-builder.toml

# 3. Pack build
pack build rerp-accounting-general-ledger:buildpack-smoke \
  --path . \
  --builder rerp-builder \
  --descriptor project.toml \
  -e BP_RUST_WORKSPACE_DIR=microservices \
  -e BP_RUST_PACKAGE=rerp_accounting_general_ledger
```

## CI

The `.github/workflows/buildpack-smoke.yml` workflow runs on push to `main` and `chore/implement-octopilot-build`. It uses `ghcr.io/octopilot/rust:0.1.5` (or the version in `.github/buildpack-builder.toml`).

**Before first run**: Ensure rust 0.1.5 is published (or update the workflow to use your buildpack version).

## Project Layout

- `project.toml` — Project descriptor with octopilot/rust + inline buildpack for RERP layout (copy impl/config, gen/doc, gen/static_site)
- `.github/buildpack-builder.toml` — Minimal builder (Rust only)

## Design

See [DESIGN-BUILDPACKS-INTEGRATION.md](DESIGN-BUILDPACKS-INTEGRATION.md) Phase 1.
