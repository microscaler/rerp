# Organization and Company Model

> The organization/company entity and its relationship to addresses.

**Status:** partially-verified

## Company/Organizations Service

- Runs on port 8009 (or should)
- Endpoint: `GET /api/v1/company/organizations/me/addresses`
- Returns 502 in dev — stub impl returns `[]`

## Known Issues

- No `AddressBookEntry` model exists in company service
- Stub implementation returns empty array
- May also have DB connection failure (company service may not be running)

## BFF Proxy

- `/api/v1/organizations/*` → `localhost:8009`

## Code Anchors

- Service path: `microservices/company/` (or `infrastructure/`)
- Impl: `microservices/company/impl/`
- Port registry: `port-registry.json`
