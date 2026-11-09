# CRM-RT-01: rerp_crm_runtime — request context and errors

- **Status:** done
- **Commit:** 083e5dc3
- **Acceptance criteria:**
  - AC-1 ✅ No header, malformed header, or a workspace with no membership all fail 403 `workspace_forbidden`
    before any handler logic (`from_request` is the only way to build a request context); test
    `a_missing_malformed_or_foreign_workspace_is_workspace_forbidden` covers all four shapes plus the
    verified-claims-missing case (401 `unauthenticated`, fail closed like `crm/accounts/impl/src/auth.rs`).
  - AC-2 ✅ `require_role` allows exactly the listed roles; TC-2 (sales calling an owner/admin operation →
    403 `role_forbidden`) and the owner-inverse are both tested.
  - AC-3 ✅ `to_json` output is validated against the `ErrorResponse` constraints (required `code`/`reason`/
    `message`, `reason` pattern `^[a-z][a-z0-9_.]*$`, `fields[]` shape) for every constructor;
    `every_service_declares_the_identical_error_response_schema` asserts the nine CRM specs carry a
    byte-identical `ErrorResponse` schema, so validating against one validates against all.
  - AC-4 ✅ `RequestContext::system(workspace, worker)` is the only context constructor that does not take a
    request; it takes the workspace as an argument and never defaults one (test asserts the exact fields).
  - AC-5 ✅ 18 unit tests, every branch of both modules; `cargo clippy -p rerp_crm_runtime --all-targets
    -- -D warnings` passes, `tooling/scripts/lint_rust.py` passes.
- **Files changed:** `microservices/crm/runtime/src/context.rs`, `microservices/crm/runtime/src/errors.rs`,
  this report, `docs/llmwiki/log.md`. (Separate commit: `tooling/scripts/lint_rust.py` ruff-format fix that
  `just qa` required.)
- **Gates:**
  - `python3 openapi/crm/tools/check_specs.py` → `CRM contracts OK: 9 services, 511 operations, every example valid`
  - `cargo fmt -p rerp_crm_runtime -- --check` → clean
  - `cargo clippy -p rerp_crm_runtime --all-targets -- -D warnings` → clean
  - `cargo test -p rerp_crm_runtime` → `test result: ok. 18 passed; 0 failed`
  - `python3 tooling/scripts/lint_rust.py --no-check` → clean (31 hand-written crates)
- **Contract or design problems found:** none in the nine service contracts. Note: the legacy
  `openapi/crm/{accounts,pipeline,conversion,...}` specs keep the pre-convention `{code, message}` error body
  without `reason`; the AC-3 test is scoped to the nine story services listed in `docs/crm/README.md`, which
  is correct scope but worth knowing when those legacy services are retired.
- **Needs approval:** none — no new dependencies (crate still uses only `brrtrouter serde serde_json chrono uuid`).
- **Notes for the next story:**
  - API decisions beyond the story text, for the supervisor: `ApiError::unauthenticated()` (401) added because
    `from_request` must fail closed without claims; `rate_limited` keeps its seconds in `details` (the
    `ErrorResponse` schema has no other slot) with a `retry_after_secs()` accessor for the `Retry-After` header;
    `RequestContext::system` stores `role: Role::ReadOnly` so nothing can infer privilege from it, and
    `require_role` passes for `ActorKind::System` (workers act under their own use-case logic);
    `WORKSPACE_HEADER` const exported for controllers.
  - `ActorKind::ApiKey` is not constructed by `from_request`: the API-key claims shape is not defined anywhere
    yet. The key middleware (CRM-WSP-04) should build the context directly with `actor_kind: ApiKey`.
  - The concrete HTTP `MembershipSource` (GET `/workspaces/{id}/members` → `MembershipPage`) is deliberately not
    in this story: it needs the API-key/service-principal plumbing CRM-WSP-04 delivers. `CachedMemberships`
    (60 s TTL, `invalidate`/`invalidate_workspace` for `member.changed`) wraps any source today; wire it when
    CRM-RT-02 lands the consumer that calls `invalidate_workspace`.
  - CRM-RT-02 (outbox/relay/consumer) and CRM-RT-03 will fill `events.rs`/`audit.rs`/`idempotency.rs`; the
    guard SQL for their transactions is `context::workspace_guard_sql()`.

## Review

CodeRabbit round 1 (commit 29454f5), findings and resolutions — fix commit **a6375808**:

1. **Major (Security):** an in-flight membership fetch could re-cache a role after `invalidate_workspace`.
   ✅ Fixed: per-lookup generation stamp; a fetch that overlaps any invalidation drops its result
   (`insert_if_not_invalidated`, double-checked under the lock). Test `a_fetch_overlapping_an_invalidation_is_not_cached`.
2. **Major (Stability):** unbounded cache growth from repeated distinct workspace ids (negative entries were
   never removed). ✅ Fixed: `MEMBERSHIP_CACHE_MAX_ENTRIES` (10 000) with expired-sweep-then-evict on insert.
   Test `the_cache_is_bounded_and_evicts`.
3. **Minor:** `forbidden`/`conflict` passed caller-supplied reasons through unvalidated, so a malformed reason
   could violate the `ErrorResponse` pattern. ✅ Fixed: pattern-checked in the constructors; malformed →
   `invalid_state` with the intended value kept in `details`. Test `malformed_reasons_cannot_reach_the_response`.
4. **Nitpick:** the TTL test proved nothing about hits and depended on wall-clock scheduling. ✅ Fixed:
   hit/refetch proven by source call count with 1 h and zero TTLs; no clock waiting.
5. **Nitpick:** the schema-validation test hardcoded `required:` keys, so it could not catch a contract that
   makes e.g. `details` required. ✅ Fixed: required keys are read from the spec's schema text; the validator
   checks them per body.
6. **Docstring coverage warning (pre-merge check, not a required CI check):** ✅ added `///` docs to the public
   helpers that lacked them (`Role::as_str`, `ActorKind::as_str`, `FieldError::new`, `StaticMemberships::new/role`).
