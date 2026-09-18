# Suite BFF Flow

> How the BFF generator aggregates a suite's microservice OpenAPI specs into a single BFF spec.

**Status:** partially-verified

## Overview

```
openapi/{suite}/bff-suite-config.yaml
    → reads bff_service_name
    → walks openapi/{suite}/{name}/openapi.yaml for each service
    → openapi/{suite}/openapi_bff.yaml (aggregated spec)
```

## BFF Config Format

`openapi/{suite}/bff-suite-config.yaml`:
```yaml
bff_service_name: <suite-name>-bff
services:
  - <service-name-1>
  - <service-name-2>
  ...
```

## Key Conventions

- Suites are inferred **dynamically**: `rerp` and tooling list `openapi/` subdirs containing `bff-suite-config.yaml`.
- No hardcoded suite/BFF mappings — adding a new suite's config auto-registers it.
- The current BFF is **non-functional** — the frontend uses vite proxy to bypass it entirely.
- BFF redesign is a future priority.

## Code Anchors

- BFF generator: `tooling/` (bff-generator CLI)
- Config: `openapi/accounting/bff-suite-config.yaml`
- Output: `openapi/accounting/openapi_bff.yaml`
