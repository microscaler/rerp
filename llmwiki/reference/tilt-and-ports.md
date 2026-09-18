# Kubernetes-native Tilt and service ports

> The active RERP development deployment contract.

**Status:** verified against shared-k8s on 2026-07-14

## Service networking

- Every application container listens on port `8080`.
- Every application Service is `ClusterIP` and maps `8080` to `8080`.
- Kubernetes DNS, namespaces and service names provide isolation; per-service
  port allocation and `port-registry.json` are retired.
- Only public entry points receive ingress or an explicit Tilt port-forward.
  The current accounting slice forwards host `8080` to invoice `8080`.

## Shared platform

RERP uses the `shared-k8s` context and registry `10.177.76.220:5000`.
PostgreSQL, Redis, MinIO and observability remain owned by the sibling
`shared-k8s-cluster` repository. RERP owns its `rerp` namespace and application
configuration.

## Active Tilt resources

The root `Tiltfile` currently declares only delivered runtime components:

- `invoice`
- `rerp-namespace`
- `rerp-database-env`
- `rerp-db-init` (manual migration/bootstrap action)
- `rerp-object-store`
- `invoice-contract-refresh` (manual)
- `invoice-tests` (manual)

Placeholder accounting directories are not presented as runnable workloads.
