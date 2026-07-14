# Goal 3: Shared-Cluster Delivery

## Objective

Make RERP repeatably deployable through Tilt and systemd to the Microscaler shared Kubernetes cluster, using the same artifacts and health expectations needed by a self-hosted deployment.

## Broad Outcomes

- A valid, minimal Tilt graph for active RERP services.
- A stable systemd unit and documented developer lifecycle.
- RERP namespace, database bootstrap, migrations, secrets, and service discovery.
- Reusable Helm values without stale shared-Kind assumptions.
- Health, readiness, metrics, logs, traces, and resource limits.
- Deployment conventions suitable for both dog-food and external open-source operators.

## Initial Acceptance Gates

- Restarting tilt-rerp.service produces a healthy Tilt session.
- The RERP namespace and active service resources are created declaratively.
- Migrations complete before services become Ready.
- Invoice, GL, and gateway pods become Ready and survive restart.
- East-west service discovery works without developer-specific hostnames.
- No plaintext production credential is committed or rendered.
- Metrics and logs identify tenant-safe request traces without leaking financial data.

## Questions To Thrash Out

- Which resources belong to shared-k8s and which belong to RERP?
- Should database creation be cluster bootstrap, RERP migration, or an operator responsibility?
- What is the supported local development topology outside Microscaler's cluster?
- What are the minimum observability and backup requirements for the dog-food gate?
- How will release images be built and signed for public consumption?

## Dependencies

- Goal 1 for a reliable toolchain.
- Goal 2 for the active service inventory.
