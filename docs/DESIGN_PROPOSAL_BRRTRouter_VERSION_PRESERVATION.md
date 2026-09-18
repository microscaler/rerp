# Design Proposal: Preserve Version Numbers in brrtrouter-gen Generated Cargo.toml Files

**Date:** 2025-01-26  
**Status:** Design Phase - Awaiting Review  
**Related Issues:** Version downgrade from 0.1.2 → 0.1.0 in microservice Cargo.toml files

---

## Executive Summary

When `brrtrouter-gen` regenerates microservice code (during bootstrap or Tiltfile regeneration), it overwrites the `version` field in `Cargo.toml` with a hardcoded default value (`"0.1.0"`). This causes version drift from the workspace version (currently `"0.1.2"`), breaking version consistency across the repository.

This document proposes a solution to use the **root `Cargo.toml` `[workspace.package].version`** as the single source of truth. The version is read from the root workspace and passed to `brrtrouter-gen` via a new `--version` CLI argument. If `--version` is not provided, `brrtrouter-gen` defaults to `"0.1.0"` (appropriate for new services).

**Key Design Decision**: Root `Cargo.toml` at the project root is the authoritative source, not individual microservice `Cargo.toml` files. This ensures all microservices always use the same version as the workspace.

---

## Problem Statement

### Current Behavior

1. **Initial Bootstrap**: When `rerp bootstrap microservice` runs, it calls `brrtrouter-gen` which generates a `Cargo.toml` with `version = "0.1.0"` (hardcoded default).

2. **Version Bump**: The release process (`rerp release bump`) updates all `Cargo.toml` files to match the workspace version (e.g., `"0.1.2"`).

3. **Regeneration**: When OpenAPI specs change and Tiltfile triggers regeneration, `brrtrouter-gen` runs again with `--force`, overwriting the `Cargo.toml` and resetting the version back to `"0.1.0"`.

4. **Result**: Version inconsistency between workspace (`0.1.2`) and microservices (`0.1.0`).

### Impact

- **Version Drift**: Microservice versions become out of sync with workspace version
- **Release Process**: Manual intervention required to restore versions after regeneration
- **CI/CD Risk**: Potential for incorrect versioning in releases if not caught
- **Developer Experience**: Confusing when versions mysteriously revert

### Evidence

```bash
# Current state (after regeneration):
microservices/Cargo.toml:           version = "0.1.2"  # Workspace
microservices/accounting/*/Cargo.toml: version = "0.1.0"  # Generated (wrong!)
```

---

## Root Cause Analysis

### Where Version Gets Overwritten

1. **Bootstrap Flow** (`tooling/src/rerp_tooling/bootstrap/microservice.py:261-294`):
   ```python
   def generate_code_with_brrtrouter(spec_path, output_dir, project_root):
       cmd = [brrtrouter_bin, "generate", "--spec", spec_path, 
              "--output", output_dir, "--force"]
       subprocess.run(cmd, ...)  # No version parameter passed
   ```

2. **Tiltfile Regeneration** (`Tiltfile:132-170`):
   ```python
   ../BRRTRouter/target/debug/brrtrouter-gen generate \
       --spec ./openapi/... \
       --output ./microservices/accounting/... \
       --force  # No version parameter
   ```

3. **BRRTRouter Template**: The Askama template in BRRTRouter that generates `Cargo.toml` likely has:
   ```toml
   version = "0.1.0"  # Hardcoded default
   ```

### Why This Was Acceptable Before

- Early development: All services started at `0.1.0`
- Manual versioning: Versions were managed manually
- No automation: Release process didn't exist

### Why It's a Problem Now

- Automated releases: `rerp release bump` updates all versions
- Frequent regeneration: Tiltfile regenerates on spec changes
- Version consistency: Required for proper dependency management and releases

---

## Solution Design

### Approach: Use Root Workspace Version as Source of Truth

The solution has two components:

1. **RERP Tooling**: Always read version from root `Cargo.toml` `[workspace.package].version`, pass it to `brrtrouter-gen`
2. **BRRTRouter**: Accept `--version` CLI argument and use it in the `Cargo.toml` template

### Design Principles

- **Single Source of Truth**: Root `Cargo.toml` `[workspace.package].version` is the authoritative version
- **Backward Compatible**: If `--version` is not provided, `brrtrouter-gen` defaults to `"0.1.0"` (assumes new service)
- **Non-Breaking**: Existing workflows continue to work
- **Explicit**: Version is explicitly passed from root workspace
- **Idempotent**: Regeneration with same version produces same result
- **Consistent**: All microservices always use the same version as the root workspace

---

## Proposed Implementation

### Phase 1: BRRTRouter Changes

#### 1.1 CLI Argument

Add `--version` flag to `brrtrouter-gen generate`:

```rust
// In BRRTRouter/src/bin/brrtrouter-gen/cli.rs or similar
#[derive(Parser)]
pub struct GenerateArgs {
    // ... existing args ...
    
    /// Version for generated Cargo.toml [package].version
    /// If not provided, defaults to "0.1.0"
    #[arg(long, default_value = "0.1.0")]
    pub version: String,
}
```

#### 1.2 Template Variable

Update the Askama template for `Cargo.toml`:

```toml
# Before:
version = "0.1.0"

# After:
version = "{{ version }}"
```

Where `version` comes from the CLI argument.

#### 1.3 Template Context

Pass version to template context:

```rust
// In BRRTRouter code generation logic
let context = TemplateContext {
    // ... existing fields ...
    version: args.version.clone(),
};
```

### Phase 2: RERP Tooling Changes

#### 2.1 Extract Version Function

Add function to read version from existing `Cargo.toml`:

```python
# In tooling/src/rerp_tooling/bootstrap/microservice.py

def _extract_version_from_cargo_toml(cargo_toml_path: Path) -> str | None:
    """Extract version from [package].version in Cargo.toml.
    
    Returns version string (without 'v' prefix) or None if not found.
    """
    if not cargo_toml_path.exists():
        return None
    
    text = cargo_toml_path.read_text()
    in_package = False
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("["):
            in_package = s.strip("[]").strip() == "package"
            continue
        if in_package:
            m = re.match(r'^\s*version\s*=\s*"v?([^"]+)"', line)
            if m:
                return m.group(1)
    return None
```

#### 2.2 Update Bootstrap Function

Modify `generate_code_with_brrtrouter` to use root workspace version:

```python
def generate_code_with_brrtrouter(
    spec_path: Path, 
    output_dir: Path, 
    project_root: Path
) -> None:
    """Generate code with brrtrouter-gen, using root workspace version."""
    brrtrouter_bin = project_root.parent / "BRRTRouter" / "target" / "debug" / "brrtrouter-gen"
    manifest = project_root.parent / "BRRTRouter" / "Cargo.toml"
    
    # Extract version from root Cargo.toml (single source of truth)
    workspace_version = _extract_root_workspace_version(project_root)
    
    # Build command with version (if available)
    base_cmd = [
        "generate",
        "--spec", str(spec_path),
        "--output", str(output_dir),
    ]
    
    # Only pass --version if we found it (brrtrouter-gen will default to "0.1.0" if not provided)
    if workspace_version:
        base_cmd.extend(["--version", workspace_version])
    
    base_cmd.append("--force")
    
    if brrtrouter_bin.exists():
        cmd = [str(brrtrouter_bin)] + base_cmd
    else:
        cmd = [
            "cargo", "run",
            "--manifest-path", str(manifest),
            "--bin", "brrtrouter-gen",
            "--",
        ] + base_cmd
    
    subprocess.run(cmd, check=True, capture_output=True, text=True, cwd=str(project_root))
    print("✅ Code generation complete")
```

#### 2.3 Update Tiltfile

Modify `create_microservice_gen` to use root workspace version:

```python
def create_microservice_gen(name, spec_file, output_dir):
    local_resource(
        '%s-service-gen' % name,
        cmd='''
            set -e
            echo "🔄 Regenerating %s service from OpenAPI spec..."
            
            # Extract version from root Cargo.toml (single source of truth)
            ROOT_CARGO="./Cargo.toml"
            VERSION=""
            if [ -f "$ROOT_CARGO" ]; then
                VERSION=$(grep -A 5 '^\[workspace.package\]' "$ROOT_CARGO" | \
                          grep -E '^\s*version\s*=' | \
                          head -1 | \
                          sed -E 's/.*version\s*=\s*"v?([^"]+)".*/\\1/' || echo "")
            fi
            
            # Build command - only add --version if we found it
            # (brrtrouter-gen will default to "0.1.0" if --version not provided)
            GEN_CMD="../BRRTRouter/target/debug/brrtrouter-gen generate \
                --spec ./openapi/%s \
                --output ./microservices/accounting/%s"
            
            if [ -n "$VERSION" ]; then
                GEN_CMD="$GEN_CMD --version $VERSION"
            fi
            
            GEN_CMD="$GEN_CMD --force"
            
            # Use the built debug binary directly for speed
            $GEN_CMD || \
            cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen -- \
                generate \
                --spec ./openapi/%s \
                --output ./microservices/accounting/%s \
                $(if [ -n "$VERSION" ]; then echo "--version $VERSION"; fi) \
                --force
            
            # Fix Cargo.toml paths to point to BRRTRouter repository
            echo "🔧 Fixing Cargo.toml dependency paths..."
            if [ -f ./microservices/accounting/%s/Cargo.toml ]; then
                tooling/.venv/bin/rerp ci fix-cargo-paths ./microservices/accounting/%s/Cargo.toml
            fi
            
            echo "✅ %s service regeneration complete"
        ''' % (name, spec_file, output_dir, spec_file, output_dir, 
               output_dir, output_dir, name),
        # ... rest of config ...
    )
```

---

## Rollout Plan

This section details the step-by-step rollout plan, including coordination between BRRTRouter (external dependency) and RERP (local changes).

### Overview

The rollout requires coordination because:
1. **BRRTRouter is external**: Located at `../BRRTRouter` (separate repository)
2. **Backward compatibility**: RERP changes must work with both old and new BRRTRouter versions
3. **Testing dependency**: RERP changes can't be fully tested until BRRTRouter supports `--version`

### Phase 1: BRRTRouter Changes (External Dependency)

**Location**: `../BRRTRouter` repository  
**Branch**: Create feature branch (e.g., `feat/version-arg`)  
**Estimated Effort**: 2-4 hours + testing + PR review

#### Step 1.1: Add CLI Argument

**File**: `BRRTRouter/src/bin/brrtrouter-gen/cli.rs` (or equivalent CLI definition)

```rust
#[derive(Parser)]
pub struct GenerateArgs {
    // ... existing args ...
    
    /// Version for generated Cargo.toml [package].version
    /// If not provided, defaults to "0.1.0"
    #[arg(long, default_value = "0.1.0")]
    pub version: String,
}
```

**Action Items**:
- [ ] Add `--version` argument with default value `"0.1.0"`
- [ ] Update help text/documentation
- [ ] Ensure argument is optional (backward compatible)

#### Step 1.2: Update Askama Template

**File**: `BRRTRouter/templates/Cargo.toml.askama` (or equivalent template location)

**Before**:
```toml
version = "0.1.0"
```

**After**:
```toml
version = "{{ version }}"
```

**Action Items**:
- [ ] Locate Cargo.toml template file
- [ ] Replace hardcoded `"0.1.0"` with `{{ version }}` variable
- [ ] Verify template syntax is correct

#### Step 1.3: Pass Version to Template Context

**File**: `BRRTRouter/src/bin/brrtrouter-gen/generate.rs` (or equivalent generation logic)

```rust
let context = TemplateContext {
    // ... existing fields ...
    version: args.version.clone(),
};
```

**Action Items**:
- [ ] Locate template context struct
- [ ] Add `version: String` field to context
- [ ] Pass `args.version` to context
- [ ] Update context struct definition

#### Step 1.4: Testing in BRRTRouter

**Test Cases**:
1. **Default behavior** (no `--version`):
   ```bash
   brrtrouter-gen generate --spec spec.yaml --output output/
   # Should generate version = "0.1.0"
   ```

2. **With `--version`**:
   ```bash
   brrtrouter-gen generate --spec spec.yaml --output output/ --version "0.1.2"
   # Should generate version = "0.1.2"
   ```

3. **With RC version**:
   ```bash
   brrtrouter-gen generate --spec spec.yaml --output output/ --version "0.1.2-rc.1"
   # Should generate version = "0.1.2-rc.1"
   ```

**Action Items**:
- [ ] Add unit tests for CLI argument parsing
- [ ] Add integration tests for template rendering
- [ ] Test backward compatibility (no `--version` flag)
- [ ] Verify generated Cargo.toml has correct version

#### Step 1.5: BRRTRouter Release

**Action Items**:
- [ ] Create PR in BRRTRouter repository
- [ ] Get code review approval
- [ ] Merge to `main` branch
- [ ] **Important**: Tag release or note commit SHA for RERP to reference
- [ ] Update BRRTRouter documentation/README

**Coordination Note**: RERP team needs to know:
- Commit SHA or release tag of BRRTRouter with `--version` support
- When BRRTRouter changes are merged/available

---

### Phase 2: RERP Local Changes (After BRRTRouter Support)

**Prerequisites**: 
- ✅ BRRTRouter supports `--version` flag (Phase 1 complete)
- ✅ BRRTRouter changes merged and available locally

**Location**: `rerp` repository (current branch: `feat/release-ci-integration`)  
**Estimated Effort**: 3-5 hours

#### Step 2.1: Update BRRTRouter Dependency (If Needed)

**If BRRTRouter uses git dependency in RERP**:

**File**: `microservices/Cargo.toml` or relevant workspace Cargo.toml

```toml
brrtrouter = { git = "https://github.com/microscaler/BRRTRouter", branch = "main" }
# OR if using specific commit:
# brrtrouter = { git = "https://github.com/microscaler/BRRTRouter", rev = "<commit-sha>" }
```

**Action Items**:
- [ ] Verify BRRTRouter git dependency points to updated branch/commit
- [ ] Run `cargo update` to fetch latest BRRTRouter
- [ ] Verify `brrtrouter-gen --help` shows `--version` flag

**If BRRTRouter is local path dependency**:

**Action Items**:
- [ ] Pull latest BRRTRouter changes: `cd ../BRRTRouter && git pull`
- [ ] Rebuild BRRTRouter: `cargo build --bin brrtrouter-gen`
- [ ] Verify `../BRRTRouter/target/debug/brrtrouter-gen --help` shows `--version` flag

#### Step 2.2: Add Version Extraction Function

**File**: `tooling/src/rerp_tooling/bootstrap/microservice.py`

**Action Items**:
- [ ] Add `_extract_root_workspace_version()` function (see Phase 2.1 in Implementation section)
- [ ] Add unit tests for version extraction:
  - Valid version in root Cargo.toml
  - Missing root Cargo.toml
  - Missing `[workspace.package]` section
  - Missing `version` field
  - Version with `v` prefix
  - RC version format
- [ ] Run tests: `pytest tooling/tests/test_bootstrap_microservice.py -v`

#### Step 2.3: Update Bootstrap Function

**File**: `tooling/src/rerp_tooling/bootstrap/microservice.py`

**Action Items**:
- [ ] Update `generate_code_with_brrtrouter()` to:
  - Call `_extract_root_workspace_version(project_root)`
  - Only add `--version` to command if version is found
  - Handle case where version is `None` (don't pass flag)
- [ ] Add integration test:
  - Mock `brrtrouter-gen` call
  - Verify `--version` is passed when root version exists
  - Verify `--version` is NOT passed when root version missing
- [ ] Test bootstrap flow:
  ```bash
  rerp bootstrap microservice test-service
  # Verify generated Cargo.toml has root workspace version
  ```

#### Step 2.4: Update Tiltfile

**File**: `Tiltfile`

**Action Items**:
- [ ] Update `create_microservice_gen()` function (see Phase 2.3 in Implementation section)
- [ ] Extract version from root `Cargo.toml` (not microservice Cargo.toml)
- [ ] Only add `--version` flag if version is found
- [ ] Test Tiltfile syntax: `tilt validate` (if available)
- [ ] Manual test: Trigger regeneration in Tilt and verify version preserved

#### Step 2.5: Testing & Validation

**Test Scenarios**:

1. **Bootstrap New Service** (no existing Cargo.toml):
   ```bash
   rerp bootstrap microservice new-service
   # Expected: Generated Cargo.toml has root workspace version (e.g., "0.1.2")
   ```

2. **Regenerate Existing Service** (with existing Cargo.toml):
   ```bash
   # Change OpenAPI spec
   # Tiltfile triggers regeneration
   # Expected: Version stays at root workspace version, not reset to "0.1.0"
   ```

3. **Version Bump Flow**:
   ```bash
   rerp release bump patch  # Updates root Cargo.toml to "0.1.3"
   # Regenerate service
   # Expected: Generated Cargo.toml has "0.1.3"
   ```

4. **Edge Case: Missing Root Cargo.toml**:
   ```bash
   # Temporarily rename Cargo.toml
   rerp bootstrap microservice test-service
   # Expected: Generated Cargo.toml has "0.1.0" (brrtrouter-gen default)
   ```

**Action Items**:
- [ ] Run all test scenarios above
- [ ] Verify no regressions in existing bootstrap flow
- [ ] Verify Tiltfile regeneration works correctly
- [ ] Check that version bump process still works

#### Step 2.6: Update Documentation

**Files to Update**:
- `tooling/README.md` - Document version preservation behavior
- `AGENT.md` - Update bootstrap instructions if needed
- `docs/DESIGN_PROPOSAL_BRRTRouter_VERSION_PRESERVATION.md` - Mark as implemented

**Action Items**:
- [ ] Document that microservice versions come from root workspace
- [ ] Document that regeneration preserves workspace version
- [ ] Add note about BRRTRouter version requirement

---

### Phase 3: Migration & Cleanup

**After Both Phases Complete**

#### Step 3.1: Fix Existing Version Drift

**Current State**: All microservice Cargo.toml files have `version = "0.1.0"` but root has `"0.1.2"`

**Action Items**:
- [ ] Run `rerp release bump patch` (or appropriate bump) to restore all versions
- [ ] OR manually update all microservice versions to match root
- [ ] Verify all microservice Cargo.toml files have correct version

#### Step 3.2: Verify End-to-End Flow

**Action Items**:
- [ ] Make a change to an OpenAPI spec
- [ ] Trigger Tiltfile regeneration
- [ ] Verify microservice Cargo.toml version stays at root workspace version
- [ ] Verify no version drift occurs

#### Step 3.3: Monitor & Validate

**Action Items**:
- [ ] Monitor for any version drift issues
- [ ] Verify bootstrap of new services uses root version
- [ ] Document any edge cases discovered

---

## Implementation Plan

### Step 1: BRRTRouter Changes (External Dependency)

**Location**: `../BRRTRouter` repository

1. Add `--version` CLI argument to `brrtrouter-gen generate`
2. Update Askama template to use `{{ version }}` variable
3. Pass version to template context
4. Test with default value (`"0.1.0"`) to ensure backward compatibility
5. Update BRRTRouter documentation

**Dependencies**: None (can be done independently)

**Estimated Effort**: 2-4 hours

### Step 2: RERP Bootstrap Tooling

**Location**: `tooling/src/rerp_tooling/bootstrap/microservice.py`

1. Add `_extract_version_from_cargo_toml()` function
2. Add `_extract_workspace_version()` function
3. Update `generate_code_with_brrtrouter()` to extract and pass version
4. Add unit tests for version extraction
5. Test bootstrap flow with existing microservice (should preserve version)

**Dependencies**: Step 1 (BRRTRouter must support `--version`)

**Estimated Effort**: 2-3 hours

### Step 3: Tiltfile Updates

**Location**: `Tiltfile`

1. Update `create_microservice_gen()` to extract version before regeneration
2. Add version extraction logic (shell script)
3. Pass `--version` to `brrtrouter-gen` commands
4. Test regeneration flow (change OpenAPI spec, verify version preserved)

**Dependencies**: Step 1 (BRRTRouter must support `--version`)

**Estimated Effort**: 1-2 hours

### Step 4: Testing & Validation

1. **Bootstrap New Service**: Verify version matches workspace
2. **Regenerate Existing**: Verify version preserved
3. **Version Bump**: Verify bump still works, regeneration preserves new version
4. **Edge Cases**: 
   - No existing Cargo.toml (new service)
   - Missing version in Cargo.toml
   - Invalid version format

**Estimated Effort**: 2-3 hours

---

## Edge Cases & Considerations

### Edge Case 1: Root Cargo.toml Not Found

**Solution**: Don't pass `--version` flag, let `brrtrouter-gen` default to `"0.1.0"`.

**Implementation**: Check `if root_cargo.exists()` before extracting version. If not found, skip `--version` flag.

### Edge Case 2: Missing Version in Root Cargo.toml

**Solution**: Don't pass `--version` flag, let `brrtrouter-gen` default to `"0.1.0"`.

**Implementation**: Return `None` from extraction function if version not found. Only add `--version` to command if value is not `None`.

### Edge Case 3: Version Format Mismatch

**Solution**: Normalize version (strip `v` prefix) before passing to `brrtrouter-gen`.

**Implementation**: Use regex to extract version, always strip `v` prefix. Validate format matches semver pattern.

### Edge Case 4: RC Versions

**Solution**: Pass RC versions as-is (e.g., `"0.1.2-rc.1"`).

**Implementation**: Regex pattern supports prerelease suffixes: `r'^\s*version\s*=\s*"v?(\d+\.\d+\.\d+(?:-[\w.-]+)?)"'`

### Design Decision: Single Source of Truth

**Decision**: **Root `Cargo.toml` `[workspace.package].version` is the authoritative source**

**Rationale**:
- **Consistency**: All microservices always use the same version as the root workspace
- **Simplicity**: One source of truth, no complex fallback logic
- **Alignment**: Matches how `rerp release bump` works (reads from root/components, updates all)
- **New Services**: If root version unavailable, `brrtrouter-gen` defaults to `"0.1.0"` (appropriate for new services)

---

## Migration Strategy

### For Existing Services

1. **Immediate Fix**: Run `rerp release bump patch` (or appropriate bump) to restore versions
2. **After BRRTRouter Update**: Regeneration will preserve versions going forward
3. **No Breaking Changes**: Existing workflows continue to work

### For New Services

1. Bootstrap creates service with workspace version (or `0.1.0` if workspace unavailable)
2. Subsequent regenerations preserve the version
3. Version bump process continues to work as before

---

## Testing Strategy

### Unit Tests

1. **Version Extraction**:
   - Extract from `[package].version`
   - Extract from `[workspace.package].version`
   - Handle missing file
   - Handle missing version field
   - Handle `v` prefix

2. **Version Resolution**:
   - Existing version → use it
   - No existing → use workspace
   - No workspace → use default

### Integration Tests

1. **Bootstrap Flow**:
   - Bootstrap new service → version matches workspace
   - Bootstrap with existing Cargo.toml → version preserved

2. **Regeneration Flow**:
   - Regenerate with version `0.1.2` → stays `0.1.2`
   - Regenerate after version bump → new version preserved

3. **Version Bump Flow**:
   - Bump to `0.1.3` → all services updated
   - Regenerate → version stays `0.1.3`

### Manual Testing Checklist

- [ ] Bootstrap new microservice (version matches workspace)
- [ ] Regenerate existing microservice (version preserved)
- [ ] Version bump all services (versions updated)
- [ ] Regenerate after bump (versions preserved)
- [ ] Tiltfile regeneration (version preserved)
- [ ] Edge case: No existing Cargo.toml (uses workspace/default)

---

## Rollback Plan

If issues arise during rollout:

### Rollback Phase 2 (RERP) Only

If RERP changes cause issues but BRRTRouter is fine:

1. **RERP Tooling**: Remove version extraction logic, revert `generate_code_with_brrtrouter()` to not pass `--version`
2. **Tiltfile**: Revert `create_microservice_gen()` to not extract/pass version
3. **Result**: BRRTRouter will use default `"0.1.0"` (backward compatible)

### Rollback Phase 1 (BRRTRouter) Only

If BRRTRouter changes cause issues:

1. **BRRTRouter**: Remove `--version` argument, revert template to hardcoded `"0.1.0"`
2. **RERP**: Keep changes but they won't pass `--version` (will be ignored by old BRRTRouter)
3. **Result**: System works but versions reset to `"0.1.0"` (current behavior)

### Full Rollback

If both need to be reverted:

1. **BRRTRouter**: Revert all changes, restore hardcoded `"0.1.0"`
2. **RERP Tooling**: Remove version extraction, revert to original code
3. **Tiltfile**: Revert to original regeneration logic
4. **Result**: Back to current state (versions reset to `"0.1.0"` on regeneration)

**Note**: Rollback is safe because:
- Default behavior (`"0.1.0"`) is preserved if `--version` not provided
- Changes are additive (no breaking changes)
- Can revert independently in each component
- RERP changes gracefully handle missing `--version` support (don't pass flag)

---

## Success Criteria

1. ✅ **Version Preservation**: Regenerating microservice code preserves existing version
2. ✅ **Bootstrap Alignment**: New services start with workspace version (or sensible default)
3. ✅ **No Breaking Changes**: Existing workflows continue to work
4. ✅ **Version Consistency**: All microservices stay in sync with workspace version after regeneration
5. ✅ **Backward Compatible**: Works with BRRTRouter that doesn't support `--version` (falls back to default)

---

## Open Questions

1. **Version Format Validation**: Should we validate version format before passing to `brrtrouter-gen`?
   - **Decision**: Yes, use regex pattern `\d+\.\d+\.\d+(?:-[\w.-]+)?` to ensure valid semver format
   - **Rationale**: Prevents passing invalid versions that could break BRRTRouter template rendering

2. **Error Handling**: What should happen if root `Cargo.toml` exists but version is malformed?
   - **Decision**: Skip `--version` flag, let `brrtrouter-gen` default to `"0.1.0"`
   - **Rationale**: Fail-safe behavior - better to use default than crash or pass invalid version

3. **RC Versions**: How should we handle `-rc.N` versions?
   - **Decision**: Pass as-is (e.g., `"0.1.2-rc.1"`)
   - **Rationale**: BRRTRouter template should accept any string value for version field

4. **Version Source Clarification**: Root `Cargo.toml` vs `components/Cargo.toml`?
   - **Decision**: Use root `Cargo.toml` (at project root)
   - **Rationale**: Root is the top-level workspace, and release bump ensures it stays in sync with components

---

## Dependencies

### External

- **BRRTRouter Repository**: Must support `--version` CLI argument
- **BRRTRouter Askama Templates**: Must use `{{ version }}` variable

### Internal

- **RERP Bootstrap Tooling**: Must extract and pass version
- **Tiltfile**: Must extract and pass version during regeneration

---

## Timeline Estimate

### Phase 1: BRRTRouter (External)
- **BRRTRouter Changes**: 2-4 hours
- **BRRTRouter Testing**: 1-2 hours
- **BRRTRouter PR & Review**: 1-2 hours (depends on review cycle)

**Phase 1 Total**: 4-8 hours

### Phase 2: RERP (Local)
- **RERP Bootstrap Updates**: 2-3 hours
- **Tiltfile Updates**: 1-2 hours
- **Testing & Validation**: 2-3 hours
- **Documentation**: 1 hour

**Phase 2 Total**: 6-9 hours

### Phase 3: Migration
- **Fix Version Drift**: 0.5 hours
- **End-to-End Validation**: 1 hour

**Phase 3 Total**: 1.5 hours

**Grand Total**: 11.5-18.5 hours (spread across multiple sessions due to BRRTRouter dependency)

**Note**: Phase 2 cannot start until Phase 1 is complete and BRRTRouter changes are available locally.

---

## References

- `tooling/src/rerp_tooling/bootstrap/microservice.py` - Bootstrap implementation
- `Tiltfile` - Regeneration flow
- `tooling/src/rerp_tooling/release/bump.py` - Version bump process
- `microservices/Cargo.toml` - Workspace version source of truth
- `microservices/accounting/*/Cargo.toml` - Generated microservice Cargo.toml files

---

## Coordination & Communication

### Between Teams

**BRRTRouter Team** (if separate):
- Notify when `--version` support is merged
- Provide commit SHA or release tag
- Coordinate testing if needed

**RERP Team**:
- Wait for BRRTRouter changes before starting Phase 2
- Test with latest BRRTRouter before implementing
- Report any issues back to BRRTRouter team

### Checklist Before Starting

**Before Phase 1 (BRRTRouter)**:
- [ ] Confirm BRRTRouter repository location and access
- [ ] Identify BRRTRouter maintainer/team for coordination
- [ ] Review BRRTRouter codebase structure (CLI, templates, context)

**Before Phase 2 (RERP)**:
- [ ] ✅ BRRTRouter `--version` support merged and available
- [ ] ✅ BRRTRouter changes tested and working
- [ ] ✅ Local BRRTRouter updated (git pull or cargo update)
- [ ] ✅ Verified `brrtrouter-gen --help` shows `--version` flag
- [ ] ✅ Current RERP branch is ready for changes

### Risk Mitigation

**Risk**: BRRTRouter changes delayed or rejected
- **Mitigation**: RERP changes are backward compatible (don't pass `--version` if not supported)

**Risk**: Version extraction fails silently
- **Mitigation**: Only pass `--version` if version is found; otherwise let BRRTRouter default

**Risk**: Tiltfile shell script errors
- **Mitigation**: Test Tiltfile syntax and regeneration flow thoroughly

---

## Approval

**Status**: Awaiting Review

**Next Steps**:
1. ✅ Review this design proposal and rollout plan
2. ✅ Approve approach and implementation plan
3. ✅ Coordinate BRRTRouter changes (Phase 1)
4. ⏳ Wait for BRRTRouter changes to be merged/available
5. ⏳ Implement RERP changes (Phase 2)
6. ⏳ Test and validate end-to-end
7. ⏳ Fix version drift (Phase 3)
8. ⏳ Deploy and monitor

**Critical Path**: Phase 1 (BRRTRouter) must complete before Phase 2 (RERP) can begin.

---

**Document Version**: 2.0  
**Last Updated**: 2025-01-26  
**Changes**: Added comprehensive rollout plan with BRRTRouter coordination
