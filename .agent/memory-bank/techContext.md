# Tech Context

**Rust / Cargo:** Workspaces — `components/` (optional shared impls), `entities/` (rerp_entities, Lifeguard), `microservices/` (accounting + BFF). BRRTRouter (OpenAPI→Rust): `brrtrouter-gen lint`, `brrtrouter-gen generate`. Local: `path = "../../BRRTRouter"`, `path = "../../lifeguard"`; CI: `patch-brrtrouter-for-ci.py` → `git = "https://github.com/microscaler/..."`.

**Suites and BFF:** Each **suite** has its own BFF. `openapi/{suite}/bff-suite-config.yaml`, `openapi/{suite}/openapi_bff.yaml`. `bff-generator` (pip): `generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml`. After gen: `fix_cargo_toml_paths.py` on microservice `Cargo.toml`. assign-port discovers suites by listing `openapi/*/bff-suite-config.yaml` and reading `bff_service_name` from each; no hardcoded suite names. Full spec: `.agent/memory-bank/suiteArchitecture.md`.

**Build:** `build-microservice.sh` (workspace --arch amd64|arm64|arm7, --copy-only); `host-aware-build.py` for components. **Multi-arch:** amd64 (musl), arm64/arm7 (cross); `build_artifacts/amd64|arm64|arm/`; Docker `TARGETARCH`. **Containers:** `build-and-push-microservice-containers.sh`; 10 multi-arch images to GHCR; `docker/microservices/Dockerfile.{service}`. `build-multiarch` = components; `build-push-containers` = microservices (separate).

**Run:** Tilt, Kind (`kind-rerp`), Helm `helm/rerp-microservice`, k8s `k8s/microservices`. Scripts: `build-microservice.sh`, `copy-microservice-binary-simple.sh`, `build-microservice-docker-simple.sh`, `generate_system_bff.py`.
