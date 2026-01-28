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

The base image (`ghcr.io/microscaler/rerp-base:latest`) provides a minimal runtime environment:
- Alpine Linux 3.19
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure
- Proper permissions for Tilt live updates

The base image is built and pushed by CI (`.github/workflows/base-images.yml`). Locally, build and tag it with:
```bash
rerp docker build-base
```
This tags the image with all three names:
- **Local**: `rerp-base:latest`
- **GHCR**: `ghcr.io/<owner>/rerp-base:latest` (owner from `GHCR_OWNER` or `GITHUB_REPOSITORY_OWNER`, default `microscaler`)
- **Kind registry**: `localhost:5001/rerp-base:latest` (for Tilt / local Kind cluster)

If `DOCKERHUB_ORG` (or `DOCKERHUB_OWNER`) is set, it also tags `docker.io/<org>/rerp-base:latest`. Use `rerp docker build-base --push` to push to remote registries (requires login). Service builds pull or use the GHCR image when available.

## Service Images

Service-specific Dockerfiles are generated from the template using:
```bash
rerp docker generate-dockerfile <system> <module> [--port N]
```

Example:
```bash
rerp docker generate-dockerfile auth idam --port 8000
```

This generates `docker/microservices/Dockerfile.auth_idam`. Run from repo root after `just init`.

## Build Process

**Tilt flow (single-arch, local/Kind):** Tilt runs `rerp docker copy-binary` and `rerp docker build-image-simple`; see `Tiltfile` and `tooling/README.md`.

**Manual / multi-arch (components, e.g. auth_idam):**
1. **Build Rust binaries**: `tooling/.venv/bin/rerp build <system>_<module> [arch]` (run `just init` first). Use `all` for amd64+arm64+arm7.
2. **Copy binaries**: `tooling/.venv/bin/rerp docker copy-multiarch <system> <module> [arch]` (arch: amd64, arm64, arm7, or all).
3. **Build Docker image**: `tooling/.venv/bin/rerp docker build-multiarch <system> <module> <image_name> [--tag N] [--push]`.

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
