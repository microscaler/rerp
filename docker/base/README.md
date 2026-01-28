# RERP Base Docker Image

This directory contains the base Docker image for all RERP microservices.

## Base Image

The base image (`ghcr.io/microscaler/rerp-base:latest`) provides:
- Minimal Alpine Linux 3.19 runtime
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure (`/app/config`, `/app/doc`, `/app/static_site`)
- Proper permissions for live updates in Tilt

## Building the Base Image

CI builds and pushes the image to GHCR (`.github/workflows/base-images.yml`). Locally:
```bash
rerp docker build-base
```
This builds from `docker/base/Dockerfile` and tags both `rerp-base:latest` and `ghcr.io/microscaler/rerp-base:latest`.

## Usage

Service Dockerfiles use the base image via build-arg (see `docker/microservices/Dockerfile.template`):

```dockerfile
ARG BASE_IMAGE=ghcr.io/microscaler/rerp-base:latest
FROM ${BASE_IMAGE}
# ... service-specific configuration
```

This ensures all services have a consistent runtime environment while keeping image sizes minimal.
