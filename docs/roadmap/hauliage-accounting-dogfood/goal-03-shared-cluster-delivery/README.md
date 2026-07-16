# Goal 3: Shared-Cluster Delivery

## Objective

Make RERP repeatably deployable through Flux to the Microscaler shared
Kubernetes cluster, using the same product-owned profiles, parameterized images,
and health expectations needed by a self-hosted deployment. Tilt is the image
build and passive acceptance inner loop; it is not a second deployer.

## Broad Outcomes

- A valid, minimal Tilt graph for active RERP image builds and deployment acceptance.
- Flux-owned product source, ordered foundation/services reconciliation, and drift correction.
- RERP namespace, role/database bootstrap, Tilt-owned dev migrations, secrets,
  and service discovery.
- Reusable Helm values without stale shared-Kind assumptions.
- Health, readiness, metrics, logs, traces, and resource limits.
- Deployment conventions suitable for both dog-food and external open-source operators.
- A Flux-owned suspended catalog for source components that have not passed
  functional delivery gates.

## Initial Acceptance Gates

- Restarting Tilt produces the delivered image-build resources and no Kubernetes manifests.
- Flux creates the RERP namespace/profile/bootstrap and active services declaratively.
- The Flux role/database and object-store bootstrap completes before services
  reconcile; the manual Tilt migration gate completes before accepting those
  services as development-ready.
- Invoice and GL pods become Ready and survive restart.
- Every other Accounting HelmRelease is Flux-owned, suspended, labelled
  `scaffold-only`, and has no Deployment.
- East-west service discovery works without developer-specific hostnames.
- No plaintext production credential is committed or rendered.
- Metrics and logs identify tenant-safe request traces without leaking financial data.

The dev acceptance gate now proves that each active service publishes
BRRTRouter metric samples, emits structured JSON logs, and targets the shared
OpenTelemetry collector. The cluster does not yet install a Prometheus-compatible
scraper/rule evaluator, so alert evaluation, dashboards, and retention policy
remain a shared-platform dependency rather than an RERP-local monitoring stack.

## Questions To Thrash Out

- What is the production-grade replacement for the dev-only Git image update flow?
- What is the supported local development topology outside Microscaler's cluster?
- What are the minimum observability and backup requirements for the dog-food gate?
- What off-cluster target will hold PostgreSQL and Accounting-document backups
  before pilot data is accepted?

## Deferred Infrastructure Work

- [ ] **INFRA-DEFERRED-001 — Signed release images, SBOM/provenance, and public
  release pipeline.** RERP will consume the organisation's Octopilot platform
  rather than create a parallel release pipeline. This remains deferred until
  Octopilot supports multi-binary suite builds, per-service packaging, and a
  release manifest covering all selected suite binaries. The enabling work
  belongs in the Octopilot workspace at
  `/Users/casibbald/Workspace/remote/octopilot`; see
  <https://octopilot.app/docs/github-actions>.

The current Flux `dev-*` image path is deliberately a rapid-development path,
not the public release mechanism.

- [ ] **INFRA-PLATFORM-001 — Metrics collection and alert evaluation.** Add a
  shared-cluster metrics backend, RERP service discovery/scraping, tenant-safe
  dashboards, retention policy, and alerts for availability, error rate,
  latency, saturation, backup failure, and restore-drill freshness.
- [ ] **INFRA-PLATFORM-002 — Off-cluster recovery target.** Replicate encrypted
  PostgreSQL backups and versioned Accounting documents outside the shared
  cluster's MinIO failure domain before accepting pilot financial data.

## Dependencies

- Goal 1 for a reliable toolchain.
- Goal 2 for the active and suspended service inventories.
