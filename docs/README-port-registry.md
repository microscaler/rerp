# Port Registry System

## Overview

RERP uses an automated port registry system to manage port assignments for microservices. This prevents port conflicts and automatically updates configuration files.

## Features

- **Automatic port assignment**: Assigns ports starting at 8001
- **Reserved ports**: Automatically skips port 8080 (conflicts with many systems)
- **Config file updates**: Automatically updates Helm values and kind-config.yaml
- **Port tracking**: Maintains a registry of all port assignments

## Usage

Use the `rerp` CLI from the `tooling` package: run `just init` once, then `tooling/.venv/bin/rerp ports` (or `rerp` if `tooling/.venv/bin` is on PATH).

### Assign a Port to a Service

```bash
# Assign next available port and update config files
rerp ports assign <service-name> --update-configs

# Example
rerp ports assign invoice --update-configs
```

### Query a Port Assignment

```bash
rerp ports query <service-name>

# Example
rerp ports query general-ledger
```

### List All Assignments

```bash
rerp ports list
```

### Release a Port

```bash
rerp ports release <service-name>
```

### Update Config Files Only

If a port is already assigned but config files need updating:

```bash
rerp ports update-configs <service-name>
```

## Port Numbering

- **Service Ports**: Start at 8001, increment sequentially
- **NodePorts**: Calculated as `31000 + (port - 8000)`
  - Example: Service port 8001 → NodePort 31001
  - Example: Service port 8002 → NodePort 31002
  - Range: 31000-31999 (1000 ports available)
  - **Avoids conflicts** with PriceWhisperer (uses 30000-30999)
- **Reserved Ports**: 8080 (never assigned)
- **Kubernetes NodePort Valid Range**: 30000-32767 (Kubernetes constraint)

## Files Updated

When you assign a port with `--update-configs`, the script automatically updates:

1. **Helm Values** (`helm/rerp-microservice/values/<service>.yaml`):
   - `service.port`
   - `service.containerPort`
   - `service.nodePort`
   - `service.name`
   - `image.name`
   - `app.serviceName`
   - `app.binaryName`

2. **Kind Config** (`kind-config.yaml`): Adds port mapping only for ports **outside** 8001–8099 (Tilt-managed range). See "Kind and Tilt: Avoid Port Conflicts" below.

3. **OpenAPI servers** (so Swagger "Try it" uses the correct localhost port):
   - **BFF**: `openapi/{suite}/bff-suite-config.yaml` → `metadata.servers` localhost URL set to `http://localhost:<port>`. Regenerate `openapi/{suite}/openapi_bff.yaml` with `bff-generator generate-spec --config openapi/{suite}/bff-suite-config.yaml --output openapi/{suite}/openapi_bff.yaml` (e.g. `openapi/accounting/` for the accounting suite).
   - **Suite microservices** (`openapi/{suite}/{name}/openapi.yaml`): A `http://localhost:<port>/api/v1/{suite}/{name}` server entry is added or updated. `rerp ports` discovers suites by listing `openapi/*/bff-suite-config.yaml` and microservices by walking `openapi/{suite}/{name}/openapi.yaml`.

4. **Port Registry** (`scripts/port-registry.json`):
   - Records the assignment
   - Updates `next_port` for next assignment

## Registry File

The registry is stored in `scripts/port-registry.json`:

```json
{
  "version": "1.0",
  "next_port": 8002,
  "reserved_ports": [8080],
  "assignments": {
    "general-ledger": 8001,
    "invoice": 8002
  },
  "metadata": {
    "description": "Port registry for RERP microservices",
    "last_updated": "2026-01-23T10:00:00",
    "notes": "Ports start at 8001. Port 8080 is reserved due to conflicts."
  }
}
```

## Integration with Bootstrap Script

When creating a new service with `rerp bootstrap microservice`, you can:

1. Run bootstrap script first (creates service structure)
2. Then assign port: `rerp ports assign <service-name> --update-configs`

Or integrate port assignment into the bootstrap script itself.

## Examples

### Complete Workflow for New Service

```bash
# 1. Bootstrap the service
tooling/.venv/bin/rerp bootstrap microservice invoice

# 2. Assign a port and update configs (if not already in registry)
rerp ports assign invoice --update-configs

# 3. Verify assignment
rerp ports query invoice
rerp ports list
```

### Check All Assignments

```bash
rerp ports list
```

Output:
```
Port Assignments:
--------------------------------------------------
  general-ledger              Port: 8001  NodePort: 31001
  invoice                     Port: 8002  NodePort: 31002
  accounts-receivable         Port: 8003  NodePort: 31003
--------------------------------------------------

Total: 3 services
```

## Troubleshooting

### Port Already Assigned

If you try to assign a port to a service that already has one:

```bash
rerp ports assign general-ledger
# Output: ℹ️  Service 'general-ledger' already has port 8001
```

Use `--force` to reassign:

```bash
rerp ports assign general-ledger --force --update-configs
```

### Config Files Not Found

If Helm values file doesn't exist, the script will warn you:

```
⚠️  Helm values file not found: helm/rerp-microservice/values/invoice.yaml
   Create it with: rerp bootstrap microservice invoice
```

Create the service first, then assign the port.

## Validate and Reconcile

### Scan All Sources for Conflicts

```bash
rerp ports validate
rerp ports validate --json   # machine-readable
```

Or `just validate-ports` (runs `rerp ports validate`).

**Scans:** `port-registry.json`, `helm/rerp-microservice/values/*.yaml`, `kind-config.yaml` (hostPort), `Tiltfile` (get_service_port), all `openapi/{suite}/bff-suite-config.yaml` (suites discovered by listing; includes `services.*.port`), each `openapi/{suite}/openapi_bff.yaml` (localhost server), each `openapi/{suite}/{name}/openapi.yaml` (localhost server).

**Reports:** Duplicate `service.port` in helm, duplicate `hostPort` in kind, kind `hostPort` in 8001–8099 (conflicts with Tilt port-forwards), registry/helm/Tiltfile mismatches, and **OpenAPI localhost server port** mismatches (bff-suite-config, openapi_bff, and each suite microservice spec). Fix OpenAPI with `rerp ports update-configs <service>`; for a BFF, regenerate `openapi/{suite}/openapi_bff.yaml` after `update-configs <bff_service_name>`.

Run before `just dev-up` or in CI to catch conflicts.

### Reconcile Registry from Helm

```bash
rerp ports reconcile
rerp ports reconcile --update-configs
```

Adds any service in `helm/.../values/*.yaml` that is missing from the registry, using the port from helm. If that port is already taken, skips with a warning.

### Fix Duplicate Ports (Dedup)

When several services share the same `service.port` in helm (e.g. `invoice` and `ftebe` on 8002), use:

```bash
rerp ports fix-duplicates
rerp ports fix-duplicates --dry-run   # show plan only
```

Or `just fix-duplicate-ports`.

- **Suite** services (from each `openapi/{suite}/bff-suite-config.yaml`: `services` and `bff_service_name`) **keep** their port.
- **Others** (e.g. idam, amd, marketing, billing, ftebe) are assigned the next free port and their **helm values are updated** (`service.port`, `service.containerPort`, `service.nodePort` only; `app.binaryName`, `image`, etc. are left as-is).
- Run `rerp ports validate` after to confirm.

## Kind and Tilt: Avoid Port Conflicts

**Tilt** sets `port_forwards` (e.g. `8001:8001`) so you can reach services on `localhost:8001`.  
**Kind** `extraPortMappings` with `hostPort: 8001` also binds the host. If both exist, you get **"address already in use"**.

- Ports **8001–8099** are **Tilt-managed**: only Tilt’s port-forwards bind them. `kind-config.yaml` must **not** use `hostPort` in that range.
- `rerp ports update-configs` **does not** add kind `hostPort` for 8001–8099.
- `rerp ports validate` warns if kind has `hostPort` in 8001–8099.

## Best Practices

1. **Always use `--update-configs`** when assigning ports to automatically update config files
2. **Run `validate`** before `just dev-up` and in CI
3. **Check assignments** with `list` before creating new services
4. **Don't manually edit** port numbers in config files - use `rerp ports`
5. **Release ports** when removing services to keep the registry clean

## Future Enhancements

- Integration with `rerp bootstrap microservice` for automatic port assignment
- Export to documentation (e.g., update RERP_PREPARATION_PLAN.md automatically)
