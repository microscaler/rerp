# Goal 1: Reproducible Development Baseline

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

## Questions To Thrash Out

- Should the root workspace own entities and all suites, or should suite workspaces remain intentionally separate?
- Which generated artifacts are committed, and which are reproducible build outputs?
- What is the supported minimum toolchain and platform matrix?
- How are sibling Microscaler dependencies pinned for open-source consumers?
- Which checks are required locally, in CI, and before release?

## Dependencies

None. This is the first execution goal.
