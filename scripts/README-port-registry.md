# Port Registry System

## Overview

RERP uses an automated port registry system to manage port assignments for microservices. This prevents port conflicts and automatically updates configuration files.

## Features

- **Automatic port assignment**: Assigns ports starting at 8001
- **Reserved ports**: Automatically skips port 8080 (conflicts with many systems)
- **Config file updates**: Automatically updates Helm values and kind-config.yaml
- **Port tracking**: Maintains a registry of all port assignments

## Usage

### Assign a Port to a Service

```bash
# Assign next available port and update config files
./scripts/assign-port.py assign <service-name> --update-configs

# Example
./scripts/assign-port.py assign invoice --update-configs
```

### Query a Port Assignment

```bash
./scripts/assign-port.py query <service-name>

# Example
./scripts/assign-port.py query general-ledger
```

### List All Assignments

```bash
./scripts/assign-port.py list
```

### Release a Port

```bash
./scripts/assign-port.py release <service-name>
```

### Update Config Files Only

If a port is already assigned but config files need updating:

```bash
./scripts/assign-port.py update-configs <service-name>
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

2. **Kind Config** (`kind-config.yaml`):
   - Adds port mapping entry

3. **Port Registry** (`scripts/port-registry.json`):
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

When creating a new service with `bootstrap_microservice.py`, you can:

1. Run bootstrap script first (creates service structure)
2. Then assign port: `./scripts/assign-port.py assign <service-name> --update-configs`

Or integrate port assignment into the bootstrap script itself.

## Examples

### Complete Workflow for New Service

```bash
# 1. Bootstrap the service
./scripts/bootstrap_microservice.py invoice

# 2. Assign a port and update configs
./scripts/assign-port.py assign invoice --update-configs

# 3. Verify assignment
./scripts/assign-port.py query invoice
./scripts/assign-port.py list
```

### Check All Assignments

```bash
./scripts/assign-port.py list
```

Output:
```
Port Assignments:
--------------------------------------------------
  general-ledger              Port: 8001  NodePort: 30801
  invoice                     Port: 8002  NodePort: 30802
  accounts-receivable         Port: 8003  NodePort: 30803
--------------------------------------------------

Total: 3 services
```

## Troubleshooting

### Port Already Assigned

If you try to assign a port to a service that already has one:

```bash
./scripts/assign-port.py assign general-ledger
# Output: ℹ️  Service 'general-ledger' already has port 8001
```

Use `--force` to reassign:

```bash
./scripts/assign-port.py assign general-ledger --force --update-configs
```

### Config Files Not Found

If Helm values file doesn't exist, the script will warn you:

```
⚠️  Helm values file not found: helm/rerp-microservice/values/invoice.yaml
   Create it with: bootstrap_microservice.py invoice
```

Create the service first, then assign the port.

## Best Practices

1. **Always use `--update-configs`** when assigning ports to automatically update config files
2. **Check assignments** with `list` before creating new services
3. **Don't manually edit** port numbers in config files - use the script
4. **Release ports** when removing services to keep the registry clean

## Future Enhancements

- Integration with `bootstrap_microservice.py` for automatic port assignment
- Port conflict detection across all config files
- Port range validation
- Export to documentation (e.g., update RERP_PREPARATION_PLAN.md automatically)
