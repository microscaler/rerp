# RERP Microservice Helm Chart

Reusable Helm chart for RERP microservices (accounting, auth, etc.).

In shared-k8s, humans and Tilt do not install this chart directly. Product
profiles declare Flux `HelmRelease` resources and Flux owns install, upgrade,
rollback and drift correction. The commands below are local render/debug
examples only.

Every workload listens on port `8080`, and every Kubernetes Service maps
`8080` to container port `8080`. Services are internal `ClusterIP` resources;
host exposure belongs to an ingress, BFF, or an explicit Tilt port-forward.
Consequently, service-specific values only identify the workload and do not
allocate ports or NodePorts.

## Usage

### Deploy a specific microservice

```bash
# Accounting local render examples (not the shared-k8s deployment path)
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

- **Accounting**: all seventeen current source components. Only General Ledger
  and Invoice are active in the dev Flux profile; the other fifteen are
  represented by suspended catalog HelmReleases until their activation gates
  pass.
- **Other**: `idam`, `marketing`, `amd`, `billing`, `ftebe`

## Customization

Override via `--set`, e.g.:

```bash
helm install general-ledger ./helm/rerp-microservice -f ./helm/rerp-microservice/values/general-ledger.yaml --set deployment.replicas=3
```

## Configuration

The chart creates a ConfigMap, Deployment, and internal ClusterIP Service. The
Deployment can import the shared database ConfigMap and Secret through
`databaseEnv`, while service-specific credentials such as the invoice object
store use `extraEnv` with Kubernetes `valueFrom` entries.

### Coroutine stack sizing

`BRRTR_STACK_SIZE` configures May coroutine stacks used by BRRTRouter; it is not
the native OS-thread stack. An undersized value can abort the process with a
coroutine stack-overflow message and exit code 139. The chart baseline is
128 KiB (`0x20000`). General Ledger and Invoice use 256 KiB (`0x40000`), the
BRRTRouter default maximum, because their handlers combine typed validation,
Lifeguard/RLS transactions, accounting validation, and response serialization.

Set overrides in the owning service values file rather than patching a live
Deployment:

```yaml
env:
  BRRTR_STACK_SIZE: "0x40000"
```

Any increase must be considered alongside concurrency and pod memory limits;
do not lower the shared baseline without a measured stack-usage test.
