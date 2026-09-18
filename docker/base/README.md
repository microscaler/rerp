# RERP Base Docker Image

This directory contains the base Docker image for all RERP microservices.

## Base Image

The base image (`rerp-base:latest`) provides:
- Minimal Alpine Linux runtime
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure (`/app/config`, `/app/doc`, `/app/static_site`)
- Proper permissions for live updates in Tilt

## Building the Base Image

```bash
rerp docker build-base
```

## Usage

The single parameterized service Dockerfile inherits from this base image:

```dockerfile
ARG BASE_IMAGE=rerp-base:latest
FROM ${BASE_IMAGE}
```

This ensures all services have a consistent runtime environment while keeping image sizes minimal.
