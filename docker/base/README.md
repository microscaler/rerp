# RERP Base Docker Image

This directory contains the base Docker image for all RERP microservices.

## Base Image

The base image (`rerp/base:latest`) provides:
- Minimal Alpine Linux 3.19 runtime
- Runtime dependencies (ca-certificates, libgcc, tzdata)
- Standard directory structure (`/app/config`, `/app/doc`, `/app/static_site`)
- Proper permissions for live updates in Tilt

## Building the Base Image

```bash
docker build -t rerp/base:latest -f docker/base/Dockerfile .
```

## Usage

Service-specific Dockerfiles inherit from this base image:

```dockerfile
FROM rerp/base:latest
# ... service-specific configuration
```

This ensures all services have a consistent runtime environment while keeping image sizes minimal.
