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
        labels=['microservices-lint'],
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
        labels=['microservices-codegen'],
        allow_parallel=True,
    )

# Binary name mappings
# These match the package names in Cargo.toml (BRRTRouter uses package name as binary name)
BINARY_NAMES = {
    'general-ledger': 'general_ledger',
    'invoice': 'invoice',
    'accounts-receivable': 'accounts_receivable',
    'accounts-payable': 'accounts_payable',
    'bank-sync': 'bank_sync',
    'asset': 'asset',
    'budget': 'budget',
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
        resource_deps=['%s-service-gen' % name],  # Wait for code generation
        labels=['microservices-build'],
        allow_parallel=True,
    )
    
    return binary_name, artifact_path

# Helper function to create deployment resource for a microservice
def create_microservice_deployment(name):
    binary_name = BINARY_NAMES.get(name, '%s_service_api' % name.replace('-', '_'))
    target_path = 'microservices/target/x86_64-unknown-linux-musl/debug/%s' % binary_name
    artifact_path = 'build_artifacts/%s' % binary_name
    dockerfile = 'docker/microservices/Dockerfile.%s' % name
    image_name = 'localhost:5001/rerp-%s' % name
    
    # 1. Copy binary from workspace build to artifacts and create SHA256 hash
    hash_path = 'build_artifacts/%s.sha256' % binary_name
    local_resource(
        'copy-%s' % name,
        './scripts/copy-microservice-binary-simple.sh %s %s %s' % (target_path, artifact_path, binary_name),
        deps=[target_path, './scripts/copy-microservice-binary-simple.sh'],
        resource_deps=['build-%s' % name],
        labels=['microservices-build'],
        allow_parallel=True,
    )
    
    # 2. Build and push Docker image
    local_resource(
        'docker-%s' % name,
        './scripts/build-microservice-docker-simple.sh %s %s %s %s' % (image_name, dockerfile, hash_path, artifact_path),
        deps=[hash_path, artifact_path, dockerfile, './scripts/build-microservice-docker-simple.sh'],
        resource_deps=['copy-%s' % name],
        labels=['microservices-build'],
        allow_parallel=False,
    )
    
    # 3. Custom build for Tilt live updates
    custom_build(
        image_name,
        'docker tag %s:tilt $EXPECTED_REF && docker push $EXPECTED_REF' % image_name,
        deps=[artifact_path, 'microservices/accounting/%s/config' % name, 'microservices/accounting/%s/doc' % name, 'microservices/accounting/%s/static_site' % name],
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
        labels=['accounting'],
        auto_init=True,
        trigger_mode=TRIGGER_MODE_AUTO,
    )

# ====================
# Load Kubernetes Namespace
# ====================

# Load microservices namespace
k8s_yaml(kustomize('./k8s/microservices'))

# ====================
# Accounting Microservices
# ====================

# General Ledger Service
create_microservice_lint('general-ledger', 'accounting/general-ledger/openapi.yaml')
create_microservice_gen('general-ledger', 'accounting/general-ledger/openapi.yaml', 'general-ledger')
create_microservice_build_resource('general-ledger')
create_microservice_deployment('general-ledger')

# ====================
# Backend for Frontend (BFF) Spec Generation
# ====================
# The BFF spec aggregates all accounting microservice paths and is automatically
# regenerated whenever any microservice OpenAPI spec changes.

local_resource(
    'bff-spec-gen',
    cmd='''
        echo "🔄 Regenerating Accounting BFF OpenAPI spec from accounting microservice specs (idempotent clobber)..."
        python3 ./scripts/generate_bff_spec.py accounting
        
        echo "✅ Accounting BFF spec regeneration complete (file clobbered)"
    ''',
    deps=[
        # Accounting services - list all accounting service OpenAPI specs
        './openapi/accounting/general-ledger/openapi.yaml',
        './openapi/accounting/invoice/openapi.yaml',
        './openapi/accounting/accounts-receivable/openapi.yaml',
        './openapi/accounting/accounts-payable/openapi.yaml',
        './openapi/accounting/bank-sync/openapi.yaml',
        './openapi/accounting/asset/openapi.yaml',
        './openapi/accounting/budget/openapi.yaml',
        './openapi/accounting/edi/openapi.yaml',
        './openapi/accounting/financial-reports/openapi.yaml',
        # Script itself
        './scripts/generate_bff_spec.py',
    ],
    ignore=[
        './openapi/accounting/openapi_bff.yaml',  # Don't watch the generated file (clobbered each run)
    ],
    resource_deps=[],  # No dependencies - runs when any microservice spec changes
    labels=['microservices-codegen'],
    allow_parallel=True,
)

# ====================
# Example: Additional Services
# ====================
# Uncomment as services are bootstrapped:
# create_microservice_lint('invoice', 'accounting/invoice/openapi.yaml')
# create_microservice_gen('invoice', 'accounting/invoice/openapi.yaml', 'invoice')
# create_microservice_build_resource('invoice')
# create_microservice_deployment('invoice')
