# Frontend Architecture

> The RERP frontend and its proxy configuration.

**Status:** partially-verified

## Overview

- Frontend dev server uses Vite
- Vite proxy config: `frontend/vite.config.js`
- BFF layer is **bypassed** in dev — frontend hits microservices directly via proxy

## Proxy Map

| Path | Target |
|------|--------|
| `/api/v1/organizations/*` | `localhost:8009` |
| `/api/v1/consignments/*` | `localhost:8003` |
| `/api/v1/fleet/*` | `localhost:8002` |

## BFF Status

**Non-functional.** The BFF is a planned future redesign target. Current setup:
- Frontend → vite proxy → microservices directly
- BFF exists as a concept with accounting suite BFF generated
- But no routing through BFF in dev

## Shared UI

- `ui/shared/` — Shared UI components
- `ui/website/` — Website frontend

## Code Anchors
- Vite config: `frontend/vite.config.js`
- UI components: `ui/`
- BFF config: `openapi/accounting/bff-suite-config.yaml`
