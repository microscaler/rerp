# TASK: bring RERP's accounting crates back into the build

You are Hermes, a coding agent running on ms02 against a local model. This file
is your brief. Read all of it before you start, and re-read the section for the
phase you are on whenever you are unsure what to do next.

## Where you are

- Worktree: `/home/casibbald/Workspace/hermes-lab/rerp`, branch
  `hermes/accounting-build`. Work **only** here.
- BRRTRouter (the code generator) is the sibling clone
  `/home/casibbald/Workspace/hermes-lab/BRRTRouter`. It is **read-only** for you.
  The `rerp gen ...` tooling uses it automatically (`../BRRTRouter`), including
  its prebuilt `target/debug/brrtrouter-gen`.
- This overrides the "Desktop development environment" section of
  `AGENTS.md`: for this task do **not** use `~/Workspace/microscaler/rerp` or any
  other checkout, and do not create new checkouts or worktrees. Every other rule
  in `AGENTS.md` and `CONTRIBUTING.md` applies.

Shell setup (each terminal command starts fresh, so prefix it when needed):

```bash
source ~/.cargo/env; export PATH=$PWD/tooling/.venv/bin:$PATH CARGO_BUILD_JOBS=8
```

Cargo commands run from `microservices/` (the Rust workspace root).

## Background

Since commit b668b228, `microservices/Cargo.toml` lists only the CRM crates in
`default-members`, because the accounting crates do not compile (about 780
errors). Measured on this branch at 293d8f2d with
`cargo check --workspace --all-targets --keep-going` (errors per crate, counted
once): bff 671, accounts-payable 55, bank-sync 12, asset 11, edi 11, budget 7,
revenue-recognition 0. All `gen/` crates compile; the errors are in `impl/`.
There are three causes:

1. **Generator bug (fixed upstream).** BRRTRouter generated a body-less 2xx
   response's `Response` type from the error schema, so success responses got
   `code`/`details`/`message` fields. The fix lives in BRRTRouter; you only
   regenerate with the fixed generator. Do not work around this bug by hand.
2. **Gen drift.** Commit 525467c7 changed the OpenAPI specs (`nullable: true`
   became type lists such as `type: [string, "null"]`) without regenerating the
   `gen/` crates. The specs are correct; do not revert them.
3. **TODO handler stubs** in `impl/` that no longer match the regenerated
   `gen/` types.

## Hard rules

1. **Never hand-edit anything under `microservices/**/gen/`.** Generated code
   changes only by running the generator. If a compile error can only be fixed
   by editing `gen/` or BRRTRouter, stop and report it (see Stopping criteria).
2. Do not edit `openapi/**` specs, BRRTRouter, or anything outside this worktree.
3. Do not change the BRRTRouter `rev` in `microservices/Cargo.toml` or
   `tooling/pyproject.toml` unless the phase instructions below tell you the
   exact rev to use.
4. Real fixes only. Do not silence errors with `#[allow(...)]`, `todo!()`,
   `unimplemented!()`, `unreachable!()`, commenting code out, deleting tests,
   deleting crates or removing crates from `members`. Replacing a handler whose
   body is *only* a generated TODO stub with a freshly generated stub is fine.
   Before replacing a handler, check it really is a pure stub (no hand-written
   logic); if it has real logic, fix it by hand instead and keep the logic.
5. Git: commit on `hermes/accounting-build` only. Small commits, one logical
   step each (per crate or per phase), conventional-commit messages
   (`fix(accounting/asset): ...`, `chore(accounting): regenerate gen crates`).
   End every commit message with the line
   `Generated-by: Hermes Agent v0.21.5 (local Qwen3.8 via vLLM)`.
   Pre-commit hooks must pass: **never** use `--no-verify`, never set
   `SKIP=`, never edit `.pre-commit-config.yaml` or the hook scripts.
   Never `git push`, never `git reset --hard`, never rebase, never amend a
   commit you did not just make, never touch `main`.
6. Commit `Cargo.lock` changes that come from your work (this worktree has no
   local-path cargo patches, so the lock stays clean).
7. Keep a short progress log at `/home/casibbald/hrm/progress.md` (outside the
   repo; append, do not rewrite): what you did, the error counts per crate, and
   what is next. Update it after each commit. If you are restarted, read it
   first.
8. When code changes, append one concise entry to `docs/llmwiki/log.md` (an
   AGENTS.md rule). One entry for the whole task, in your final commit, is enough.

## Commands

Work out the package names with:

```bash
cd microservices && cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys;[print(p["name"],p["manifest_path"]) for p in json.load(sys.stdin)["packages"]]'
```

Verify:

```bash
cargo check -p <package> --all-targets 2>&1 | grep -E '^(error|warning: unused)' | sort | uniq -c   # quick count
cargo check -p <package> --all-targets --message-format short 2>&1 | grep -E '^[^ ]+: error' | head -40
cargo test  -p <package>
cargo check --workspace --all-targets          # whole workspace
cargo test  --workspace --lib --no-fail-fast   # same as `just test-rust`
cargo fmt --all --check
cd .. && just qa                                # tooling lint + tests
```

Regenerate (from the worktree root):

```bash
rerp gen suite accounting --service <service>    # regenerate one gen/ crate from its spec
rerp gen stubs accounting <service> --force      # regenerate impl/ handler stubs (only for pure TODO stubs)
```

Run `rerp gen suite --help` and `rerp gen stubs --help` once to confirm the exact
arguments before using them. Inspect `git diff --stat` after every generator
run, and make sure it changed only what you expected.

Builds are slow on this shared machine: prefer `cargo check -p <one crate>` while
iterating, and run the workspace-wide checks only at the end of a phase.

## Phases

### Phase W (warm-up, read-only): triage

`accounting/revenue-recognition` turned out to compile already on this branch, so
the warm-up is a read-only triage instead. **Change no files and make no
commits in this phase** (the progress log outside the repo is the exception).

For the four small failing crates (`rerp_accounting_budget`,
`rerp_accounting_asset`, `rerp_accounting_edi`, `rerp_accounting_bank_sync`):
1. Run `cargo check -p <package> --all-targets --message-format short` and list
   each distinct error (file:line and message).
2. Classify each one as cause 1 (generator bug: success `Response` has
   `code`/`details`/`message`), cause 2 (gen drift from the 525467c7 spec change),
   or cause 3 (impl stub out of date), with one line of evidence each: what the
   spec says, what `gen/` currently says, what `impl/` expects. Read the actual
   files; do not guess.
3. Say, per crate, which errors you expect to disappear after regeneration and
   which will need a hand fix in `impl/`.
Write the result to the progress log and as your final message. Then stop.

### Phase 1 (only when told the BRRTRouter fix rev): regenerate gen crates

You will be given the BRRTRouter revision that contains the generator fix.
1. Set that rev for `brrtrouter` and `brrtrouter_macros` in
   `microservices/Cargo.toml` and for `brrtrouter-tooling` in
   `tooling/pyproject.toml` (the operator has already checked out the same rev in
   `../BRRTRouter` and rebuilt `brrtrouter-gen`). Run `cargo update -p brrtrouter`
   from `microservices/` and commit.
2. Regenerate every accounting service's `gen/` crate with `rerp gen suite`.
   Commit the regeneration on its own (generated changes only).
3. Record per-crate error counts (`cargo check -p`) in the progress log.

### Phase 2: stubs and impl fixes, one crate at a time

Order: revenue-recognition (re-check), budget, asset, edi, bank-sync,
accounts-payable, bff. For each crate:
1. `cargo check -p <impl package> --all-targets`; read the errors.
2. Pure TODO stubs that no longer match the generated types: regenerate with
   `rerp gen stubs accounting <service> --force`, then review `git diff` and
   restore any hand-written code the regeneration overwrote.
3. Fix remaining errors by hand in `impl/` against the **generated** types
   (read the `gen/` source to learn the real type and field names; do not guess).
4. `cargo test -p <impl package>`; commit when it compiles and its tests pass.
Accounting crates that already compile must stay compiling.

### Phase 3: put them back into the default build

1. Add every accounting `gen` and `impl` crate (and `accounting/core`,
   `accounting/entities`) to `default-members` in `microservices/Cargo.toml`, and
   update the comment above it so it no longer says accounting does not compile.
   Leave `skaffold.yaml` alone, but mention in your final report whether its
   `BP_RUST_PACKAGES` also needs updating.
2. From `microservices/`: `cargo build` (bare, default members),
   `cargo check --workspace --all-targets`, `cargo test --workspace --lib --no-fail-fast`,
   `cargo fmt --all --check`. Then `just qa` from the root.
3. Add the `docs/llmwiki/log.md` entry and commit.

## Stopping criteria

Stop and write your final report (as your last message, and in the progress log)
when any of these is true:

- **Done:** the current phase's goal is met, verified by the commands above, and
  committed with a clean `git status`.
- **Blocked by rules:** the only fix you can find needs editing `gen/`, BRRTRouter,
  the specs, or bypassing pre-commit.
- **Stuck:** the same error survives three different fix attempts, or a crate's
  error count has not gone down after 30 minutes of work on it. Revert your
  uncommitted changes for that crate with `git checkout -- <paths you changed>`,
  record what you tried, and stop.
- **Environment problem:** the toolchain, generator or network fails in a way
  that is not about the code.

The final report lists: commits made (hash and subject), error counts per crate
before and after, test results, anything you skipped or were unsure about, and
anything a reviewer should look at closely.
