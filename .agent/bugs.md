# Bug Tracker Index

This file serves as an index to all bugs found and fixed in the RERP codebase. Each bug has its own detailed file in the `bugs/` directory.

**Purpose:** Complete bug documentation to prevent recurring issues.

---

## Fixed Bugs

### 2025-01-24

- [BUG-2025-01-24-01: Tiltfile local_resource Success Echo Masks Failed Lint/Gen](bugs/BUG-2025-01-24-01.md)  
  **Status:** ✅ FIXED | **Priority:** High | **Severity:** Silent failure—Tilt reports success when operations fail

- [BUG-2025-01-24-02: create_microservice_build_resource Dead target_path/artifact_path and Inconsistent Path Logic](bugs/BUG-2025-01-24-02.md)  
  **Status:** ✅ FIXED | **Priority:** Medium | **Severity:** Maintainability—dead code and misleading inconsistency

---

## Open Bugs

*No open bugs at this time.*

---

## Bug Report Template

When creating a new bug file, use this template:

```markdown
# BUG-YYYY-MM-DD-NN: [Bug Title]

**Date:** YYYY-MM-DD  
**Status:** 🔴 OPEN / 🟡 IN PROGRESS / ✅ FIXED  
**Priority:** Low / Medium / High / Critical  
**Severity:** Bug / Regression / Performance / Security

## Summary

[Brief description of the bug]

## Discovery

**Date:** YYYY-MM-DD  
**Source:** [How it was discovered]  
**Severity:** `low` / `medium` / `high` / `critical`  
**Status:** `open` / `in_progress` / `fixed`

## Location

- **File:** `path/to/file.rs`
- **Lines:** XXX-YYY

## Description

[Detailed description of the bug]

## Root Cause

[What caused the bug]

## Fix

[How it was fixed]

## Testing

[Tests added/updated]

## Impact

[What was affected]

## Related Files

- `file1.rs` - Description
- `file2.rs` - Description

## Verification

- [ ] Bug identified and root cause analyzed
- [ ] Fix implemented
- [ ] Test added to verify fix
- [ ] Tests run and passing
- [ ] Integration tests verify fix works in real scenarios
```
