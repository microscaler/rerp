# Components Removal and Microservices Reorganization

## Executive Summary

This document outlines the plan to:
1. **Remove** the `./components/` directory completely
2. **Reorganize** `./microservices/` to use `gen/` and `impl/` subdirectories
3. **Update** all references from `components/` to `microservices/`
4. **Filter** `./components` from git history

## Current State Analysis

### Current `./microservices/accounting/` Structure

Currently, each service has a flat structure with both generated and implementation code mixed:

```
microservices/accounting/{service}/
├── Cargo.toml
├── config/
│   └── config.yaml
├── doc/
│   ├── index.html
│   └── openapi.yaml
├── src/
│   ├── main.rs              # Generated entry point
│   ├── lib.rs               # Generated library root
│   ├── registry.rs          # Generated registry
│   ├── handlers/            # Generated handlers (from OpenAPI)
│   │   ├── mod.rs
│   │   ├── types.rs
│   │   └── {operation}.rs
│   └── controllers/         # Business logic (manually written)
│       ├── mod.rs
│       └── {operation}.rs
└── static_site/
    └── index.html
```

### Target Structure

The new structure separates generated code from implementation:

```
microservices/{suite}/{service}/
├── gen/                     # Generated code (from OpenAPI spec)
│   ├── Cargo.toml
│   ├── doc/
│   │   ├── index.html
│   │   └── openapi.yaml
│   └── src/
│       ├── main.rs          # Generated entry point
│       ├── lib.rs           # Generated library root
│       ├── registry.rs      # Generated registry
│       ├── handlers/        # Generated handlers
│       │   ├── mod.rs
│       │   ├── types.rs
│       │   └── {operation}.rs
│       └── controllers/     # Generated controller stubs (overwritten by brrtrouter)
│           ├── mod.rs
│           └── {operation}.rs
├── impl/                    # Business logic (not overwritten)
│   ├── Cargo.toml
│   ├── config/
│   │   └── config.yaml
│   └── src/
│       ├── main.rs          # Implementation entry point (uses gen crate)
│       └── controllers/     # Business logic implementations
│           ├── mod.rs
│           └── {operation}.rs
└── static_site/            # Static assets (if needed)
    └── index.html
```

## Services Inventory

### Accounting Suite (Current)

| Service | Current Path | Gen Path | Impl Path | Crate Name (gen) | Crate Name (impl) |
|---------|-------------|----------|-----------|------------------|-------------------|
| general-ledger | `microservices/accounting/general-ledger/` | `microservices/accounting/general-ledger/gen/` | `microservices/accounting/general-ledger/impl/` | `rerp_accounting_general_ledger_gen` | `rerp_accounting_general_ledger` |
| invoice | `microservices/accounting/invoice/` | `microservices/accounting/invoice/gen/` | `microservices/accounting/invoice/impl/` | `rerp_accounting_invoice_gen` | `rerp_accounting_invoice` |
| accounts-receivable | `microservices/accounting/accounts-receivable/` | `microservices/accounting/accounts-receivable/gen/` | `microservices/accounting/accounts-receivable/impl/` | `rerp_accounting_accounts_receivable_gen` | `rerp_accounting_accounts_receivable` |
| accounts-payable | `microservices/accounting/accounts-payable/` | `microservices/accounting/accounts-payable/gen/` | `microservices/accounting/accounts-payable/impl/` | `rerp_accounting_accounts_payable_gen` | `rerp_accounting_accounts_payable` |
| bank-sync | `microservices/accounting/bank-sync/` | `microservices/accounting/bank-sync/gen/` | `microservices/accounting/bank-sync/impl/` | `rerp_accounting_bank_sync_gen` | `rerp_accounting_bank_sync` |
| asset | `microservices/accounting/asset/` | `microservices/accounting/asset/gen/` | `microservices/accounting/asset/impl/` | `rerp_accounting_asset_gen` | `rerp_accounting_asset` |
| budget | `microservices/accounting/budget/` | `microservices/accounting/budget/gen/` | `microservices/accounting/budget/impl/` | `rerp_accounting_budget_gen` | `rerp_accounting_budget` |
| edi | `microservices/accounting/edi/` | `microservices/accounting/edi/gen/` | `microservices/accounting/edi/impl/` | `rerp_accounting_edi_gen` | `rerp_accounting_edi` |
| financial-reports | `microservices/accounting/financial-reports/` | `microservices/accounting/financial-reports/gen/` | `microservices/accounting/financial-reports/impl/` | `rerp_accounting_financial_reports_gen` | `rerp_accounting_financial_reports` |
| bff | `microservices/accounting/bff/` | `microservices/accounting/bff/gen/` | `microservices/accounting/bff/impl/` | `rerp_accounting_bff_gen` | `rerp_accounting_bff` |

**Total: 10 services in accounting suite**

### File Movement Details

For each service, the following files need to be moved:

#### Files Moving to `gen/`:
- `src/handlers/` → `gen/src/handlers/`
- `src/lib.rs` → `gen/src/lib.rs`
- `src/registry.rs` → `gen/src/registry.rs`
- `src/main.rs` → `gen/src/main.rs` (temporary, will be replaced)
- `src/controllers/` → `gen/src/controllers/` (generated stubs)
- `doc/` → `gen/doc/`
- `static_site/` → `gen/static_site/` (if exists)
- `Cargo.toml` → `gen/Cargo.toml` (modified for library crate)

#### Files Moving to `impl/`:
- `config/` → `impl/config/`
- `src/controllers/` → `impl/src/controllers/` (copy from gen, then preserve)
- New `impl/src/main.rs` (created, uses gen crate)
- New `impl/Cargo.toml` (created, binary crate depending on gen)

## Migration Script Plan

### Python Script Structure

The migration should be implemented as a Python script in `tooling/src/rerp_tooling/migration/reorganize_microservices.py` that:

1. Takes a service name and suite as arguments
2. Performs all file moves and directory creation
3. Generates new Cargo.toml files
4. Updates workspace Cargo.toml
5. Validates the migration

### Step 1: Reorganize Each Service

For each service in `microservices/accounting/`, perform:

```bash
# Example for general-ledger
SERVICE="general-ledger"
SUITE="accounting"
BASE="microservices/${SUITE}/${SERVICE}"

# 1. Create gen/ directory structure
mkdir -p "${BASE}/gen/src/handlers"
mkdir -p "${BASE}/gen/src/controllers"
mkdir -p "${BASE}/gen/doc"
mkdir -p "${BASE}/gen/static_site"

# 2. Move generated code to gen/
mv "${BASE}/src/handlers" "${BASE}/gen/src/"
mv "${BASE}/src/lib.rs" "${BASE}/gen/src/"
mv "${BASE}/src/registry.rs" "${BASE}/gen/src/"
mv "${BASE}/src/main.rs" "${BASE}/gen/src/"
mv "${BASE}/doc" "${BASE}/gen/"
mv "${BASE}/static_site" "${BASE}/gen/" 2>/dev/null || true

# 3. Move generated controller stubs to gen/src/controllers
mv "${BASE}/src/controllers" "${BASE}/gen/src/controllers" 2>/dev/null || mkdir -p "${BASE}/gen/src/controllers"

# 4. Create impl/ directory structure
mkdir -p "${BASE}/impl/src/controllers"
mkdir -p "${BASE}/impl/config"

# 5. Move implementation code to impl/
# Copy controllers from gen to impl (they will be overwritten in gen, but preserved in impl)
cp -r "${BASE}/gen/src/controllers"/* "${BASE}/impl/src/controllers/" 2>/dev/null || true
mv "${BASE}/config" "${BASE}/impl/"

# 6. Create impl/src/main.rs that uses gen crate
# (This will be generated by script)

# 7. Update Cargo.toml files
# gen/Cargo.toml: Library crate for generated code
# impl/Cargo.toml: Binary crate that depends on gen crate
```

### Step 2: Update Cargo.toml Files

**gen/Cargo.toml** (Library crate):
```toml
[package]
name = "rerp_accounting_general_ledger_gen"
version = "0.1.2"
edition = "2021"

[lib]
name = "rerp_accounting_general_ledger_gen"
path = "src/lib.rs"

[dependencies]
# Same as current workspace dependencies
```

**impl/Cargo.toml** (Binary crate):
```toml
[package]
name = "rerp_accounting_general_ledger"
version = "0.1.2"
edition = "2021"

[[bin]]
name = "rerp_accounting_general_ledger"
path = "src/main.rs"

[dependencies]
rerp_accounting_general_ledger_gen = { path = "../gen" }
# Other dependencies from workspace
```

**microservices/Cargo.toml** (Workspace):
```toml
[workspace]
members = [
    "accounting/general-ledger/gen",
    "accounting/general-ledger/impl",
    "accounting/invoice/gen",
    "accounting/invoice/impl",
    # ... etc for all services
]
```

### Step 3: Update impl/src/main.rs

The impl main.rs should:
1. Use the gen crate for handlers and registry
2. Import controllers from impl/src/controllers
3. Register controllers with the dispatcher

**Template for impl/src/main.rs:**
```rust
// ⚠️ This file uses generated code from the gen crate
// ⚠️ Business logic controllers are in impl/src/controllers/

use rerp_accounting_general_ledger_gen::*;
use rerp_accounting_general_ledger_gen::handlers::*;
use rerp_accounting_general_ledger_gen::registry::*;

// Import implementation controllers (business logic)
mod controllers;
use controllers::*;

// Copy the rest of main.rs from gen/src/main.rs
// but ensure controllers are imported from impl/src/controllers
// The dispatcher registration should use controllers from impl module
```

**Key Changes:**
- Import gen crate instead of local handlers
- Import controllers from `impl/src/controllers` module
- Keep all the server setup code from current main.rs
- Ensure controller registration uses impl controllers

### Step 4: Update Controller Files

Each controller in `impl/src/controllers/{operation}.rs` needs to import from gen crate:

**Before (current):**
```rust
use crate::handlers::create_account::{Request, Response};
```

**After (new structure):**
```rust
use rerp_accounting_general_ledger_gen::handlers::create_account::{Request, Response};
```

The controller implementation stays the same, only imports change.

## Files Referencing `components/`

### Documentation Files (Remove/Update)
- `./components/README.md` - **DELETE**
- `./components/STRUCTURE.md` - **DELETE**
- `./components/SETUP_COMPLETE.md` - **DELETE**
- `./AGENT.md` - **UPDATE** (line 17: remove `components/` reference)
- `./CONTRIBUTING.md` - **UPDATE** (remove components references)
- `./README.md` - **UPDATE** (remove components references)
- `./docs/MICROSERVICES_VS_COMPONENTS_ANALYSIS.md` - **DELETE** (superseded by this doc)

### Configuration Files (Update)
- `./Cargo.toml` - **UPDATE** (line 2: change `members = ["components"]` to `members = ["microservices"]`)
- `./components/Cargo.toml` - **DELETE** (entire directory)

### Tooling Files (Update)
- `./tooling/src/rerp_tooling/release/bump.py` - **UPDATE** (line 167: remove components from search)
- `./tooling/src/rerp_tooling/cli/main.py` - **UPDATE** (any components references)
- `./tooling/src/rerp_tooling/cli/pre_commit.py` - **UPDATE** (if it checks components)
- `./tooling/src/rerp_tooling/docker/unpack_build_bins.py` - **UPDATE** (if references components)
- `./tooling/src/rerp_tooling/docker/copy_multiarch.py` - **UPDATE** (if references components)
- `./tooling/src/rerp_tooling/ci/patch_brrtrouter.py` - **UPDATE** (if references components)
- `./tooling/src/rerp_tooling/bff/generate_system.py` - **UPDATE** (if references components)

### Test Files (Update)
- `./tooling/tests/test_release_bump.py` - **UPDATE** (line 232: remove components/Cargo.toml)
- `./tooling/tests/test_build_host_aware.py` - **UPDATE** (if tests components)
- `./tooling/tests/test_docker_copy_multiarch.py` - **UPDATE** (if tests components)
- `./tooling/tests/test_bff_generate_system.py` - **UPDATE** (if tests components)

### OpenAPI Files (Update)
- All `openapi/**/openapi.yaml` files that reference `components/` in `$ref` paths - **UPDATE**

### UI Files (Update)
- `./ui/website/src/App.tsx` - **UPDATE** (if imports from components)

### Justfile (Update)
- `./justfile` - **UPDATE** (any components references)

### CI/CD Files (Update)
- `./.github/workflows/ci.yml` - **UPDATE** (multiple references to components)
- `./.github/workflows/README.md` - **UPDATE** (documentation references)
- `./.github/workflows/release.yml` - **VERIFY** (uses tooling that may reference components)
- `./.github/workflows/deploy-website.yml` - **VERIFY** (likely no changes needed)
- `./.github/workflows/base-images.yml` - **VERIFY** (likely no changes needed)

## Git History Filtering

To remove `./components` from git history:

```bash
# WARNING: This rewrites history. Coordinate with team before running.

# 1. Use git filter-repo (recommended) or git filter-branch
git filter-repo --path components --invert-paths

# OR using git filter-branch (older method):
git filter-branch --force --index-filter \
  "git rm -rf --cached --ignore-unmatch components" \
  --prune-empty --tag-name-filter cat -- --all

# 2. Force push (coordinate with team!)
git push origin --force --all
git push origin --force --tags
```

**Important Notes:**
- This rewrites git history
- All team members must re-clone or reset their local repos
- Coordinate with team before executing
- Consider creating a backup branch first

## Migration Checklist

### Phase 1: Preparation
- [ ] Create backup branch
- [ ] Document current state
- [ ] Notify team of upcoming changes

### Phase 2: Reorganization
- [ ] Create migration script for each service
- [ ] Test migration on one service (general-ledger)
- [ ] Verify build works after reorganization
- [ ] Migrate all accounting services
- [ ] Update microservices/Cargo.toml workspace members

### Phase 3: Remove Components
- [ ] Update all files referencing `components/`
- [ ] Remove `./components/` directory
- [ ] Update root `Cargo.toml` to reference `microservices`
- [ ] Update all documentation

### Phase 4: Git History
- [ ] Coordinate with team
- [ ] Filter git history to remove components
- [ ] Force push (with team approval)
- [ ] Update all team members

### Phase 5: Verification
- [ ] Verify all builds pass
- [ ] Verify all tests pass
- [ ] Verify CI/CD works
- [ ] Verify documentation is updated
- [ ] Verify no remaining references to `components/`

## Implementation Notes

### BRRTRouter Generation

After reorganization, BRRTRouter should:
1. Generate code into `microservices/{suite}/{service}/gen/`
2. Generate controller stubs into `gen/src/controllers/` (overwritable)
3. **NOT** overwrite `impl/src/controllers/` (preserved business logic)

### Crate Naming Convention

- **Generated crate**: `rerp_{suite}_{service}_gen` (e.g., `rerp_accounting_general_ledger_gen`)
- **Implementation crate**: `rerp_{suite}_{service}` (e.g., `rerp_accounting_general_ledger`)

### Controller Import Pattern

In `impl/src/controllers/{operation}.rs`:
```rust
// Import generated handler from gen crate
use rerp_accounting_general_ledger_gen::handlers::create_account::*;

// Implement business logic
pub async fn create_account(...) -> Result<...> {
    // Business logic here
    // Can call gen crate types and utilities
}
```

## GitHub Actions Workflow Updates

### Summary of Required Changes

| Workflow File | Changes Required | Priority | Notes |
|--------------|------------------|----------|-------|
| `ci.yml` | **11 updates** | **CRITICAL** | Version source, cache paths/keys, build commands, artifact paths |
| `README.md` | **3 updates** | High | Documentation only |
| `release.yml` | **0 updates** | Medium | Workflow fine, but tooling (`bump.py`) needs update |
| `deploy-website.yml` | **0 updates** | Low | No changes needed |
| `base-images.yml` | **0 updates** | Low | No changes needed |
| `Tiltfile` | **10+ updates** | **CRITICAL** | Code gen paths, build deps, live update syncs, binary paths |
| `bootstrap/microservice.py` | **5+ updates** | **CRITICAL** | Directory creation, code gen paths, config paths |

### `.github/workflows/ci.yml` - Detailed Update Instructions

This is the main CI workflow that requires the most updates. Here are the specific changes needed:

#### Job: `vars` (Version Validation)

**Lines 54-57**: Update version source from `components/Cargo.toml` to `microservices/Cargo.toml`

**Before:**
```yaml
# Read current version from components/Cargo.toml
CURRENT=$(grep -A 5 '\[workspace.package\]' components/Cargo.toml | grep -E '^\s*version\s*=' | head -1 | sed -E 's/.*version\s*=\s*"v?([^"]+)".*/\1/' || echo "")
if [ -z "$CURRENT" ]; then
  echo "Warning: Could not read version from components/Cargo.toml, skipping validation"
```

**After:**
```yaml
# Read current version from microservices/Cargo.toml
CURRENT=$(grep -A 5 '\[workspace.package\]' microservices/Cargo.toml | grep -E '^\s*version\s*=' | head -1 | sed -E 's/.*version\s*=\s*"v?([^"]+)".*/\1/' || echo "")
if [ -z "$CURRENT" ]; then
  echo "Warning: Could not read version from microservices/Cargo.toml, skipping validation"
```

#### Job: `build-and-test` (Build and Test)

**Line 220**: Update cache path from `components/target` to `microservices/target`

**Before:**
```yaml
path: |
  ~/.cargo/registry
  ~/.cargo/git
  components/target
key: ${{ runner.os }}-cargo-rerp-${{ hashFiles('components/**/Cargo.toml', 'components/Cargo.lock') }}
```

**After:**
```yaml
path: |
  ~/.cargo/registry
  ~/.cargo/git
  microservices/target
key: ${{ runner.os }}-cargo-rerp-${{ hashFiles('microservices/**/Cargo.toml', 'microservices/Cargo.lock') }}
```

**Lines 248, 253, 258, 263**: Update all `cd components` commands to `cd microservices`

**Before:**
```yaml
- name: Format check
  run: |
    cd components
    cargo fmt --check || (echo "❌ Code formatting check failed. Run 'cargo fmt' to fix." && exit 1)

- name: Lint with Clippy
  run: |
    cd components
    cargo clippy --workspace -- -D warnings

- name: Run tests
  run: |
    cd components
    cargo test --workspace --lib

- name: Check documentation
  run: |
    cd components
    cargo doc --workspace --no-deps || true
```

**After:**
```yaml
- name: Format check
  run: |
    cd microservices
    cargo fmt --check || (echo "❌ Code formatting check failed. Run 'cargo fmt' to fix." && exit 1)

- name: Lint with Clippy
  run: |
    cd microservices
    cargo clippy --workspace -- -D warnings

- name: Run tests
  run: |
    cd microservices
    cargo test --workspace --lib

- name: Check documentation
  run: |
    cd microservices
    cargo doc --workspace --no-deps || true
```

**Note**: After reorganization, the workspace structure changes. These commands may need to be updated to handle the `gen/` and `impl/` subdirectories. Consider:
- Running `cargo fmt` from `microservices/` root (workspace level)
- Or running separately for each suite: `cd microservices/accounting && cargo fmt --check`

#### Job: `build-multiarch` (Multi-Architecture Build)

**Line 308**: Update cache key hash files

**Before:**
```yaml
key: ${{ runner.os }}-cargo-registry-${{ hashFiles('components/**/Cargo.toml', 'components/Cargo.lock') }}
```

**After:**
```yaml
key: ${{ runner.os }}-cargo-registry-${{ hashFiles('microservices/**/Cargo.toml', 'microservices/Cargo.lock') }}
```

**Lines 315-316**: Update cache path and key

**Before:**
```yaml
path: components/target
key: ${{ runner.os }}-cargo-rerp-target-${{ matrix.architecture }}-${{ hashFiles('components/**/Cargo.toml', 'components/Cargo.lock') }}
```

**After:**
```yaml
path: microservices/target
key: ${{ runner.os }}-cargo-rerp-target-${{ matrix.architecture }}-${{ hashFiles('microservices/**/Cargo.toml', 'microservices/Cargo.lock') }}
```

**Line 348**: Update artifact upload path

**Before:**
```yaml
path: components/target/*/release/rerp_*
```

**After:**
```yaml
path: microservices/target/*/release/rerp_*
```

**Note**: After reorganization, binary paths may change. Verify the actual binary locations after migration:
- May be: `microservices/target/*/release/rerp_accounting_*_impl`
- Or: `microservices/accounting/*/impl/target/*/release/rerp_accounting_*`

#### Job: `download-copy-package-push-containers`

This job downloads artifacts and validates them. No direct `components/` references, but verify:
- Artifact names remain the same (`microservices-binaries-*`)
- Binary paths in `build_artifacts/` are correct

#### Job: `build-push-service` (Container Build and Push)

**Line 492**: Dockerfile path reference

**Before:**
```yaml
file: docker/microservices/Dockerfile.${{ matrix.service }}
```

**After:**
```yaml
# No change needed - Dockerfile paths remain the same
file: docker/microservices/Dockerfile.${{ matrix.service }}
```

**Note**: After reorganization, Dockerfiles may need updates to:
- Copy from `microservices/{suite}/{service}/impl/` instead of `microservices/{suite}/{service}/`
- Update binary paths if crate names change

### `.github/workflows/README.md` - Documentation Updates

**Line 15**: Update artifact description

**Before:**
```markdown
- **Build Multi-Architecture**: Builds workspace and microservices for amd64, arm64, arm7 via `cross`; uploads `rerp-binaries-*` (components) and `microservices-binaries-*` (amd64, arm64, arm) artifacts.
```

**After:**
```markdown
- **Build Multi-Architecture**: Builds workspace and microservices for amd64, arm64, arm7 via `cross`; uploads `rerp-binaries-*` (microservices) and `microservices-binaries-*` (amd64, arm64, arm) artifacts.
```

**Line 81**: Update version source documentation

**Before:**
```markdown
- **Bump, tag and push**: Checkout → bump (read `components/Cargo.toml`, compute next, write to all Cargo.toml) → **generate release notes** (commits since last tag → OpenAI or Anthropic per `provider` → `release-body.md`) → check for changes → commit `chore(release): vX.Y.Z` → tag `vX.Y.Z` → push (EndBug/add-and-commit) → **create GitHub Release** (softprops/action-gh-release) with `body_path: release-body.md`.
```

**After:**
```markdown
- **Bump, tag and push**: Checkout → bump (read `microservices/Cargo.toml`, compute next, write to all Cargo.toml) → **generate release notes** (commits since last tag → OpenAI or Anthropic per `provider` → `release-body.md`) → check for changes → commit `chore(release): vX.Y.Z` → tag `vX.Y.Z` → push (EndBug/add-and-commit) → **create GitHub Release** (softprops/action-gh-release) with `body_path: release-body.md`.
```

**Line 89**: Update version source documentation

**Before:**
```markdown
**Version source**: `components/Cargo.toml` `[workspace.package].version`. The same value is written to all `[package]` / `[workspace.package].version` in Cargo.toml across the repo, including the **root `Cargo.toml`** `[workspace.package].version` (which is explicitly kept in sync even if it has drifted).
```

**After:**
```markdown
**Version source**: `microservices/Cargo.toml` `[workspace.package].version`. The same value is written to all `[package]` / `[workspace.package].version` in Cargo.toml across the repo, including the **root `Cargo.toml`** `[workspace.package].version` (which is explicitly kept in sync even if it has drifted).
```

### `.github/workflows/release.yml` - Verification

This workflow uses `rerp release bump` which internally reads version from Cargo.toml files. Verify:

1. **Tooling Update Required**: The `rerp release bump` command in `tooling/src/rerp_tooling/release/bump.py` must be updated to read from `microservices/Cargo.toml` instead of `components/Cargo.toml`

2. **No Direct Changes**: The workflow file itself doesn't need changes, but the tooling it calls does.

### `.github/workflows/deploy-website.yml` - Verification

**Status**: ✅ **No changes needed** - This workflow only handles UI/website deployment and doesn't reference `components/` or `microservices/` directories.

### `.github/workflows/base-images.yml` - Verification

**Status**: ✅ **No changes needed** - This workflow only builds base Docker images and doesn't reference `components/` or `microservices/` directories.

## Tilt Configuration Updates

### `Tiltfile` - Detailed Update Instructions

The Tiltfile orchestrates local development builds and deployments. After reorganization, multiple paths need updates to reflect the `gen/` and `impl/` structure.

#### Function: `create_microservice_gen()` - Code Generation

**Lines 141-147**: Update output paths from service root to `gen/` subdirectory

**Before:**
```python
--output ./microservices/accounting/%s \
```

**After:**
```python
--output ./microservices/accounting/%s/gen \
```

**Lines 151-152**: Update Cargo.toml path check

**Before:**
```python
if [ -f ./microservices/accounting/%s/Cargo.toml ]; then
    tooling/.venv/bin/rerp ci fix-cargo-paths ./microservices/accounting/%s/Cargo.toml
fi
```

**After:**
```python
if [ -f ./microservices/accounting/%s/gen/Cargo.toml ]; then
    tooling/.venv/bin/rerp ci fix-cargo-paths ./microservices/accounting/%s/gen/Cargo.toml
fi
```

**Lines 162-165**: Update ignore patterns for generated code

**Before:**
```python
ignore=[
    './microservices/accounting/%s/src' % output_dir,  # Don't watch generated files
    './microservices/accounting/%s/doc' % output_dir,
    './microservices/accounting/%s/config' % output_dir,
    './microservices/accounting/%s/static_site' % output_dir,
],
```

**After:**
```python
ignore=[
    './microservices/accounting/%s/gen/src' % output_dir,  # Don't watch generated files
    './microservices/accounting/%s/gen/doc' % output_dir,
    './microservices/accounting/%s/impl/config' % output_dir,  # Config moved to impl
    './microservices/accounting/%s/gen/static_site' % output_dir,
],
```

#### Function: `create_microservice_build_resource()` - Build

**Line 228**: Update Cargo.toml dependency path

**Before:**
```python
deps=[
    './microservices/accounting/%s/Cargo.toml' % name,
    './microservices/accounting/%s/src' % name,
    'tooling/pyproject.toml',
],
```

**After:**
```python
deps=[
    './microservices/accounting/%s/gen/Cargo.toml' % name,  # Generated crate
    './microservices/accounting/%s/impl/Cargo.toml' % name,  # Implementation crate
    './microservices/accounting/%s/gen/src' % name,  # Generated source
    './microservices/accounting/%s/impl/src' % name,  # Implementation source
    'tooling/pyproject.toml',
],
```

**Note**: After reorganization, the build command may need to change:
- Current: `rerp build microservice {name}` builds from workspace
- May need: Build both `gen` and `impl` crates, or build from workspace root

**Line 247**: Update target path for binary location

**Before:**
```python
target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % package_name
```

**After:**
```python
# Binary is built from impl crate, but package name may change
# Verify actual binary name after reorganization
target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % package_name
# OR if impl crate has different name:
# target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s_impl' % package_name
```

**Note**: Package names may change. Verify:
- Gen crate: `rerp_accounting_{service}_gen`
- Impl crate: `rerp_accounting_{service}` (binary name)

#### Function: `create_microservice_deployment()` - Deployment

**Line 280**: Update dependencies for custom_build

**Before:**
```python
deps=[artifact_path, hash_path, 'microservices/accounting/%s/config' % name, 'microservices/accounting/%s/doc' % name, 'microservices/accounting/%s/static_site' % name],
```

**After:**
```python
deps=[
    artifact_path, 
    hash_path, 
    'microservices/accounting/%s/impl/config' % name,  # Config moved to impl
    'microservices/accounting/%s/gen/doc' % name,  # Doc in gen
    'microservices/accounting/%s/gen/static_site' % name,  # Static site in gen
],
```

**Lines 284-286**: Update live_update sync paths

**Before:**
```python
live_update=[
    sync(artifact_path, '/app/%s' % binary_name),
    sync('microservices/accounting/%s/config/' % name, '/app/config/'),
    sync('microservices/accounting/%s/doc/' % name, '/app/doc/'),
    sync('microservices/accounting/%s/static_site/' % name, '/app/static_site/'),
    run('kill -HUP 1', trigger=[artifact_path]),
],
```

**After:**
```python
live_update=[
    sync(artifact_path, '/app/%s' % binary_name),
    sync('microservices/accounting/%s/impl/config/' % name, '/app/config/'),  # Config in impl
    sync('microservices/accounting/%s/gen/doc/' % name, '/app/doc/'),  # Doc in gen
    sync('microservices/accounting/%s/gen/static_site/' % name, '/app/static_site/'),  # Static in gen
    run('kill -HUP 1', trigger=[artifact_path]),
],
```

#### Comment Updates

**Line 313**: Update comment about "components"

**Before:**
```python
# All accounting components: each has lint, gen, build, deploy.
```

**After:**
```python
# All accounting microservices: each has lint, gen, build, deploy.
```

**Line 332**: Update comment about workspace members

**Before:**
```python
# All accounting gens must complete before any build (so microservices/Cargo.toml members exist)
```

**After:**
```python
# All accounting gens must complete before any build (so microservices/Cargo.toml workspace members exist)
# After reorganization, workspace members are: accounting/{service}/gen and accounting/{service}/impl
```

### `tooling/src/rerp_tooling/tilt/setup.py` - Verification

**Line 18**: Directory creation

**Status**: ✅ **No changes needed** - Creates `microservices/accounting` directory which is still valid after reorganization (gen/ and impl/ are subdirectories)

### `tooling/src/rerp_tooling/tilt/teardown.py` - Verification

**Line 30**: Docker image removal

**Status**: ✅ **No changes needed** - Image names remain the same (`localhost:5001/rerp-accounting-{service}:tilt`)

### `tooling/src/rerp_tooling/bootstrap/microservice.py` - Update Required

**Line 309**: Crate directory path

**Before:**
```python
crate_dir = project_root / "microservices" / "accounting" / service_name
```

**After:**
```python
# Bootstrap creates both gen and impl directories
gen_dir = project_root / "microservices" / "accounting" / service_name / "gen"
impl_dir = project_root / "microservices" / "accounting" / service_name / "impl"
```

**Line 311**: Config path

**Before:**
```python
config_path = crate_dir / "config" / "config.yaml"
```

**After:**
```python
config_path = impl_dir / "config" / "config.yaml"
```

**Line 323**: Code generation

**Before:**
```python
generate_code_with_brrtrouter(spec_path, crate_dir, project_root)
```

**After:**
```python
# Generate code into gen directory
generate_code_with_brrtrouter(spec_path, gen_dir, project_root)
# Create impl directory structure
create_impl_structure(impl_dir, service_name, project_root)
```

**Line 325**: Cargo.toml path

**Before:**
```python
crate_cargo = crate_dir / "Cargo.toml"
```

**After:**
```python
gen_cargo = gen_dir / "Cargo.toml"
impl_cargo = impl_dir / "Cargo.toml"
# Fix paths for both
if gen_cargo.exists():
    run_fix_cargo_paths(gen_cargo, project_root)
if impl_cargo.exists():
    run_fix_cargo_paths(impl_cargo, project_root)
```

**Line 335**: Tiltfile update function

**Status**: ⚠️ **Needs update** - The `update_tiltfile()` function will need to handle new paths

### Tilt Post-Migration Verification Checklist

After updating Tiltfile, verify:

- [ ] Code generation outputs to `gen/` directory
- [ ] Build commands work with new crate structure
- [ ] Binary paths are correct (may be in `impl/target/` or workspace `target/`)
- [ ] Live update syncs work with new directory structure
- [ ] Config files sync from `impl/config/`
- [ ] Doc files sync from `gen/doc/`
- [ ] Static site files sync from `gen/static_site/`
- [ ] All services build and deploy correctly
- [ ] Tilt UI shows all resources correctly

### Cache Key Considerations

After reorganization, cache keys may need updates:

1. **Workspace Structure Change**: With `gen/` and `impl/` subdirectories, Cargo.toml locations change
2. **Cache Invalidation**: Existing caches will be invalid after migration (intentional - forces fresh builds)
3. **New Cache Keys**: Consider separate caches for:
   - `microservices/target` (workspace builds)
   - `microservices/accounting/*/gen/target` (generated code builds)
   - `microservices/accounting/*/impl/target` (implementation builds)

### Post-Migration Verification Checklist

After updating workflows, verify:

- [ ] All jobs run successfully
- [ ] Cache keys are correct and caches are being used
- [ ] Artifact uploads/downloads work correctly
- [ ] Binary paths in artifacts match actual build outputs
- [ ] Version validation reads from correct Cargo.toml
- [ ] Format/lint/test commands work with new structure
- [ ] Container builds use correct Dockerfile paths and binary locations

## Next Steps

1. Review and approve this plan
2. Create detailed migration script
3. Test on single service
4. Execute full migration
5. Update all references (including GitHub Actions)
6. Remove components directory
7. Filter git history (with team coordination)
8. Verify all CI/CD workflows pass
