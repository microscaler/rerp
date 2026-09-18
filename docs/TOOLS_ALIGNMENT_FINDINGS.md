# RERP → Hauliage Alignment: Findings & Roadmap

**Date:** 2026-04-24
**Scope:** Layout, tooling, OpenAPI, Tiltfile, Helm across RERP and Hauliage

---

## Executive Summary

RERP and Hauliage share the same BRRTRouter codegen foundation but diverge significantly in three areas:

1. **Project layout** — RERP uses suite-nested (`openapi/<suite>/<service>/`), Hauliage uses flat (`openapi/<service>/`). This was the critical bug we fixed.
2. **Tooling integration** — RERP was importing hauliage's workspace patches (flat-layout monkey-patches), which would break suite-nested discovery. We fixed this with a proper wrapper.
3. **OpenAPI spec maturity** — Only accounting/general-ledger is production-ready. 70/71 services are skeleton specs.

The Tiltfile and Helm configs are structurally sound — RERP's Tiltfile is already aligned with hauliage patterns. The Helm templates match closely, with minor differences in security handling and liveness probe paths.

### What We Fixed This Session

| Issue | Status | Details |
|-------|--------|---------|
| Tooling import path | ✅ Fixed | Changed from `brrtrouter_tooling.workspace.cli.main` (flat-layout patches) to raw `brrtrouter_tooling.cli` (suite-nested) |
| Build CLI routing | ✅ Fixed | `rerp build microservice <name>` → `brrtrouter build <suite>_<module> --package rerp_<suite>_<module>` |
| Docker CLI routing | ✅ Fixed | `rerp docker build-image-simple ... --service <name>` → `--system=<suite> --module=<name>` |
| Dockerfile template | ✅ Fixed | Reverted from `{suite}/{module}` placeholders to `{{system}}/{{module}}` for suite-nested layout |
| Tooling shim | ✅ Built | `tooling/src/rerp_tooling/cli/main.py` — thin wrapper with translation layer |

---

## 1. Layout Analysis

### Current States

```
RERP (suite-nested, CORRECT):
openapi/
  accounting/
    general-ledger/openapi.yaml
    invoice/openapi.yaml
    ...
  sales/
    order/openapi.yaml
    ...

Hauliage (flat):
openapi/
  fleet/openapi.yaml
  consignments/openapi.yaml
  identity/openapi.yaml
  ...
```

```
RERP microservices (suite-nested, CORRECT):
microservices/
  accounting/
    general-ledger/gen/ impl/
    invoice/gen/ impl/
  sales/
    order/gen/ impl/
  Cargo.toml (workspace)

Hauliage microservices (flat):
microservices/
  fleet/gen/ impl/
  consignments/gen/ impl/
  identity/gen/ impl/
  Cargo.toml (workspace)
```

### Hauliage's Flat-Layout Patches

Hauliage's workspace CLI (`brrtrouter_tooling/workspace/cli/`) patches discovery to:
1. Assume all services belong to suite "hauliage"
2. Discover from flat `microservices/<name>/` layout
3. Generate package names as `hauliage_<name>`

**RERP must NOT use these patches.** RERP's package names are `rerp_{suite}_{module}` and its layout is suite-nested.

### Fix Applied

```python
# OLD (broken): imports hauliage's workspace CLI with flat-layout patches
from brrtrouter_tooling.workspace.cli.main import main

# NEW (correct): imports raw CLI, uses suite-nested discovery
from brrtrouter_tooling.cli import gen_cmd, build, docker_cmd, bff
```

The wrapper (`tooling/src/rerp_tooling/cli/main.py`) translates RERP-specific command patterns:
- `rerp build microservice <name>` → `brrtrouter build <suite>_<name> --package rerp_<suite>_<name>`
- `rerp docker build-image-simple ... --service <name>` → `--system=<suite> --module=<name>`

---

## 2. Tooling Comparison

### Command Mapping

| RERP Command | Raw CLI Equivalent | Status |
|-------------|-------------------|--------|
| `rerp gen suite <suite> --service <name>` | `brrtrouter gen suite <suite> --service <name>` | ✅ Pass-through |
| `rerp gen stubs <suite> <name> --force` | `brrtrouter gen stubs <suite> <name> --force` | ✅ Pass-through |
| `rerp build microservice <name>` | `brrtrouter build <suite>_<name> --package rerp_<suite>_<name>` | ✅ Translated |
| `rerp docker copy-binary <src> <dest> <bin>` | `brrtrouter docker copy-binary <src> <dest> <bin>` | ✅ Pass-through |
| `rerp docker build-image-simple <img> <tmpl> <hash> <artifact> --service <name>` | `brrtrouter docker build-image-simple <img> <hash> <artifact> --system=<suite> --module=<name>` | ✅ Translated |
| `rerp docker build-base` | `brrtrouter docker build-base` | ✅ Pass-through |
| `rerp bff generate-system` | `brrtrouter bff generate-system` | ✅ Pass-through |

### Architecture Detection

Both RERP and Hauliage use identical architecture detection:
```python
host_machine = str(local('uname -m', quiet=True)).strip()
if host_machine in ['arm64', 'aarch64']:
    TARGET_ARCH_NAME = 'arm64'
    TARGET_RUST_TRIPLE = 'aarch64-unknown-linux-musl'
else:
    TARGET_ARCH_NAME = 'amd64'
    TARGET_RUST_TRIPLE = 'x86_64-unknown-linux-musl'
```

### Docker Pruning

Both use identical `docker_prune_settings`:
```
disable=False, max_age_mins=30, keep_recent=1, interval_hrs=1
```

---

## 3. Tiltfile Comparison

### Structural Similarities

Both Tiltfiles follow the same pattern:
1. K8s context configuration (`allow_k8s_contexts(['kind-kind'])`)
2. Docker prune settings
3. Base Docker image build
4. Tooling venv setup
5. Architecture detection
6. Per-service: lint → gen → build → docker → deploy (Helm)
7. BFF spec generation
8. Shared infrastructure (database, Redis)

### Key Differences

| Feature | RERP | Hauliage | Notes |
|---------|------|----------|-------|
| Lint tooling | ✅ Has `lint-tooling` | ❌ Missing | RERP is more complete |
| Test tooling | ✅ Has `test-tooling` | ❌ Missing | RERP is more complete |
| Liveness probe | `/health` | `/metrics` | RERP uses standard health check |
| Readiness probe | `/health` | `/health` | Same |
| Env from secret | ❌ Missing | ✅ Has | Hauliage loads secrets from K8s Secret |
| Config volume | ✅ Has | ✅ Has | Same |

### RERP-Specific Strengths

RERP's Tiltfile is actually **more complete** than Hauliage's in some areas:
- Has `lint-tooling` and `test-tooling` resources
- More thorough comment documentation
- Has `rerp-db-init` and `rerp-apply-migrations` for database lifecycle
- Has `rerp-migrate` for schema migration

### RERP-Specific Weaknesses

- Missing `envFrom` secret loading in deployment (Hauliage has this)
- ConfigMap is 88 lines (RERP) vs 12 lines (Hauliage) — RERP's configmap is bloated

---

## 4. Helm Values Comparison

### Template Structure

Both use identical template structure:
```
helm/<name>-microservice/
  Chart.yaml
  values.yaml              # shared defaults
  values/<service>.yaml    # per-service overrides
  templates/
    deployment.yaml
    service.yaml
    configmap.yaml
```

### Deployment Config Comparison

| Config | RERP general-ledger | Hauliage consignments | Notes |
|--------|-------------------|----------------------|-------|
| image.tag | `tilt` | `tilt` | Same for dev |
| pullPolicy | `IfNotPresent` | `Always` | **Difference** |
| replicas | 1 | 1 | Same for dev |
| serviceType | NodePort | NodePort | Same |
| port | 8001 | 8003 | Service-specific |
| nodePort | 31001 | 31003 | Service-specific |

### Environment Variables

Both define the same base env vars:
- `RUST_LOG`, `RUST_BACKTRACE`, `BRRTR_LOG_FORMAT`
- `BRRTR_LOG_SAMPLING_MODE`, `BRRTR_LOG_ASYNC`
- `BRRTR_LOG_INCLUDE_LOCATION`, `BRRTR_STACK_SIZE`

**Hauliage extra:** `BRRTR_DEBUG_SESSION`
**RERP missing:** `BRRTR_DEBUG_SESSION`

---

## 5. OpenAPI Spec Audit (Detailed)

### Scorecard

| Area | RERP Current | Target | Gap |
|------|-------------|--------|-----|
| Suites with BFF config | 1/27 | All 27 | **Critical** |
| Suites with security | 1/27 | All 27 | **Critical** |
| Suites with tags | 1/27 | All 27 | **High** |
| Suites with pagination | 1/27 | All 27 | **High** |
| Suites with error schemas | 1/27 | All 27 | **High** |
| Suites with health endpoints | 1/27 | All 27 | **Medium** |
| Avg endpoints/service | ~2.6 | 10-20 | **Critical** |
| Response codes/endpoint | 2-5 | 4-8 | **Medium** |

### The Accounting Suite (1/27 Mature)

Only `general-ledger` is production-ready with:
- 21 endpoints
- `bearerAuth` security scheme
- Pagination support
- 8 response codes (200, 201, 204, 400, 401, 403, 404, 409)
- 38 schemas

The other 8 accounting services are skeletons (4-6 endpoints each, no security, no pagination).

### 26 Other Suites

All have the same skeleton structure:
- BFF config: ❌
- Security: ❌
- Tags: ❌
- Pagination: ❌
- Health endpoints: ❌

---

## 6. Roadmap

### Phase 1: Tooling Stabilization (Week 1) — **IN PROGRESS**

- [x] Fix tooling import path (suite-nested vs flat-layout)
- [x] Build CLI translation layer
- [x] Docker CLI translation layer
- [x] Revert Dockerfile.template
- [ ] Run `cargo build` for all accounting services (verify build works end-to-end)
- [ ] Run Playwright BDD tests for all accounting services
- [ ] Document tooling wrapper API in `tooling/README.md`

### Phase 2: OpenAPI Foundation (Week 2-3)

- [ ] Add `bearerAuth` security scheme to ALL 27 suite specs
- [ ] Add global `security: [bearerAuth: []]` to ALL specs
- [ ] Add tags to ALL 71 services: `{suite-name}` + `{service-name}`
- [ ] Create shared `ErrorResponse` and `ValidationError` schemas
- [ ] Add BFF configs to 26 remaining suites
- [ ] Add health endpoints to all services: `/health` GET
- [ ] Add standard error responses (400, 401, 403, 404, 422, 500)

### Phase 3: Accounting Suite Completion (Week 4-6)

- [ ] Enrich 8 skeleton accounting services to 15+ endpoints each
- [ ] Add pagination to all list endpoints
- [ ] Add all standard error responses
- [ ] Add idempotency keys to POST/PUT/DELETE
- [ ] Add webhook event schemas
- [ ] Run brrtrouter-gen lint on all specs

### Phase 4: Other Suites Iterative Build (Week 7-12)

Each week: 2-3 suites → full spec enrichment → codegen → build → test

- Week 7-8: auth, product, inventory
- Week 9-10: sales, purchase, crm
- Week 11-12: hr, manufacturing, website

### Phase 5: Helm/Deployment Improvements (Week 13)

- [ ] Add `envFrom` secret loading to deployment template
- [ ] Reduce configmap size (88 → ~15 lines)
- [ ] Add `BRRTR_DEBUG_SESSION` env var
- [ ] Consider changing `pullPolicy: IfNotPresent` → `Always`
- [ ] Add pod anti-affinity for multi-replica deployments
- [ ] Add service account for RBAC

---

## 7. Quick Reference: RERP Package Naming

RERP uses a consistent naming scheme derived from the suite-nested layout:

| Component | Pattern | Example |
|-----------|---------|---------|
| OpenAPI spec | `openapi/<suite>/<service>/openapi.yaml` | `openapi/accounting/general-ledger/openapi.yaml` |
| Cargo workspace member | `microservices/<suite>/<service>/impl` | `microservices/accounting/general-ledger/impl` |
| Cargo package name | `rerp_<suite>_<module>` | `rerp_accounting_general_ledger` |
| Docker image | `localhost:5001/rerp-<suite>-<name>` | `localhost:5001/rerp-accounting-general-ledger` |
| Helm values | `helm/rerp-microservice/values/<name>.yaml` | `helm/rerp-microservice/values/general-ledger.yaml` |
| Kubernetes service | `<name>.<namespace>.svc.cluster.local` | `general-ledger.rerp.svc.cluster.local` |

---

## 8. Files Changed This Session

| File | Action | Purpose |
|------|--------|---------|
| `tooling/src/rerp_tooling/cli/main.py` | Created/Replaced | CLI wrapper with translation layer |
| `tooling/pyproject.toml` | Created | Packaging config for rerp_tooling |
| `Tiltfile` | Modified | Reverted Dockerfile.template placeholders |
| `docs/OPENAPI_SPEC_AUDIT.md` | Created | Detailed OpenAPI spec analysis |
| `docs/TOOLS_ALIGNMENT_FINDINGS.md` | Created (this file) | Comprehensive findings & roadmap |

---

## 9. What NOT to Change (Preserve)

The following are already well-aligned with hauliage and should NOT be modified:

1. **Tiltfile structure** — Per-service resource pattern is correct
2. **Helm template structure** — deployment/service/configmap is correct
3. **Architecture detection** — ARM/AMD64 detection is correct
4. **Docker prune settings** — Identical and correct
5. **BFF spec generation** — Already working for accounting
6. **Tooling venv management** — Shared with BRRTRouter, correct
7. **K8s context configuration** — `kind-kind` context is correct
