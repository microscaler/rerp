# CRM-RT-04: rerp_crm_runtime — signed tokens for the edge

**Crate:** `microservices/crm/runtime` (module `tokens`) · **Build step:** 2 · **Depends on:** CRM-RT-01 · **Points:** 3

## Goal

Links in messages and pages (open pixel, click, unsubscribe, preferences, confirm, calendar) carry a signed token the
edge verifies alone, so it answers at once without a database (ADR 004).

## API to provide

```rust
pub enum TokenKind { Open, Click, Unsubscribe, Preferences, Confirm, Calendar }
pub struct Claims { pub kind: TokenKind, pub workspace_id: Uuid, pub subject: String /* send id, subscription id, ... */,
                    pub target: Option<String> /* click URL */, pub expires_at: Option<DateTime<Utc>> }
pub fn sign(key: &[u8], claims: &Claims) -> String;        // URL-safe; HMAC-SHA256 over a compact encoding
pub fn verify(key: &[u8], token: &str) -> Result<Claims, TokenError>;   // Invalid, Expired, WrongKind
```

Keys come from Secret Manager (`crm-token-key`), with the previous key accepted during rotation (`verify` takes both).

## Acceptance criteria

- AC-1 Round trip for every kind; a token changed by one character fails.
- AC-2 Confirm tokens expire after 7 days (compliance setting); click and open tokens do not expire.
- AC-3 Tokens are ≤ 200 characters for a click with a 100-character URL.
- AC-4 Verifying with the previous key works until rotation completes.

## Verify

```bash
cd microservices && cargo test -p rerp_crm_runtime tokens
```
