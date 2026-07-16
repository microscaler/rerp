# Goal 3: Shared-Cluster Delivery

## Objective

Make RERP repeatably deployable through Flux to the Microscaler shared
Kubernetes cluster, using the same product-owned profiles, parameterized images,
and health expectations needed by a self-hosted deployment. Tilt is the image
build and passive acceptance inner loop; it is not a second deployer.

## Broad Outcomes

- A valid, minimal Tilt graph for active RERP image builds and deployment acceptance.
- Flux-owned product source, ordered foundation/services reconciliation, and drift correction.
- RERP namespace, database bootstrap, migrations, secrets, and service discovery.
- Reusable Helm values without stale shared-Kind assumptions.
- Health, readiness, metrics, logs, traces, and resource limits.
- Deployment conventions suitable for both dog-food and external open-source operators.
- A Flux-owned suspended catalog for source components that have not passed
  functional delivery gates.

## Initial Acceptance Gates

- Restarting Tilt produces the delivered image-build resources and no Kubernetes manifests.
- Flux creates the RERP namespace/profile/bootstrap and active services declaratively.
- Migrations and object-store provisioning complete before services become Ready.
- Invoice and GL pods become Ready and survive restart.
- Every other Accounting HelmRelease is Flux-owned, suspended, labelled
  `scaffold-only`, and has no Deployment.
- East-west service discovery works without developer-specific hostnames.
- No plaintext production credential is committed or rendered.
- Metrics and logs identify tenant-safe request traces without leaking financial data.

## Questions To Thrash Out

- What is the production-grade replacement for the dev-only Git image update flow?
- What is the supported local development topology outside Microscaler's cluster?
- What are the minimum observability and backup requirements for the dog-food gate?
- How will release images be built and signed for public consumption?

## Dependencies

- Goal 1 for a reliable toolchain.
- Goal 2 for the active and suspended service inventories.
