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
tilt_port = cfg.get('tilt_port', '10352')  # Default to 10352 to avoid conflicts

# Set the Tilt UI port
os.putenv('TILT_PORT', tilt_port)

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
# Microservices Build System
# ====================

# Helper function to create build resources for a microservice
def create_microservice_build(system, module, port=8000):
    """Create build resources for a RERP microservice."""
    
    # Convert module name to binary name
    binary_name = 'rerp_%s_%s_impl' % (system, module.replace('-', '_'))
    hash_path = './build_artifacts/%s.sha256' % binary_name
    artifact_path = './build_artifacts/%s' % binary_name
    dockerfile = './docker/microservices/Dockerfile.%s_%s' % (system, module)
    image_name = 'localhost:5001/rerp-%s-%s' % (system, module)
    
    # Build workspace (all microservices at once) - only runs once
    local_resource(
        'workspace-build',
        cmd='''
            echo "🔨 Building all RERP microservices..."
            python3 ./scripts/host-aware-build.py workspace amd64
            echo "✅ Workspace build complete"
        ''',
        deps=[
            './components/Cargo.toml',
            './components/**/*.rs',
            './scripts/host-aware-build.py',
        ],
        ignore=[
            './components/target',
            './build_artifacts',
        ],
        labels=['microservices-build'],
        allow_parallel=False,
    )
    
    # Copy binary to build_artifacts
    local_resource(
        '%s-%s-copy' % (system, module),
        cmd='''
            echo "📦 Copying %s-%s binary..."
            ./scripts/copy-microservice-binary.sh %s %s
            echo "✅ Binary copied"
        ''' % (system, module, system, module),
        deps=[
            './components/target/x86_64-unknown-linux-musl/release/%s' % binary_name,
            './scripts/copy-microservice-binary.sh',
        ],
        resource_deps=['workspace-build'],
        labels=['microservices-build'],
        allow_parallel=True,
    )
    
    # Generate Dockerfile if it doesn't exist
    local_resource(
        '%s-%s-dockerfile' % (system, module),
        cmd='''
            echo "📝 Generating Dockerfile for %s-%s..."
            python3 ./scripts/generate-dockerfile.py %s %s %s
            echo "✅ Dockerfile generated"
        ''' % (system, module, system, module, port),
        deps=[
            './docker/microservices/Dockerfile.template',
            './scripts/generate-dockerfile.py',
        ],
        ignore=[dockerfile],  # Don't watch the generated file
        labels=['microservices-docker'],
        allow_parallel=True,
    )
    
    # Build and push Docker image
    local_resource(
        '%s-%s-docker' % (system, module),
        cmd='''
            echo "🔨 Building Docker image for %s-%s..."
            ./scripts/build-microservice-docker.sh %s %s %s %s
            echo "✅ Docker image built and pushed"
        ''' % (system, module, system, module, image_name, port),
        deps=[
            hash_path,
            artifact_path,
            dockerfile,
            './scripts/build-microservice-docker.sh',
        ],
        resource_deps=[
            '%s-%s-copy' % (system, module),
            '%s-%s-dockerfile' % (system, module),
        ],
        labels=['microservices-docker'],
        allow_parallel=True,
    )
    
    # Build Docker image for Tilt
    docker_build(
        image_name,
        '.',
        dockerfile=dockerfile,
        platform='linux/amd64',
        only=[
            artifact_path,
            './components/%s/%s_impl/config' % (system, module),
            './components/%s/%s/doc' % (system, module),
            './components/%s/%s_impl/static_site' % (system, module),
            dockerfile,
        ],
        resource_deps=[
            '%s-%s-docker' % (system, module),
        ],
    )
    
    return image_name

# ====================
# Microservices Code Generation (BRRTRouter)
# ====================

# Helper function to create a lint resource for a microservice
def create_microservice_lint(name, spec_file):
    local_resource(
        '%s-lint' % name,
        cmd='''
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
def create_microservice_gen(name, spec_file, output_dir):
    local_resource(
        '%s-service-gen' % name,
        cmd='''
            echo "🔄 Regenerating %s service from OpenAPI spec..."
            # Use the built debug binary directly for speed (instant vs minutes for cargo run)
            ../BRRTRouter/target/debug/brrtrouter-gen generate \
                --spec ./openapi/%s \
                --output ./microservices/accounting/%s \
                --force || \
            cargo run --manifest-path ../BRRTRouter/Cargo.toml --bin brrtrouter-gen -- \
                generate \
                --spec ./openapi/%s \
                --output ./microservices/accounting/%s \
                --force
            
            # Fix Cargo.toml paths to point to BRRTRouter repository
            echo "🔧 Fixing Cargo.toml dependency paths..."
            if [ -f ./microservices/accounting/%s/Cargo.toml ]; then
                python3 ./scripts/fix_cargo_toml_paths.py ./microservices/accounting/%s/Cargo.toml
            fi
            
            echo "✅ %s service regeneration complete"
        ''' % (name, spec_file, output_dir, spec_file, output_dir, output_dir, output_dir, name),
        deps=[
            './openapi/%s' % spec_file,
            './scripts/fix_cargo_toml_paths.py',
        ],
        ignore=[
            './microservices/accounting/%s/src' % output_dir,  # Don't watch generated files
            './microservices/accounting/%s/doc' % output_dir,
            './microservices/accounting/%s/config' % output_dir,
            './microservices/accounting/%s/static_site' % output_dir,
        ],
        resource_deps=['%s-lint' % name],  # Wait for linting to pass before generating code
        labels=['acc_' + name],
        allow_parallel=True,
    )

# Cargo [package] names from brrtrouter-gen (for cargo -p and for binary output path)
# BFF: brrtrouter-gen derives "rerp_accounting_backend_for_frontend_api" from OpenAPI info.title
PACKAGE_NAMES = {
    'general-ledger': 'general_ledger',
    'invoice': 'invoice_management',
    'accounts-receivable': 'accounts_receivable',
    'accounts-payable': 'accounts_payable',
    'bank-sync': 'bank_synchronization',
    'asset': 'asset_management',
    'budget': 'budgeting',
    'edi': 'edi___compliance',
    'financial-reports': 'financial_reports',
    'bff': 'rerp_accounting_backend_for_frontend_api',
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
    binary_name = BINARY_NAMES.get(name, '%s_service_api' % name.replace('-', '_'))
    # Target path is in workspace target directory (in microservices directory)
    # Using debug builds for active development (faster compilation, better debugging)
    target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % binary_name
    artifact_path = 'build_artifacts/%s' % binary_name
    
    # Build the service binary
    local_resource(
        'build-%s' % name,
        './scripts/build-microservice.sh %s' % name,
        deps=[
            './microservices/accounting/%s/Cargo.toml' % name,
            './microservices/accounting/%s/src' % name,
            './scripts/build-microservice.sh',
        ],
        ignore=[
            './microservices/target',
            './build_artifacts',
        ],
        resource_deps=['accounting-all-gens'],  # Wait for all accounting codegen (so workspace members exist)
        labels=['acc_' + name],
        allow_parallel=True,
    )
    
    return binary_name, artifact_path

# Helper function to create deployment resource for a microservice
def create_microservice_deployment(name):
    package_name = PACKAGE_NAMES.get(name, BINARY_NAMES.get(name, name.replace('-', '_')))
    binary_name = BINARY_NAMES.get(name, '%s_service_api' % name.replace('-', '_'))
    # Binary on disk uses Cargo [package] name; copy dest uses BINARY_NAMES for Docker/Helm.
    # Use build_artifacts/amd64/ so Dockerfile build_artifacts/${TARGETARCH}/ works (Tilt = amd64).
    target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % package_name
    artifact_path = 'build_artifacts/amd64/%s' % binary_name
    dockerfile = 'docker/microservices/Dockerfile.%s' % name
    image_name = 'localhost:5001/rerp-%s' % name

    # 1. Copy binary from workspace build to artifacts and create SHA256 hash
    hash_path = 'build_artifacts/amd64/%s.sha256' % binary_name
    local_resource(
        'copy-%s' % name,
        './scripts/copy-microservice-binary-simple.sh %s %s %s' % (target_path, artifact_path, binary_name),
        deps=[target_path, './scripts/copy-microservice-binary-simple.sh'],
        resource_deps=['build-%s' % name],
        labels=['acc_' + name],
        allow_parallel=True,
    )
    
    # 2. Build and push Docker image
    local_resource(
        'docker-%s' % name,
        './scripts/build-microservice-docker-simple.sh %s %s %s %s' % (image_name, dockerfile, hash_path, artifact_path),
        deps=[hash_path, artifact_path, dockerfile, './scripts/build-microservice-docker-simple.sh'],
        resource_deps=['copy-%s' % name],
        labels=['acc_' + name],
        allow_parallel=False,
    )
    
    # 3. Custom build for Tilt live updates
    # Ensure image exists (build if custom_build runs before docker-%s), then push to localhost:5001
    # or, if registry is not running, use kind load (no registry needed)
    custom_build(
        image_name,
        ('(docker image inspect %s:tilt >/dev/null 2>&1) || ./scripts/build-microservice-docker-simple.sh %s %s %s %s' % (image_name, image_name, dockerfile, hash_path, artifact_path)
         + ' && (docker push $EXPECTED_REF 2>/dev/null || kind load docker-image $EXPECTED_REF --name rerp)'),
        deps=[artifact_path, hash_path, 'microservices/accounting/%s/config' % name, 'microservices/accounting/%s/doc' % name, 'microservices/accounting/%s/static_site' % name],
        tag='tilt',
        live_update=[
            sync(artifact_path, '/app/%s' % binary_name),
            sync('microservices/accounting/%s/config/' % name, '/app/config/'),
            sync('microservices/accounting/%s/doc/' % name, '/app/doc/'),
            sync('microservices/accounting/%s/static_site/' % name, '/app/static_site/'),
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
# All accounting components: each has lint, gen, build, deploy. One label per resource with
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

# All accounting gens must complete before any build (so microservices/Cargo.toml members exist)
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
