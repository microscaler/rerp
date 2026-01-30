# RERP Development Environment
# Run with: tilt up
#
# This Tiltfile orchestrates the build and deployment of all RERP microservices
# using a base Docker image and pre-built binaries.

# ====================
# Configuration
# ====================

allow_k8s_contexts(['kind-rerp'])

# Configure Tilt web UI port
update_settings(k8s_upsert_timeout_secs=60)
config.define_string('tilt_port', args=False, usage='Port for Tilt web UI')
cfg = config.parse()
tilt_port = cfg.get('tilt_port', '10351')  # Default to 10351 to avoid conflicts

# Set the Tilt UI port
os.putenv('TILT_PORT', tilt_port)

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
    '**/.eggs',
    '**/dist',
    '**/.hypothesis',
]

# Rebuild on tooling src/pyproject changes. Run `just init` before first `tilt up`.
local_resource(
    'build-tooling',
    'just build-tooling',
    deps=[
        './tooling/src',
        './tooling/pyproject.toml',
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)

# Ruff check --fix + format (fix in place). Same deps as build-tooling plus tests.
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

# Pytest for tooling (rerp_tooling).
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
# Base Docker Image
# ====================
# Note: Base image is not currently used - each service builds its own image
# Uncomment if you want to use a shared base image:
# docker_build(
#     'rerp/base',
#     '.',
#     dockerfile='./docker/base/Dockerfile',
#     platform='linux/amd64',
#     only=[
#         './docker/base',
#     ],
# )

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
            ../BRRTRouter/target/debug/brrtrouter-gen lint \
                --spec ./openapi/%s \
                --fail-on-error || \
            cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen -- \
                lint \
                --spec ./openapi/%s \
                --fail-on-error
            
            echo "✅ %s OpenAPI spec linting passed"
        ''' % (name, spec_file, spec_file, name),
        deps=[
            './openapi/%s' % spec_file,
        ],
        resource_deps=[],
        labels=['acc_' + name],
        allow_parallel=True,
    )

# Helper function to create a code generation resource for a microservice
# Uses rerp gen suite so BRRTRouter gets --package-name and fix_cargo_paths runs (single source of truth).
def create_microservice_gen(name, spec_file, output_dir):
    local_resource(
        '%s-service-gen' % name,
        cmd='tooling/.venv/bin/rerp gen suite accounting --service %s' % name,
        deps=[
            './openapi/%s' % spec_file,
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/accounting/%s/gen/src' % output_dir,  # Don't watch generated files
            './microservices/accounting/%s/gen/doc' % output_dir,
            './microservices/accounting/%s/impl/config' % output_dir,  # Config moved to impl
            './microservices/accounting/%s/gen/static_site' % output_dir,
        ],
        resource_deps=['%s-lint' % name],  # Wait for linting to pass before generating code
        labels=['acc_' + name],
        allow_parallel=True,
    )

# Cargo [package] names for impl crates (binary path under microservices/target/.../debug/).
# Must match tooling/src/rerp_tooling/build/constants.py PACKAGE_NAMES.
PACKAGE_NAMES = {
    'general-ledger': 'rerp_accounting_general_ledger',
    'invoice': 'rerp_accounting_invoice',
    'accounts-receivable': 'rerp_accounting_accounts_receivable',
    'accounts-payable': 'rerp_accounting_accounts_payable',
    'bank-sync': 'rerp_accounting_bank_sync',
    'asset': 'rerp_accounting_asset',
    'budget': 'rerp_accounting_budget',
    'edi': 'rerp_accounting_edi',
    'financial-reports': 'rerp_accounting_financial_reports',
    'bff': 'rerp_accounting_bff',
}

# Artifact and container binary names (build_artifacts/, /app/, Helm app.binaryName)
# These match Dockerfiles and values/*.yaml; may differ from package name.
BINARY_NAMES = {
    'general-ledger': 'general_ledger',
    'invoice': 'invoice',
    'accounts-receivable': 'accounts_receivable',
    'accounts-payable': 'accounts_payable',
    'bank-sync': 'bank_sync',
    'asset': 'asset',
    'budget': 'budget',
    'edi': 'edi',
    'financial-reports': 'financial_reports',
    'bff': 'bff',
}

# Port mappings
def get_service_port(name):
    ports = {
        'general-ledger': '8001',
        'invoice': '8002',
        'accounts-receivable': '8003',
        'accounts-payable': '8004',
        'bank-sync': '8005',
        'asset': '8006',
        'budget': '8007',
        'edi': '8008',
        'financial-reports': '8009',
        'bff': '8010',
    }
    return ports.get(name, '8080')

# Helper function to create build resource for a microservice
def create_microservice_build_resource(name):
    # Build the service binary. rerp build microservice maps name -> Cargo [package] and
    # emits to microservices/target/x86_64-unknown-linux-musl/debug/<package_name>.
    # create_microservice_deployment's copy step uses PACKAGE_NAMES for that path and
    # BINARY_NAMES for build_artifacts/amd64/<binary_name> (Docker/Helm).
    local_resource(
        'build-%s' % name,
        'tooling/.venv/bin/rerp build microservice %s' % name,
        deps=[
            './microservices/accounting/%s/gen/Cargo.toml' % name,  # Generated crate
            './microservices/accounting/%s/impl/Cargo.toml' % name,  # Implementation crate
            './microservices/accounting/%s/gen/src' % name,  # Generated source
            './microservices/accounting/%s/impl/src' % name,  # Implementation source
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/target',
            './build_artifacts',
        ],
        resource_deps=['accounting-all-gens'],  # Wait for all accounting codegen (so workspace members exist)
        labels=['acc_' + name],
        allow_parallel=True,
    )

# Helper function to create deployment resource for a microservice
def create_microservice_deployment(name):
    package_name = PACKAGE_NAMES.get(name, BINARY_NAMES.get(name, name.replace('-', '_')))
    binary_name = BINARY_NAMES.get(name, '%s_service_api' % name.replace('-', '_'))
    # Binary on disk uses Cargo [package] name; copy dest uses BINARY_NAMES for Docker/Helm.
    # Use build_artifacts/amd64/ so Dockerfile build_artifacts/${TARGETARCH}/ works (Tilt = amd64).
    target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % package_name
    artifact_path = 'build_artifacts/amd64/%s' % binary_name
    # Single template driven by --service; no per-service Dockerfile
    dockerfile_template = 'docker/microservices/Dockerfile.template'
    image_name = 'localhost:5001/rerp-accounting-%s' % name

    # 1. Copy binary from workspace build to artifacts and create SHA256 hash
    hash_path = 'build_artifacts/amd64/%s.sha256' % binary_name
    local_resource(
        'copy-%s' % name,
        'tooling/.venv/bin/rerp docker copy-binary %s %s %s' % (target_path, artifact_path, binary_name),
        deps=[target_path, 'tooling/pyproject.toml'],
        resource_deps=['build-%s' % name],
        labels=['acc_' + name],
        allow_parallel=True,
    )
    
    # 2. Build and push Docker image (template rendered on the fly with --service)
    local_resource(
        'docker-%s' % name,
        'tooling/.venv/bin/rerp docker build-image-simple %s %s %s %s --service %s' % (image_name, dockerfile_template, hash_path, artifact_path, name),
        deps=[hash_path, artifact_path, dockerfile_template, 'tooling/pyproject.toml'],
        resource_deps=['copy-%s' % name],
        labels=['acc_' + name],
        allow_parallel=False,
    )
    
    # 3. Custom build for Tilt live updates
    # Ensure image exists (build if custom_build runs before docker-%s), then push to localhost:5001
    # or, if registry is not running, use kind load (no registry needed)
    custom_build(
        image_name,
        ('(docker image inspect %s:tilt >/dev/null 2>&1) || tooling/.venv/bin/rerp docker build-image-simple %s %s %s %s --service %s' % (image_name, image_name, dockerfile_template, hash_path, artifact_path, name)
         + ' && (docker push %s:tilt 2>/dev/null || kind load docker-image %s:tilt --name rerp)' % (image_name, image_name)),
        deps=[artifact_path, hash_path, 'microservices/accounting/%s/impl/config' % name, 'microservices/accounting/%s/gen/doc' % name, 'microservices/accounting/%s/gen/static_site' % name],
        tag='tilt',
        live_update=[
            sync(artifact_path, '/app/%s' % binary_name),
            sync('microservices/accounting/%s/impl/config/' % name, '/app/config/'),  # Config in impl
            sync('microservices/accounting/%s/gen/doc/' % name, '/app/doc/'),  # Doc in gen
            sync('microservices/accounting/%s/gen/static_site/' % name, '/app/static_site/'),  # Static in gen
            run('kill -HUP 1', trigger=[artifact_path]),
        ],
    )
    
    # 4. Deploy using Helm
    k8s_yaml(helm('./helm/rerp-microservice', name=name, namespace='rerp', values=['./helm/rerp-microservice/values/%s.yaml' % name]))
    
    # 5. Kubernetes resource configuration
    k8s_resource(
        name,
        port_forwards=['%s:%s' % (get_service_port(name), get_service_port(name))],
        resource_deps=['docker-%s' % name],
        labels=['acc_' + name],
        auto_init=True,
        trigger_mode=TRIGGER_MODE_AUTO,
    )

# ====================
# Kubernetes Namespace
# ====================
# The "rerp" namespace is created at cluster creation (e.g. just dev-up applies
# k8s/microservices/namespace.yaml). Tilt does not manage it.

# ====================
# Accounting Microservices
# ====================
# All accounting microservices: each has lint, gen, build, deploy. One label per resource with
# acc_ prefix (e.g. acc_general-ledger, acc_accounts-receivable, acc_bff, acc_all_gens).

ACCOUNTING_SERVICES = [
    'general-ledger',
    'invoice',
    'accounts-receivable',
    'accounts-payable',
    'bank-sync',
    'asset',
    'budget',
    'edi',
    'financial-reports',
]

for name in ACCOUNTING_SERVICES:
    create_microservice_lint(name, 'accounting/%s/openapi.yaml' % name)
    create_microservice_gen(name, 'accounting/%s/openapi.yaml' % name, name)

# All accounting gens must complete before any build (so microservices/Cargo.toml workspace members exist)
# After reorganization, workspace members are: accounting/{service}/gen and accounting/{service}/impl
# Includes bff-service-gen (BFF spec is generated by bff-spec-gen, then bff-lint, then bff-service-gen)
local_resource(
    'accounting-all-gens',
    'echo "✅ All accounting codegen complete"',
    resource_deps=['%s-service-gen' % name for name in ACCOUNTING_SERVICES] + ['bff-service-gen'],
    labels=['acc_all_gens'],
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
# Uses the standalone bff-generator (pip install bff-generator).

local_resource(
    'bff-spec-gen',
    cmd='''
        set -e
        echo "🔄 Regenerating Accounting BFF OpenAPI spec (bff-generator)..."
        bff-generator generate-spec --config openapi/accounting/bff-suite-config.yaml --output openapi/accounting/openapi_bff.yaml
        echo "✅ Accounting BFF spec regeneration complete"
    ''',
    deps=[
        # Suite config
        './openapi/accounting/bff-suite-config.yaml',
        # Accounting service OpenAPI specs
        './openapi/accounting/general-ledger/openapi.yaml',
        './openapi/accounting/invoice/openapi.yaml',
        './openapi/accounting/accounts-receivable/openapi.yaml',
        './openapi/accounting/accounts-payable/openapi.yaml',
        './openapi/accounting/bank-sync/openapi.yaml',
        './openapi/accounting/asset/openapi.yaml',
        './openapi/accounting/budget/openapi.yaml',
        './openapi/accounting/edi/openapi.yaml',
        './openapi/accounting/financial-reports/openapi.yaml',
    ],
    ignore=[
        './openapi/accounting/openapi_bff.yaml',  # Don't watch the generated file
    ],
    resource_deps=[],
    labels=['acc_bff'],
    allow_parallel=True,
)

# BFF microservice: lint generated spec, gen from openapi_bff.yaml, build, deploy
# Lint depends on bff-spec-gen so openapi_bff.yaml exists
local_resource(
    'bff-lint',
    cmd='''
        set -e
        echo "🔍 Linting BFF OpenAPI spec..."
        ../BRRTRouter/target/debug/brrtrouter-gen lint \
            --spec ./openapi/accounting/openapi_bff.yaml \
            --fail-on-error || \
        cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen -- \
            lint \
            --spec ./openapi/accounting/openapi_bff.yaml \
            --fail-on-error
        echo "✅ BFF OpenAPI spec linting passed"
    ''',
    deps=['./openapi/accounting/openapi_bff.yaml'],
    resource_deps=['bff-spec-gen'],
    labels=['acc_bff'],
    allow_parallel=True,
)
create_microservice_gen('bff', 'accounting/openapi_bff.yaml', 'bff')
create_microservice_build_resource('bff')
create_microservice_deployment('bff')

# ====================
# Website (UI)
# ====================
# SolidJS site in docker/website; build and push to localhost:5001 or kind-load
custom_build(
    'localhost:5001/rerp-website',
    'docker build -f docker/website/Dockerfile -t localhost:5001/rerp-website:tilt . && (docker push localhost:5001/rerp-website:tilt 2>/dev/null || kind load docker-image localhost:5001/rerp-website:tilt --name rerp)',
    deps=['./ui/website', './ui/shared', './docker/website'],
    tag='tilt',
)

k8s_yaml('k8s/website.yaml')
k8s_resource(
    'website',
    port_forwards=['3000:8080'],
    labels=['ui'],
)
