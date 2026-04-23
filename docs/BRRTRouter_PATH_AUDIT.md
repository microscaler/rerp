# BRRTRouter and lifeguard path audit

Local development uses `../../BRRTRouter`, `../../lifeguard` (and variants) so we can change those repos and iterate. CI does not have those paths; we patch Cargo path deps → git in CI.

## 1. Cargo.toml – path dependencies (PATCH in CI)

### BRRTRouter

`brrtrouter` / `brrtrouter_macros` with `path = "…/BRRTRouter…"` →  
`{ git = "https://github.com/microscaler/BRRTRouter", branch = "main" }`.

| File | Role | Path form |
|------|------|-----------|
| `components/Cargo.toml` | [workspace.dependencies] | `path = "../../BRRTRouter"` |
| `microservices/Cargo.toml` | [workspace.dependencies] | `path = "../../BRRTRouter"` |
| `microservices/accounting/*/Cargo.toml` | If codegen emits a direct path | `path = "../../../BRRTRouter"` or similar |

### Lifeguard

`lifeguard`, `lifeguard-derive`, `lifeguard-migrate` with `path = "…/lifeguard…"` →  
`{ git = "https://github.com/microscaler/lifeguard", branch = "main" }`.

| File | Role | Path form |
|------|------|-----------|
| `microservices/Cargo.toml` | [workspace.dependencies] | `path = "../../lifeguard"`, `../../lifeguard/lifeguard-derive` |
| `entities/Cargo.toml` | [dependencies], [build-dependencies] | `path = "../../lifeguard"`, `../../lifeguard/lifeguard-derive`, `../../lifeguard/lifeguard-migrate` |

All member crates use `brrtrouter = { workspace = true }` and inherit from the workspace; they are **not** patched.

## 2. Cargo.toml – workspace only (no patch)

| File pattern | Content | Action |
|--------------|---------|--------|
| `components/**/Cargo.toml` (except `components/Cargo.toml`) | `brrtrouter = { workspace = true }` | None |
| `microservices/accounting/general-ledger/Cargo.toml` | `brrtrouter = { workspace = true }` | None |

## 3. Non‑Cargo: BRRTRouter paths (local / Tilt only, not used in CI)

| File | Usage | Path | CI? |
|------|-------|------|-----|
| `Tiltfile` | `brrtrouter-gen` binary and `--manifest-path` | `../BRRTRouter` | No – Tilt not run in CI |
| `scripts/fix_cargo_toml_paths.py` | Sets `path` in **generated** microservice Cargo.toml | Computed `../../../BRRTRouter` from service dir | No – run only during local/Tilt codegen |
| `scripts/bootstrap_microservice.py` | Resolves `brrtrouter-gen` and BRRTRouter manifest | `project_root.parent / "BRRTRouter"` | No – local bootstrap only |

## 4. Documentation (no patch)

| File | Content |
|------|---------|
| `components/SETUP_COMPLETE.md` | `../../BRRTRouter` in prose |
| `components/STRUCTURE.md` | Example `brrtrouter = { path = "../../BRRTRouter" }` |
| `components/README.md` | `../BRRTRouter` in prose |
| `README.md` | BRRTRouter links and examples |

## 5. Cargo.lock

After patching any Cargo.toml, the corresponding workspace’s `Cargo.lock` can still reference `path+file:///.../BRRTRouter` or `.../lifeguard`.  
**Action:** run in that workspace:

```bash
cargo update -p brrtrouter -p brrtrouter_macros
cargo update -p lifeguard -p lifeguard-derive -p lifeguard-migrate
```

The patch script runs these for `components/` and `microservices/` when it changes their Cargo.toml (or `entities/`, which is a dep of `microservices/`).

## 6. Patch script

- **Script:** `scripts/patch-brrtrouter-for-ci.py`
- **Discovery:** all `**/Cargo.toml` under the repo
- **Patterns:**  
  - BRRTRouter: `(brrtrouter|brrtrouter_macros)\s*=\s*\{\s*path\s*=\s*"[^"]*BRRTRouter[^"]*"[^}]*\}`  
  - Lifeguard: `(lifeguard|lifeguard-derive|lifeguard-migrate)\s*=\s*\{\s*path\s*=\s*"[^"]*lifeguard[^"]*"[^}]*\}`
- **Replace:** BRRTRouter → `{ git = "https://github.com/microscaler/BRRTRouter", branch = "main" }`; lifeguard → `{ git = "https://github.com/microscaler/lifeguard", branch = "main" }`
- **Modes:** default patch; `--audit` (list only); `--dry-run` (show changes, no write)
- **Post‑patch:** `cargo update -p brrtrouter -p brrtrouter_macros` and `-p lifeguard -p lifeguard-derive -p lifeguard-migrate` in `components/` and `microservices/` when those workspaces (or `entities/`) were patched

## 7. Path variants to match

**BRRTRouter:** `../../BRRTRouter`, `../../BRRTRouter/brrtrouter_macros`, `../BRRTRouter`, `../../../BRRTRouter`, trailing comma, optional `version`.

**Lifeguard:** `../../lifeguard`, `../../lifeguard/lifeguard-derive`, `../../lifeguard/lifeguard-migrate`, with or without `version = "0.1.0"`.

All are matched by the regex; no hardcoded path list.
