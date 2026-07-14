# RERP accounting development environment
#
# RERP uses the shared-k8s cluster and registry. Platform services such as
# PostgreSQL, Redis, MinIO and observability are owned by shared-k8s-cluster;
# this Tiltfile only declares RERP-owned configuration and workloads.

SHARED_K8S_KUBECONFIG = os.path.abspath('../shared-k8s-cluster/kubeconfig/shared-k8s.yaml')
SHARED_K8S_REGISTRY = '10.177.76.220:5000'
DEV_IMAGE = 'localhost:5001/rerp-accounting-invoice'
SERVICE_HTTP_PORT = '8080'
RUST_ENV_PREFIX = 'export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH" && '

if not os.path.exists(SHARED_K8S_KUBECONFIG):
    fail('shared-k8s kubeconfig not found: %s' % SHARED_K8S_KUBECONFIG)

allow_k8s_contexts(['shared-k8s'])
os.putenv('KUBECONFIG', SHARED_K8S_KUBECONFIG)
default_registry(SHARED_K8S_REGISTRY)
update_settings(k8s_upsert_timeout_secs=60)

docker_prune_settings(
    disable=False,
    max_age_mins=30,
    keep_recent=1,
    interval_hrs=1,
)

# The shared cluster owns platform namespaces; RERP owns its application
# namespace, so a plain `tilt up` can bootstrap the application boundary.
k8s_yaml('k8s/rerp/namespace.yaml')
k8s_resource(
    new_name='rerp-namespace',
    objects=['rerp:namespace'],
    labels=['data'],
)

# Shared database connection values and credentials used by invoice.
k8s_yaml('k8s/rerp/rerp-database-env.yaml')
k8s_resource(
    new_name='rerp-database-env',
    objects=[
        'rerp-database-config:configmap:rerp',
        'rerp-db-credentials:secret:rerp',
    ],
    resource_deps=['rerp-namespace'],
    labels=['data'],
)

# Database bootstrap is deliberately visible and manual: the current SQL
# migrations predate a migration ledger and are not all safe to replay. Run it
# once for a new database; use RERP_APPLY_MIGRATIONS_ONLY=1 for later additions.
local_resource(
    'rerp-db-init',
    './microservices/accounting/scripts/setup-db.sh',
    deps=[
        './microservices/accounting/scripts/setup-db.sh',
        './microservices/accounting/migrations',
        './microservices/accounting/sql',
    ],
    labels=['data'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=False,
)

# Provision a private MinIO bucket and a least-privilege RERP application user,
# then materialize the namespace-local Secret consumed by the Helm chart.
local_resource(
    'rerp-object-store',
    './microservices/accounting/scripts/setup-object-store.sh',
    deps=['./microservices/accounting/scripts/setup-object-store.sh'],
    resource_deps=['rerp-namespace'],
    labels=['data'],
    trigger_mode=TRIGGER_MODE_AUTO,
    auto_init=True,
    allow_parallel=False,
)

# Contract regeneration is a deliberate operation because generated source is
# checked in. It remains available in the Tilt UI without rewriting source on
# every startup.
local_resource(
    'invoice-contract-refresh',
    '''set -eu
export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH"
../BRRTRouter/target/debug/brrtrouter-gen lint \\
  --spec ./microservices/accounting/invoice/openapi/phase1.yaml \\
  --fail-on-error
tooling/.venv/bin/rerp gen suite accounting --service invoice
''',
    deps=[
        './microservices/accounting/invoice/openapi/phase1.yaml',
        './tooling/pyproject.toml',
    ],
    labels=['invoice'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=False,
)

local_resource(
    'invoice-tests',
    RUST_ENV_PREFIX + 'cd microservices && cargo test -p rerp_accounting_invoice',
    deps=[
        './microservices/accounting/core/src',
        './microservices/accounting/entities/src',
        './microservices/accounting/invoice/gen/src',
        './microservices/accounting/invoice/impl/src',
        './microservices/accounting/invoice/impl/Cargo.toml',
    ],
    ignore=['./microservices/target'],
    labels=['invoice', 'tests'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)

# The image build is intentionally self-contained: compile the static musl
# binary, copy it into the Docker build context, build the runtime image and
# push Tilt's content-addressed reference to the shared registry.
custom_build(
    DEV_IMAGE,
    RUST_ENV_PREFIX + '''set -eu
cargo build \\
  --manifest-path microservices/Cargo.toml \\
  --package rerp_accounting_invoice \\
  --target x86_64-unknown-linux-musl
install -d build_artifacts/amd64
install -m 0755 \\
  microservices/target/x86_64-unknown-linux-musl/debug/rerp_accounting_invoice \\
  build_artifacts/amd64/invoice
sha256sum build_artifacts/amd64/invoice > build_artifacts/amd64/invoice.sha256
docker build \\
  --file docker/microservices/Dockerfile.invoice \\
  --build-arg TARGETARCH=amd64 \\
  --tag "$EXPECTED_REF" \\
  .
docker push "$EXPECTED_REF"
''',
    deps=[
        './microservices/Cargo.toml',
        './microservices/Cargo.lock',
        './microservices/accounting/core',
        './microservices/accounting/entities',
        './microservices/accounting/invoice/gen',
        './microservices/accounting/invoice/impl',
        './docker/microservices/Dockerfile.invoice',
    ],
    tag='tilt',
)

# Deploy only the accounting components that currently produce a real runtime.
# Placeholder directories are intentionally not represented as fake workloads.
k8s_yaml(helm(
    './helm/rerp-microservice',
    name='invoice',
    namespace='rerp',
    values=[
        './helm/rerp-microservice/values/invoice.yaml',
        './helm/rerp-microservice/values/_database-shared-k8s.yaml',
        './helm/rerp-microservice/values/_redis-shared-k8s.yaml',
        './helm/rerp-microservice/values/_sesame-idam-shared-k8s.yaml',
    ],
))

k8s_resource(
    workload='invoice',
    objects=['invoice-config:configmap:rerp'],
    port_forwards=['8080:%s' % SERVICE_HTTP_PORT],
    resource_deps=['rerp-database-env', 'rerp-object-store'],
    labels=['invoice'],
    trigger_mode=TRIGGER_MODE_AUTO,
    auto_init=True,
)
