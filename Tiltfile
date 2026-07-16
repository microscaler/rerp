# RERP Accounting image development.
#
# Flux owns namespaces, SOPS profiles, bootstrap Jobs, Helm releases and runtime
# reconciliation. Tilt owns only local code generation/builds and publishing
# monotonically tagged development images to the shared registry.
#

SHARED_K8S_REGISTRY = '10.177.76.220:5000'
SHARED_K8S_KUBECONFIG = os.path.abspath('../shared-k8s-cluster/kubeconfig/shared-k8s.yaml')
RUST_ENV_PREFIX = 'export PATH="$HOME/.cargo/bin:/usr/local/bin:$PATH" && '

# Tilt still evaluates the current kube context before allowing any local()
# command, even when the file declares no Kubernetes resources.
allow_k8s_contexts(['shared-k8s'])
default_registry(SHARED_K8S_REGISTRY)

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
# Suite-aware runtime discovery
# ====================
# Runtime descriptors are derived from checked-in contracts, Cargo manifests,
# and Helm values. This is the single service inventory used by both the CLI and
# Tilt; suites are not flattened and package names are not guessed.
RUNTIME_SERVICES = decode_json(str(local(
    'PYTHONPATH=tooling/src python3 -m rerp_tooling.runtime descriptors --root .',
    quiet=True,
)))
DELIVERED_SERVICE_NAMES = ['general-ledger', 'invoice']
DELIVERED_SERVICES = [
    service for service in RUNTIME_SERVICES
    if service['suite'] == 'accounting' and service['service'] in DELIVERED_SERVICE_NAMES
]
if sorted([service['service'] for service in DELIVERED_SERVICES]) != sorted(DELIVERED_SERVICE_NAMES):
    fail('Accounting delivered-service descriptors are incomplete')

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
    db_pass = os.environ.get('RERP_DB_PASSWORD', '')
    test_env = {'RERP_DB_PASSWORD': db_pass} if db_pass else {}
    local_resource(
        'test-%s' % name,
        'cd microservices && cargo test -p %s' % package_name,
        env=test_env,
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
# Image Chain for Each Delivered Service
# ====================
def create_microservice_image(service):
    """Copy one binary and publish a monotonically tagged dev image."""
    name = service['resource_name']
    binary_name = service['binary_name']
    target_path = 'microservices/target/%s/debug/%s' % (TARGET_RUST_TRIPLE, binary_name)
    artifact_path = 'build_artifacts/%s/%s/%s' % (TARGET_ARCH_NAME, service['suite'], binary_name)
    hash_path = artifact_path + '.sha256'
    dockerfile = 'docker/microservices/Dockerfile'
    image_name = service['image_name']

    # 1. Copy binary from workspace build to artifacts (also creates SHA256 hash)
    local_resource(
        'copy-%s' % name,
        '%s docker copy-binary %s %s %s' % (rerp_bin, target_path, artifact_path, binary_name),
        deps=[target_path, 'tooling/pyproject.toml'],
        resource_deps=['build-%s' % name],
        labels=[name],
        allow_parallel=True,
    )

    # 2. A local resource is deliberate: Tilt discards custom_build targets
    # which are not referenced by a Kubernetes manifest. Flux, not Tilt,
    # consumes these pushed images.
    registry_image = '%s/%s' % (SHARED_K8S_REGISTRY, image_name)
    _build_and_push = '''set -eu
%s docker build-image-simple %s %s %s %s --suite %s --service %s
DEV_REF="%s:dev-$(date +%%s%%N)"
docker tag %s:tilt "$DEV_REF"
docker push "$DEV_REF"
echo "Published $DEV_REF for Flux image discovery"
''' % (
        rerp_bin,
        image_name,
        dockerfile,
        hash_path,
        artifact_path,
        service['suite'],
        service['service'],
        registry_image,
        image_name,
    )
    local_resource(
        'image-%s' % name,
        _build_and_push,
        deps=[artifact_path, hash_path, dockerfile, service['config_dir'], service['doc_dir'], service['static_dir'], 'tooling/pyproject.toml'],
        resource_deps=['build-base-image', 'copy-%s' % name],
        labels=[name, 'images'],
        allow_parallel=True,
    )

for service in DELIVERED_SERVICES:
    create_microservice_lint(service)
    create_microservice_gen(service)
    create_microservice_stubs_resource(service)


# All gens must complete before any build (so workspace members exist)
local_resource(
    'rerp-all-gens',
    'echo "✅ Delivered RERP Accounting codegen complete"',
    resource_deps=['%s-service-gen' % service['resource_name'] for service in DELIVERED_SERVICES],
    labels=['all_gens'],
    allow_parallel=False,
)

for service in DELIVERED_SERVICES:
    create_microservice_build_resource(service, ['rerp-all-gens'])
    create_microservice_image(service)

# Database migration image. Flux runs this as a gated Job before reconciling
# the service Helm releases; Tilt only publishes the content-addressed image.
DB_INIT_IMAGE = 'rerp-accounting-db-init'
DB_INIT_DOCKERFILE = 'docker/jobs/Dockerfile'
DB_INIT_REF = '%s/%s' % (SHARED_K8S_REGISTRY, DB_INIT_IMAGE)
DB_INIT_BUILD = '''set -eu
docker build --build-arg RERP_SUITE=accounting -f %s -t %s:tilt .
DEV_REF="%s:dev-$(date +%%s%%N)"
docker tag %s:tilt "$DEV_REF"
docker push "$DEV_REF"
echo "Published $DEV_REF for Flux image discovery"
''' % (DB_INIT_DOCKERFILE, DB_INIT_IMAGE, DB_INIT_REF, DB_INIT_IMAGE)
local_resource(
    'image-%s' % DB_INIT_IMAGE,
    DB_INIT_BUILD,
    deps=[
        DB_INIT_DOCKERFILE,
        'microservices/accounting/scripts/db-init-job.sh',
        'microservices/accounting/migrations',
        'microservices/accounting/sql',
    ],
    labels=['database', 'images'],
    allow_parallel=True,
)

# Passive post-deploy acceptance. This extracts the useful watch/rollout cycle
# from the Skaffold-era script without giving Tilt deployment ownership.
local_resource(
    'accept-accounting-deployment',
    ('python3 microservices/accounting/scripts/validate-deployment.py '
     + '--kubeconfig "%s" --timeout 600' % SHARED_K8S_KUBECONFIG),
    deps=[
        'microservices/accounting/scripts/validate-deployment.py',
        'deployment-configuration/profiles/dev/rerp/accounting',
    ],
    labels=['acceptance', 'deploy'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=False,
)
