# Port Registry and Tilt Configuration

> How dev services are configured and port-managed in RERP's Tilt-based dev environment.

**Status:** partially-verified

## Tilt Configuration

- Tilt runs on port **10352**
- Tilt host: `TILT_HOST=0.0.0.0`
- Dev environment is managed via Tilt files (Tiltfile at project root)

## Port Registry

- File: `port-registry.json` at **project root**
- Used by `rerp ports` command and all automation
- All 9+ microservices register their ports here

## Service Ports (Typical)

| Service | Suite | Typical Port |
|---------|-------|-------------|
| company/organizations | core | 8009 |
| consignments | supply-chain | 8003 |
| fleet | supply-chain | 8002 |
| *(other services)* | *(their suite)* | *(assigned by Tilt)* |

## DB Configuration

- Single Postgres instance shared by all services
- `DB_POOL_MAX` env var controls pool size (default 10 in dev)
- Connection pool is shared across all 9 microservices
- Replica routing: disabled

## Frontend

- Frontend dev server uses vite proxy to reach microservices directly
- BFF layer is bypassed in dev (non-functional)
- Proxy config: `frontend/vite.config.js`
