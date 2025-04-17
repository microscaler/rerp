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

# Build base runtime image for all microservices
docker_build(
    'rerp/base',
    '.',
    dockerfile='./docker/base/Dockerfile',
    platform='linux/amd64',
    only=[
        './docker/base',
    ],
)

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
# Example: Build Core Services
# ====================
# Uncomment and configure services as needed

# Phase 1: Core Foundation
# create_microservice_build('auth', 'idam', 8000)
# create_microservice_build('auth', 'rbac', 8001)
# create_microservice_build('infrastructure', 'gateway', 8002)
# create_microservice_build('product', 'catalog', 8003)

# Phase 2: Business Operations
# create_microservice_build('crm', 'core', 8010)
# create_microservice_build('sales', 'core', 8011)

# Add more services as needed...

# ====================
# Kubernetes Resources
# ====================

# Load Kubernetes manifests
# k8s_yaml(kustomize('./k8s'))

# Example: Deploy a service
# k8s_resource(
#     'rerp-auth-idam',
#     port_forwards='8000:8000',
#     labels=['microservices'],
# )
