# System-Level BFF Generation

> **Status: HISTORICAL_SNAPSHOT.** The original script and paths below are
> retired. The current contract remains that suite configuration is
> authoritative. Services may keep
> concise local paths such as `/payments`, while `gateway_path_style: prefixed`
> publishes `/accounts-payable/payments` and `/accounts-receivable/payments`.
> Every source `operationId` must nevertheless be unique across the suite. BFF
> generation fails on duplicate operation IDs or duplicate final path/method
> pairs so contract ambiguity is caught before Rust generation.

## Overview

System-level Backend for Frontend (BFF) OpenAPI specifications are automatically generated from sub-service specs, following the same pattern as PriceWhisperer.

## How It Works

### Generation Script

The script `scripts/generate_system_bff.py`:

1. **Discovers Sub-Services**: Automatically finds all sub-services in `openapi/{system}/` directories
2. **Aggregates Paths**: Combines all paths from sub-service specs into a single BFF spec
3. **Prefixes Schemas**: Prefixes schema names with service names to avoid conflicts (e.g., `GeneralLedgerAccount`, `AccountsPayableInvoice`)
4. **Updates References**: Updates all `$ref` references to use prefixed schema names
5. **Adds Metadata**: Adds `x-service` and `x-service-base-path` extensions to track which service handles each path
6. **Idempotent & Clobber**: Completely overwrites the output file each time, ensuring consistent output

### Key Features

- **Automatic Discovery**: No manual configuration needed - discovers services from directory structure
- **Schema Conflict Resolution**: Prefixes schemas to avoid naming conflicts
- **Reference Updates**: Automatically updates all `$ref` references to use prefixed names
- **Deterministic Output**: Sorted paths and schemas ensure identical output on each run
- **Service Tracking**: `x-service` extensions identify which service handles each endpoint

## Usage

### Generate for Single System

```bash
cd /Users/casibbald/Workspace/microscaler/rerp
python3 scripts/generate_system_bff.py accounting
```

### Generate for All Systems

```bash
cd /Users/casibbald/Workspace/microscaler/rerp
python3 scripts/generate_system_bff.py
```

## Output Structure

Each system-level `openapi.yaml` contains:

1. **Info Section**: System title, description, and version
2. **Servers**: API gateway base URL
3. **Tags**: Aggregated tags from all sub-services
4. **Paths**: All paths from sub-services with `x-service` metadata
5. **Components**: 
   - Prefixed schemas from all sub-services
   - Common parameters (Page, Limit, Search)
   - Error schema

## Example: Accounting System

The `accounting/openapi.yaml` aggregates:
- 9 sub-services (general-ledger, accounts-payable, accounts-receivable, etc.)
- 46+ paths from all sub-services
- Prefixed schemas (e.g., `GeneralLedgerAccount`, `AccountsPayableInvoice`)
- Service routing metadata for each endpoint

## Auto-Update Strategy

### Option 1: Pre-commit Hook
Add a pre-commit hook to regenerate BFF specs when sub-service specs change:

```bash
# .git/hooks/pre-commit
python3 scripts/generate_system_bff.py
git add openapi/*/openapi.yaml
```

### Option 2: Watch Script
Create a watch script that monitors sub-service specs and regenerates BFF specs:

```bash
# scripts/watch_bff.py
# Uses watchdog or similar to monitor openapi/{system}/*/openapi.yaml
# Regenerates system BFF when sub-service specs change
```

### Option 3: CI/CD Integration
Add to CI/CD pipeline to ensure BFF specs are always up-to-date:

```yaml
# .github/workflows/update-bff.yml
- name: Generate System BFF Specs
  run: python3 scripts/generate_system_bff.py
```

## Comparison with PriceWhisperer

| Feature | PriceWhisperer | RERP |
|---------|---------------|------|
| Discovery | Manual SERVICE_CONFIG | Automatic directory scan |
| Output | Single BFF per service type | System-level BFF per system |
| Schema Prefixing | ✅ Yes | ✅ Yes |
| Reference Updates | ✅ Yes | ✅ Yes |
| Service Metadata | `x-service`, `x-service-port` | `x-service`, `x-service-base-path` |
| Idempotent | ✅ Yes | ✅ Yes |
| Clobber Approach | ✅ Yes | ✅ Yes |

## Next Steps

1. ✅ Implement BFF generation script
2. ⏳ Add pre-commit hook for auto-update
3. ⏳ Add watch script for development
4. ⏳ Integrate into CI/CD pipeline
5. ⏳ Document gateway implementation using BFF specs

---

**Status**: ✅ BFF generation implemented - Ready for auto-update integration
