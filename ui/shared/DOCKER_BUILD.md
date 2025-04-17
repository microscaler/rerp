# Docker Build Configuration for Shared Components

## Overview

The shared components in `ui/shared/` are used by both the website and FTE applications. This document explains how the Docker build process handles shared components.

## Path Resolution

The `@shared` alias in `vite.config.ts` resolves to:
```typescript
'@shared': path.resolve(__dirname, '../shared')
```

This works in both contexts:
- **Local Development**: `ui/website/../shared` = `ui/shared/` ✅
- **Docker Build**: `/build/../shared` = `/shared/` ✅

## Dockerfile Configuration

The Dockerfile copies the shared directory to match the path resolution:

```dockerfile
# Copy shared components to parent directory (matches path resolution)
# This allows @shared alias to resolve correctly
COPY ui/shared/ ../shared/

# Copy source files
COPY ui/website/ ./
```

This ensures that when Vite resolves `@shared` from `/build/`, it correctly finds `/shared/`.

## Tiltfile Configuration

The Tiltfile watches both `ui/website/` and `ui/shared/` for changes:

```python
docker_build(
    'pricewhisperer/website',
    '.',
    dockerfile='./docker/website/Dockerfile',
    platform='linux/amd64',
    only=[
        './ui/website',
        './ui/shared',  # Watch shared components
        './docker/website',
    ],
    ...
)
```

## Troubleshooting

If you see errors like:
```
Could not load /shared/header/Header (imported by src/App.tsx): ENOENT: no such file or directory
```

Check:
1. ✅ Dockerfile copies `ui/shared/` to `../shared/`
2. ✅ Tiltfile includes `./ui/shared` in the `only` list
3. ✅ `vite.config.ts` has the `@shared` alias configured
4. ✅ `tsconfig.json` has the `@shared` path mapping

## Testing

To test the Docker build locally:
```bash
docker build -f docker/website/Dockerfile -t test-website-build .
```

The build should complete successfully with:
```
✓ 215 modules transformed.
✓ built in X.XXs
```

