# CRM-RT-01: rerp_crm_runtime — request context and errors

**Crate:** `microservices/crm/runtime` (modules `context`, `errors`) · **Build step:** 0 · **Depends on:** — · **Points:** 5

## Goal

One way for every CRM service to know who is calling, in which workspace, with which role, and to answer errors in the
shared shape. Every service story depends on this.

## Read first

- `openapi/crm/docs/09-api-conventions.md` §3 (workspaces, roles), §6 (errors).
- `openapi/crm/docs/14-data-model.md` §1 (row-level security, `crm.workspace_id`).
- `microservices/crm/accounts/impl/src/auth.rs` and `identity.rs`: how a Sesame JWT's claims are read today.
- `docs/adrs/004-crm-capture-intake-and-workspaces.md` §6.

## API to provide

```rust
// context.rs
pub enum Role { Owner, Admin, Marketer, Sales, ReadOnly }            // parse from "owner", ..., "read-only"
pub enum ActorKind { User, ApiKey, System }
pub struct RequestContext {
    pub workspace_id: uuid::Uuid,
    pub actor_kind: ActorKind,
    pub actor_id: String,          // Sesame sub, API key id, or "system:<worker>"
    pub role: Role,
    pub request_id: Option<String>,
}
impl RequestContext {
    /// From the validated claims and the X-Workspace-Id header. Looks up the caller's membership (MembershipSource),
    /// 403 workspace_forbidden when absent.
    pub fn from_request(claims: Option<&serde_json::Value>, workspace_header: Option<&str>,
                        request_id: Option<&str>, memberships: &dyn MembershipSource) -> Result<Self, ApiError>;
    /// For workers and consumers: an explicit workspace, actor System.
    pub fn system(workspace_id: uuid::Uuid, worker: &str) -> Self;
    /// 403 role_forbidden unless self.role is in `allowed` (the operation's x-required-role).
    pub fn require_role(&self, allowed: &[Role]) -> Result<(), ApiError>;
}
pub trait MembershipSource { fn role_of(&self, workspace_id: uuid::Uuid, user_id: &str) -> Option<Role>; }
/// SQL every transaction runs first so row-level security applies:
pub fn workspace_guard_sql() -> &'static str;  // "SELECT set_config('crm.workspace_id', $1, true)"

// errors.rs
pub struct ApiError { pub status: u16, pub reason: String, pub message: String,
                      pub details: Option<String>, pub fields: Vec<FieldError> }
pub struct FieldError { pub field: String, pub message: String }
impl ApiError {
    pub fn not_found() -> Self;                       // 404 not_found
    pub fn forbidden(reason: &str) -> Self;           // 403
    pub fn conflict(reason: &str, message: &str) -> Self;  // 409
    pub fn invalid(fields: Vec<FieldError>) -> Self;  // 422 validation_failed
    pub fn rate_limited(retry_after_secs: u32) -> Self;  // 429
    pub fn to_json(&self, request_id: Option<&str>) -> serde_json::Value; // the ErrorResponse body
}
```

The membership source used by services is an HTTP client to crm/workspace with a cache (60 s), dropped on
`member.changed`; crm/workspace itself reads its own table. Put the client behind the trait so tests use a map.

## Acceptance criteria

- AC-1 A request without `X-Workspace-Id`, with a malformed one, or for a workspace the caller is not a member of fails
  with 403 `workspace_forbidden` before any handler logic.
- AC-2 `require_role` allows exactly the listed roles.
- AC-3 `ApiError::to_json` validates against `ErrorResponse` in every CRM spec (reason pattern `^[a-z][a-z0-9_.]*$`).
- AC-4 `RequestContext::system` is the only way to get a context without a request; it never defaults a workspace.
- AC-5 Unit tests cover every branch; `cargo clippy -p rerp_crm_runtime --all-targets -- -D warnings` passes.

## Test cases

| ID | Case | Expected |
|---|---|---|
| TC-1 | No header / bad uuid / non-member | 403 `workspace_forbidden` each |
| TC-2 | Member with role sales calls an owner/admin operation | 403 `role_forbidden` |
| TC-3 | Each constructor's JSON against the audience spec's ErrorResponse | Valid |

## Verify

```bash
cd microservices && cargo test -p rerp_crm_runtime && cargo clippy -p rerp_crm_runtime --all-targets -- -D warnings
```
