# Contributing to RERP

Thank you for your interest in contributing to **RERP** (Rust Enterprise Resource Planning)! 🎉

This guide will help you get started with contributing to the project. It is
also the canonical contract for RERP's suite and microservice layout. New code,
generation, migration, build, and deployment tooling must preserve these
boundaries.

---

## Getting Started

### Prerequisites

- **Rust toolchain** (stable) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Python 3.12+** (for generation scripts)
- **Git** for version control
- **BRRTRouter** - [BRRTRouter repository](https://github.com/microscaler/BRRTRouter) (for code generation)

### Quick Start

1. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/microscaler/rerp.git
   cd rerp
   ```

2. **Explore the project structure**:
   ```bash
   # View suite-nested service specifications
   find openapi -mindepth 3 -maxdepth 3 -name openapi.yaml

   # View generated/implementation crate pairs
   find microservices -mindepth 3 -maxdepth 3 -type d \
     \( -name gen -o -name impl \)
   ```

3. **Generate system BFF specs** (optional, for testing): run `just init` then `rerp bff generate-system`.

4. **Pre-commit hooks** (recommended): run `just init` then `just install-hooks`. Before each commit this runs:
   - **Tooling QA**: `just qa` (lint, format-check, tooling tests)
   - **microservices-fmt**: if `microservices/` changed vs HEAD, runs
     `just fmt-rust` for the suite-nested Rust workspace; fast if Tilt has
     recently built.

---

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Your Changes

- Follow Rust best practices and conventions
- Ensure code compiles without warnings
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
cd microservices

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Format code
cargo fmt

# Check linting
cargo clippy --workspace
```

### 4. Commit Your Changes

Commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. This is enforced in CI via commitlint; PRs with invalid commit messages will fail.

Format: `<type>[optional scope]: <description>` (subject line max 1500 characters; extra space for agentic tracking)

Examples:
- `feat: add new service endpoint`
- `feat(auth): add login endpoint`
- `fix: correct validation logic`
- `docs: update API documentation`
- `test: add tests for inventory service`
- `chore(deps): bump crate-x to 1.2`

Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `chore`, `ci`, `build`, `revert`.

To validate before pushing: `npx commitlint --from HEAD~1` (requires Node; or rely on CI).

### 5. Submit a Pull Request

- Push your branch to your fork
- Create a pull request with a clear description
- Reference any related issues
- Ensure CI checks pass

---

## Areas for Contribution

We welcome contributions in the following areas:

### OpenAPI Specifications

- **Enhance service specs**: Add complete schemas, examples, and detailed descriptions
- **Add missing endpoints**: Identify and document additional API endpoints
- **Improve documentation**: Enhance service descriptions and parameter documentation

### Service Implementation

- **Implement business logic**: Add handlers and controllers for services
- **Add validation**: Implement request/response validation
- **Error handling**: Improve error handling and user feedback

### Testing

- **Unit tests**: Add tests for individual services and functions
- **Integration tests**: Test service interactions and workflows
- **Load testing**: Performance and scalability testing

### Documentation

- **User guides**: Create guides for using RERP services
- **API documentation**: Improve OpenAPI specifications
- **Code examples**: Add practical usage examples
- **Planning documents**: Design proposals, PRDs, and analysis documents (see "Planning Documents" section below)
- **Current authority**: Start at [`docs/README.md`](./docs/README.md); do not
  infer authority from filenames, timestamps, or LLM wiki summaries
- **Lifecycle and supersession**: Follow
  [`docs/DOCUMENTATION_GOVERNANCE.md`](./docs/DOCUMENTATION_GOVERNANCE.md)
- **Architecture decisions**: Allocate and register ADRs through
  [`docs/adrs/README.md`](./docs/adrs/README.md)

### CI/CD

- **Enhance automation**: Improve GitHub Actions workflows
- **Add checks**: Additional quality checks and validations
- **Deployment**: Improve deployment processes

---

## Planning Documents

**⚠️ CRITICAL: All planning, analysis, design proposals, and implementation status documents MUST be created in `./docs/` or its subdirectories.**

Placement does not make a document authoritative. New documents must use the
controlled lifecycle and metadata in
[`docs/DOCUMENTATION_GOVERNANCE.md`](./docs/DOCUMENTATION_GOVERNANCE.md).
Accepted decisions live in ADRs; approved requirements remain in PRDs or modes
of operation; analyses and completion reports remain dated evidence. Replaced
documents are explicitly marked and linked to their successor rather than
silently deleted.

### Document Organization

- **`docs/ai/`** - AI-generated planning, analysis, and implementation status documents
- **`docs/adrs/`** - Architecture Decision Records (ADRs)
- **`docs/`** (root) - Design proposals, PRDs, and other planning documents

### Naming Conventions

- Design proposals: `docs/DESIGN_PROPOSAL_*.md`
- PRDs: `docs/*_PRD.md`
- Analysis documents: `docs/*_ANALYSIS.md`
- Status documents: `docs/*_STATUS.md` or `docs/*_COMPLETE.md`
- Implementation status: `docs/ai/*_COMPLETE.md` or `docs/ai/*_STATUS.md`

### Examples

✅ **Correct locations:**
- `docs/DESIGN_PROPOSAL_RELEASE_CI_INTEGRATION.md`
- `docs/VERSIONING_STRATEGY_ANALYSIS.md`
- `docs/ACCOUNTING_SUITE_ENRICHMENT_PRD.md`
- `docs/ai/OPENAPI_ENRICHMENT_ANALYSIS.md`

❌ **Incorrect locations (NOT ALLOWED):**
- `DESIGN_PROPOSAL_*.md` (project root)
- `VERSIONING_STRATEGY_*.md` (project root)
- `ANALYSIS_*.md` (project root)

### Why This Matters

- Keeps project root clean and organized
- Makes documentation easy to find
- Follows standard documentation structure
- Prevents accidental commits of planning documents in wrong locations

**If you find planning documents in the project root, move them to `./docs/` immediately.**

## Code Standards

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Run `cargo clippy` and fix warnings
- Write comprehensive doc comments
- Maintain test coverage (target: 65% minimum, 80% preferred)

### OpenAPI Specifications

- Use OpenAPI 3.1.0 format
- Include complete path definitions
- Define request/response schemas
- Add operation summaries and descriptions
- Follow RESTful conventions

### Git Workflow

- Use [Conventional Commits](https://www.conventionalcommits.org/) (enforced in CI)
- Keep commits focused and atomic
- Rebase on main before submitting PRs
- Write clear PR descriptions

---

## Suite and microservice architecture

RERP is a multi-suite platform. Hauliage is useful prior art for the anatomy of
one microservice, but Hauliage is a single product suite and therefore has a
flat service tree. RERP adds a mandatory suite level to every contract, runtime,
database, test, and deployment path.

The hierarchy is:

```text
RERP platform
└── suite
    ├── suite-owned shared assets
    └── microservice
        ├── generated API contract
        └── user-owned implementation
```

### Canonical paths

For an Accounting invoice service:

```text
openapi/accounting/invoice/openapi.yaml

microservices/accounting/
├── README.md
├── core/
├── entities/
├── migrations/
├── sql/
├── scripts/
├── tests/
└── invoice/
    ├── README.md
    ├── gen/
    └── impl/
```

The same shape applies to every suite:

```text
openapi/<suite>/<service>/openapi.yaml
microservices/<suite>/<service>/gen/
microservices/<suite>/<service>/impl/
deployment-configuration/profiles/<environment>/rerp/<suite>/
```

Do not flatten a RERP service to `microservices/<service>`. Do not put
suite-specific `entities/`, `migrations/`, `sql/`, `scripts/`, `tests/`, or
`core/` directories at the repository root. Root-level assets are reserved for
capabilities that genuinely apply to every installed suite.

### Suite-owned assets

Each suite owns its complete optional installation boundary:

| Path | Responsibility |
|---|---|
| `microservices/<suite>/README.md` | Suite purpose, service inventory, runtime and verification guidance |
| `microservices/<suite>/core/` | Pure shared domain calculations and invariants; no transport or database executor abstraction |
| `microservices/<suite>/entities/` | Genuinely suite-wide foundation persistence models only |
| `microservices/<suite>/migrations/` | Generated and authored migrations for that suite, plus suite-local application order |
| `microservices/<suite>/sql/` | Suite-owned database contracts that are not entity-generated |
| `microservices/<suite>/scripts/` | Suite-specific development and operational wrappers |
| `microservices/<suite>/tests/` | Cross-service acceptance and suite installation-isolation tests |
| `deployment-configuration/profiles/<environment>/rerp/<suite>/` | Suite-owned non-secret properties and SOPS-encrypted runtime credentials |

The presence of all suites in one Cargo workspace is a development convenience;
it is not permission to install every suite. Build, Helm/Tilt selection, and
migration application must all select the intended suites explicitly.

Environment profiles preserve the same installation boundary. Product and
suite configuration belongs in RERP, not in
`shared-gitops-k8s-cluster/deployment-configuration/`. Following the SAM Flux
pattern, the platform repository composes a Flux `GitRepository` and
`Kustomization` which source and reconcile the product-owned path. Tilt must
not apply the same resources or render/apply Helm workloads. The RERP Tiltfile
is an image-development loop: it may generate/build code and publish ordered
dev images, while Flux owns bootstrap Jobs, Helm releases, rollout, and drift
correction. The platform repository also owns the other side
of shared infrastructure contracts—for example, Pgpool's custom-user source
must carry the same rotated database credential as the owning RERP suite
profile. Never commit plaintext secrets or duplicate environment-specific
values in Helm defaults.

### Complete HTTP microservice anatomy

Every HTTP microservice has an OpenAPI contract, a generated crate, and a real
implementation crate:

```text
openapi/<suite>/<service>/
└── openapi.yaml                    # authoritative HTTP contract

microservices/<suite>/<service>/
├── README.md                       # human-owned service documentation
├── gen/                            # disposable BRRTRouter output
│   ├── Cargo.toml
│   ├── config/
│   ├── doc/                        # generated OpenAPI/API documentation
│   ├── static_site/
│   └── src/
│       ├── controllers/
│       ├── handlers/
│       ├── registry.rs
│       └── lib.rs
└── impl/                           # deployable, user-owned service
    ├── Cargo.toml
    ├── build.rs                    # Lifeguard entity-registry generation
    ├── config/
    ├── seeds/
    ├── src/
    │   ├── controllers/            # HTTP adapters only
    │   ├── services/               # use cases and application orchestration
    │   ├── models/                 # service-owned Lifeguard persistence models
    │   ├── validators/             # domain/request validation
    │   ├── impl_registry.rs        # real-handler registration/overrides
    │   ├── lib.rs                  # reusable implementation surface
    │   └── main.rs                 # executable composition and startup
    └── tests/
        ├── bdd/
        ├── features/
        ├── integration/
        └── openapi_parity.rs
```

Directories are added when the service needs them; empty scaffolding is not
required. Their ownership does not change:

- `controllers/` translate the generated transport contract to application
  calls and map results to responses. They do not own large business workflows.
- `services/` own application use cases and orchestration within that service.
- `models/` own that service's persistence entities and database views.
- `validators/` enforce rules not completely expressible in OpenAPI or database
  constraints.
- `seeds/` contain service-owned development/reference data, not schema DDL.
- `tests/` prove the real implementation rather than generated example handlers.
- `config/` contains runtime configuration packaged for that service.
- `build.rs` exposes the Lifeguard entity registry consumed by the top-level
  migrator.
- `impl_registry.rs` ensures implemented controllers replace generated
  examples/stubs.

`gen/` is disposable. Never fix business behavior, generated documentation, or
generated request/response types by editing it directly. Change the OpenAPI
source or BRRTRouter generator and regenerate.

### Entity and database ownership

Every database table or view has exactly one `LifeModel` owner and exactly one
migration source.

Use the following decision order:

1. If a model belongs to one service, place it in that service's
   `impl/src/models/`.
2. If several services use the concept but one service is its natural owner,
   keep the model with the owner and reuse the owner's library/API rather than
   copying the `LifeModel`.
3. Only a concept that is genuinely foundational to the whole suite and has no
   natural service owner belongs in `microservices/<suite>/entities/`.
4. Never mirror a service model in the suite entity crate, and never define the
   same table in multiple service implementations.

The top-level migrator must reject duplicate table ownership. Similar Rust
filenames are not the deciding factor; the effective schema/table identity is.

`microservices/<suite>/core/` is not an entity repository and not another
service implementation. It contains pure reusable domain policy only.

### Human and generated documentation

- `openapi/<suite>/<service>/openapi.yaml` is the public API source of truth.
- `<service>/gen/doc/` and `<service>/gen/static_site/` are generated artifacts.
- `<service>/README.md` documents human-owned service purpose, boundaries,
  dependencies, configuration, operations, and verification.
- `<suite>/README.md` documents suite-wide installation and service ownership.
- Architecture decisions, PRDs, discoveries, and plans belong under `docs/`.

Do not place human-authored architecture or operating knowledge under `gen/`;
it will be overwritten.

### Tests and seeds

- Unit tests live beside the code they exercise or in the service `impl/tests/`.
- Service integration, BDD, feature, and OpenAPI-parity tests live under the
  service `impl/tests/`.
- Cross-service workflows and suite installation-isolation tests live under
  `microservices/<suite>/tests/`.
- Seeds live under the owning service's `impl/seeds/` and use the filename and
  idempotency rules enforced by Lifeguard migration tooling.
- Schema changes are not seeds, and generated schema migrations are not test
  fixtures.

### BFFs and non-HTTP workers

A suite BFF is still a service and keeps its own generated and implementation
crates:

```text
openapi/<suite>/openapi_bff.yaml
microservices/<suite>/bff/gen/
microservices/<suite>/bff/impl/
```

Its OpenAPI document is aggregated from the suite configuration, but real
composition and orchestration belong in `bff/impl/`.

A worker or scheduled processor that has no HTTP surface does not need an empty
`gen/` crate. It remains owned by its suite and has its own package, source,
configuration, and tests, for example:

```text
microservices/<suite>/workers/<worker>/
├── Cargo.toml
├── config/
├── src/
└── tests/
```

## Migration architecture

RERP has one top-level migration tool at `microservices/migrator/`, adapted from
Hauliage's proven service-registry pattern. RERP's additional requirement is
strict suite qualification.

The migrator consumes providers identified by `(suite, service)` and entity
registries exported by service `impl` crates or the suite foundation entity
crate. It must:

1. Require explicit suite selection; no implicit "install everything" default.
2. Generate, validate, plan, and apply only the selected suite.
3. Write only beneath `microservices/<suite>/migrations/`.
4. Produce suite-local `apply_order.txt` and seed ordering.
5. Preserve service subdirectories within the suite migration set.
6. Fail when two providers claim the same effective table/view.
7. Fail on entity generation errors rather than silently omitting a provider.
8. Prevent migration paths or order entries from crossing suite boundaries.
9. Avoid cross-suite database foreign keys unless an accepted ADR explicitly
   makes one suite a required dependency of the other.

Example migration products:

```text
microservices/accounting/migrations/
├── apply_order.txt
├── foundation/
├── general-ledger/
├── invoice/
└── accounts-receivable/

microservices/documents/migrations/
├── apply_order.txt
├── core/
└── render/
```

The migrator may depend on service implementation libraries because their
`build.rs` registries are the Hauliage pattern. Those dependencies must be
grouped or feature-gated by suite so an Accounting-only migration build does
not require Documents, HR, or any other optional suite.

Never create a repository-root `migrations/` directory. Never make each service
container race to apply migrations independently; migration application is an
installation/suite operation.

## Adding or changing a service

Before opening a PR for a service change:

1. Identify the owning suite and service.
2. Update `openapi/<suite>/<service>/openapi.yaml` when the HTTP contract changes.
3. Regenerate `gen/`; do not hand-edit it.
4. Implement behavior in the service `impl/` using the ownership rules above.
5. Verify every persistence table has one owner and migration provider.
6. Add service tests, plus suite tests when behavior crosses service boundaries.
7. Generate and validate migrations for the owning suite only.
8. Update the service and suite README when ownership or operations change.
9. Ensure Tilt/Helm/Docker descriptors use the full `(suite, service)` identity.

## Project structure overview

```
rerp/
├── microservices/
│   ├── Cargo.toml                    # shared workspace and lockfile
│   ├── migrator/                     # single suite-aware migration tool
│   └── <suite>/
│       ├── core/
│       ├── entities/
│       ├── migrations/
│       ├── sql/
│       ├── scripts/
│       ├── tests/
│       └── <service>/{gen,impl}/
├── openapi/
│   └── <suite>/
│       ├── bff-suite-config.yaml
│       ├── openapi_bff.yaml
│       └── <service>/openapi.yaml
├── tooling/                         # suite-aware RERP CLI and build tooling
├── helm/                            # generic chart plus suite/service values
├── docker/                          # parameterized runtime image definitions
├── deployment-configuration/
│   └── profiles/<env>/rerp/<suite>/ # product config + SOPS credentials
├── docs/
│   ├── ai/
│   ├── adrs/
│   └── *.md
└── .github/
```

**⚠️ Important**: The project root should NOT contain planning documents. All planning, analysis, design proposals, and status documents must be in `./docs/` or subdirectories.

The complete ownership rules—not just the abbreviated tree—are normative for
new services and structural corrections.

---

## Testing Requirements

### Before Submitting

Run Rust workspace commands from `microservices/`:

- ✅ All tests pass: `cd microservices && cargo test --workspace`
- ✅ Code compiles: `cd microservices && cargo build --workspace`
- ✅ No warnings: `cd microservices && cargo clippy --workspace -- -D warnings`
- ✅ Formatted: `cd microservices && cargo fmt --check`
- ✅ Documentation builds: `cd microservices && cargo doc --workspace`

### Test Coverage

- Minimum coverage: **65%**
- Target coverage: **80%**
- Critical paths: **100%**

---

## Questions?

- **Issues**: [GitHub Issues](https://github.com/microscaler/rerp/issues)
- **Discussions**: [GitHub Discussions](https://github.com/microscaler/rerp/discussions)

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

---

Thank you for contributing to RERP! 🚀
