# Goal 1: Reproducible Development Baseline

- **Status**: Active
- **Started**: 2026-07-14
- **Restart baseline**: d0412f3

## Objective

Make RERP safe and predictable to build, generate, test, and change before accounting behavior is added.

## Why This Exists

The current repository has workspace, dependency, generated-code, and development-environment drift. A world-class open-source project cannot require undocumented local knowledge to reach a buildable state.

## Broad Outcomes

- One authoritative Cargo workspace shape.
- Entities, generated crates, implementation crates, and tooling have explicit ownership.
- BRRTRouter, Lifeguard, and may_minihttp versions are aligned and reproducible.
- OpenAPI lint and generation are deterministic.
- Generated examples cannot masquerade as implemented behavior.
- Repository setup works for a clean self-hosted checkout as well as the hosted development environment.
- Existing uncommitted infrastructure work is reconciled without losing user changes.

## Initial Acceptance Gates

- Cargo metadata reports the intended workspace members.
- The active accounting crates and entities compile from documented commands.
- Unit and contract tests run from documented commands.
- Regeneration produces no unexplained diff.
- OpenAPI lint fails CI on invalid contracts.
- Accounting routes with no implementation are not exposed as successful example responses.
- A new contributor can follow one setup document without relying on a developer's home-directory state.

## Decision 1: Explicit Workspace Boundaries

RERP will retain two intentional Cargo workspace boundaries:

- the root workspace owns shared in-repository crates, initially entities;
- microservices/Cargo.toml owns deployable generated and implementation crates.

The root manifest must not pretend to redirect Cargo to another workspace. The
microservices workspace remains authoritative for service versioning,
generation, build tooling, CI, and releases.

This decision preserves current generator and release contracts while making
Cargo commands deterministic. It does not prevent a later unification, but such
a change must update tooling and CI as an explicit migration rather than relying
on unsupported nested-workspace behavior.

## Decision 2: Lock Deployable Dependencies

The root workspace currently contains the shared `rerp-entities` library and
does not commit its lockfile. The `microservices` workspace contains deployable
binaries and commits `microservices/Cargo.lock`.

The service lockfile is the reproducible record of Git revisions, including the
Microscaler `may_minihttp` fork. Manifest source policy remains authoritative;
the lockfile prevents an unchanged service commit from resolving a different
runtime graph in CI or a release build.

## Development Baseline Commands

Run these commands from the repository root on the supported build host:

```bash
cargo metadata --no-deps --format-version 1
cargo check -p rerp-entities --lib
cargo metadata --manifest-path microservices/Cargo.toml --no-deps --format-version 1
cargo check --manifest-path microservices/Cargo.toml -p rerp_accounting_invoice
cargo check --manifest-path microservices/Cargo.toml \
  -p rerp_accounting_edi_gen -p rerp_accounting_bff_gen
```

The final command validates contract generation only; it does not assert that
the BFF implementation is runtime-ready.

## Verified Progress

Verified on `ms02` on 2026-07-14:

- root metadata reports `rerp-entities` as the sole root workspace member;
- `rerp-entities` compiles and its build discovers 37 entities;
- the invoice implementation compiles against the rustls-capable
  `may_minihttp` fork at revision `fbf23621`;
- EDI and accounting BFF generated crates compile after BRRTRouter learned to
  emit OpenAPI string enums;
- the dependency update removed the duplicate crates.io `may_minihttp`,
  AWS-LC, and Reqwest resolution paths from the service lockfile.

The broad `cargo check --workspace` gate is not yet green. It now reaches the
stale accounting BFF implementation and reports 728 errors from obsolete
generated example controllers, including handlers that are absent from the
current 320-operation BFF contract. This is evidence for Goal 2's narrow
runtime work, not a reason to regenerate hundreds of fake implementations.

CI currently runs `cargo test --workspace --lib`, which does not compile these
binary implementations. Goal 1 remains active until CI has an explicit active
binary gate and a clean-checkout dependency strategy for sibling Microscaler
repositories.

## Questions To Thrash Out

- Which generated artifacts are committed, and which are reproducible build outputs?
- What is the supported minimum toolchain and platform matrix?
- How are sibling Microscaler dependencies pinned for open-source consumers?
- Which checks are required locally, in CI, and before release?

## Dependencies

None. This is the first execution goal.
