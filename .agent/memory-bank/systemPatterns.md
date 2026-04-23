# System Patterns

**Tilt pipeline (per service):** `create_microservice_lint` (brrtrouter-gen lint, `||` cargo run fallback) → `create_microservice_gen` (brrtrouter-gen generate, then `fix_cargo_toml_paths.py`) → `create_microservice_build_resource` → `create_microservice_deployment` (copy, docker, custom_build+live_update, helm, k8s_resource). **`set -e`** in all `local_resource` cmd shells so failures are not masked by trailing `echo "✅ ..."`.

**Suites and BFF:** RERP is **suite-based**: each suite has microservices + one BFF. `openapi/{suite}/bff-suite-config.yaml`, `openapi/{suite}/openapi_bff.yaml`. **BFF (accounting today):** `bff-spec-gen` (bff-generator) → `bff-lint` → `create_microservice_gen('bff', ...)`. `accounting-all-gens` waits on all `*-service-gen` including `bff-service-gen`. assign-port and port-registry: discover suites by listing `openapi/*/bff-suite-config.yaml` and reading `bff_service_name`; no script edits when adding a new suite BFF.

**Naming:** `PACKAGE_NAMES` (Cargo -p, brrtrouter info.title) vs `BINARY_NAMES` (Docker/Helm/build_artifacts); can differ (e.g. invoice→invoice_management pkg, invoice binary).

**CI:** `patch-brrtrouter-for-ci`; `build-and-test` (components); `build-push-containers` (microservices, 10 images, multi-arch amd64+arm64+arm/v7). Proposals: smoke-test microservices in build-and-test; post-push `imagetools inspect` to verify platforms. `boostrap-accounting-suit` temporarily in container-job `if`; remove when validated.

**IDAM:** Analysed for separate repo + git dep; not yet extracted. **BFF generator:** Externalised as `bff-generator` (PyPI/git); RERP uses `bff-suite-config.yaml`.
