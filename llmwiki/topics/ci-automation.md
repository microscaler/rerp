# CI Automation

> GitHub Actions CI for OpenAPI validation, BFF generation, build, and multi-arch test.

**Status:** partially-verified

## CI Workflow

File: `.github/workflows/ci.yml`

## Pipeline Steps

1. **Validate OpenAPI** — Check all 71 OpenAPI specs are valid
2. **BFF generation dry run** — Ensure BFF specs can be regenerated
3. **Build** — `cargo build --workspace`
4. **Test** — `cargo test --workspace`
5. **Multi-arch** — Cross-architecture builds

## Key Files

- CI: `.github/workflows/ci.yml`
- Setup doc: `docs/ai/CI_AUTOMATION_SETUP.md`
- Status: `docs/ai/FIRST_CI_AUTOMATION.md`

## BFF Generation in CI

- Suite BFFs generated from `bff-suite-config.yaml` files
- System BFF generated via `rerp bff generate-system`
- Generated BFF specs go to `openapi/{suite}/openapi_bff.yaml`

## Tooling

- `just init` sets up the venv for tooling
- `tooling/.venv/bin/rerp` is the main automation CLI
- `bff-generator` is the BFF generation CLI
