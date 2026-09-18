# RERP Docker Images

This directory contains Docker configuration for RERP microservices.

## Structure

```
docker/
├── base/                    # Base runtime image for all services
│   ├── Dockerfile          # Base Alpine-based image
│   └── README.md
├── microservices/
│   └── Dockerfile          # One parameterized runtime image definition
└── website/                # Website Docker configuration
    ├── Dockerfile          # Multi-stage build for SolidJS website
    └── nginx.conf         # Nginx configuration for serving website
```

## Base Image

The base image (`rerp-base:latest`) provides a minimal runtime environment:
- Alpine Linux 3.19
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure
- Proper permissions for Tilt live updates

Build the base image:
```bash
rerp docker build-base
```

## Service Images

Every suite uses `docker/microservices/Dockerfile`. Suite, service and artifact
identity are supplied as build arguments by `rerp docker build-image-simple`.
The builder creates a temporary context containing exactly one verified binary,
its config, OpenAPI document and static documentation. It does not send the RERP
repository or binaries belonging to other suites to Docker.

For releases, `rerp docker stage-image-context` creates the equivalent narrow
context with verified `amd64`, `arm64`, and `arm` binaries. BuildKit supplies
`TARGETARCH`, so the same Dockerfile selects the correct `/app/service` binary.

## Build Process

**Tilt flow:** Tilt builds the debug microservice, copies and hashes the exact
binary, invokes one custom service-image build, pushes Tilt's immutable image
reference to the shared registry, and deploys it with Helm.

**Manual / multi-arch (components, e.g. auth_idam):**
1. **Build Rust binaries**: `tooling/.venv/bin/rerp build <system>_<module> [arch]` (run `just init` first). Use `all` for amd64+arm64+arm7.
2. **Copy binaries**: `tooling/.venv/bin/rerp docker copy-multiarch <system> <module> [arch]` (arch: amd64, arm64, arm7, or all).
3. **Build Docker image**: `tooling/.venv/bin/rerp docker build-multiarch <system> <module> <image_name> [--tag N] [--push]`.

The build process:
- Cross-compiles Rust binaries to `x86_64-unknown-linux-musl`
- Copies binaries to `build_artifacts/`
- Builds service images from the single parameterized Dockerfile
- Pushes Tilt image references to the shared registry

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
- Builds and pushes Docker images
- Deploys to Kubernetes

See `Tiltfile` for the complete orchestration.
