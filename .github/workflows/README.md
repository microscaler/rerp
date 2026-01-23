# GitHub Actions Workflows

## Available Workflows

### `ci.yml`

**Purpose**: Main CI pipeline — validate OpenAPI specs (including BFF generation), validate port assignments, build, test, multi-architecture builds, and (on main/develop/tags) build and push container images to GHCR and optionally Docker Hub.

**Triggers**: Push and pull requests to `main` and `develop`; tags `v*`.

**Jobs** (order: validate-openapi → validate-ports → build-and-test → build-multiarch → build-push-containers on push):
- **Validate OpenAPI Specs**: Validates all `openapi.yaml` files and runs `scripts/generate_system_bff.py` as a dry run to ensure BFF generation works.
- **Validate Port Assignments**: Runs `scripts/assign-port.py validate` to check for duplicate ports in helm/kind/Tiltfile and mismatches with the port registry.
- **Build and Test**: Patches deps for CI, builds workspace (amd64), example service (auth_idam), and accounting microservices (smoke test), format/lint/test/doc.
- **Build Multi-Architecture**: Builds for amd64, arm64, arm7 via `cross` (runs after build-and-test).
- **Build and Push Containers**: After `build-multiarch` succeeds, on **push** to `main`, `develop`, tags `v*`, or (temporarily) `boostrap-accounting-suit`. Plan: once direct pushes to `main` are off, only merges to `main` (a push to `main`) will trigger this; remove `boostrap-accounting-suit` from the `if` when the container process is validated. Builds multi-arch (linux/amd64, linux/arm64, linux/arm/v7) images for all accounting microservices, pushes to GHCR and optionally Docker Hub, then verifies each image manifest. This job does **not** use the artifacts uploaded by `build-multiarch`; it rebuilds microservices from scratch. See `docs/CONTAINER_RELEASE_DESIGN_PROPOSAL.md` §2.9 and §2.10.

**BFF generation**: The `generate_system_bff.py` script is exercised in the validate-openapi job. To regenerate and commit BFF specs, run `python3 scripts/generate_system_bff.py` locally and commit.

---

## Containers published to GHCR and Docker Hub

When **Build and Push Containers** runs (push to `main`/`develop` or tags `v*`), the following images are published.

### GHCR (always)

| Image | Description |
|-------|-------------|
| `ghcr.io/<owner>/rerp-general-ledger` | General Ledger |
| `ghcr.io/<owner>/rerp-invoice` | Invoice Management |
| `ghcr.io/<owner>/rerp-accounts-receivable` | Accounts Receivable |
| `ghcr.io/<owner>/rerp-accounts-payable` | Accounts Payable |
| `ghcr.io/<owner>/rerp-bank-sync` | Bank Synchronization |
| `ghcr.io/<owner>/rerp-asset` | Asset Management |
| `ghcr.io/<owner>/rerp-budget` | Budgeting |
| `ghcr.io/<owner>/rerp-edi` | EDI & Compliance |
| `ghcr.io/<owner>/rerp-financial-reports` | Financial Reports |
| `ghcr.io/<owner>/rerp-bff` | Accounting BFF API |

`<owner>` is `github.repository_owner` (e.g. `microscaler`). Each image is multi-arch: `linux/amd64`, `linux/arm64`, `linux/arm/v7`. Tags: on tags `v*` → the version (e.g. `v1.2.3` → `1.2.3`); on `main` → `sha-<short-sha>` and `latest`; on `develop` → `sha-<short-sha>` only.

### Docker Hub (optional)

If `vars.DOCKERHUB_ORG` is set (e.g. `microscaler`), the same 10 images are also pushed to `docker.io/<DOCKERHUB_ORG>/rerp-<name>:<tag>`.

**Required repository configuration** (when using Docker Hub):

- **Variable**: `DOCKERHUB_ORG` — e.g. `microscaler`. If empty or `null`, Docker Hub login and push are skipped.
- **Secrets**: `DOCKERHUB_USERNAME`, `DOCKERHUB_TOKEN` — used by `docker/login-action` for Docker Hub.

*Note: `secret-manager-controller` uses `secrets.MICROSCALER_DOCKER` with a fixed `casibbald` username for Docker Hub. For RERP, use `DOCKERHUB_ORG` + `DOCKERHUB_USERNAME` + `DOCKERHUB_TOKEN`. To publish under the same Docker Hub user as secret-manager, set `DOCKERHUB_ORG=microscaler`, `DOCKERHUB_USERNAME=casibbald`, and `DOCKERHUB_TOKEN` to the same value as `MICROSCALER_DOCKER`.*

---

## Future Workflows

Additional workflows will be added for:
- Code generation from OpenAPI specs
- Testing generated services
- Deployment automation
- Documentation generation
