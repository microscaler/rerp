# RERP Docker Images

This directory contains Docker configuration for RERP microservices.

## Structure

```
docker/
├── base/                    # Base runtime image for all services
│   ├── Dockerfile          # Base Alpine-based image
│   └── README.md
├── microservices/          # Service-specific Dockerfiles
│   ├── Dockerfile.template # Template for generating service Dockerfiles
│   └── Dockerfile.*        # Generated service-specific Dockerfiles
└── website/                # Website Docker configuration
    ├── Dockerfile          # Multi-stage build for SolidJS website
    └── nginx.conf         # Nginx configuration for serving website
```

## Base Image

The base image (`rerp/base:latest`) provides a minimal runtime environment:
- Alpine Linux 3.19
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure
- Proper permissions for Tilt live updates

Build the base image:
```bash
docker build -t rerp/base:latest -f docker/base/Dockerfile .
```

## Service Images

Service-specific Dockerfiles are generated from the template using:
```bash
python3 scripts/generate-dockerfile.py <system> <module> [port]
```

Example:
```bash
python3 scripts/generate-dockerfile.py auth idam 8000
```

This generates `docker/microservices/Dockerfile.auth_idam`.

## Build Process

1. **Build Rust binaries**: `python3 scripts/host-aware-build.py <system>_<module> [architecture]`
2. **Copy binaries**: `scripts/copy-multiarch-binary.sh <system> <module>` (for multi-arch) or `scripts/copy-microservice-binary.sh <system> <module>` (amd64 only)
3. **Build Docker image**: `scripts/build-multiarch-docker.sh <system> <module> <image_name> [tag] [push]` (for multi-arch) or `scripts/build-microservice-docker.sh <system> <module> <image_name> [port]` (amd64 only)

The build process:
- Cross-compiles Rust binaries to `x86_64-unknown-linux-musl`
- Copies binaries to `build_artifacts/`
- Generates service-specific Dockerfiles from template
- Builds Docker images using the base image
- Pushes images to the local registry for Tilt

## Website Docker Image

The website Docker image builds and serves the RERP brochure site:

```bash
# Build website image
docker build -t rerp/website:latest -f docker/website/Dockerfile .
```

This uses a multi-stage build:
1. **Builder stage**: Builds the SolidJS application using Node.js
2. **Runtime stage**: Serves the built site using nginx

The nginx configuration:
- Supports SPA routing (all routes serve index.html)
- Enables gzip compression
- Sets appropriate cache headers
- Includes security headers

## Tilt Integration

Tilt orchestrates the entire build process:
- Watches source files for changes
- Rebuilds binaries when code changes
- Regenerates Dockerfiles when needed
- Builds and pushes Docker images
- Deploys to Kubernetes

See `Tiltfile` for the complete orchestration.
