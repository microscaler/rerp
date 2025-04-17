# RERP Website Docker Configuration

This directory contains Docker configuration for building and serving the RERP brochure website.

## Files

- **Dockerfile** - Multi-stage build for the SolidJS website
- **nginx.conf** - Nginx configuration for serving the built site

## Dockerfile

The Dockerfile uses a multi-stage build:

1. **Builder stage** (`node:20-alpine`):
   - Installs Yarn v1.22.22
   - Copies package files and installs dependencies
   - Copies shared components (`ui/shared/`)
   - Copies website source (`ui/website/`)
   - Builds the SolidJS application

2. **Runtime stage** (`nginx:alpine`):
   - Copies built application from builder
   - Copies nginx configuration
   - Serves on port 8080 (non-root port)

## Building the Website Image

```bash
# Build from RERP root directory
docker build -t rerp/website:latest -f docker/website/Dockerfile .
```

## Nginx Configuration

The nginx configuration provides:
- **SPA routing support** - All routes serve `index.html` for client-side routing
- **Gzip compression** - Reduces transfer size for text assets
- **Static asset caching** - 1 year cache for images, CSS, JS, fonts
- **Security headers** - X-Frame-Options, X-Content-Type-Options, etc.
- **404 handling** - Redirects 404s to index.html for SPA routing

## Running the Website

```bash
# Run the website container
docker run -p 8080:8080 rerp/website:latest

# Access at http://localhost:8080
```

## Development

For local development, use the SolidJS dev server:

```bash
cd ui/website
yarn install
yarn dev
```

The Docker image is primarily for production builds and deployment.
