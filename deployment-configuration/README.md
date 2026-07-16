# RERP deployment profiles

RERP owns its product and suite configuration under:

```text
deployment-configuration/profiles/<environment>/rerp/<suite>/
```

The Accounting dev profile is
`deployment-configuration/profiles/dev/rerp/accounting/`. It has three explicit
reconciliation boundaries:

```text
accounting/
├── runtime/       # namespace-local ConfigMap and application Secrets
├── bootstrap/     # rerunnable database and object-store Jobs in data
├── services/      # delivered Accounting Helm releases only
└── catalog/       # Flux-owned, suspended scaffold Helm releases
```

Non-secret settings live in `runtime/application.properties`; secrets are
SOPS-encrypted dotenv files and are split by least-privilege Kubernetes
Secret. Bootstrap has separately encrypted copies of the product credentials
because Kubernetes forbids cross-namespace Secret references. Platform
administrator credentials are never copied into the RERP namespace.

Platform configuration remains in `shared-gitops-k8s-cluster`. In particular,
PostgreSQL HA's `postgres-ha` profile owns Pgpool's custom-user credential
source. Its RERP password must be rotated together with Accounting's
`rerp-db-credentials` password.

## Dev reconciliation

The shared cluster's `product-components` GitOpsSet reads its component
inventory and creates one Flux `GitRepository` for `microscaler/rerp`. The
`rerp-accounting` foundation Kustomization decrypts runtime credentials and
waits for both bootstrap Jobs to complete. Only then may the dependent
`rerp-accounting-services` Kustomization reconcile General Ledger and Invoice
Helm releases. `force: true` is limited to foundation so changed immutable Job
templates are safely recreated in dev. The database Job owns only the Pgpool
credential contract, application role, database, schema, default privileges,
and login verification; it never applies application migrations or seeds.

The independent `rerp-accounting-catalog` Kustomization applies and prunes the
other fifteen Accounting HelmRelease declarations with `spec.suspend: true`.
It does not wait on suspended release health or gate active services. This
moves lifecycle ownership to Flux without installing placeholder APIs.

Tilt builds and publishes only the three delivered images: database bootstrap,
General Ledger, and Invoice. During rapid development, its manual
`accounting-apply-migrations` resource applies the ordered Accounting
migrations, Sesame RLS contract, seeds, and post-migration grants. Its manual
`accept-accounting-deployment` resource
passively checks Flux readiness, bootstrap completion, selected/deployed image
identity, rollout availability, service health, and that all catalog releases
remain suspended with no Deployment. It also verifies BRRTRouter metric samples,
structured JSON logging, and the cluster OpenTelemetry collector target; it
never deploys or forces reconciliation. Flux owns configuration, bootstrap,
Helm releases, rollout and drift correction. ImageRepository/ImagePolicy
objects discover the monotonic `dev-<nanoseconds>` tags. Automated Git writes remain deliberately
disabled until a scoped RERP write deploy key is provisioned; see the shared
cluster's `docs/product-image-automation.md`.

Encrypt and inspect secrets only on ms02:

```bash
export SOPS_AGE_KEY_FILE=~/.config/sops/age/flux-shared-gitops
sops --encrypt --in-place --input-type dotenv --output-type dotenv \
  deployment-configuration/profiles/dev/rerp/accounting/runtime/application.secrets.env
sops --decrypt \
  deployment-configuration/profiles/dev/rerp/accounting/runtime/application.secrets.env
```

Never commit plaintext credentials, decrypted render directories, age private
keys, or kubeconfigs.
