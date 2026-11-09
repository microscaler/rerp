//! The request context: the verified Sesame claims, `X-Workspace-Id`, the caller's role in that workspace (memberships cached, dropped on `member.changed`), the request id; `require_role` against `x-required-role`; and opening a transaction with `set_config('crm.workspace_id', ..., true)` so row-level security applies. A missing or foreign workspace is 403 `workspace_forbidden`; background work must set the workspace explicitly and fails closed.
//!
//! Story: CRM-RT-01 (docs/crm/stories/).

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

use serde_json::Value;
use uuid::Uuid;

use crate::errors::ApiError;

/// The header every CRM operation (except the edge and the `x-no-workspace` ones) requires.
pub const WORKSPACE_HEADER: &str = "X-Workspace-Id";

/// A role a caller holds in one workspace (`09-api-conventions.md` §3).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Role {
    Owner,
    Admin,
    Marketer,
    Sales,
    ReadOnly,
}

impl Role {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Admin => "admin",
            Self::Marketer => "marketer",
            Self::Sales => "sales",
            Self::ReadOnly => "read-only",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A wire role string that is not one of the five workspace roles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnknownRole(pub String);

impl fmt::Display for UnknownRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown workspace role `{}`", self.0)
    }
}

impl std::error::Error for UnknownRole {}

impl FromStr for Role {
    type Err = UnknownRole;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "owner" => Ok(Self::Owner),
            "admin" => Ok(Self::Admin),
            "marketer" => Ok(Self::Marketer),
            "sales" => Ok(Self::Sales),
            "read-only" => Ok(Self::ReadOnly),
            other => Err(UnknownRole(other.to_owned())),
        }
    }
}

/// Who is calling: a Sesame user, an API key acting as a service principal, or this suite's own worker.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorKind {
    User,
    ApiKey,
    System,
}

impl ActorKind {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::ApiKey => "api-key",
            Self::System => "system",
        }
    }
}

impl fmt::Display for ActorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Who is calling, into which workspace, with which role, for which request. Every service handler builds one of
/// these before any handler logic runs.
#[derive(Clone, Debug)]
pub struct RequestContext {
    pub workspace_id: Uuid,
    pub actor_kind: ActorKind,
    /// Sesame `sub`, API key id, or `system:<worker>`.
    pub actor_id: String,
    pub role: Role,
    pub request_id: Option<String>,
}

impl RequestContext {
    /// From the validated claims and the `X-Workspace-Id` header, looking the caller's membership up in
    /// `memberships`. A missing, unverified or `sub`-less token is 401 `unauthenticated`; a missing or malformed
    /// header, or no membership, is 403 `workspace_forbidden` — before any handler logic.
    ///
    /// An API key calling as a service principal is built by the key middleware (CRM-WSP-04 defines its claims
    /// shape) with `actor_kind: ApiKey`; here verified claims are a Sesame user.
    pub fn from_request(
        claims: Option<&Value>,
        workspace_header: Option<&str>,
        request_id: Option<&str>,
        memberships: &dyn MembershipSource,
    ) -> Result<Self, ApiError> {
        let claims = claims.ok_or_else(ApiError::unauthenticated)?;
        let actor_id = claims
            .get("sub")
            .and_then(Value::as_str)
            .filter(|sub| !sub.is_empty())
            .ok_or_else(ApiError::unauthenticated)?
            .to_owned();
        let workspace_id = workspace_header
            .and_then(|raw| Uuid::parse_str(raw).ok())
            .ok_or_else(|| ApiError::forbidden("workspace_forbidden"))?;
        let role = memberships
            .role_of(workspace_id, &actor_id)
            .ok_or_else(|| ApiError::forbidden("workspace_forbidden"))?;
        Ok(Self {
            workspace_id,
            actor_kind: ActorKind::User,
            actor_id,
            role,
            request_id: request_id.map(str::to_owned),
        })
    }

    /// For workers and consumers: an explicit workspace, `ActorKind::System`, actor `system:<worker>`. The only way
    /// to get a context without a request; it never defaults a workspace. The stored role is `ReadOnly` so nothing
    /// may infer privilege from it — `require_role` passes for a system actor because workers act under their own
    /// use-case logic, not the human role matrix.
    #[must_use]
    pub fn system(workspace_id: Uuid, worker: &str) -> Self {
        Self {
            workspace_id,
            actor_kind: ActorKind::System,
            actor_id: format!("system:{worker}"),
            role: Role::ReadOnly,
            request_id: None,
        }
    }

    /// 403 `role_forbidden` unless the caller's role is exactly one of `allowed` (the operation's `x-required-role`).
    /// A system actor passes; see `system`.
    pub fn require_role(&self, allowed: &[Role]) -> Result<(), ApiError> {
        if self.actor_kind == ActorKind::System {
            return Ok(());
        }
        if allowed.contains(&self.role) {
            Ok(())
        } else {
            Err(ApiError::forbidden("role_forbidden"))
        }
    }
}

/// Where a caller's role in a workspace is looked up. Services use the HTTP client to `crm/workspace` behind this
/// trait (wrap it in `CachedMemberships`); `crm/workspace` itself reads its own table; tests use
/// `StaticMemberships`.
pub trait MembershipSource {
    fn role_of(&self, workspace_id: Uuid, user_id: &str) -> Option<Role>;
}

/// A fixed map of memberships — the source tests and service tests use.
#[derive(Clone, Debug, Default)]
pub struct StaticMemberships {
    roles: HashMap<(Uuid, String), Role>,
}

impl StaticMemberships {
    #[must_use]
    pub fn new(entries: impl IntoIterator<Item = (Uuid, String, Role)>) -> Self {
        Self {
            roles: entries
                .into_iter()
                .map(|(workspace_id, user_id, role)| ((workspace_id, user_id), role))
                .collect(),
        }
    }

    #[must_use]
    pub fn role(&self, workspace_id: Uuid, user_id: &str) -> Option<Role> {
        self.roles.get(&(workspace_id, user_id.to_owned())).copied()
    }
}

impl MembershipSource for StaticMemberships {
    fn role_of(&self, workspace_id: Uuid, user_id: &str) -> Option<Role> {
        self.role(workspace_id, user_id)
    }
}

/// Wraps any `MembershipSource` with a per-workspace-and-user cache (default 60 s, dropped wholesale or per
/// workspace on `member.changed` — consumers call `invalidate_workspace` when CRM-RT-02 delivers the event).
/// Non-membership is cached too, so a flood of foreign-workspace calls does not re-hit the source.
pub struct CachedMemberships<S> {
    source: S,
    ttl: Duration,
    entries: Mutex<HashMap<MembershipKey, CacheEntry>>,
}

#[derive(Eq, Hash, PartialEq)]
struct MembershipKey {
    workspace_id: Uuid,
    user_id: String,
}

struct CacheEntry {
    fetched_at: std::time::Instant,
    role: Option<Role>,
}

/// The story's 60 s membership cache TTL.
pub const MEMBERSHIP_CACHE_TTL: Duration = Duration::from_secs(60);

impl<S: MembershipSource> CachedMemberships<S> {
    #[must_use]
    pub fn new(source: S) -> Self {
        Self::with_ttl(source, MEMBERSHIP_CACHE_TTL)
    }

    /// A non-default TTL (tests; nothing in production should change it).
    #[must_use]
    pub fn with_ttl(source: S, ttl: Duration) -> Self {
        Self {
            source,
            ttl,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Drop every cached membership (the cache is global on `member.changed`).
    pub fn invalidate(&self) {
        self.locked().clear();
    }

    /// Drop one workspace's cached memberships (`member.changed` for that workspace).
    pub fn invalidate_workspace(&self, workspace_id: Uuid) {
        self.locked()
            .retain(|key, _| key.workspace_id != workspace_id);
    }

    #[must_use]
    pub fn cached_len(&self) -> usize {
        self.locked().len()
    }

    fn locked(&self) -> MutexGuard<'_, HashMap<MembershipKey, CacheEntry>> {
        match self.entries.lock() {
            Ok(guard) => guard,
            // A panic elsewhere must not poison every later lookup: take the map as it is.
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

impl<S: MembershipSource> MembershipSource for CachedMemberships<S> {
    fn role_of(&self, workspace_id: Uuid, user_id: &str) -> Option<Role> {
        let key = MembershipKey {
            workspace_id,
            user_id: user_id.to_owned(),
        };
        {
            let entries = self.locked();
            if let Some(entry) = entries.get(&key) {
                if entry.fetched_at.elapsed() < self.ttl {
                    return entry.role;
                }
            }
        }
        let role = self.source.role_of(workspace_id, user_id);
        self.locked().insert(
            MembershipKey {
                workspace_id,
                user_id: user_id.to_owned(),
            },
            CacheEntry {
                fetched_at: std::time::Instant::now(),
                role,
            },
        );
        role
    }
}

/// SQL every transaction runs first so row-level security applies (Lifeguard binds the workspace id as `$1`).
#[must_use]
pub fn workspace_guard_sql() -> &'static str {
    "SELECT set_config('crm.workspace_id', $1, true)"
}

/// A counting source to prove cache hits and refetches (test-only use, but harmless to keep public for service tests).
#[cfg(test)]
#[derive(Default)]
struct CountingMemberships {
    inner: StaticMemberships,
    calls: std::cell::Cell<usize>,
}

#[cfg(test)]
impl MembershipSource for CountingMemberships {
    fn role_of(&self, workspace_id: Uuid, user_id: &str) -> Option<Role> {
        self.calls.set(self.calls.get() + 1);
        self.inner.role_of(workspace_id, user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn workspace() -> Uuid {
        Uuid::parse_str("01940f6e-2f00-7abc-8901-234567890abc").expect("test workspace id")
    }

    fn other_workspace() -> Uuid {
        Uuid::parse_str("01940f6e-2f00-7abc-8901-234567890abd").expect("test workspace id")
    }

    const USER: &str = "a1000001-0001-4000-8000-000000000004";

    fn claims() -> Value {
        serde_json::json!({ "sub": USER, "tenant_id": "pricewhisperer" })
    }

    fn memberships() -> StaticMemberships {
        StaticMemberships::new([
            (workspace(), USER.to_owned(), Role::Sales),
            (workspace(), "owner-1".to_owned(), Role::Owner),
        ])
    }

    #[test]
    fn roles_parse_and_render_the_wire_names() {
        for (raw, role) in [
            ("owner", Role::Owner),
            ("admin", Role::Admin),
            ("marketer", Role::Marketer),
            ("sales", Role::Sales),
            ("read-only", Role::ReadOnly),
        ] {
            assert_eq!(raw.parse::<Role>(), Ok(role), "{raw}");
            assert_eq!(role.to_string(), raw);
        }
        for bad in ["", "read_only", "readonly", "OWNER", "owner ", "root"] {
            assert!(bad.parse::<Role>().is_err(), "{bad} must not parse");
        }
    }

    #[test]
    fn from_request_builds_the_context_for_a_member() {
        let context = RequestContext::from_request(
            Some(&claims()),
            Some(&workspace().to_string()),
            Some("req-7"),
            &memberships(),
        )
        .expect("member request");
        assert_eq!(context.workspace_id, workspace());
        assert_eq!(context.actor_kind, ActorKind::User);
        assert_eq!(context.actor_id, USER);
        assert_eq!(context.role, Role::Sales);
        assert_eq!(context.request_id.as_deref(), Some("req-7"));
    }

    #[test]
    fn from_request_fails_closed_without_verified_claims() {
        for claims in [
            None,
            Some(&serde_json::json!({})),
            Some(&serde_json::json!({"sub": ""})),
        ] {
            let error = RequestContext::from_request(
                claims,
                Some(&workspace().to_string()),
                None,
                &memberships(),
            )
            .expect_err("must not build a context");
            assert_eq!(error.status, 401, "{claims:?}");
            assert_eq!(error.reason, "unauthenticated");
        }
    }

    #[test]
    fn a_missing_malformed_or_foreign_workspace_is_workspace_forbidden() {
        // TC-1: no header, bad uuid, non-member — 403 workspace_forbidden each, before any handler logic.
        let member_claims = claims();
        let stranger_claims = serde_json::json!({ "sub": "stranger-1" });
        let member_ws = workspace().to_string();
        let foreign_ws = other_workspace().to_string();
        let cases: Vec<(&str, Option<&Value>, Option<&str>)> = vec![
            ("no header", Some(&member_claims), None),
            ("malformed header", Some(&member_claims), Some("not-a-uuid")),
            (
                "non-member workspace",
                Some(&member_claims),
                Some(foreign_ws.as_str()),
            ),
            (
                "stranger claims",
                Some(&stranger_claims),
                Some(member_ws.as_str()),
            ),
        ];
        for (name, claims, header) in cases {
            let error = RequestContext::from_request(claims, header, None, &memberships())
                .expect_err(&format!("{name} must be rejected"));
            assert_eq!(error.status, 403, "{name}");
            assert_eq!(error.reason, "workspace_forbidden", "{name}");
        }
    }

    #[test]
    fn require_role_allows_exactly_the_listed_roles() {
        // TC-2: member with role sales calls an owner/admin operation.
        let sales = RequestContext::from_request(
            Some(&claims()),
            Some(&workspace().to_string()),
            None,
            &memberships(),
        )
        .expect("member request");
        let denied = sales
            .require_role(&[Role::Owner, Role::Admin])
            .expect_err("sales is not owner/admin");
        assert_eq!(denied.status, 403);
        assert_eq!(denied.reason, "role_forbidden");
        assert!(sales.require_role(&[Role::Sales]).is_ok());
        assert!(sales.require_role(&[Role::ReadOnly, Role::Sales]).is_ok());

        let owner = RequestContext::from_request(
            Some(&serde_json::json!({ "sub": "owner-1" })),
            Some(&workspace().to_string()),
            None,
            &memberships(),
        )
        .expect("owner request");
        assert!(owner.require_role(&[Role::Owner, Role::Admin]).is_ok());
        assert!(owner.require_role(&[Role::Sales]).is_err());
    }

    #[test]
    fn system_never_defaults_a_workspace_and_acts_as_itself() {
        let context = RequestContext::system(workspace(), "send-worker");
        assert_eq!(context.workspace_id, workspace());
        assert_eq!(context.actor_kind, ActorKind::System);
        assert_eq!(context.actor_id, "system:send-worker");
        assert_eq!(context.request_id, None);
        // Workers act under their own use-case logic, not the human role matrix.
        assert!(context.require_role(&[Role::Owner]).is_ok());
    }

    #[test]
    fn the_guard_sql_sets_the_transaction_scoped_workspace() {
        assert_eq!(
            workspace_guard_sql(),
            "SELECT set_config('crm.workspace_id', $1, true)"
        );
    }

    #[test]
    fn the_cache_serves_hits_then_refetches_after_the_ttl() {
        let counting = CountingMemberships {
            inner: memberships(),
            calls: Cell::new(0),
        };
        let cache = CachedMemberships::with_ttl(counting, Duration::from_millis(20));
        assert_eq!(cache.role_of(workspace(), USER), Some(Role::Sales));
        assert_eq!(cache.role_of(workspace(), USER), Some(Role::Sales));
        assert_eq!(cache.cached_len(), 1, "the second lookup must not refetch");

        // Wait out the TTL without blocking a may worker (this is a test on its own thread anyway).
        let deadline = std::time::Instant::now() + Duration::from_millis(30);
        while std::time::Instant::now() < deadline {
            std::hint::spin_loop();
        }
        assert_eq!(cache.role_of(workspace(), USER), Some(Role::Sales));
        assert_eq!(
            cache.cached_len(),
            1,
            "the entry was replaced, not duplicated"
        );
    }

    #[test]
    fn the_cache_also_holds_non_membership_and_clears_on_demand() {
        let cache = CachedMemberships::new(StaticMemberships::default());
        assert_eq!(cache.role_of(workspace(), USER), None);
        assert_eq!(cache.cached_len(), 1, "non-membership is cached too");

        let other = CachedMemberships::new(StaticMemberships::new([
            (workspace(), USER.to_owned(), Role::Admin),
            (other_workspace(), USER.to_owned(), Role::Marketer),
        ]));
        assert_eq!(other.role_of(workspace(), USER), Some(Role::Admin));
        assert_eq!(other.role_of(other_workspace(), USER), Some(Role::Marketer));
        other.invalidate_workspace(workspace());
        assert_eq!(other.cached_len(), 1, "only the changed workspace drops");
        other.invalidate();
        assert_eq!(other.cached_len(), 0);
    }

    #[test]
    fn a_stale_cache_entry_for_a_removed_member_clears_after_invalidation() {
        // member.changed path: without the event, a removed member keeps their cached role only until the TTL.
        let counting = CountingMemberships {
            inner: memberships(),
            calls: Cell::new(0),
        };
        let cache = CachedMemberships::new(counting);
        assert_eq!(cache.role_of(workspace(), USER), Some(Role::Sales));
        cache.invalidate_workspace(workspace());
        assert_eq!(
            cache.cached_len(),
            0,
            "the event drops the workspace at once"
        );
        assert_eq!(cache.role_of(workspace(), USER), Some(Role::Sales));
        assert_eq!(
            cache.source.calls.get(),
            2,
            "the refetch went to the source"
        );
    }
}
