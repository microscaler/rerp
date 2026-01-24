# GitHub Actions Workflows

## Available Workflows

### `ci.yml`

**Purpose**: Main CI pipeline — validate OpenAPI specs (including BFF generation), validate port assignments, build, test, multi-architecture builds, and (on main/develop/tags) build and push container images to GHCR and optionally Docker Hub.

**Triggers**: Push and pull requests to `main` and `develop`; tags `v*`.

**Jobs** (order: validate-openapi → validate-ports → build-and-test → build-multiarch → build-push-containers on push):
- **Validate OpenAPI Specs**: Validates all `openapi.yaml` files and runs `rerp bff generate-system` as a dry run to ensure BFF generation works.
- **Validate Port Assignments**: Runs `rerp ports validate` to check for duplicate ports in helm/kind/Tiltfile and mismatches with the port registry.
- **Build and Test**: Patches deps for CI, builds workspace (amd64), example service (auth_idam), and accounting microservices (smoke test), format/lint/test/doc.
- **Build Multi-Architecture**: Builds for amd64, arm64, arm7 via `cross` (runs after build-and-test).
- **Build and Push Containers**: After `build-multiarch` succeeds, on **push** to `main`, `develop`, tags `v*`, or (temporarily) `boostrap-accounting-suit`. Plan: once direct pushes to `main` are off, only merges to `main` (a push to `main`) will trigger this; remove `boostrap-accounting-suit` from the `if` when the container process is validated. Builds multi-arch (linux/amd64, linux/arm64, linux/arm/v7) images for all accounting microservices, pushes to GHCR and optionally Docker Hub, then verifies each image manifest. This job does **not** use the artifacts uploaded by `build-multiarch`; it rebuilds microservices from scratch. See `docs/CONTAINER_RELEASE_DESIGN_PROPOSAL.md` §2.9 and §2.10.

**BFF generation**: `rerp bff generate-system` is exercised in the validate-openapi job. To regenerate and commit BFF specs, run `rerp bff generate-system` locally (e.g. `tooling/.venv/bin/rerp bff generate-system` after `just init`) and commit.

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

### `base-images.yml`

**Purpose**: Build and publish `rerp-base` to GHCR (and optionally Docker Hub). Uses registry cache and change detection to skip builds when `docker/base` Dockerfiles are unchanged.

**Triggers**: Push and pull requests to `main` and `develop` when `docker/base/Dockerfile`, `docker/base/Dockerfile.multiarch`, or this workflow change; **`workflow_dispatch`** (manual run with optional `version` and `force` inputs).

**Jobs**:
- **Build rerp-base**: On change or on `workflow_dispatch`/`force`, builds and pushes `ghcr.io/<owner>/rerp-base` (and optionally `docker.io/<DOCKERHUB_ORG>/rerp-base`) for `linux/amd64` and `linux/arm64`.

---

### `deploy-website.yml`

**Purpose**: Build `ui/website` (SolidJS) and deploy to **GitHub Pages**.

**Triggers**: Push to `main` when `.github/**`, `ui/website/**`, or `ui/shared/**` change; `workflow_dispatch`.

**Jobs**:
- **build**: Node 22, Yarn 1.22.22, `yarn install` and `yarn build` in `ui/website` with `VITE_BASE_URL` and `VITE_BASE_PATH` set for `https://<owner>.github.io/<repo>/`. Uploads `ui/website/dist` as an artifact.
- **deploy**: Uses `github-pages` environment; downloads the artifact, runs `configure-pages`, `upload-pages-artifact`, and `deploy-pages`.

**Required repository configuration**:

1. **Settings → Pages → Build and deployment**: Source = **GitHub Actions**.
2. **Environments**: The `github-pages` environment is created when you first deploy. Optional: add a custom domain under Pages settings.

The site will be at `https://<owner>.github.io/<repo>/` (e.g. `https://microscaler.github.io/rerp/`).

---

## Dependabot

Dependency updates are managed by [Dependabot](https://docs.github.com/en/code-security/dependabot) via `.github/dependabot.yml`.

**Ecosystems**: npm (ui/website), pip (tooling), cargo (workspace + entities), docker (docker/base, docker/website), github-actions.

**Schedule**: Weekly on Monday 09:00 UTC. Minor and patch updates are grouped per ecosystem to reduce PR volume; major updates stay as separate PRs.

**Not scanned**: `docker/microservices/Dockerfile.<service>` (Dependabot only recognizes a file named `Dockerfile`). Path and git deps outside the repo (e.g. `brrtrouter`, `lifeguard*`) are ignored.

---

## Future Workflows

Additional workflows will be added for:
- Code generation from OpenAPI specs
- Testing generated services
- Deployment automation
- Documentation generation
