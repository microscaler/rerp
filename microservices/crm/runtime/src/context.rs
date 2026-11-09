//! The request context: the verified Sesame claims, `X-Workspace-Id`, the caller's role in that workspace (memberships cached, dropped on `member.changed`), the request id; `require_role` against `x-required-role`; and opening a transaction with `set_config('crm.workspace_id', ..., true)` so row-level security applies. A missing or foreign workspace is 403 `workspace_forbidden`; background work must set the workspace explicitly and fails closed.
//!
//! Story: CRM-RT-01 (docs/crm/stories/).
