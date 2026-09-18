# RERP agent rules

RERP is a multi-suite product. Before changing service structure, persistence
models, migrations, generated code, build tooling, Tilt, Helm, or Docker, read
the canonical contributor contract:

- [`CONTRIBUTING.md`](./CONTRIBUTING.md), especially **Suite and microservice
  architecture** and **Migration architecture**.
- [`docs/README.md`](./docs/README.md) for the current human authority map and
  the distinction between intended and delivered behaviour.
- [`docs/llmwiki/index.md`](./docs/llmwiki/index.md), then only the pages relevant
  to the task.
- Tail [`docs/llmwiki/log.md`](./docs/llmwiki/log.md) for recent work and known
  drift.

The following rules are non-negotiable summaries of `CONTRIBUTING.md`:

1. **Preserve the suite boundary.** RERP paths are
   `openapi/<suite>/<service>/...` and
   `microservices/<suite>/<service>/...`. Do not flatten them to Hauliage's
   single-suite layout, and do not put suite-specific entities, migrations,
   SQL, tests, scripts, or core libraries at the repository root.
2. **Preserve the full service boundary.** Every HTTP service has a generated
   `gen/` contract crate and a user-owned `impl/` crate. Controllers, application
   services, persistence models, validators, configuration, seeds, and service
   tests belong to that service's `impl/` tree as described in
   `CONTRIBUTING.md`.
3. **One table, one entity owner.** A `LifeModel` belongs either to one service's
   `impl/src/models/` or, only for a genuinely suite-wide foundation concept, to
   `microservices/<suite>/entities/`. Never define the same table in both or in
   multiple services.
4. **Never hand-edit generated code.** `gen/` is disposable BRRTRouter output.
   Change `openapi/<suite>/<service>/openapi.yaml` and regenerate. Real behavior
   belongs in `impl/`.
5. **Migration tooling is top-level; migration products are suite-local.** The
   single tool lives at `microservices/migrator/`. It must select suites
   explicitly and write/apply only
   `microservices/<suite>/migrations/`. It must never recreate a repository-root
   `migrations/` directory or silently accept duplicate table ownership.
6. **Installation is suite-selective.** Do not make one suite depend on another
   suite's entity crate or database tables merely because both are present in
   the source workspace. Cross-suite integration is API/event based unless an
   accepted ADR says otherwise.
7. **Product deployment profiles stay suite-qualified in RERP.** Use
   `deployment-configuration/profiles/<environment>/rerp/<suite>/`. Store
   non-secret environment configuration in `application.properties`, encrypt
   credentials with SOPS, and never place a product profile in the shared
   platform GitOps repository. Flux sources and reconciles the profile from
   RERP; Tilt must not independently apply it. Tilt publishes development
   images only; Flux owns bootstrap Jobs, Helm releases, rollout, and drift
   correction. Platform-side dependencies such
   as Pgpool retain their own matching configuration there.

## Desktop development environment

- Do not create another local checkout.
- The desktop mirror is
  `/Users/casibbald/Workspace/remote/microscaler/rerp`.
- Run builds, tests, generation, Git commands, Tilt operations, and other shell
  commands on `ms02` in `~/Workspace/microscaler/rerp`.
- Preserve unrelated dirty work. Never reset or discard changes you did not
  create.
- Never push without explicit human authorization.

When code or authoritative documentation changes, reconcile the relevant LLM
wiki page and append a concise entry to `docs/llmwiki/log.md`.

The LLM wiki is derived knowledge. It cannot accept, supersede, or revive an
architecture decision or product requirement. Follow
[`docs/DOCUMENTATION_GOVERNANCE.md`](./docs/DOCUMENTATION_GOVERNANCE.md) and
update [`docs/authority.json`](./docs/authority.json) when current authority
changes.
