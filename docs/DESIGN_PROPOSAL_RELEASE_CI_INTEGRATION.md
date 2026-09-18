# Design Proposal: RERP Release-CI Integration Pattern

## Executive Summary

This proposal adapts the weave-gitops release-CI integration pattern to RERP, centralizing release job control logic and enabling manual re-runs of release builds via workflow_dispatch.

## Current State Analysis

### Weave-GitOps Pattern (Reference Implementation)

**release.yaml:**
- Creates tag and pushes using `REPO_PAT` (required because `GITHUB_TOKEN` pushes don't trigger workflows)
- Tag push automatically triggers `ci.yaml` via `on: push: tags: ["v*"]`

**ci.yaml:**
- **`vars` job** (lines 42-55): Centralizes release job control logic
  - Outputs `run_release_jobs: true` when:
    - `is_release_tag == 'true'` (detected via `weavetooling ci is-tag`)
    - OR `workflow_dispatch` with `run_release_jobs=true` input
  - Uses tooling command: `weavetooling ci is-tag` to check if `GITHUB_REF` starts with `refs/tags/v`
- **Release jobs** (build-push-gitops-server, build-and-push-chart, goreleaser):
  - Depend on: `needs: [build, vars]`
  - Conditional: `if: needs.vars.outputs.run_release_jobs == 'true'`
- **workflow_dispatch input**: `run_release_jobs` (boolean, default: false)

### RERP Current State

**release.yml:**
- Creates tag and pushes using `REPO_PAT`
- Tag push automatically triggers `ci.yml` via `on: push: tags: ['v*']`

**ci.yml:**
- **No `vars` job**: Release job control is scattered across jobs
- **Inline conditionals**: `if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')`
- **No `workflow_dispatch` support**: Cannot manually trigger release builds
- **Release jobs**:
  - `download-copy-package-push-containers`: `if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v') && needs.build-multiarch.result == 'success'`
  - `build-push-service`: `if: needs.download-copy-package-push-containers.result == 'success'`
  - `verify-published-images`: `if: needs.download-copy-package-push-containers.result == 'success' && needs.build-push-service.result == 'success'`

**Missing Tooling:**
- No `rerp ci is-tag` command (weave-gitops has `weavetooling ci is-tag`)

## Proposed Changes

### 1. Add `rerp ci is-tag` Command

**Location:** `tooling/src/rerp_tooling/ci/is_tag.py`

**Functionality:**
- Read `GITHUB_REF` environment variable
- Return `true` if `GITHUB_REF` starts with `refs/tags/v`
- Return `false` otherwise
- Print result to stdout (for GITHUB_OUTPUT)

**Implementation:**
```python
"""Check if GITHUB_REF is a release tag (refs/tags/v*)."""
import os
import sys

def run() -> int:
    ref = os.environ.get("GITHUB_REF", "")
    is_tag = "true" if ref.startswith("refs/tags/v") else "false"
    print(is_tag)
    return 0
```

**CLI Integration:**
- Add `is-tag` subcommand to `rerp ci` in `tooling/src/rerp_tooling/cli/ci.py`
- Add parser entry in `tooling/src/rerp_tooling/cli/main.py`

### 2. Add `vars` Job to ci.yml

**Location:** `.github/workflows/ci.yml`

**New Job:**
```yaml
vars:
  name: Set CI vars
  runs-on: ubuntu-latest
  outputs:
    run_release_jobs: ${{ steps.set_vars.outputs.is_release_tag == 'true' || (github.event_name == 'workflow_dispatch' && github.event.inputs.run_release_jobs == 'true') }}
  steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-python@v5
      with:
        python-version: "3.12"
        cache: "pip"
    - run: pip install -e ./tooling
    - id: set_vars
      run: echo "is_release_tag=$(rerp ci is-tag)" >> $GITHUB_OUTPUT
```

**Placement:** Before `conventional-commits` job (runs first, no dependencies)

### 3. Add `workflow_dispatch` Input to ci.yml

**Location:** `.github/workflows/ci.yml` `on:` section

**Addition:**
```yaml
on:
  push:
    branches: [main, develop]
    tags: ['v*']
  pull_request:
    branches: [main, develop]
  workflow_dispatch:
    inputs:
      run_release_jobs:
        description: "Run container build and push jobs (release build). Set when dispatching to (re-)run release, or when invoked by release.yaml."
        required: false
        default: false
        type: boolean
```

### 4. Add Concurrency Control to Protect Release Builds ⚠️ CRITICAL

**Location:** `.github/workflows/ci.yml` (after `on:` section, before `permissions:`)

**Problem:**
- Currently RERP has **NO concurrency settings** in ci.yml
- This means release builds (tag pushes) can be cancelled by:
  - Dependabot PRs
  - Other branch pushes
  - Manual workflow dispatches
- **This is disruptive**: Requires deleting tags and re-running releases

**Solution:**
Add concurrency control that **NEVER cancels tag-based (release) builds**:

```yaml
concurrency:
  # Each ref gets its own group (tags, branches, PRs are isolated)
  group: ${{ github.workflow }}-${{ github.ref }}
  # CRITICAL: Only cancel PRs, NEVER cancel pushes (branches) or tags (releases)
  # Tags are never PRs, so they're automatically protected
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

**How It Works:**
1. **Concurrency Groups:**
   - Each ref gets unique group: `RERP CI-refs/tags/v0.39.0`, `RERP CI-refs/heads/main`, `RERP CI-refs/pull/123/merge`
   - Tags are isolated from branches and PRs (different groups)
   - Each tag gets its own group (multiple releases can run in parallel if needed)

2. **Cancel Behavior:**
   - **Tags**: `cancel-in-progress: false` (tags are never PRs, so condition evaluates to false)
   - **Branches**: `cancel-in-progress: false` (branches are pushes, not PRs)
   - **PRs**: `cancel-in-progress: true` (cancel old PR builds when new PR commit pushed)

3. **Result:**
   - ✅ Release builds (tags) **NEVER cancelled** - tags are not PRs, so condition is always false
   - ✅ Dependabot PRs can't cancel release builds (different groups)
   - ✅ Branch pushes can't cancel release builds (different groups, and condition is false)
   - ✅ Multiple PRs can run concurrently (each has unique group)
   - ✅ New PR commits cancel old PR builds (expected behavior for PRs)

**Why This Works:**
- `github.event_name == 'pull_request'` is `false` for tag pushes and branch pushes
- Tags and branches get different concurrency groups than PRs
- Even if a PR runs simultaneously, it can't cancel a tag build (different groups)
- Simple and explicit: only PRs cancel in-progress, everything else runs to completion

**Why This Matters:**
- **Release builds are critical**: They create container images, push to registries, create GitHub Releases
- **Tag deletion is disruptive**: Requires manual intervention, re-running release workflow
- **Dependabot is frequent**: Can trigger many PRs that would otherwise cancel release builds
- **Enterprise requirement**: Release builds must complete successfully, cannot be interrupted

**Comparison to Weave-GitOps:**
- Weave-gitops uses: `cancel-in-progress: ${{ github.event_name == 'pull_request' }}`
- This proposal uses the same pattern (proven approach)
- **Improvement**: Explicitly documented and tested for RERP's use case
- **Result**: Same protection, better documentation

**Testing:**
1. Start a release build (tag push)
2. Immediately trigger a dependabot PR or branch push
3. Verify release build continues and completes (not cancelled)
4. Verify other builds run in parallel (don't block each other)
5. Verify PR builds can still cancel old PR builds (expected behavior)

### 5. Update Release Jobs to Use `vars` Output

**Jobs to Update:**
1. `download-copy-package-push-containers`
2. `build-push-service`
3. `verify-published-images`

**Change Pattern:**
- **Before:** `if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v') && needs.build-multiarch.result == 'success'`
- **After:** `if: needs.vars.outputs.run_release_jobs == 'true' && needs.build-multiarch.result == 'success'`

**Specific Updates:**

**download-copy-package-push-containers:**
```yaml
needs: [build-multiarch, vars]  # Add vars dependency
if: needs.vars.outputs.run_release_jobs == 'true' && needs.build-multiarch.result == 'success'
```

**build-push-service:**
```yaml
needs: [download-copy-package-push-containers, vars]  # Add vars dependency
if: needs.vars.outputs.run_release_jobs == 'true' && needs.download-copy-package-push-containers.result == 'success'
```

**verify-published-images:**
```yaml
needs: [download-copy-package-push-containers, build-push-service, vars]  # Add vars dependency
if: needs.vars.outputs.run_release_jobs == 'true' && needs.download-copy-package-push-containers.result == 'success' && needs.build-push-service.result == 'success'
```

### 6. Update `conventional-commits` Job

**Current:** `if: "!startsWith(github.ref, 'refs/tags/')"`

**Proposed:** Keep as-is (still skip on tags, but now we have centralized logic in `vars`)

**Note:** The `vars` job should run even on tags, so it doesn't need this conditional.

### 7. Update `tooling` Job Dependency

**Current:** `needs: [conventional-commits]` with conditional

**Proposed:** Keep as-is (tooling job logic is fine, it just needs to handle skipped conventional-commits)

## Benefits

1. **Centralized Logic**: Single source of truth for "should run release jobs" in `vars` job
2. **Manual Re-runs**: Can manually trigger release builds via workflow_dispatch with `run_release_jobs=true`
3. **Consistency**: Matches weave-gitops pattern for easier maintenance
4. **Flexibility**: Can extend `vars` job to add more conditional logic (e.g., skip RC builds, only build on main branch tags, future API versioning decisions)
5. **Debugging**: Easier to debug release job conditions by checking `vars` job output
6. **Future-Proof**: `vars` job pattern supports future enhancements (e.g., API versioning strategy, RC tag filtering) without major refactoring
7. **🛡️ Release Build Protection**: Concurrency control ensures release builds are NEVER cancelled by dependabot PRs, branch pushes, or other workflow runs (critical for enterprise reliability)

## Migration Path

1. **Phase 1**: Add `rerp ci is-tag` command (non-breaking)
2. **Phase 2**: Add concurrency control to protect release builds (⚠️ CRITICAL - do this early)
3. **Phase 3**: Add `vars` job and `workflow_dispatch` input (non-breaking, jobs still work with old conditionals)
4. **Phase 4**: Update release jobs to use `vars.outputs.run_release_jobs` (remove old conditionals)
5. **Phase 5**: Test with manual workflow_dispatch trigger
6. **Phase 6**: Test with actual tag push from release.yml
7. **Phase 7**: Verify concurrency protection (trigger dependabot PR during release build)

## Testing Strategy

1. **Unit Test**: `rerp ci is-tag` with various `GITHUB_REF` values
2. **Integration Test**: Run ci.yml workflow_dispatch with `run_release_jobs=true` on a test branch
3. **E2E Test**: Full release flow: release.yml → tag push → ci.yml release jobs

## Edge Cases

1. **RC Tags**: Currently `v*` matches both `v0.1.0` and `v0.1.0-rc.1`. Should we filter RC tags?
   - **Proposal**: Keep current behavior (build on all v* tags). Can add filtering later if needed.
   - **Future**: If API versioning is implemented (see "Future Considerations" section), we could add `is_rc_tag` output to `vars` job to conditionally skip builds for RC tags
2. **Fork Releases**: Same pattern works (REPO_PAT with fork access)
3. **Concurrent Runs**: 
   - **Current**: No concurrency control - release builds can be cancelled ⚠️
   - **Proposed**: Concurrency groups protect release builds (see section 4)
   - **Critical**: Release builds (tags) must NEVER be cancelled by other runs
4. **Version Format**: Current semantic versioning (X.Y.Z) is appropriate for application releases. See `docs/VERSIONING_STRATEGY_ANALYSIS.md` for future API versioning considerations.
5. **Dependabot Interference**: 
   - **Problem**: Dependabot PRs can trigger during release builds and cancel them
   - **Solution**: Concurrency control with `cancel-in-progress: false` for tags
   - **Verification**: Test by triggering dependabot PR during active release build
6. **Version Downgrades**: 
   - **Problem**: No protection against building/pushing older versions (see Bug 3)
   - **Solution**: Add version validation using GitHub API (see "Critical Bugs" section)
   - **Critical**: Must prevent accidental downgrades that could affect production
7. **workflow_dispatch Latest Tag Overwrite**: 
   - **Problem**: workflow_dispatch from main can overwrite production `latest` tag (see Bug 2)
   - **Solution**: Never set `extra_tag=latest` for non-tag builds
   - **Critical**: Production `latest` tag must only be set for official release tags
8. **Version Inconsistency**: 
   - **Problem**: Workspace and package versions can diverge (see Bug 1)
   - **Solution**: Add validation and alignment process (see "Critical Bugs" section)
   - **Prevention**: Enhance `rerp release bump` to maintain consistency

## Open Questions

1. Should we filter RC tags from release builds? (Currently builds on all `v*` tags)
2. Should we add additional `vars` outputs (e.g., `is_rc_tag`, `is_full_release`)?
3. Should we add a `rerp ci` command to check other ref types (e.g., `is-main-branch`)?

## Future Considerations: API Versioning Strategy (Gated for Later)

**Status**: ⏸️ **Gated - Not for immediate implementation**

This section documents a future consideration for API versioning strategy. This is **not part of the current proposal** and should be explicitly chosen for implementation when needed.

### Context

RERP currently uses **Semantic Versioning (X.Y.Z)** for application releases, which is appropriate for:
- Container images: `rerp-accounting-general-ledger:0.39.0`
- Binary distributions: CLI tools, service executables
- Helm charts: Chart version `0.39.0`
- Git tags: `v0.39.0`, `v0.39.0-rc.2`

However, as an enterprise application with 71 microservices exposing OpenAPI APIs, there may be value in adding **Kubernetes-style API versioning** to communicate API stability guarantees to enterprise customers.

### Proposed Hybrid Approach (Future)

**Application Releases**: Continue using Semantic Versioning (X.Y.Z)
- ✅ Keep current approach (no changes needed)
- ✅ Standard for enterprise applications
- ✅ Clear communication: patch = bug fix, minor = feature, major = breaking

**API Contracts**: Add Kubernetes-Style Versioning (v1alpha1, v1beta1, v1)
- ⚠️ **Future consideration**: Add version to OpenAPI paths
- Example: `/api/v1alpha1/accounts` (experimental) vs `/api/v1/accounts` (stable)
- Benefits:
  - Clear API stability communication to enterprise customers
  - Can evolve APIs independently of application releases
  - Industry standard (Kubernetes, Istio, etc.)

### Implementation Gate

**Decision Required**: Explicitly choose to implement when:
- Enterprise customers need API stability guarantees
- You want to distinguish experimental vs stable APIs
- You need to evolve APIs independently of application releases

**Not Required For**:
- Current release-CI integration proposal
- Basic application versioning (semantic versioning is sufficient)
- Initial enterprise deployments

### Reference Documentation

See `docs/VERSIONING_STRATEGY_ANALYSIS.md` for detailed analysis comparing:
- Pure Semantic Versioning (current approach)
- Kubernetes-Style API Versioning
- Hybrid Approach (recommended for future consideration)

### Implementation Notes (If Chosen Later)

If API versioning is implemented in the future:

1. **Update OpenAPI specs** to include version in paths:
   ```yaml
   paths:
     /api/v1alpha1/accounts:  # Instead of /api/accounts
   ```

2. **Update BFF generation** to preserve API versions

3. **Document API versioning policy**:
   - `v1alpha1`: Experimental, breaking changes allowed
   - `v1beta1`: More stable, breaking changes possible
   - `v1`: Stable, no breaking changes

4. **Support multiple API versions** during transitions

**This is explicitly gated and should not be implemented as part of the current release-CI integration work.**

## Critical Bugs Identified and Handling Strategies

### Bug 1: Version Inconsistency Between Workspace and Microservice Packages ⚠️

**Issue:**
- **Location**: `microservices/Cargo.toml` and individual microservice `Cargo.toml` files
- **Problem**: Workspace version (`microservices/Cargo.toml`) is `0.1.2`, but all 10 accounting microservice packages have version `0.1.0`
- **Impact**: 
  - Creates inconsistency in version management
  - May cause build/compilation issues if workspace and package versions diverge
  - Unclear which version is authoritative
- **Affected Services**: All 10 accounting microservices (accounts-payable, accounts-receivable, asset, bank-sync, bff, budget, edi, financial-reports, general-ledger, invoice)

**Root Cause:**
- Appears to be unrelated to release CI integration changes
- Likely from a previous version bump that didn't update all packages consistently
- Or intentional downgrade that wasn't fully applied

**Handling Strategy:**

1. **Investigation Phase:**
   - Determine intended version: Should workspace and packages both be `0.1.0` or `0.1.2`?
   - Check git history to see when divergence occurred
   - Verify if this was intentional or accidental

2. **Resolution Options:**
   - **Option A**: Align all to `0.1.2` (if workspace version is correct)
     - Update all 10 microservice packages to `0.1.2`
     - Use `rerp release bump` to ensure consistency going forward
   - **Option B**: Align all to `0.1.0` (if packages are correct)
     - Update workspace version to `0.1.0`
     - Verify this doesn't break existing releases/tags
   - **Option C**: Use workspace version as source of truth
     - Update all packages to match workspace version
     - Document that workspace version is authoritative

3. **Prevention:**
   - Enhance `rerp release bump` to validate workspace and package version consistency
   - Add pre-commit hook or CI check to detect version mismatches
   - Document version management policy in `CONTRIBUTING.md`

4. **Implementation:**
   - Create separate issue/PR for version alignment (don't mix with release CI integration)
   - Use `rerp release bump` or manual update to fix inconsistency
   - Add validation to prevent future mismatches

### Bug 2: workflow_dispatch Latest Tag Overwrite Risk ⚠️ CRITICAL

**Issue:**
- **Location**: `.github/workflows/ci.yml` lines 386-397 (Set image tag step)
- **Problem**: When `workflow_dispatch` is triggered from `main` branch with `run_release_jobs=true`, the image tagging logic sets:
  - `tag=sha-${GITHUB_SHA::7}` (development build hash)
  - `extra_tag=latest` (production tag)
- **Impact**: 
  - **CRITICAL**: Overwrites production `latest` tag with a development build
  - Users pulling `latest` would get an untested development build instead of the official release
  - Could cause production outages if development build has bugs
  - Violates semantic versioning and release process

**Current Logic:**
```yaml
if [[ "${{ github.ref }}" == refs/tags/v* ]]; then
  echo "tag=${GITHUB_REF#refs/tags/v}" >> "$GITHUB_OUTPUT"
  echo "extra_tag=latest" >> "$GITHUB_OUTPUT"
else
  echo "tag=sha-${GITHUB_SHA::7}" >> "$GITHUB_OUTPUT"
  if [[ "${{ github.ref }}" == refs/heads/main ]]; then
    echo "extra_tag=latest" >> "$GITHUB_OUTPUT"  # ⚠️ BUG: Sets latest for non-tag builds
  fi
fi
```

**Handling Strategy:**

1. **Immediate Fix:**
   - **Never set `extra_tag=latest` for non-tag builds**, even when `run_release_jobs=true`
   - `latest` tag should ONLY be set for official release tags (`refs/tags/v*`)
   - Development builds should use `sha-<hash>` only (no `latest` tag)

2. **Updated Logic:**
   ```yaml
   if [[ "${{ github.ref }}" == refs/tags/v* ]]; then
     echo "tag=${GITHUB_REF#refs/tags/v}" >> "$GITHUB_OUTPUT"
     echo "extra_tag=latest" >> "$GITHUB_OUTPUT"
   else
     echo "tag=sha-${GITHUB_SHA::7}" >> "$GITHUB_OUTPUT"
     # Never set extra_tag=latest for non-tag builds (even workflow_dispatch)
     echo "extra_tag=" >> "$GITHUB_OUTPUT"
   fi
   ```

3. **Rationale:**
   - `workflow_dispatch` with `run_release_jobs=true` is for **re-running** existing release builds, not creating new releases
   - If you need to create a new release, use `release.yml` workflow which creates a proper tag
   - Development builds should never overwrite production `latest` tag
   - Only official release tags (created by `release.yml`) should tag images as `latest`

4. **Documentation:**
   - Update `workflow_dispatch` input description to clarify it's for re-running existing releases
   - Document that `latest` tag is only set for official release tags
   - Add warning in workflow comments about this critical behavior

5. **Testing:**
   - Test `workflow_dispatch` from `main` with `run_release_jobs=true`
   - Verify images are tagged with `sha-<hash>` only (no `latest`)
   - Verify tag-based builds still set `latest` correctly

### Bug 3: Version Downgrade Protection ⚠️ CRITICAL

**Issue:**
- **Location**: Release process (tag creation and CI validation)
- **Problem**: No validation to prevent version downgrades or building from outdated code
- **Impact**: 
  - **CRITICAL**: Could push images with older versions, causing production downgrades
  - If someone builds from an old unrebased branch, could tag images with version lower than current production
  - No protection against accidental downgrades during manual releases
  - Could overwrite newer images with older versions

**Scenarios:**
1. Developer builds from old branch (e.g., `v0.38.0` when current is `v0.39.0`)
2. Manual tag creation with wrong version number
3. Re-running old release workflow that creates downgrade
4. Git history manipulation or force-push scenarios

**Handling Strategy:**

1. **Add Version Validation Command:**
   - **Location**: `tooling/src/rerp_tooling/ci/validate_version.py`
   - **Functionality**: 
     - Query GitHub API to get latest release tag
     - Compare current version (from `components/Cargo.toml`) with latest GitHub release
     - Fail if current version is less than or equal to latest release (prevent downgrade)
     - Support `--allow-same` flag for patch releases of same version (if needed)

2. **Implementation:**
   ```python
   """Validate version to prevent downgrades."""
   import os
   import sys
   from packaging import version
   import requests
   
   def get_latest_github_tag(repo: str, token: str) -> str:
       """Get latest release tag from GitHub API."""
       # Implementation using GitHub API
       # Returns version string (e.g., "0.39.0")
   
   def validate_version(current: str, latest: str, allow_same: bool = False) -> int:
       """Validate current version is greater than latest."""
       # Returns 0 if valid, 1 if downgrade detected
   ```

3. **Integration Points:**
   - **In `vars` job**: Add version validation step before setting `run_release_jobs`
   - **In `release.yml`**: Validate version before creating tag
   - **In `ci.yml` release jobs**: Validate version before building/pushing images

4. **Validation Logic:**
   ```yaml
   - name: Validate version (prevent downgrade)
     id: validate_version
     run: |
       CURRENT=$(rerp release current-version)  # New command to read from Cargo.toml
       LATEST=$(rerp ci get-latest-tag --repo ${{ github.repository }})
       rerp ci validate-version --current "$CURRENT" --latest "$LATEST" || exit 1
   ```

5. **Error Handling:**
   - **Downgrade Detected**: Fail workflow with clear error message
   - **Same Version**: 
     - Default: Fail (prevent accidental re-release)
     - Option: Allow with `--allow-same` flag for patch releases
   - **Upgrade**: Allow (normal case)
   - **GitHub API Failure**: 
     - Option A: Fail (strict mode - recommended)
     - Option B: Warn but continue (lenient mode - not recommended for production)

6. **GitHub API Integration:**
   - Use `GITHUB_TOKEN` for authentication (automatically provided in GitHub Actions)
   - Query: `GET /repos/{owner}/{repo}/releases/latest`
   - Fallback: If no releases exist, allow any version (first release)
   - Cache result to avoid rate limiting

7. **Edge Cases:**
   - **First Release**: No previous tags, allow any version
   - **RC Releases**: Compare RC versions correctly (e.g., `0.39.0-rc.2` < `0.39.0`)
   - **Prerelease Handling**: Decide if RC can be "downgrade" from full release
   - **Force Override**: Add `--force` flag for emergency overrides (documented, requires admin)

8. **Documentation:**
   - Document version validation in `CONTRIBUTING.md`
   - Add troubleshooting guide for version validation failures
   - Document emergency override process (if implemented)

9. **Testing:**
   - Test with version higher than latest (should pass)
   - Test with version lower than latest (should fail)
   - Test with same version (should fail by default)
   - Test with first release (no previous tags, should pass)
   - Test with RC versions (should handle correctly)
   - Test GitHub API failure scenarios

10. **Future Enhancements:**
    - Add version validation to pre-commit hooks
    - Add version validation to `rerp release bump` command
    - Support version validation in local development (warn, don't fail)
    - Add version history tracking (database or file-based)

**Implementation Priority:**
1. **High**: Version validation in `vars` job (prevents downgrade at CI entry point)
2. **High**: Version validation in `release.yml` (prevents creating downgrade tags)
3. **Medium**: Version validation in release jobs (defense in depth)
4. **Low**: Local development validation (nice to have)

## Files to Modify

### Core Release CI Integration
1. `tooling/src/rerp_tooling/ci/is_tag.py` (new)
2. `tooling/src/rerp_tooling/cli/ci.py` (add is-tag handler)
3. `tooling/src/rerp_tooling/cli/main.py` (add is-tag parser entry)
4. `.github/workflows/ci.yml` (add concurrency control, vars job, workflow_dispatch input, update release jobs)
5. `tooling/tests/test_ci_is_tag.py` (new, unit tests)

### Bug Fixes (Critical)
6. `.github/workflows/ci.yml` (fix workflow_dispatch latest tag bug - Bug 2)
7. `tooling/src/rerp_tooling/ci/validate_version.py` (new - Bug 3: version downgrade protection)
8. `tooling/src/rerp_tooling/ci/get_latest_tag.py` (new - Bug 3: GitHub API integration)
9. `tooling/src/rerp_tooling/cli/ci.py` (add validate-version and get-latest-tag handlers)
10. `tooling/src/rerp_tooling/cli/main.py` (add validate-version and get-latest-tag parser entries)
11. `tooling/tests/test_ci_validate_version.py` (new, unit tests)
12. `microservices/Cargo.toml` and individual microservice `Cargo.toml` files (Bug 1: version alignment - separate PR)

## Dependencies

### Core Release CI Integration
- No external dependencies (uses standard library `os`)
- Requires Python 3.12+ (already in use)

### Bug Fixes
- **Bug 3 (Version Validation)**: 
  - `requests` library for GitHub API calls (or use `urllib` from standard library)
  - `packaging` library for version comparison (or implement custom semver comparison)
  - GitHub API access via `GITHUB_TOKEN` (automatically provided in GitHub Actions)

## Rollback Plan

If issues arise:
1. Revert release job conditionals to use inline `startsWith(gITHUB_REF, 'refs/tags/v')`
2. Keep `vars` job (non-breaking, just unused)
3. Remove `workflow_dispatch` input if needed

## Success Criteria

### Core Release CI Integration
1. ✅ `rerp ci is-tag` command works correctly
2. ✅ **Concurrency control protects release builds** - tag-based builds are NEVER cancelled
3. ✅ `vars` job outputs `run_release_jobs: true` on tag pushes
4. ✅ `vars` job outputs `run_release_jobs: true` on workflow_dispatch with input=true
5. ✅ Release jobs run on tag pushes (existing behavior preserved)
6. ✅ Release jobs can be manually triggered via workflow_dispatch
7. ✅ All existing CI jobs continue to work
8. ✅ **Dependabot PRs cannot cancel release builds** (verified by test)
9. ✅ **Branch pushes cannot cancel release builds** (verified by test)

### Critical Bug Fixes
10. ✅ **Bug 2 Fixed**: workflow_dispatch from main does NOT set `latest` tag (only tag-based builds set `latest`)
11. ✅ **Bug 3 Fixed**: Version validation prevents downgrades (fails if version <= latest GitHub release)
12. ✅ **Bug 3 Verified**: Version validation works with RC versions, first releases, and GitHub API failures
13. ✅ **Bug 1 Tracked**: Version inconsistency documented and tracked in separate issue/PR
