# CRM-PTL-01: Move the portal and the mock to the new contracts

**Repository:** PriceWhisperer (`ui/platform`, `microservices/pact-mock-server/seeds/crm`) · **Build step:** after each service · **Points:** 8

## Goal

Every change in `openapi/crm/docs/12-portal-migration.md` is made in the portal's CRM client and the mock seeds, so the
portal runs against the new contracts (first through the mock, then service by service against RERP).

## Steps

1. Path map in `ui/platform/src/utils/crm/api.ts` from 12-portal-migration.md §4; `X-Workspace-Id` on every call.
2. Field changes (§2): organisation_*, expected_close_date, decimal strings for money, `reason` on errors, `limit`.
3. Seeds: move `path` fields to the new services and paths; money as decimal strings.
4. Run the Epic 55 contract check (US_55_37) against `openapi/crm/*/openapi.yaml`.

## Acceptance criteria

- AC-1 The contract check passes for all 237 portal operations.
- AC-2 `npx tsc --noEmit` passes; every CRM screen renders against the mock.
