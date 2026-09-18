# RERP Multi-Architecture Build System

RERP supports building and deploying microservices for multiple architectures:
- **AMD64** (x86_64) - Intel/AMD 64-bit servers
- **ARM64** (aarch64) - Apple Silicon, ARM64 servers
- **ARM7** (armv7) - Raspberry Pi clusters

## Quick Start

### Build for All Architectures

```bash
# Build workspace for all architectures
python3 scripts/host-aware-build.py workspace all

# Build specific service for all architectures
python3 scripts/host-aware-build.py auth_idam all
```

### Build and Push Multi-Arch Docker Images

```bash
# Build and push to Docker Hub
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

This creates:
- `rerp/auth-idam:latest-amd64`
- `rerp/auth-idam:latest-arm64`
- `rerp/auth-idam:latest-arm7`
- `rerp/auth-idam:latest` (multi-arch manifest)

## Architecture Support

| Architecture | Rust Target | Platform | Use Case |
|-------------|-------------|----------|----------|
| amd64 | x86_64-unknown-linux-musl | linux/amd64 | Cloud servers, Intel/AMD |
| arm64 | aarch64-unknown-linux-musl | linux/arm64 | Apple Silicon, ARM64 servers |
| arm7 | armv7-unknown-linux-musleabihf | linux/arm/v7 | Raspberry Pi clusters |

## Build Scripts

### `host-aware-build.py` ⭐ (Recommended)

Host-aware build script that detects architecture and builds accordingly. Python version with better error handling and maintainability.

**Usage:**
```bash
# Build for current architecture (host-aware)
python3 scripts/host-aware-build.py workspace
python3 scripts/host-aware-build.py auth_idam

# Build for specific architecture
python3 scripts/host-aware-build.py auth_idam amd64
python3 scripts/host-aware-build.py auth_idam arm64
python3 scripts/host-aware-build.py auth_idam arm7

# Build for all architectures
python3 scripts/host-aware-build.py auth_idam all
```


**Features:**
- Auto-detects host architecture
- Supports cross-compilation via `cargo-zigbuild` (macOS) or native toolchains (Linux)
- Installs Rust targets automatically
- Builds workspace or individual services

### `build-multiarch-docker.sh`

Builds and pushes multi-architecture Docker images.

**Usage:**
```bash
# Build multi-arch images (local only)
./scripts/build-multiarch-docker.sh <system> <module> <image_name> <tag>

# Build and push to Docker Hub
./scripts/build-multiarch-docker.sh <system> <module> <image_name> <tag> push
```

**Example:**
```bash
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

**Process:**
1. Builds binaries for all architectures (amd64, arm64, arm7)
2. Copies binaries to `build_artifacts/<system>_<module>/<arch>/`
3. Builds base images for each architecture
4. Creates Docker images for each architecture
5. Creates multi-architecture manifest
6. Optionally pushes to Docker Hub

### `copy-multiarch-binary.sh`

Copies built binaries for all architectures to `build_artifacts/`.

**Usage:**
```bash
# Copy all architectures
./scripts/copy-multiarch-binary.sh auth idam

# Copy specific architecture
./scripts/copy-multiarch-binary.sh auth idam amd64
```

## Prerequisites

### Cross-Compilation Tools

**For macOS:**
```bash
# Install cargo-zigbuild for cross-compilation
cargo install cargo-zigbuild
```

**For Linux:**
```bash
# Install musl cross-compilers
# For ARM64
sudo apt-get install gcc-aarch64-linux-gnu
# For ARM7
sudo apt-get install gcc-arm-linux-gnueabihf
```

### Docker Buildx

Enable Docker Buildx for multi-architecture builds:
```bash
# Create and use buildx builder
docker buildx create --name multiarch --use
docker buildx inspect --bootstrap

# Verify support
docker buildx ls
```

### Rust Targets

The build script automatically installs Rust targets, but you can install them manually:
```bash
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add armv7-unknown-linux-musleabihf
```

## Docker Hub Configuration

### Login

```bash
docker login
```

### Push Images

```bash
# Build and push to Docker Hub
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

### Image Naming Convention

- Individual architecture images: `<image>:<tag>-<arch>`
- Multi-arch manifest: `<image>:<tag>`

Example:
- `rerp/auth-idam:latest-amd64`
- `rerp/auth-idam:latest-arm64`
- `rerp/auth-idam:latest-arm7`
- `rerp/auth-idam:latest` (manifest)

## Build Process

### 1. Build Binaries

```bash
# Build for all architectures
python3 scripts/host-aware-build.py auth_idam all
```

Creates binaries in:
- `components/target/x86_64-unknown-linux-musl/release/rerp_auth_idam_impl`
- `components/target/aarch64-unknown-linux-musl/release/rerp_auth_idam_impl`
- `components/target/armv7-unknown-linux-musleabihf/release/rerp_auth_idam_impl`

### 2. Copy Binaries

```bash
./scripts/copy-multiarch-binary.sh auth idam
```

Copies to:
- `build_artifacts/auth_idam/amd64/rerp_auth_idam_impl`
- `build_artifacts/auth_idam/arm64/rerp_auth_idam_impl`
- `build_artifacts/auth_idam/arm7/rerp_auth_idam_impl`

### 3. Build Docker Images

```bash
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

Creates and pushes:
- Architecture-specific images
- Multi-architecture manifest

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Build and push multi-arch images
  run: |
    docker buildx create --use
    ./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam ${{ github.sha }} push
```

## Troubleshooting

### Cross-Compilation Fails

**macOS:**
- Ensure `cargo-zigbuild` is installed: `cargo install cargo-zigbuild`
- Verify Rust targets: `rustup target list --installed`

**Linux:**
- Install cross-compilers: `sudo apt-get install gcc-aarch64-linux-gnu gcc-arm-linux-gnueabihf`
- Set up environment variables (handled by script)

### Docker Buildx Issues

```bash
# Reset buildx
docker buildx rm multiarch
docker buildx create --name multiarch --use
docker buildx inspect --bootstrap
```

### Missing Binaries

Ensure binaries are built before copying:
```bash
python3 scripts/host-aware-build.py auth_idam all
./scripts/copy-multiarch-binary.sh auth idam
```

## Best Practices

1. **Always build for all architectures** when preparing releases
2. **Use semantic versioning** for Docker image tags
3. **Test on target architectures** before deploying
4. **Use multi-arch manifests** for simplified deployment
5. **Cache base images** to speed up builds

## References

- [Docker Multi-Architecture Images](https://docs.docker.com/build/building/multi-platform/)
- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)
- [cargo-zigbuild](https://github.com/messense/cargo-zigbuild)
