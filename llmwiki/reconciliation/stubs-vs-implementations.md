# Stubs vs Implementations

> Track which services have full implementations vs. stub scaffolding.

**Status:** unverified (needs reconciliation pass)

## Method

For each service:
1. Read `microservices/{suite}/{name}/impl/src/lib.rs` (or `main.rs`)
2. Check if handlers return stub responses or actual business logic
3. Verify OpenAPI paths match implemented endpoints

## Known Stubs

### Consignments Service (8003)
- `POST /api/v1/consignments/jobs/drafts` → returns 403
  - OpenAPI has no security schemes, but brrtrouter default auth may be blocking
  - Draft ID: `c0000001-0001-4000-8000-000000000001`

### Company/Organizations Service (8009)
- `GET /api/v1/company/organizations/me/addresses` → returns 502
  - Stub impl returns `[]` (empty array)
  - No `AddressBookEntry` model exists in company service
  - May also be a DB connection failure (company service not running)

## Implementation Status Categories

| Category | Meaning |
|----------|---------|
| Full | Handlers implement actual business logic |
| Stub | Handlers return placeholder/empty responses |
| Missing | Handler not implemented at all |
| Auth-blocked | Handler exists but 403 due to auth mismatch |
