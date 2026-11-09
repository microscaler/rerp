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
