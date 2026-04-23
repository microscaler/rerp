# Progress

**Done:** Foundation (dirs, scripts, Helm, K8s); entity migration (47 entities, 9 domains, `rerp_entities`); all 10 accounting services + BFF bootstrapped (lint, gen, build, deploy in Tilt). Tiltfile: `set -e` in create_microservice_lint, create_microservice_gen, bff-spec-gen, bff-lint. Farm init; Memory Bank created and enriched from `docs/`.

**Container release:** 10 multi-arch images (general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff) via `build-push-containers`; CONTAINER_RELEASE_DESIGN_PROPOSAL documents flow and proposals (smoke in build-and-test, post-push verify). Branch `boostrap-accounting-suit` gated for container job during validation.

**Pre-commit hooks:** `.pre-commit-config.yaml` with `qa` (just qa: lint, format-check, pytest) and `microservices-fmt` (if `microservices/` changed: `just fmt-rust` for components+entities). `just fmt-rust`: `cd components && cargo fmt --all`, `find entities -name '*.rs' -exec rustfmt {} +`. `just install-hooks` after `just init`. Both hooks use `pass_filenames: false`. Doc in CONTRIBUTING.

**Planned / proposals:** Smoke-test microservices in build-and-test; verify published images per-arch; remove `boostrap-accounting-suit` from container `if`; IDAM separate repo; BFF generator already externalised (`bff-generator`). Business logic implementation in handlers; DB (Lifeguard, migrations).
