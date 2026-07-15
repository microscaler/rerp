# Suite BFF Generation

> How BFF specs are generated for each suite from bff-suite-config.yaml.

**Status:** partially-verified

## Command

```bash
bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml
```

## Dynamic Suite Discovery

Suites are **not hardcoded**:
1. `rerp` tooling lists `openapi/` subdirs
2. Filters those containing `bff-suite-config.yaml`
3. Reads `bff_service_name` from config
4. Walks `openapi/{suite}/{name}/openapi.yaml` for each microservice

Adding a new suite = adding its `bff-suite-config.yaml`. No code changes needed.

## Current BFF Status

**The BFF is non-functional.** Frontend bypasses it via vite proxy (`frontend/vite.config.js`).

Proxy map:
- `/api/v1/organizations/*` → localhost:8009
- `/api/v1/consignments/*` → localhost:8003
- `/api/v1/fleet/*` → localhost:8002

## System BFF Generation

```bash
rerp bff generate-system
```

Uses `tooling/.venv/bin/rerp` after `just init`.

## Code Anchors
- Config: `openapi/accounting/bff-suite-config.yaml`
- Output: `openapi/accounting/openapi_bff.yaml`
- Historical architecture snapshot:
  `docs/history/architecture-snapshots/SYSTEM_BFF_GENERATION.md`
