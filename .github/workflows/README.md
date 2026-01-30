# GitHub Actions Workflows

## Available Workflows

### `ci.yml`

**Purpose**: Main CI pipeline — validate OpenAPI specs (including BFF generation), validate port assignments, build, test, multi-architecture builds, and (on main/develop/tags) build and push container images to GHCR and optionally Docker Hub.

**Triggers**: Push and pull requests to `main` and `develop`; tags `v*`.

**Jobs** (order: validate-openapi → validate-ports → build-and-test → build-multiarch → on push: download-copy-package-push-containers → build-push-service (matrix) → verify-published-images):
- **Validate OpenAPI Specs**: Validates all `openapi.yaml` files and runs `rerp bff generate-system` as a dry run to ensure BFF generation works.
- **Validate Port Assignments**: Runs `rerp ports validate` to check for duplicate ports in helm/kind/Tiltfile and mismatches with the port registry.
- **Build and Test**: Patches deps for CI, builds workspace (amd64), example service (auth_idam), and accounting microservices (smoke test), format/lint/test/doc.
- **Build Multi-Architecture**: Builds workspace and microservices for amd64, arm64, arm7 via `cross`; uploads `rerp-binaries-*` (microservices) and `microservices-binaries-*` (amd64, arm64, arm) artifacts.
- **Download Copy Package and Push Containers** (on **push** to `main`, `develop`, or tags `v*`): Copies microservices binaries from `build-multiarch` artifacts into `build_artifacts/`, validates layout, sets image tag, and uploads `build-artifacts` for the matrix job.
- **Build and push \<service\>** (matrix over 10 microservices: general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff): For each service, downloads `build-artifacts`, runs Extract metadata → Build and push → Attest. Uses `docker/microservices/Dockerfile.\<service\>`, multi-arch linux/amd64, linux/arm64, linux/arm/v7. (The website is deployed via GitHub Pages in `deploy-website.yml`, not as a container here.)
- **Verify published images**: After all matrix jobs succeed, runs `docker buildx imagetools inspect` on all 10 microservice images to confirm manifests and platforms.

**BFF generation**: `rerp bff generate-system` is exercised in the validate-openapi job. To regenerate and commit BFF specs, run `rerp bff generate-system` locally (e.g. `tooling/.venv/bin/rerp bff generate-system` after `just init`) and commit.

**brrtrouter_tooling**: The `rerp` CLI and tooling tests depend on `brrtrouter_tooling` (build, gen, docker, openapi, ci). It is not a dependency in `tooling/pyproject.toml` because it lives in the separate [BRRTRouter](https://github.com/microscaler/BRRTRouter) repo. CI installs it from GitHub in every job that sets up the tooling venv: `pip install "brrtrouter-tooling @ git+https://github.com/microscaler/BRRTRouter.git@main#subdirectory=tooling"`. Locally, install it the same way or `pip install -e ../BRRTRouter/tooling` when BRRTRouter is a sibling of the repo.

---

## Containers published to GHCR and Docker Hub

When the container pipeline runs (push to `main`/`develop` or tags `v*`: **Download Copy Package and Push Containers** → **Build and push \<service\>** matrix → **Verify published images**), the following 10 microservice images are published. The website is deployed separately via **deploy-website.yml** to GitHub Pages.

### GHCR (always)

| Image | Description |
|-------|-------------|
| `ghcr.io/<owner>/rerp-accounting-general-ledger` | General Ledger |
| `ghcr.io/<owner>/rerp-accounting-invoice` | Invoice Management |
| `ghcr.io/<owner>/rerp-accounting-accounts-receivable` | Accounts Receivable |
| `ghcr.io/<owner>/rerp-accounting-accounts-payable` | Accounts Payable |
| `ghcr.io/<owner>/rerp-accounting-bank-sync` | Bank Synchronization |
| `ghcr.io/<owner>/rerp-accounting-asset` | Asset Management |
| `ghcr.io/<owner>/rerp-accounting-budget` | Budgeting |
| `ghcr.io/<owner>/rerp-accounting-edi` | EDI & Compliance |
| `ghcr.io/<owner>/rerp-accounting-financial-reports` | Financial Reports |
| `ghcr.io/<owner>/rerp-accounting-bff` | Accounting BFF API |

`<owner>` is `github.repository_owner` (e.g. `microscaler`). Each image is multi-arch: `linux/amd64`, `linux/arm64`, `linux/arm/v7`. Tags: on tags `v*` → the version (e.g. `v1.2.3` → `1.2.3`); on `main` → `sha-<short-sha>` and `latest`; on `develop` → `sha-<short-sha>` only.

### Docker Hub (optional)

If `vars.DOCKERHUB_ORG` is set (e.g. `microscaler`), the same 10 images are also pushed to `docker.io/<DOCKERHUB_ORG>/rerp-accounting-<name>:<tag>`.

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

### `release.yml`

**Purpose**: Bump the version in all Cargo.toml, generate release notes from commits (OpenAI or Anthropic), commit, create tag `vX.Y.Z`, push, and create a **GitHub Release** with the generated notes. The tag push triggers **ci.yml** (build and publish containers with the version as the image tag). See **docs/RELEASE_MANAGEMENT_PRD.md**.

**Triggers**: **`workflow_dispatch` only** (manual run).

**Inputs**:
- **bump**: `patch` (default, bump Z) | `minor` (bump Y, Z→0) | `major` (bump X, Y Z→0)
- **branch**: Branch to release from (default `main`)
- **provider**: `anthropic` (default) | `openai` — AI provider for release notes generation

**Jobs**:
- **Bump, tag and push**: Checkout → bump (read `microservices/Cargo.toml`, compute next, write to all Cargo.toml) → **generate release notes** (commits since last tag → OpenAI or Anthropic per `provider` → `release-body.md`) → check for changes → commit `chore(release): vX.Y.Z` → tag `vX.Y.Z` → push (EndBug/add-and-commit) → **create GitHub Release** (softprops/action-gh-release) with `body_path: release-body.md`.

**Required secrets** (use the one for the chosen `provider`; default is anthropic):
- **`ANTHROPIC_API_KEY`** — when `provider=anthropic` (default; https://console.anthropic.com/). If missing and provider is anthropic, the generate-notes step fails.
- **`OPENAI_API_KEY`** — when `provider=openai` (https://platform.openai.com/api-keys). If missing and provider is openai, the generate-notes step fails.

**Release notes template**: `.github/release-notes-template.md` defines the structure (Summary, Features, Fixes, Other). Use `{{VERSION}}` for the version; `[brackets]` are hints for the model. Override via `--template` in the workflow or run `rerp release generate-notes` locally.

**Version source**: `microservices/Cargo.toml` `[workspace.package].version`. The same value is written to all `[package]` / `[workspace.package].version` in Cargo.toml across the repo, including the **root `Cargo.toml`** `[workspace.package].version` (which is explicitly kept in sync even if it has drifted).

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
