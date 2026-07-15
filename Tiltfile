# RERP Accounting Development Environment
#
# RERP uses the shared-k8s cluster and registry. Platform services such as
# PostgreSQL, Redis, MinIO and observability are owned by shared-k8s-cluster;
# this Tiltfile only declares RERP-owned configuration and workloads.
#
# All services run on port 8080 (no NodePort).
#
# Pattern: each service follows the hauliage chain — lint → gen → build → copy → docker → deploy.
# The BFF is dynamically generated: spec-gen → lint → gen → build → deploy.
#

SHARED_K8S_KUBECONFIG = os.path.abspath('../shared-k8s-cluster/kubeconfig/shared-k8s.yaml')
SHARED_K8S_REGISTRY = '10.177.76.220:5000'
SERVICE_HTTP_PORT = '8080'
RUST_ENV_PREFIX = 'export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH" && '

# ====================
# Shared platform cluster
# ====================
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

# ====================
# BRRTRouter / Tooling Setup
# ====================
# BRRTRouter checkout for brrtrouter-gen lint fallback (cargo run).
# Override when layout differs: export BRRTROUTER_ROOT=/absolute/path/to/BRRTRouter
_brrtrouter_env = os.getenv('BRRTROUTER_ROOT', '').strip().rstrip('/')
brrtrouter_root = _brrtrouter_env if _brrtrouter_env else '../BRRTRouter'
os.putenv('BRRTROUTER_ROOT', str(local('realpath "%s"' % brrtrouter_root, quiet=True)).strip())

# Python venv for RERP/BRRTRouter tooling (shared). Override:
#   export BRRTROUTER_VENV=/path/to/venv
# Default: ~/.local/share/brrtrouter/venv
_brrtrouter_venv_env = os.getenv('BRRTROUTER_VENV', '').strip().rstrip('/')
_home = os.getenv('HOME', '') or os.getenv('USERPROFILE', '')
brrtrouter_venv = _brrtrouter_venv_env if _brrtrouter_venv_env else (_home + '/.local/share/brrtrouter/venv' if _home else '.local/share/brrtrouter/venv')

# Tooling ignore patterns (avoid build storms from bytecode, caches, packaging artifacts)
TOOLING_IGNORE = [
    '**/*.pyc',
    '**/*.pyo',
    '**/__pycache__',
    '**/.pytest_cache',
    '**/.coverage',
    '**/.coverage.*',
    '**/htmlcov',
    '**/coverage.xml',
    '**/.ruff_cache',
    '**/.mypy_cache',
    '**/*.egg',
    '**/*.egg-info',
    '**/*.egg-info/**',
    '**/*tooling.egg-info',
    '**/*tooling.egg-info/**',
    '**/.eggs',
    '**/dist',
    '**/build',
    '**/build/**',
    '**/.hypothesis',
    '**/.DS_Store',
]

# Build/activate the Python tooling venv (installs brrtrouter-tooling + rerp-tooling)
local_resource(
    'build-tooling',
    '''set -e
VENV="%s"
BRRROOT="%s"
test -d "$VENV" || python3 -m venv "$VENV"
"$VENV/bin/pip" install -U pip
"$VENV/bin/pip" install -e "$BRRROOT/tooling[dev]"
cd tooling && "$VENV/bin/pip" install -e '.[dev]'
''' % (brrtrouter_venv, brrtrouter_root),
    deps=[
        './tooling/pyproject.toml',
        '%s/tooling/pyproject.toml' % brrtrouter_root,
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)

# ====================
# RERP CLI helper (same venv as hauliage_bin)
# ====================
rerp_bin = '%s/bin/rerp' % brrtrouter_venv

# ====================
# Base Docker Image
# ====================
local_resource(
    'build-base-image',
    '%s docker build-base' % rerp_bin,
    deps=[
        'docker/base/Dockerfile',
        'docker/base/dev-entrypoint.sh',
    ],
    resource_deps=['build-tooling'],
    labels=['docker'],
    allow_parallel=True,
)

# ====================
# Kubernetes Namespace
# ====================
k8s_yaml('k8s/rerp/namespace.yaml')
k8s_resource(
    new_name='rerp-namespace',
    objects=['rerp:namespace'],
    labels=['data'],
)

# ====================
# Database Infrastructure
# ====================
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

# Database bootstrap is deliberately visible and manual.
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

# Provision a private MinIO bucket and a least-privilege RERP application user.
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

# ====================
# Suite-aware runtime discovery
# ====================
# Runtime descriptors are derived from checked-in contracts, Cargo manifests,
# and Helm values. This is the single service inventory used by both the CLI and
# Tilt; suites are not flattened and package names are not guessed.
RUNTIME_SERVICES = decode_json(str(local(
    'PYTHONPATH=tooling/src python3 -m rerp_tooling.runtime descriptors --root .',
    quiet=True,
)))
REGULAR_SERVICES = [service for service in RUNTIME_SERVICES if service['service'] != 'bff']
BFF_SERVICES = [service for service in RUNTIME_SERVICES if service['service'] == 'bff']

# ====================
# Code Generation & Lint Helpers
# ====================

def create_microservice_lint(service):
    """Lint the OpenAPI spec for a microservice using brrtrouter-gen."""
    name = service['resource_name']
    spec = service['spec_path']
    local_resource(
        '%s-lint' % name,
        cmd='''
            set -e
            echo "🔍 Linting %s OpenAPI spec..."
            # Use the built debug binary directly for speed
            %s/target/debug/brrtrouter-gen lint \
                --spec %s \
                --fail-on-error || \
            cargo run --manifest-path %s/Cargo.toml --bin brrtrouter-gen -- \
                lint \
                --spec %s \
                --fail-on-error

            echo "✅ %s OpenAPI spec linting passed"
        ''' % (name, brrtrouter_root, spec, brrtrouter_root, spec, name),
        deps=[spec],
        labels=[name],
        allow_parallel=True,
    )


def create_microservice_gen(service):
    """Generate gen/ and impl/ crates for a microservice from its OpenAPI spec."""
    name = service['resource_name']
    suite = service['suite']
    service_name = service['service']
    # Ignore generated files to avoid build storms
    ignore_list = [
        '%s/src' % service['gen_dir'],
        service['doc_dir'],
        service['static_dir'],
        '%s/config' % service['impl_dir'],
    ]
    local_resource(
        '%s-service-gen' % name,
        cmd='%s gen suite %s --service %s' % (rerp_bin, suite, service_name),
        deps=[
            service['spec_path'],
            'tooling/pyproject.toml',
        ],
        ignore=ignore_list,
        resource_deps=['build-tooling', '%s-lint' % name],
        labels=[name],
        allow_parallel=True,
    )


# Stubs resource: manual trigger only (never auto-init — --force clobbers user controllers)
def create_microservice_stubs_resource(service):
    name = service['resource_name']
    local_resource(
        'stubs-%s' % name,
        '%s gen stubs %s %s' % (rerp_bin, service['suite'], service['service']),
        deps=[
            service['spec_path'],
            service['gen_dir'],
            'tooling/pyproject.toml',
        ],
        labels=[name],
        resource_deps=['build-tooling'],
        allow_parallel=True,
        trigger_mode=TRIGGER_MODE_MANUAL,
        auto_init=False,
    )


def create_microservice_build_resource(service, resource_deps):
    """Build the service binary using `rerp build microservice`."""
    name = service['resource_name']
    local_resource(
        'build-%s' % name,
        cmd='%s build microservice %s --suite %s' % (rerp_bin, service['service'], service['suite']),
        deps=[
            '%s/Cargo.toml' % service['gen_dir'],
            '%s/Cargo.toml' % service['impl_dir'],
            '%s/src' % service['gen_dir'],
            '%s/src' % service['impl_dir'],
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/target',
            './build_artifacts',
        ],
        resource_deps=resource_deps,
        labels=[name],
        allow_parallel=True,
    )


def create_microservice_test_resource(service):
    """Manual test resource for a microservice."""
    name = service['resource_name']
    package_name = service['package_name']
    db_pass = os.environ.get('RERP_DB_PASSWORD', 'dev_password_change_in_prod')
    local_resource(
        'test-%s' % name,
        'cd microservices && cargo test -p %s' % package_name,
        env={'RERP_DB_PASSWORD': db_pass},
        deps=[
            '%s/src' % service['impl_dir'],
            '%s/tests' % service['impl_dir'],
            '%s/src' % service['gen_dir'],
            './microservices/Cargo.toml',
            'tooling/pyproject.toml',
        ],
        labels=[name],
        trigger_mode=TRIGGER_MODE_MANUAL,
        allow_parallel=True,
    )


# ====================
# Architecture Detection
# ====================
host_machine = str(local('uname -m', quiet=True)).strip()
if host_machine in ['arm64', 'aarch64']:
    TARGET_ARCH_NAME = 'arm64'
    TARGET_RUST_TRIPLE = 'aarch64-unknown-linux-musl'
else:
    TARGET_ARCH_NAME = 'amd64'
    TARGET_RUST_TRIPLE = 'x86_64-unknown-linux-musl'

# ====================
# Deployment Chain for Each Regular Service
# ====================
def create_microservice_deployment(service):
    """Full chain: copy binary → one custom image build → Helm deploy."""
    name = service['resource_name']
    binary_name = service['binary_name']
    target_path = 'microservices/target/%s/debug/%s' % (TARGET_RUST_TRIPLE, binary_name)
    artifact_path = 'build_artifacts/%s/%s/%s' % (TARGET_ARCH_NAME, service['suite'], binary_name)
    hash_path = artifact_path + '.sha256'
    dockerfile = 'docker/microservices/Dockerfile'
    image_name = 'localhost:5001/%s' % service['image_name']

    # 1. Copy binary from workspace build to artifacts (also creates SHA256 hash)
    local_resource(
        'copy-%s' % name,
        '%s docker copy-binary %s %s %s' % (rerp_bin, target_path, artifact_path, binary_name),
        deps=[target_path, 'tooling/pyproject.toml'],
        resource_deps=['build-%s' % name],
        labels=[name],
        allow_parallel=True,
    )

    # 2. Tilt owns the only service-image build. The RERP builder stages a
    # narrow context containing only this service's binary and runtime assets.
    _push_cmd = 'docker tag %s:tilt $EXPECTED_REF && docker push $EXPECTED_REF' % image_name
    custom_build(
        image_name,
        ('%s docker build-image-simple %s %s %s %s --suite %s --service %s' % (rerp_bin, image_name, dockerfile, hash_path, artifact_path, service['suite'], service['service'])
         + ' && ' + _push_cmd),
        deps=[artifact_path, hash_path, dockerfile, service['config_dir'], service['doc_dir'], service['static_dir'], 'tooling/pyproject.toml'],
        tag='tilt',
        live_update=[
            sync(artifact_path, '/app/service'),
            sync(service['doc_dir'] + '/', '/app/doc/'),
            sync(service['static_dir'] + '/', '/app/static_site/'),
            run('kill -HUP 1', trigger=[artifact_path]),
        ],
    )

    # 3. Deploy using Helm. Configuration is immutable image content; changing
    # it rebuilds the image instead of attempting to write a ConfigMap mount.
    _helm_values = [
        service['helm_values'],
        './helm/rerp-microservice/values/_database-shared-k8s.yaml',
        './helm/rerp-microservice/values/_redis-shared-k8s.yaml',
        './helm/rerp-microservice/values/_sesame-idam-shared-k8s.yaml',
    ]
    k8s_yaml(helm(
        './helm/rerp-microservice',
        name=name,
        namespace='rerp',
        values=_helm_values,
    ))

    # 5. Kubernetes resource configuration
    # Only BFF is reachable from the host (8080). Other microservices are ClusterIP.
    _port_forwards = ['8080:%s' % SERVICE_HTTP_PORT] if name == 'bff' else []
    k8s_resource(
        name,
        port_forwards=_port_forwards,
        resource_deps=['rerp-database-env', 'build-base-image', 'copy-%s' % name],
        labels=[name],
        auto_init=True,
        trigger_mode=TRIGGER_MODE_AUTO,
    )


# ====================
# Regular Services: lint → gen → build → deploy chain
# ====================
for service in REGULAR_SERVICES:
    create_microservice_lint(service)
    create_microservice_gen(service)
    create_microservice_stubs_resource(service)


# All gens must complete before any build (so workspace members exist)
local_resource(
    'rerp-all-gens',
    'echo "✅ All RERP accounting codegen complete"',
    resource_deps=['%s-service-gen' % service['resource_name'] for service in REGULAR_SERVICES],
    labels=['all_gens'],
    allow_parallel=False,
)

for service in REGULAR_SERVICES:
    create_microservice_build_resource(service, ['rerp-all-gens'])
    create_microservice_deployment(service)


# ====================
# Backend for Frontend (BFF) — Dynamically Generated
# ====================
# The BFF spec aggregates all accounting microservice paths and is automatically
# regenerated whenever any sub-service OpenAPI spec changes.
# Chain: bff-spec-gen → bff-lint → bff-gen → bff-build → bff-deploy

# 1. BFF spec generation: reads all bff-suite-config.yaml + sub-service specs → openapi_bff.yaml
#    Uses rerp bff generate-system which reads openapi/accounting/bff-suite-config.yaml
#    and merges every sub-service spec into openapi/accounting/openapi_bff.yaml
local_resource(
    'bff-spec-gen',
    cmd='''
        set -e
        echo "🔄 Regenerating RERP Accounting BFF OpenAPI spec..."
        %s bff generate-system --suite accounting
        echo "✅ RERP Accounting BFF spec regeneration complete"
    ''' % (rerp_bin,),
    deps=[
        './openapi/accounting/bff-suite-config.yaml',
    ] + [service['spec_path'] for service in REGULAR_SERVICES if service['suite'] == 'accounting'],
    ignore=[
        './openapi/accounting/openapi_bff.yaml',  # Don't watch the generated file
    ],
    resource_deps=['build-tooling'],
    labels=['bff'],
    allow_parallel=True,
)

# 2. BFF lint
local_resource(
    'bff-lint',
    cmd='''
        set -e
        echo "🔍 Linting BFF OpenAPI spec..."
        %s/target/debug/brrtrouter-gen lint \
            --spec ./openapi/accounting/openapi_bff.yaml \
            --fail-on-error || \
        cargo run --manifest-path %s/Cargo.toml --bin brrtrouter-gen -- \
            lint \
            --spec ./openapi/accounting/openapi_bff.yaml \
            --fail-on-error
        echo "✅ BFF OpenAPI spec linting passed"
    ''' % (brrtrouter_root, brrtrouter_root),
    deps=['./openapi/accounting/openapi_bff.yaml'],
    resource_deps=['bff-spec-gen'],
    labels=['bff'],
    allow_parallel=True,
)

# 3. BFF gen → 4. build → 5. deploy (uses the same descriptor-driven chain)
for service in BFF_SERVICES:
    create_microservice_gen(service)
    create_microservice_stubs_resource(service)
    create_microservice_build_resource(service, ['%s-service-gen' % service['resource_name']])
    create_microservice_deployment(service)


# ====================
# Frontend (nginx) is managed externally (not part of this Tiltfile)
# To deploy the frontend, manage it in its own workspace or add the k8s manifests here.
