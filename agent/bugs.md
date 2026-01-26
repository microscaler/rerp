# Agent bugs log

Tracked bugs and fixes for agent-discovered issues.

---

## Fixed: Dependabot – 7 npm vulnerabilities in ui/website (2025-01-24)

**Source:** GitHub Dependabot (6 high, 1 moderate) on default branch

**Issue:** `yarn audit` in `ui/website` reported: esbuild (moderate, vite; CVE), seroval (5× high, solid-js; DoS, RCE, prototype pollution), glob (high, tailwindcss > sucrase; command injection).

**Fix:** In `ui/website/package.json`:
- Upgraded direct deps: `solid-js` ^1.8.7 → ^1.9.11, `vite` ^5.0.10 → ^5.4.21, `tailwindcss` ^3.4.0 → ^3.4.19.
- Added `resolutions` to force patched transitives: `esbuild` >=0.25.0, `seroval` >=1.4.1, `glob` >=10.5.0.

Resolved: esbuild 0.27.2, seroval 1.5.0, glob 13.0.0. `yarn audit` 0 vulnerabilities; `yarn build` passes.

---

## Fixed: CodeQL – Workflow does not contain permissions (2025-01-24)

**Source:** Code scanning / CodeQL – Medium

**Issue:** The CI workflow (and any workflow used for CodeQL) did not set an explicit `permissions` block. The default grants all permissions to `GITHUB_TOKEN`. Restricting to the minimum needed (e.g. `contents: read`) follows least-privilege and satisfies CodeQL’s recommendation.

**Fix:** Add workflow-level `permissions: { contents: read }` to `.github/workflows/ci.yml`. Jobs that need more (e.g. `build-push-service`: `packages: write`, `id-token: write`, `attestations: write` for push and attest-build-provenance) keep their job-level `permissions`, which override the workflow default.

**Note:** If a dedicated CodeQL workflow (e.g. `codeql.yml`) exists, it should also include `permissions: { contents: read }` (or the minimum required for the CodeQL action).

---

## Fixed: CodeQL – Incomplete URL substring sanitization in tests (2025-01-24)

**Source:** CodeQL `py/incomplete-url-substring-sanitization` – `tooling/tests/test_ci_patch_brrtrouter.py` lines 45 and 103

**Issue:** Assertions `"github.com" in m[0][1]` and `"github.com" in txt` were flagged as incomplete URL sanitization: checking for the substring `github.com` anywhere in a string does not safely validate the host (e.g. `https://evil.github.com.evil.com` would match).

**Context:** These are **tests** asserting that `patch_brrtrouter` replaced a path dep with a known git URL. The replacement is a fixed constant; there is no user-controlled URL or sanitization. The alert is a false positive in intent, but the pattern matches the rule.

**Fix:** Replace `"github.com"` with `"microscaler/BRRTRouter"` in the assertions. That still verifies the replacement points at the expected repo without using a host substring that triggers the rule. Updated:
- `TestFindMatches::test_brrtrouter_path_dep_returns_replacement`: `assert "git" in m[0][1] and "microscaler/BRRTRouter" in m[0][1]`
- `TestPatchFile::test_patch_replaces_path_with_git`: `assert "git" in txt and "microscaler/BRRTRouter" in txt`

---

## Fixed: base-images build step ignored change detection on workflow_dispatch (2025-01-24)

**File:** `.github/workflows/base-images.yml`

**Issue:** The build step condition included `github.event_name == 'workflow_dispatch'` as an OR term, so every manual workflow_dispatch ran the build regardless of change detection or the `force` input. When `force=false`, the "Check for changes" step ran and set `changed`, but the build condition was always true for workflow_dispatch, so the result was ignored.

**Fix:** Use only `steps.check-changes.outputs.changed == 'true' || github.event.inputs.force == 'true'`. Build runs when: (1) changes were detected in docker/base, or (2) the user set `force=true` on manual dispatch. For workflow_dispatch with `force=false`, the build now correctly skips when there are no changes.

---

## Fixed: docker/metadata-action labels applied to all images (2025-01-24)

**File:** `.github/workflows/ci.yml`

**Issue:** The `docker/metadata-action` step extracted metadata only for `rerp-accounting-general-ledger` (`images: ghcr.io/.../rerp-accounting-general-ledger`), but `${{ steps.meta.outputs.labels }}` was applied to all 11 build-push steps (general-ledger, invoice, accounts-receivable, accounts-payable, bank-sync, asset, budget, edi, financial-reports, bff, website). Every image received OCI labels (e.g. `org.opencontainers.image.title`) identifying it as `rerp-accounting-general-ledger`. Tools that inspect container metadata reported incorrect image identities.

**Fix:** Run `docker/metadata-action` once per image, immediately before each `docker/build-push-action`, with the correct `images` value and a unique step `id` (e.g. `meta-invoice`, `meta-accounts-receivable`). Each build step uses `${{ steps.meta-<name>.outputs.labels }}` so OCI labels match the image being built.

---

## Fixed: `_update_refs_in_value` used substring match for schema $ref (2025-01-24)

**File:** `tooling/src/rerp_tooling/bff/generate_system.py`

**Issue:** `_update_refs_in_value` used `in` for substring matching (`if f"#/components/schemas/{old_name}" in ref`). When a schema name is a prefix of another (e.g. `Error` and `ErrorResponse`), refs to the longer name were incorrectly rewritten. A ref to `#/components/schemas/ErrorResponse` would match when processing `Error` and be changed to `#/components/schemas/ServiceErrorResponse`.

**Fix:** Use exact equality: `if ref == f"#/components/schemas/{old_name}":` and set `val["$ref"] = f"#/components/schemas/{new_name}"`. Refs are only updated when they point exactly to that schema, not when the old name is a prefix.

**Tests:** `tooling/tests/test_bff_generate_system.py` — `TestUpdateRefsInValue`: `test_exact_match_rewritten`, `test_prefix_of_schema_name_not_rewritten`, `test_unrelated_schema_unchanged`.

---

## Fixed: `update_tiltfile` dropped `m.group(3)` in three regex replacements (2025-01-24)

**File:** `tooling/src/rerp_tooling/bootstrap/microservice.py`

**Issue:** Three regex replacements in `update_tiltfile` reconstructed the match from `m.group(1)` and the new inner content but omitted `m.group(3)`, producing invalid Starlark:

1. **resource_deps + labels** (line ~183): The pattern `(resource_deps=\[)(.*?)(\]\s*labels=\['microservices-build'\])` has group 3 = `]\s*labels=...`. The replacement used `content[m.end():]` for the tail but never emitted `m.group(3)`, so the closing `]` and `labels=['microservices-build']` were lost.
2. **deps + resource_deps** (line ~190): The pattern `(deps=\[)(.*?)(\]\s*resource_deps=)` has group 3 = `]\s*resource_deps=`. Same bug: the `]` and `resource_deps=` were dropped.
3. **ports dict** (line ~198): The pattern `(ports\s*=\s*\{)(.*?)(\s*\})` has group 3 = `\s*}`. The replacement used `"\n    " + content[m.end():]`, which skipped the `}` and relied on the tail; the dict never closed.

**Fix:** Include `m.group(3)` in the replacement for each:

- resource_deps: `... + "',\n    " + m.group(3) + content[m.end() :]`
- deps: `... + "',\n    " + m.group(3) + content[m.end() :]`
- ports: `... + "\n".join(lines) + m.group(3) + content[m.end() :]` (removed the erroneous `"\n    "` so the closing `\s*}` from group 3 is used).

---

## Fixed: `update_workspace_cargo_toml` truncated workspace Cargo.toml (2025-01-24)

**File:** `tooling/src/rerp_tooling/bootstrap/microservice.py`

**Issue:** The `update_workspace_cargo_toml` function reconstructed the file content but omitted `content[m.end():]`, so everything after the `members = [...]` array was lost. This included:

- `resolver = "2"`
- `[workspace.package]`
- `[workspace.dependencies]`

Running `rerp bootstrap microservice` would corrupt `microservices/Cargo.toml` by truncating it.

**Fix:** Append the rest of the file when rebuilding the `members` section:

```python
new_content = content[: m.start()] + m.group(1) + "\n" + new_members + "]" + content[m.end() :]
```

**Reference:** Rebuilds must include `content[m.end():]` for the tail; when the regex has a group 3 for the closing part of the match, that must be emitted too (see `update_tiltfile` fix above).

---

## Fixed: `update_tiltfile` ports regex missing `re.DOTALL` (2025-01-24)

**File:** `tooling/src/rerp_tooling/bootstrap/microservice.py`

**Issue:** The regex for updating the Tiltfile `ports` dict was missing `re.DOTALL`, unlike the other regexes in `update_tiltfile` (BINARY_NAMES, resource_deps, deps). Without it, `.` in `(.*?)` does not match newlines, so the pattern `(ports\s*=\s*\{)(.*?)(\s*\})` cannot match a multi-line dict. The real `ports` dict in the Tiltfile spans multiple lines (one entry per line), so the regex never matched and the port update silently did nothing.

**Fix:** Add `re.DOTALL` to the `re.search` call:

```python
m = re.search(r"(ports\s*=\s*\{)(.*?)(\s*\})", content, re.DOTALL)
```

---

## Fixed: `_get_commits_since` crashed on invalid `--since-tag` (2025-01-24)

**File:** `tooling/src/rerp_tooling/release/notes.py`

**Issue:** `_get_commits_since` used `subprocess.run(..., check=True)` without catching `CalledProcessError`. When the user passes an invalid git ref via `--since-tag`, `run()` passes it to `_get_commits_since`, which runs `git log ref..HEAD`. Git exits non‑zero for an unknown ref, and the uncaught `CalledProcessError` produced a traceback instead of a clear error. This differed from `_get_previous_tag`, which catches `CalledProcessError` and returns `None`.

**Fix:** Wrap the `subprocess.run` in `_get_commits_since` with `try/except subprocess.CalledProcessError`. On error: print a short message including the ref and any git stderr, then `raise SystemExit(1) from e`. Example: `Invalid git ref for --since-tag: 'v99'. Check the tag or commit exists. fatal: bad revision 'v99..HEAD'`

---

## Fixed: `_replace_in_file` only updated first version per file (2025-01-24)

**File:** `tooling/src/rerp_tooling/release/bump.py`

**Issue:** `_replace_in_file` used `if in_sec and not replaced:` so that after the first replacement, `replaced` stayed `True` and no further version lines were updated. A Cargo.toml with both `[package]` and `[workspace.package]` sections with explicit `version = "X.Y.Z"` (valid for a workspace root that is also a publishable crate) had only the first section’s version updated, leaving the file with inconsistent versions.

**Fix:** Remove `and not replaced` from the condition so every `version = "old"` in a `[package]` or `[workspace.package]` section is replaced. Keep `replaced = True` when a replacement is made so the function still returns whether any change was made and the file is only written when there is a change.

**Test:** `TestReplaceInFile::test_replaces_both_package_and_workspace_package_version` — file with `[package].version` and `[workspace.package].version` both `"0.1.0"`; after replace both are `"0.1.1"`.

---

## Fixed: `_cargo_toml_paths` used absolute `p.parts`, so `/tmp` triggered `tmp` skip (2025-01-24)

**File:** `tooling/src/rerp_tooling/release/bump.py`

**Issue:** `_cargo_toml_paths` used `if any(part in p.parts for part in SKIP_PARTS)`. `p.parts` is from the full path (e.g. `/tmp/pytest-of-runner/.../components/Cargo.toml`), so the segment `tmp` from `/tmp` is in `p.parts`. With `tmp` in `SKIP_PARTS`, every Cargo.toml was excluded when the project lived under `/tmp` (e.g. Linux CI). Tests that create `tmp_path` under `/tmp` saw `_cargo_toml_paths` return `[]`, breaking `test_excludes_target`, `test_includes_root_components_entities_microservices`, `TestRun::test_success_updates_all_matching`, etc.

**Fix:** Use the path relative to `project_root` when checking SKIP_PARTS: `rel = p.relative_to(project_root)` and `if any(part in rel.parts for part in SKIP_PARTS)`. We only skip when the skip name appears as a segment under the project (e.g. `target/`, `node_modules/`), not in the absolute path (e.g. `/tmp`).

---

## Fixed: Empty error messages in `get_latest_tag.run()` (2025-01-26)

**File:** `tooling/src/rerp_tooling/ci/get_latest_tag.py` lines 50-56

**Issue:** The `run()` function printed empty strings to stderr when `GITHUB_REPOSITORY` or `GITHUB_TOKEN` were missing. This provided no feedback about why the command failed, making debugging difficult. Other error handling in the same file (line 64) and in `validate_version.py` (line 129) properly includes descriptive error messages like "Error: --latest required or set GITHUB_REPOSITORY and GITHUB_TOKEN". The empty `print("")` calls appeared to be forgotten error messages.

**Fix:** Replace empty print statements with descriptive error messages:
- `print("Error: GITHUB_REPOSITORY environment variable is required", file=sys.stderr)`
- `print("Error: GITHUB_TOKEN environment variable is required", file=sys.stderr)`

**Linting:** Added pygrep-hook rule `PGH003` to detect empty print statements to stderr/stdout (see `tooling/pyproject.toml`).

---

## Fixed: Version validation treats API failures as "no releases" (2025-01-26)

**File:** `.github/workflows/ci.yml` lines 60-68, `tooling/src/rerp_tooling/ci/get_latest_tag.py`

**Issue:** The version validation logic in CI treated GitHub API failures the same as "no previous releases." When `rerp ci get-latest-tag` failed due to network issues, rate limiting, or other errors, the workflow used `|| echo ""` which set `LATEST` to empty. The `if [ -n "$LATEST" ]` check then skipped validation, assuming no releases existed. This could allow a version downgrade to be released if the GitHub API was unavailable during a release build, undermining the protection against accidental downgrades.

**Root Cause:** The `get_latest_tag()` function had no retry logic, so transient failures (network errors, rate limits, temporary server errors) immediately failed. The workflow then masked these failures by treating them as "no releases."

**Fix:**
1. **Added retry logic with Fibonacci backoff** to `get_latest_tag()`:
   - Retries on HTTPError (except 404) and URLError
   - Uses Fibonacci sequence: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233 seconds
   - Maximum total wait time: 300 seconds
   - Logs each retry attempt with attempt number and wait time
   - Raises `SystemExit` only after all retries are exhausted

2. **Updated workflow** to fail the job if `get-latest-tag` fails after retries:
   - Removed `|| echo ""` which masked failures
   - Added explicit check: if command fails (non-zero exit), fail the job with error message
   - Only allows releases when: (1) API succeeds and returns empty (no releases), or (2) API succeeds and version validation passes

**Tests:** Added comprehensive tests for retry logic:
- `test_retries_on_http_error_then_succeeds`: Verifies retry on 503 errors
- `test_retries_on_urlerror_then_succeeds`: Verifies retry on network errors
- `test_exhausts_retries_and_raises`: Verifies SystemExit after all retries
- `test_fibonacci_backoff_sequence`: Verifies backoff sequence generation
- `test_run_fails_after_retries_exhausted`: Verifies run() returns 1 on failure
