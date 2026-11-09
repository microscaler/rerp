# Rust lints

> **Status:** Active, 2026-11-09. **Applies to:** every hand-written crate in `microservices/` that opts in, and all of
> the CRM and accounting suites. **Enforced by:** `tooling/scripts/lint_rust.py`, run by the `Rust lint` job in
> `.github/workflows/checks.yml`.

## Where the rules live

| File | Holds |
|---|---|
| `microservices/Cargo.toml`, `[workspace.lints]` | The lint levels. A crate opts in with `[lints]` / `workspace = true`. |
| `microservices/clippy.toml` | Test relaxations and the banned calls (`disallowed-methods`). |
| `tooling/scripts/lint_rust.py` | The gate: the workspace compiles, opted-in crates are clippy-clean with `-D warnings`, and no hand-written CRM or accounting crate is left out. |

Generated crates (`gen/`) do not opt in. They keep clippy's defaults and are fixed in BRRTRouter's templates, never by
hand. Levels are `warn` so a local build still runs; CI turns every warning into a failure.

## What is held

| Rule | Why |
|---|---|
| clippy `pedantic`, minus the lints listed as `allow` in `Cargo.toml` with their reason | Catches lossy casts, needless clones, awkward signatures. |
| No `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!` in production code | A handler that panics takes its coroutine down; return an error the contract describes. Tests may panic (`clippy.toml`). |
| No `println!` / `eprintln!` in production code | Use `tracing` (`tracing::info!`, `warn!`, `error!` with fields). BRRTRouter's main sets up the subscriber that ships logs to OTLP. |
| No `dbg!` | Debug output does not belong in a commit. |
| No float arithmetic | Money is `rust_decimal::Decimal` (and a decimal string on the wire). f64 loses cents. |
| No `let _ =` on a `#[must_use]` value | Handle the error, or say in an `allow` why it cannot matter. |
| Every `#[allow(...)]` has a `reason = "..."` | An exception is a decision; the reason is its record. |
| Every `unsafe` block has a `// SAFETY:` comment | Today only BRRTRouter's coroutine registration needs `unsafe`. |
| No `std::thread::sleep` (`disallowed-methods`) | Handlers run on `may` coroutines; a blocking call stalls every coroutine on that worker. Code on its own OS thread says so in an `allow`. |
| Integer casts are exact (`try_from`, `unsigned_abs`, `from_ne_bytes`), not `as` | `as` silently wraps or truncates. |

## Exceptions in place (2026-11-09)

Each is an `allow` with its reason next to the code it covers; `grep -rn 'reason = ' microservices --include=*.rs`
lists them.

| Where | Allowed | Until |
|---|---|---|
| `main.rs` of the accounting services, `crm/accounts`, `crm/pipeline` (BRRTRouter's older main template) | prints, `unwrap`/`expect` at startup, length | Those mains move to `RunAppBuilder`, as the nine new CRM services already have. |
| `crm/pipeline` revenue arithmetic | f64 | Sales (CRM-SAL) replaces pipeline with decimal strings. |
| `crm/conversion` provisioning job | prints, `thread::sleep` | Never: it is a one-off Kubernetes Job whose stdout is its log. |
| `crm/accounts` intake consumer | `thread::sleep` | Never: it runs on its own OS thread. |
| Generated code included into hand-written crates (Lifeguard registry, `impl_registry`, stub imports) | pedantic, length, unused imports | Fixed in Lifeguard / BRRTRouter. |
| `accounting/bff` | skipped by the clippy step (`SKIP_CLIPPY` in the gate): clippy's `-D warnings` reaches its generated dependency `bff/gen`, where brrtrouter-gen leaves unused imports | BRRTRouter's generator stops emitting unused imports for BFF handlers and the crate is regenerated. |

## Running it

```bash
python3 tooling/scripts/lint_rust.py            # what CI runs
python3 tooling/scripts/lint_rust.py --list     # the crates held to these rules
cd microservices && cargo clippy -p <crate> --all-targets -- -D warnings   # one crate
```

To tighten further, change a level in `[workspace.lints]`, run the gate, and fix or justify what it reports in the same
commit. Do not loosen a level to get a change through; add a scoped `allow` with a reason instead.
