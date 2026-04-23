# Components Directory References - Update Checklist

This document lists all files that reference `components/` and need to be updated or removed.

## Files to DELETE

### Components Directory Itself
- `./components/` (entire directory)
  - `./components/README.md`
  - `./components/STRUCTURE.md`
  - `./components/SETUP_COMPLETE.md`
  - `./components/Cargo.toml`
  - All subdirectories and files under `./components/`

### Superseded Documentation
- `./docs/MICROSERVICES_VS_COMPONENTS_ANALYSIS.md` (superseded by COMPONENTS_REMOVAL_AND_MICROSERVICES_REORGANIZATION.md)

## Files to UPDATE

### Root Configuration
- [ ] `./Cargo.toml`
  - Line 2: Change `members = ["components"]` to `members = ["microservices"]`
  - Lines 9-10: Remove or update comment about components workspace

### Documentation Files
- [ ] `./AGENT.md`
  - Line 17: Remove `components/` from reference, keep only `microservices/`
  
- [ ] `./CONTRIBUTING.md`
  - Update any references to `components/` directory
  - Update build/test commands if they reference components
  
- [ ] `./README.md`
  - Remove references to `components/` directory
  - Update structure diagrams
  - Update build/test instructions
  
- [ ] `./agent/bugs.md`
  - Update any references to components (if present)

### Tooling Source Files
- [ ] `./tooling/src/rerp_tooling/release/bump.py`
  - Line 167: Update comment/docstring to remove components from search paths
  - Ensure it only searches `microservices/`, `entities/`, root `Cargo.toml`
  
- [ ] `./tooling/src/rerp_tooling/cli/main.py`
  - Remove any CLI commands or help text referencing components
  - Update any path references
  
- [ ] `./tooling/src/rerp_tooling/cli/pre_commit.py`
  - Update pre-commit hook logic if it checks components directory
  
- [ ] `./tooling/src/rerp_tooling/docker/unpack_build_bins.py`
  - Update if it references components target directory
  
- [ ] `./tooling/src/rerp_tooling/docker/copy_multiarch.py`
  - Update if it references components target directory
  
- [ ] `./tooling/src/rerp_tooling/ci/patch_brrtrouter.py`
  - Update if it references components directory
  
- [ ] `./tooling/src/rerp_tooling/bff/generate_system.py`
  - Update if it references components directory

#### `./tooling/src/rerp_tooling/bootstrap/microservice.py` - **UPDATE REQUIRED**
- [ ] **Line 309**: Update `crate_dir` to create both `gen/` and `impl/` directories
- [ ] **Line 311**: Update `config_path` to point to `impl/config/config.yaml`
- [ ] **Line 323**: Update code generation to output to `gen/` directory
- [ ] **Line 325**: Update Cargo.toml handling for both `gen/Cargo.toml` and `impl/Cargo.toml`
- [ ] **Line 335**: Update `update_tiltfile()` function to handle new paths
- [ ] Add function to create `impl/` directory structure with placeholder files

### Test Files
- [ ] `./tooling/tests/test_release_bump.py`
  - Line 232: Remove `components/Cargo.toml` from assertions
  - Update test expectations
  
- [ ] `./tooling/tests/test_cli.py`
  - Update any tests that reference components
  
- [ ] `./tooling/tests/test_build_host_aware.py`
  - Update if tests reference components
  
- [ ] `./tooling/tests/test_docker_copy_multiarch.py`
  - Update if tests reference components
  
- [ ] `./tooling/tests/test_docker_unpack_build_bins.py`
  - Update if tests reference components
  
- [ ] `./tooling/tests/test_ci_patch_brrtrouter.py`
  - Update if tests reference components
  
- [ ] `./tooling/tests/test_bff_generate_system.py`
  - Update if tests reference components

### CI/CD Configuration

#### `.github/workflows/ci.yml` - **CRITICAL UPDATES REQUIRED**

This file has **multiple references** that must be updated:

- [ ] **Line 54-57** (vars job): Change version source from `components/Cargo.toml` to `microservices/Cargo.toml`
- [ ] **Line 220** (build-and-test job): Update cache path from `components/target` to `microservices/target`
- [ ] **Line 221** (build-and-test job): Update cache key hash files from `components/**/Cargo.toml` to `microservices/**/Cargo.toml`
- [ ] **Line 248** (Format check): Change `cd components` to `cd microservices`
- [ ] **Line 253** (Clippy lint): Change `cd components` to `cd microservices`
- [ ] **Line 258** (Run tests): Change `cd components` to `cd microservices`
- [ ] **Line 263** (Check docs): Change `cd components` to `cd microservices`
- [ ] **Line 308** (build-multiarch job): Update cache key hash files from `components/**/Cargo.toml` to `microservices/**/Cargo.toml`
- [ ] **Line 315** (build-multiarch job): Update cache path from `components/target` to `microservices/target`
- [ ] **Line 316** (build-multiarch job): Update cache key hash files from `components/**/Cargo.toml` to `microservices/**/Cargo.toml`
- [ ] **Line 348** (Upload binaries): Update artifact path from `components/target/*/release/rerp_*` to `microservices/target/*/release/rerp_*`

**Note**: After reorganization with `gen/` and `impl/` structure, verify:
- Binary paths may change (e.g., `microservices/accounting/*/impl/target/*/release/rerp_*`)
- Workspace commands may need to run from suite level or handle subdirectories

#### `.github/workflows/README.md` - **DOCUMENTATION UPDATES**

- [ ] **Line 15**: Update artifact description from "rerp-binaries-* (components)" to "rerp-binaries-* (microservices)"
- [ ] **Line 81**: Update version source from `components/Cargo.toml` to `microservices/Cargo.toml` in documentation
- [ ] **Line 89**: Update version source documentation from `components/Cargo.toml` to `microservices/Cargo.toml`

#### `.github/workflows/release.yml` - **VERIFY TOOLING**

- [ ] **No direct changes needed** - Workflow file is fine
- [ ] **Tooling update required**: `tooling/src/rerp_tooling/release/bump.py` must read from `microservices/Cargo.toml` instead of `components/Cargo.toml`

#### `.github/workflows/deploy-website.yml` - **NO CHANGES**

- [ ] ✅ **No changes needed** - Only handles UI deployment

#### `.github/workflows/base-images.yml` - **NO CHANGES**

- [ ] ✅ **No changes needed** - Only handles base Docker images

#### `.pre-commit-config.yaml`

- [ ] Update hook names/descriptions if they reference components
- [ ] Verify pre-commit hooks work with new microservices structure

### Build Configuration

#### `./justfile`
- [ ] Update any commands referencing components
- [ ] Update fmt/test/build commands
- [ ] Verify workspace commands work with new structure

#### `./Tiltfile` - **CRITICAL UPDATES REQUIRED**

This file has **multiple references** that must be updated for the new `gen/` and `impl/` structure:

- [ ] **Line 141-147** (`create_microservice_gen`): Update `--output` path to include `/gen` subdirectory
- [ ] **Line 151-152** (`create_microservice_gen`): Update Cargo.toml path check to `gen/Cargo.toml`
- [ ] **Line 162-165** (`create_microservice_gen`): Update ignore patterns for `gen/` and `impl/` paths
- [ ] **Line 228** (`create_microservice_build_resource`): Update deps to include both `gen/Cargo.toml` and `impl/Cargo.toml`
- [ ] **Line 229** (`create_microservice_build_resource`): Update src paths to `gen/src` and `impl/src`
- [ ] **Line 247** (`create_microservice_deployment`): Verify target_path for binary location (may change with new crate names)
- [ ] **Line 280** (`create_microservice_deployment`): Update deps to use `impl/config`, `gen/doc`, `gen/static_site`
- [ ] **Line 284-286** (`create_microservice_deployment`): Update live_update sync paths for new structure
- [ ] **Line 313**: Update comment from "components" to "microservices"
- [ ] **Line 332**: Update comment about workspace members

**Note**: After reorganization, verify:
- Binary paths may change (check actual build output)
- Package names may change (gen vs impl crates)
- Build commands may need to target specific crates

### Docker Configuration
- [ ] `./docker/README.md`
  - Update documentation if it references components
  
- [ ] `./docker/website/README.md`
  - Update if it references components

### UI Files
- [ ] `./ui/website/src/App.tsx`
  - Update if it imports from components directory
  
- [ ] `./ui/website/README.md`
  - Update if it references components
  
- [ ] `./ui/shared/README.md`
  - Update if it references components
  
- [ ] `./ui/shared/DOCKER_BUILD.md`
  - Update if it references components
  
- [ ] `./ui/shared/footer/Footer.tsx`
  - Update if it references components
  
- [ ] `./ui/shared/index.ts`
  - Update if it exports from components

### OpenAPI Files (Check for $ref paths)
Most OpenAPI files likely reference `components/schemas` in `$ref` paths. These are **OpenAPI specification references** (not directory paths), so they should **NOT** be changed. However, verify:

- [ ] All `openapi/**/openapi.yaml` files
  - Check if any `$ref: "#/components/schemas/..."` references are actually file paths
  - OpenAPI `$ref` to `#/components/schemas/...` is correct and should remain
  - Only update if there are actual file path references to `./components/` directory

### Config Files (Check for actual references)
- [ ] `./microservices/accounting/*/config/config.yaml`
  - These may contain `components` in OpenAPI schema references (which is correct)
  - Only update if there are actual directory path references

## Files That Likely DON'T Need Updates

These files contain "components" but likely in different contexts:

- OpenAPI `$ref: "#/components/schemas/..."` - This is OpenAPI spec syntax, not a directory reference
- Config files with OpenAPI schema references
- UI component references (React/SolidJS components, not the directory)

## Verification Commands

After updates, verify no remaining references:

```bash
# Search for directory references (not OpenAPI schema refs)
grep -r "components/" --include="*.rs" --include="*.toml" --include="*.yaml" --include="*.yml" --include="*.py" --include="*.md" --include="*.ts" --include="*.tsx" . | grep -v "#/components" | grep -v "components/schemas" | grep -v "components/requestBodies" | grep -v "components/responses"

# Check for components directory in git
git ls-files | grep "^components/"

# Verify Cargo.toml workspace members
grep -A 5 "members" Cargo.toml
```

## Update Priority

1. **Critical** (breaks builds):
   - `./Cargo.toml` (workspace members)
   - `./microservices/Cargo.toml` (workspace structure)
   - CI/CD workflows

2. **High** (affects tooling):
   - Tooling source files
   - Test files
   - Build scripts (justfile, Tiltfile)

3. **Medium** (affects documentation):
   - Documentation files
   - README files

4. **Low** (cleanup):
   - Comments
   - Help text
   - UI files (if not actually using components directory)
