# RERP Microservice Helm Chart

Reusable Helm chart for RERP microservices (accounting, auth, etc.).

## Usage

### Deploy a specific microservice

```bash
# Accounting (examples; see Tiltfile for full set)
helm install general-ledger ./helm/rerp-microservice -f ./helm/rerp-microservice/values/general-ledger.yaml
helm install invoice ./helm/rerp-microservice -f ./helm/rerp-microservice/values/invoice.yaml
helm install bff ./helm/rerp-microservice -f ./helm/rerp-microservice/values/bff.yaml
```

### Upgrade

```bash
helm upgrade general-ledger ./helm/rerp-microservice -f ./helm/rerp-microservice/values/general-ledger.yaml
```

### Uninstall

```bash
helm uninstall general-ledger
```

## Values Files

Service-specific values in `values/`:

- **Accounting**: `general-ledger`, `invoice`, `accounts-receivable`, `accounts-payable`, `bank-sync`, `asset`, `budget`, `edi`, `financial-reports`, `bff`
- **Other**: `idam`, `marketing`, `amd`, `billing`, `ftebe`

## Customization

Override via `--set`, e.g.:

```bash
helm install general-ledger ./helm/rerp-microservice -f ./helm/rerp-microservice/values/general-ledger.yaml --set deployment.replicas=3
```

## Configuration

The chart creates ConfigMap, Deployment, and Service (NodePort for local dev). See `values.yaml` for defaults (database, redis, observability).
