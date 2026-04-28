# RERP Development Environment
#
# Tilt's UI port is fixed before this file runs. This repo uses 10350 so the dashboard
# is reachable on the LAN (not only localhost).
#
# Plain `tilt up`: use direnv (see repo `.envrc`: `direnv allow` sets TILT_PORT=10350 + TILT_HOST=0.0.0.0), or run:
#   tilt up --port 10350 --host 0.0.0.0
#   TILT_PORT=10350 TILT_HOST=0.0.0.0 tilt up --host 0.0.0.0
# Full stack: `just dev-up` / `just dev-down` (same port).
#
# Infra: RERP connects to the shared Kind cluster (kind-kind) managed by
# microscaler/shared-kind-cluster. **Do not** run `just dev-up` from that repo
# in the same session — shared-kind-cluster uses port 10348; RERP uses 10350.
# The shared cluster provides: postgres, redis, minio, observability, faktory, etc.
# RERP deploys to namespace `rerp` and reuses all shared infrastructure via
# ClusterIP DNS (e.g. `postgres.data.svc.cluster.local:5432`).
#
# -----------------------------------------------------------------------------
# AGENT / AUTOMATION — where platform infra is defined (do not duplicate here)
# -----------------------------------------------------------------------------
# Namespace `rerp` must exist before this Tilt applies manifests.
# **`just dev-up`** from this repo creates it via `k8s/rerp/namespace.yaml`.
# Platform infra (postgres, redis, observability) comes from the sibling repo
# `microscaler/shared-kind-cluster`. Do NOT apply that Tiltfile in the same
# session — shared-kind-cluster uses its own cluster context `kind-kind` (same
# Kind cluster, different Tilt port 10348). Open that repo's Tilt UI to manage
# shared workloads.
#
# Workloads registered in shared-kind-cluster under label `data` include at least:
# postgres (primary + 2 replicas), redis, minio, mailpit, pact-*, inbucket,
# imgproxy, faktory-server, fluvio-sc, prometheus, grafana, loki, jaeger, otel-collector.
# -----------------------------------------------------------------------------

# ====================
# Configuration
# ====================

# Shared default cluster: context kind-kind (see microscaler/shared-kind-cluster).
# RERP deploys into namespace `rerp` and connects to shared services via DNS.
allow_k8s_contexts(['kind-kind'])

update_settings(k8s_upsert_timeout_secs=60)

# Configure automatic Docker pruning to prevent disk space exhaustion
docker_prune_settings(
    disable=False,
    max_age_mins=30,
    keep_recent=1,
    interval_hrs=1
)

# ====================
# Environment Variables
# ====================

# RERP suite name (currently only "accounting" is implemented)
RERP_SUITE = 'accounting'
RERP_SUITE_NAME = 'rerp-' + RERP_SUITE
RERP_SUITE_CONFIG = 'openapi/%s/bff-suite-config.yaml' % RERP_SUITE

def _unquote(value):
    return value.strip().strip('"').strip("'")

def _parse_bff_suite_config(path):
    content = str(read_file(path))
    services = []
    service_ports = {}
    service_spec_paths = {}
    bff_service_name = 'bff'
    in_services = False
    current = None

    for raw in content.split('\n'):
        stripped = raw.strip()
        if not stripped or stripped.startswith('#'):
            continue

        if stripped.startswith('bff_service_name:'):
            bff_service_name = _unquote(stripped.split(':', 1)[1])
            continue

        if stripped == 'services:':
            in_services = True
            current = None
            continue

        if in_services and not raw.startswith('  '):
            in_services = False
            current = None

        if not in_services:
            continue

        if raw.startswith('  ') and not raw.startswith('    ') and stripped.endswith(':'):
            current = stripped[:-1]
            services.append(current)
            continue

        if current and raw.startswith('    ') and stripped.startswith('port:'):
            service_ports[current] = _unquote(stripped.split(':', 1)[1])
            continue

        if current and raw.startswith('    ') and stripped.startswith('spec_path:'):
            service_spec_paths[current] = _unquote(stripped.split(':', 1)[1])
            continue

    return {
        'bff_service_name': bff_service_name,
        'services': services,
        'ports': service_ports,
        'spec_paths': service_spec_paths,
    }

def _parse_port_registry(path):
    content = str(read_file(path))
    assignments = {}
    in_assignments = False

    for raw in content.split('\n'):
        stripped = raw.strip()
        if stripped.startswith('"assignments"'):
            in_assignments = True
            continue
        if in_assignments and stripped.startswith('}'):
            in_assignments = False
            continue
        if not in_assignments or ':' not in stripped:
            continue

        key = _unquote(stripped.split(':', 1)[0])
        value = stripped.split(':', 1)[1].strip().rstrip(',')
        if value:
            assignments[key] = _unquote(value)

    return assignments

def _file_exists(path):
    return str(local('test -e "%s" && echo yes || true' % path, quiet=True)).strip() == 'yes'

def _runtime_ready_service(name):
    spec_path = SERVICE_SPEC_PATHS.get(name, '%s/openapi.yaml' % name)
    required = [
        'openapi/%s/%s' % (RERP_SUITE, spec_path),
        'microservices/%s/%s/gen/Cargo.toml' % (RERP_SUITE, name),
        'microservices/%s/%s/impl/Cargo.toml' % (RERP_SUITE, name),
        'helm/rerp-microservice/values/%s.yaml' % name,
    ]
    for path in required:
        if not _file_exists(path):
            return False
    return True

def _read_cargo_package_name(name):
    manifest = 'microservices/%s/%s/impl/Cargo.toml' % (RERP_SUITE, name)
    fallback = 'rerp_%s_%s' % (RERP_SUITE, name.replace('-', '_'))
    if not _file_exists(manifest):
        return fallback

    content = str(read_file(manifest))
    in_package = False
    for raw in content.split('\n'):
        stripped = raw.strip()
        if stripped == '[package]':
            in_package = True
            continue
        if stripped.startswith('[') and stripped != '[package]':
            in_package = False
            continue
        if in_package and stripped.startswith('name = '):
            return _unquote(stripped.split('=', 1)[1])
    return fallback

SUITE_CONFIG = _parse_bff_suite_config(RERP_SUITE_CONFIG)
PORT_REGISTRY = _parse_port_registry('port-registry.json')
DISCOVERED_ACCOUNTING_SERVICES = SUITE_CONFIG['services']
SERVICE_PORTS = SUITE_CONFIG['ports']
SERVICE_SPEC_PATHS = SUITE_CONFIG['spec_paths']
BFF_SERVICE_NAME = SUITE_CONFIG['bff_service_name']

ACCOUNTING_SERVICES = []
CONTRACT_ONLY_ACCOUNTING_SERVICES = []
for _service in DISCOVERED_ACCOUNTING_SERVICES:
    if _runtime_ready_service(_service):
        ACCOUNTING_SERVICES.append(_service)
    else:
        CONTRACT_ONLY_ACCOUNTING_SERVICES.append(_service)

print('RERP Tilt discovered %d accounting service contracts from %s' % (len(DISCOVERED_ACCOUNTING_SERVICES), RERP_SUITE_CONFIG))
print('RERP Tilt runtime-ready accounting services: %s' % ', '.join(ACCOUNTING_SERVICES))
if CONTRACT_ONLY_ACCOUNTING_SERVICES:
    print('RERP Tilt contract-only accounting services skipped until scaffolded: %s' % ', '.join(CONTRACT_ONLY_ACCOUNTING_SERVICES))

# ====================
# Base Docker Image
# ====================
# The base image is managed independently and explicitly tagged as `rerp-base:latest`.
# Doing this natively via local_resource instead of docker_build prevents Tilt from
# appending dynamic hashes, ensuring dev-sync can pull the predictable `:latest` tag.
local_resource(
    'build-base-image',
    'tooling/.venv/bin/rerp docker build-base',
    deps=[
        'docker/base/Dockerfile',
        'docker/base/dev-entrypoint.sh',
    ],
    labels=['docker'],
    allow_parallel=True,
)

# ====================
# BRRTRouter & Tooling Configuration
# ====================

# BRRTRouter checkout for brrtrouter-gen lint fallback (cargo run). Override when layouts differ:
#   export BRRTROUTER_ROOT=/absolute/path/to/BRRTRouter
# Default ../BRRTRouter = microscaler/BRRTRouter when this repo is microscaler/rerp (siblings).
_brrtrouter_env = os.getenv('BRRTROUTER_ROOT', '').strip().rstrip('/')
brrtrouter_root = _brrtrouter_env if _brrtrouter_env else '../BRRTRouter'
# Absolute path for subprocesses (rerp gen, cargo); matches tooling discover_brrtrouter_root.
os.putenv('BRRTROUTER_ROOT', str(local('realpath "%s"' % brrtrouter_root, quiet=True)).strip())

# For Tilt `deps` / `ignore` only: keep a path relative to this repo.
# If your checkout is not ../BRRTRouter, trigger build-tooling manually after editing BRRTRouter tooling.
brrtrouter_watch_root = '../BRRTRouter'

# Shared Python env for brrtrouter + rerp CLIs (one venv on disk). Override:
#   export BRRTROUTER_VENV=/path/to/venv
# Default: ~/.local/share/brrtrouter/venv
_brrtrouter_venv_env = os.getenv('BRRTROUTER_VENV', '').strip().rstrip('/')
_home = os.getenv('HOME', '') or os.getenv('USERPROFILE', '')
brrtrouter_venv = _brrtrouter_venv_env if _brrtrouter_venv_env else (_home + '/.local/share/brrtrouter/venv' if _home else '.local/share/brrtrouter/venv')
rerp_bin = '%s/bin/rerp' % brrtrouter_venv

# ====================
# Tooling (rerp CLI)
# ====================
# Ignore patterns for tooling deps to avoid build storms from bytecode, caches,
# test/lint/coverage output, and packaging artifacts.
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
    # pip install -e writes/updates PKG-INFO, SOURCES.txt, RECORD under .../src/*.egg-info/ (inside watched trees).
    '**/brrtrouter_tooling.egg-info/**',
    '**/rerp_tooling.egg-info/**',
    # setuptools / pip wheel build dirs (may appear under tooling/ or next to src)
    '**/build',
    '**/build/**',
    '**/.hypothesis',
    '**/.DS_Store',
]

# Re-run pip install -e only when packaging metadata changes. Do NOT list tooling/src as deps: pip
# constantly refreshes *.egg-info inside src/ during install, which retriggers Tilt in a tight loop.
# Editable installs already pick up .py edits without re-running pip.
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
        '%s/tooling/pyproject.toml' % brrtrouter_watch_root,
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)

# Python lint/tests for brrtrouter-tooling + rerp_tooling.
local_resource(
    'lint-tooling',
    'just lint-fix && just format',
    deps=[
        './tooling/src',
        './tooling/tests',
        './tooling/pyproject.toml',
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)

# Pytest for rerp_tooling.
local_resource(
    'test-tooling',
    'tooling/.venv/bin/pytest tooling/tests -v --tb=short',
    deps=[
        './tooling/src',
        './tooling/tests',
        './tooling/pyproject.toml',
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)

# ====================
# Dynamic Architecture Detection
# ====================
host_machine = str(local('uname -m', quiet=True)).strip()
if host_machine in ['arm64', 'aarch64']:
    TARGET_ARCH_NAME = 'arm64'
    TARGET_RUST_TRIPLE = 'aarch64-unknown-linux-musl'
else:
    TARGET_ARCH_NAME = 'amd64'
    TARGET_RUST_TRIPLE = 'x86_64-unknown-linux-musl'

# ====================
# Microservices Code Generation (BRRTRouter)
# ====================

# Helper function to create a lint resource for a microservice
def create_microservice_lint(name, spec_file):
    local_resource(
        '%s-lint' % name,
        cmd='''
            set -e
            echo "🔍 Linting %s OpenAPI spec..."
            # Use the built debug binary directly for speed
            %s/target/debug/brrtrouter-gen lint \
                --spec ./openapi/%s \
                --fail-on-error || \
            cargo run --manifest-path %s/Cargo.toml --bin brrtrouter-gen -- \
                lint \
                --spec ./openapi/%s \
                --fail-on-error
            
            echo "✅ %s OpenAPI spec linting passed"
        ''' % (name, brrtrouter_root, spec_file, brrtrouter_root, spec_file, name),
        deps=[
            './openapi/%s' % spec_file,
        ],
        resource_deps=[],
        labels=['rerp_' + name],
        allow_parallel=True,
    )

# Helper function to create a code generation resource for a microservice
# Uses rerp gen suite so BRRTRouter gets --package-name and fix_cargo_paths runs.
def create_microservice_gen(name, spec_file, output_dir):
    local_resource(
        '%s-service-gen' % name,
        cmd='%s gen suite %s --service %s' % (rerp_bin, RERP_SUITE, name),
        deps=[
            './openapi/%s' % spec_file,
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/%s/%s/gen/src' % (RERP_SUITE, output_dir),  # Don't watch generated files
            './microservices/%s/%s/gen/doc' % (RERP_SUITE, output_dir),
            './microservices/%s/%s/impl/config' % (RERP_SUITE, output_dir),  # Config moved to impl
            './microservices/%s/%s/gen/static_site' % (RERP_SUITE, output_dir),
        ],
        resource_deps=['%s-lint' % name],  # Wait for linting to pass before generating code
        labels=['rerp_' + name],
        allow_parallel=True,
    )

# Port mappings come from the suite BFF config for suite services and from the
# shared registry for the BFF/legacy services.
def get_service_port(name):
    if name in SERVICE_PORTS:
        return SERVICE_PORTS[name]
    if name in PORT_REGISTRY:
        return PORT_REGISTRY[name]
    return '8080'

def get_package_name(name):
    return _read_cargo_package_name(name)

def get_binary_name(name):
    return name.replace('-', '_')

# Helper function to create a manual resource to regenerate impl stubs for a microservice
def create_microservice_stubs_resource(name, spec_file):
    local_resource(
        'stubs-%s' % name,
        '%s gen suite %s --service %s' % (rerp_bin, RERP_SUITE, name),
        deps=[
            './openapi/%s' % spec_file,
            './microservices/%s/%s/gen' % (RERP_SUITE, name),
            'tooling/pyproject.toml',
        ],
        resource_deps=[],
        labels=['rerp_' + name],
        allow_parallel=True,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )

# Helper function to create build resource for a microservice
def create_microservice_build_resource(name):
    # Build the service binary. rerp build microservice maps name -> Cargo [package] and
    # emits to microservices/target/<arch>/debug/<package_name>.
    local_resource(
        'build-%s' % name,
        '%s build microservice %s' % (rerp_bin, name),
        deps=[
            './microservices/%s/%s/gen/Cargo.toml' % (RERP_SUITE, name),  # Generated crate
            './microservices/%s/%s/impl/Cargo.toml' % (RERP_SUITE, name),  # Implementation crate
            './microservices/%s/%s/gen/src' % (RERP_SUITE, name),  # Generated source
            './microservices/%s/%s/impl/src' % (RERP_SUITE, name),  # Implementation source
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/target',
            './build_artifacts',
        ],
        resource_deps=['rerp-accounting-all-gens'],  # Wait for all accounting codegen
        labels=['rerp_' + name],
        allow_parallel=True,
    )

# Helper function to create test resource for a microservice
def create_microservice_test_resource(name):
    package_name = get_package_name(name)
    local_resource(
        'test-%s' % name,
        'cd microservices && cargo test -p %s' % package_name,
        deps=[
            './microservices/%s/%s/impl/src' % (RERP_SUITE, name),
            './microservices/%s/%s/gen/src' % (RERP_SUITE, name),
            './microservices/Cargo.toml',
            'tooling/pyproject.toml',
        ],
        labels=['rerp_' + name],
        trigger_mode=TRIGGER_MODE_MANUAL,
        allow_parallel=True,
    )

# Helper function to create deployment resource for a microservice
def create_microservice_deployment(name):
    package_name = get_package_name(name)
    binary_name = get_binary_name(name)
    # Binary on disk uses Cargo [package] name; Dockerfile receives the service-derived runtime name.
    target_path = 'microservices/target/%s/debug/%s' % (TARGET_RUST_TRIPLE, package_name)
    artifact_path = 'build_artifacts/%s/%s' % (TARGET_ARCH_NAME, binary_name)
    # Single template driven by --service; no per-service Dockerfile
    dockerfile_template = 'docker/microservices/Dockerfile.template'
    image_name = 'localhost:5001/rerp-%s-%s' % (RERP_SUITE, name)

    # 1. Copy binary from workspace build to artifacts and create SHA256 hash
    hash_path = 'build_artifacts/%s/%s.sha256' % (TARGET_ARCH_NAME, binary_name)
    local_resource(
        'copy-%s' % name,
        '%s docker copy-binary %s %s %s' % (rerp_bin, target_path, artifact_path, binary_name),
        deps=[target_path, 'tooling/pyproject.toml'],
        resource_deps=['build-%s' % name],
        labels=['rerp_' + name],
        allow_parallel=True,
    )

    # 2. Build and push Docker image (template rendered on the fly with --service)
    local_resource(
        'docker-%s' % name,
        '%s docker build-image-simple %s %s %s %s --service %s' % (rerp_bin, image_name, dockerfile_template, hash_path, artifact_path, name),
        deps=[hash_path, artifact_path, dockerfile_template, 'tooling/pyproject.toml'],
        resource_deps=['build-base-image', 'copy-%s' % name],
        labels=['rerp_' + name],
        allow_parallel=False,
    )

    # 3. Custom build for Tilt live updates
    # Ensure image exists (build if custom_build runs before docker-%s), then push to localhost:5001
    # or, if registry is not running, use kind load (no registry needed)
    custom_build(
        image_name,
        ('%s docker build-image-simple %s %s %s %s --service %s' % (rerp_bin, image_name, dockerfile_template, hash_path, artifact_path, name)
         + ' && (docker push %s:tilt 2>/dev/null || kind load docker-image %s:tilt --name kind)' % (image_name, image_name)),
        deps=[artifact_path, hash_path, dockerfile_template, 'microservices/%s/%s/impl/config' % (RERP_SUITE, name), 'microservices/%s/%s/gen/doc' % (RERP_SUITE, name), 'microservices/%s/%s/gen/static_site' % (RERP_SUITE, name)],
        tag='tilt',
        live_update=[
            sync(artifact_path, '/app/%s' % binary_name),
            sync('microservices/%s/%s/impl/config/' % (RERP_SUITE, name), '/app/config/'),  # Config in impl
            sync('microservices/%s/%s/gen/doc/' % (RERP_SUITE, name), '/app/doc/'),  # Doc in gen
            sync('microservices/%s/%s/gen/static_site/' % (RERP_SUITE, name), '/app/static_site/'),  # Static in gen
            run('kill -HUP 1', trigger=[artifact_path]),
        ],
    )

    # 4. Deploy using Helm (postgres in `data` namespace, Redis in `data`)
    _helm_values = [
        './helm/rerp-microservice/values/%s.yaml' % name,
        './helm/rerp-microservice/values/_database-shared-kind.yaml',
        './helm/rerp-microservice/values/_redis-shared-kind.yaml',
    ]
    k8s_yaml(helm('./helm/rerp-microservice', name=name, namespace='rerp', values=_helm_values))

    # 5. Kubernetes resource configuration
    k8s_resource(
        name,
        port_forwards=['%s:%s' % (get_service_port(name), get_service_port(name))],
        resource_deps=['rerp-database-env', 'docker-%s' % name],
        labels=['rerp_' + name],
        auto_init=True,
        trigger_mode=TRIGGER_MODE_AUTO,
    )

# ====================
# Kubernetes Namespace
# ====================
# The "rerp" namespace is created at cluster creation (e.g. just dev-up).
k8s_yaml('k8s/rerp/namespace.yaml')
k8s_resource(
    new_name='rerp-namespace',
    objects=['rerp:namespace'],
    labels=['rerp'],
)

# ====================
# Accounting Microservices
# ====================
# All runtime-ready accounting microservices: each has lint, gen, build, deploy.
# Discovery starts from openapi/accounting/bff-suite-config.yaml, then filters to
# services that have generated/implementation crates and Helm values.

for name in ACCOUNTING_SERVICES:
    service_spec_file = '%s/%s' % (RERP_SUITE, SERVICE_SPEC_PATHS.get(name, '%s/openapi.yaml' % name))
    create_microservice_lint(name, service_spec_file)
    create_microservice_gen(name, service_spec_file, name)
    create_microservice_stubs_resource(name, service_spec_file)
    create_microservice_test_resource(name)


# All accounting gens must complete before any build (so microservices/Cargo.toml workspace members exist)
# Includes bff-service-gen (BFF spec is generated by rerp bff generate-system, then bff-lint, then bff-service-gen)
local_resource(
    'rerp-accounting-all-gens',
    'echo "✅ All accounting codegen complete"',
    resource_deps=['%s-service-gen' % name for name in ACCOUNTING_SERVICES] + ['bff-service-gen'],
    labels=['rerp_all_gens'],
    allow_parallel=False,
)

for name in ACCOUNTING_SERVICES:
    create_microservice_build_resource(name)
    create_microservice_deployment(name)

# ====================
# Backend for Frontend (BFF) Spec Generation
# ====================
# The BFF spec aggregates all accounting microservice paths and is automatically
# regenerated whenever any microservice OpenAPI spec changes.
# Uses the rerp bff generate-system subcommand.

local_resource(
    'bff-spec-gen',
    cmd='''
        set -e
        echo "🔄 Regenerating Accounting BFF OpenAPI spec (rerp bff generate-system --suite %s)..."
        %s bff generate-system --suite %s
        echo "✅ Accounting BFF spec regeneration complete"
    ''' % (RERP_SUITE, rerp_bin, RERP_SUITE),
    deps=[
        # Suite config
        './%s' % RERP_SUITE_CONFIG,
        # All accounting service OpenAPI specs, including contract-only services.
    ] + ['./openapi/%s/%s' % (RERP_SUITE, SERVICE_SPEC_PATHS.get(name, '%s/openapi.yaml' % name)) for name in DISCOVERED_ACCOUNTING_SERVICES],
    ignore=[
        './openapi/accounting/openapi_bff.yaml',  # Don't watch the generated file
    ],
    resource_deps=[],
    labels=['rerp_bff'],
    allow_parallel=True,
)

# BFF microservice: lint generated spec, gen from openapi_bff.yaml, build, deploy
# Lint depends on bff-spec-gen so openapi_bff.yaml exists
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
    labels=['rerp_bff'],
    allow_parallel=True,
)
create_microservice_gen(BFF_SERVICE_NAME, '%s/openapi_bff.yaml' % RERP_SUITE, BFF_SERVICE_NAME)
create_microservice_stubs_resource(BFF_SERVICE_NAME, '%s/openapi_bff.yaml' % RERP_SUITE)
create_microservice_build_resource(BFF_SERVICE_NAME)
create_microservice_deployment(BFF_SERVICE_NAME)

# ====================
# Shared Infrastructure Configuration
# ====================
# RERP connects to shared-kind-cluster infrastructure via ClusterIP DNS.
# The shared cluster (kind-kind, managed by microscaler/shared-kind-cluster) provides:
#   - Postgres: postgres.data.svc.cluster.local:5432
#   - Redis: redis.data.svc.cluster.local:6379
#   - MinIO: minio.data.svc.cluster.local:9000
#   - Observability: prometheus.observability.svc.cluster.local, loki.observability.svc.cluster.local, etc.
#
# This ConfigMap/Secret provides the connection info to Helm values at runtime.

# Shared DB ConfigMap + Secret for Helm microservices.
k8s_yaml('k8s/rerp/rerp-database-env.yaml')
k8s_resource(
    new_name='rerp-database-env',
    objects=[
        'rerp-database-config:configmap:rerp',
        'rerp-db-credentials:secret:rerp',
    ],
    labels=['rerp'],
)

# ====================
# Database Initialization
# ====================
# Setup script for RERP PostgreSQL database + schema (in-cluster).
# Waits for deployment/postgres-primary in the shared Kind cluster.
#
# Layout:
#   - Database `rerp` — app data only.
#   - Schema `rerp` — all RERP tables (search_path default for this database).
#   - Role `rerp` — login role matching helm app.config.database (password from env below).
#   - After ./migrations (apply_order.txt), optional microservices/*/impl/seeds/*.sql.
#
# Optional:
#   RERP_DB_INIT_TIMEOUT (default 600s), RERP_DB_PASSWORD (must match helm dev password).
#   RERP_APPLY_MIGRATIONS_ONLY=1 — skip role/DB creation; only wait for postgres, apply ./migrations, then GRANTs.

local_resource(
    'rerp-db-init',
    'chmod +x ./scripts/setup-db.sh && ./scripts/setup-db.sh',
    deps=['./scripts/setup-db.sh'],
    labels=['rerp'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
)

# Apply ./migrations/*.sql in apply_order.txt order via kubectl exec to deployment/postgres (database rerp).
# Use after `cargo run -p lifeguard_migrator` (or Tilt `rerp-migrate`) changes SQL.
local_resource(
    'rerp-apply-migrations',
    'RERP_APPLY_MIGRATIONS_ONLY=1 chmod +x ./scripts/setup-db.sh && ./scripts/setup-db.sh',
    deps=['./scripts/setup-db.sh', './migrations/apply_order.txt'],
    labels=['rerp'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)

# Ad-hoc: regenerate SQL under migrations/ + apply_order.txt from Lifeguard entity registries.
# Does NOT connect to PostgreSQL — only writes files. To run SQL against the cluster DB, trigger
# `rerp-apply-migrations` (or full `rerp-db-init` on a fresh env). Requires Rust toolchain.
local_resource(
    'rerp-migrate',
    'cd microservices && cargo run -p lifeguard_migrator',
    deps=[
        './microservices/migrator',
        './entities/src',
        './microservices/Cargo.toml',
    ],
    ignore=['./microservices/target'],
    labels=['rerp'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)

# ====================
# Website (UI)
# ====================
# SolidJS site in docker/website; build and push to localhost:5001 or kind-load
custom_build(
    'localhost:5001/rerp-website',
    'docker build -f docker/website/Dockerfile -t localhost:5001/rerp-website:tilt . && (docker push localhost:5001/rerp-website:tilt 2>/dev/null || kind load docker-image localhost:5001/rerp-website:tilt --name kind)',
    deps=['./ui/website', './ui/shared', './docker/website'],
    tag='tilt',
)

k8s_yaml('k8s/website.yaml')
k8s_resource(
    'website',
    port_forwards=['3001:8080'],
    labels=['rerp_ui'],
)

# ====================
# Test Integration (Frontend)
# ====================
# Helper to ensure Node/npm can be found across Mac (Homebrew) and Linux environments (NVM) natively
NODE_ENV_PREFIX = 'export PATH="/opt/homebrew/bin:/usr/local/bin:$PATH" && ([ -s "$HOME/.nvm/nvm.sh" ] && source "$HOME/.nvm/nvm.sh" || true) && '

local_resource(
    'test-frontend-bdd',
    cmd=NODE_ENV_PREFIX + 'cd frontend && npm run test:bdd 2>&1 | tee playwright-bdd.log',
    deps=[
        './frontend/e2e/features',
        './frontend/e2e/steps',
        './frontend/e2e/specs',
    ],
    labels=['tests'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)

local_resource(
    'test-frontend-e2e',
    cmd=NODE_ENV_PREFIX + 'cd frontend && npm run test:e2e 2>&1 | tee playwright-e2e.log',
    deps=[
        './frontend/e2e/specs',
        './frontend/src',
    ],
    labels=['tests'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)

# ====================
# Standalone Worker (if any)
# ====================
# Example: if RERP has a background worker (migrations, email, cron, etc.):
#
# custom_build(
#     'localhost:5001/rerp-worker',
#     'docker build -f docker/workers/Dockerfile -t localhost:5001/rerp-worker:tilt . && (docker push localhost:5001/rerp-worker:tilt 2>/dev/null || kind load docker-image localhost:5001/rerp-worker:tilt --name kind)',
#     deps=['./microservices/rerp_worker', 'docker/workers/Dockerfile'],
#     tag='tilt',
# )
#
# k8s_yaml('k8s/rerp/rerp-worker.yaml')
# k8s_resource(
#     'rerp-worker',
#     resource_deps=['rerp-database-env'],
#     labels=['rerp_workers'],
# )
