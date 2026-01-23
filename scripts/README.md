# RERP Build Scripts

This directory contains build scripts for RERP microservices.

## Scripts

### `setup-kind-registry.sh`
Starts a local Docker registry on `localhost:5001` and connects it to the Kind network so that `docker push localhost:5001/<image>:<tag>` works and the cluster can pull those images. Run automatically by `just dev-up` after the Kind cluster is created. See [Kind: Local Registry](https://kind.sigs.k8s.io/docs/user/local-registry/).

**Standalone usage:** Run after `kind create cluster`; requires the `kind` Docker network to exist.

### `host-aware-build.py`
Host-aware build script that detects the current architecture and builds accordingly. Supports cross-compilation for multiple architectures.

**Why Python?**
- ✅ Better error handling and reporting
- ✅ Easier to test and maintain
- ✅ Consistent with other RERP scripts
- ✅ Better cross-platform support
- ✅ More maintainable for complex logic

**Usage:**
```bash
# Build for current architecture
python3 scripts/host-aware-build.py workspace
python3 scripts/host-aware-build.py auth_idam

# Build for specific architecture
python3 scripts/host-aware-build.py auth_idam amd64
python3 scripts/host-aware-build.py auth_idam arm64
python3 scripts/host-aware-build.py auth_idam arm7

# Build for all architectures
python3 scripts/host-aware-build.py auth_idam all
```

**Supported Architectures:**
- `amd64` - x86_64-unknown-linux-musl (Intel/AMD 64-bit)
- `arm64` - aarch64-unknown-linux-musl (Apple Silicon, ARM64)
- `arm7` - armv7-unknown-linux-musleabihf (Raspberry Pi, ARMv7)

### `build-multiarch-docker.sh`
Builds and pushes multi-architecture Docker images to Docker Hub.

**Usage:**
```bash
# Build multi-arch images (local only)
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest

# Build and push to Docker Hub
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

This script:
1. Builds binaries for all architectures (amd64, arm64, arm7)
2. Creates Docker images for each architecture
3. Creates a multi-architecture manifest
4. Optionally pushes to Docker Hub

### `copy-multiarch-binary.sh`
Copies built binaries for all architectures to `build_artifacts/`.

**Usage:**
```bash
# Copy all architectures
./scripts/copy-multiarch-binary.sh auth idam

# Copy specific architecture
./scripts/copy-multiarch-binary.sh auth idam amd64
```

### `copy-microservice-binary.sh`
Legacy single-architecture copy script (amd64 only). Use `copy-multiarch-binary.sh` for multi-arch support.

### `build-microservice-docker.sh`
Legacy single-architecture Docker build script. Use `build-multiarch-docker.sh` for multi-arch support.

### `generate-dockerfile.py`
Generates service-specific Dockerfiles from template.

**Usage:**
```bash
python3 scripts/generate-dockerfile.py <system> <module> [port]
```

## Multi-Architecture Build Process

### 1. Build Binaries

```bash
# Build for all architectures
python3 scripts/host-aware-build.py auth_idam all
```

This creates binaries in:
- `components/target/x86_64-unknown-linux-musl/release/rerp_auth_idam_impl`
- `components/target/aarch64-unknown-linux-musl/release/rerp_auth_idam_impl`
- `components/target/armv7-unknown-linux-musleabihf/release/rerp_auth_idam_impl`

### 2. Copy Binaries

```bash
./scripts/copy-multiarch-binary.sh auth idam
```

This copies binaries to:
- `build_artifacts/auth_idam/amd64/rerp_auth_idam_impl`
- `build_artifacts/auth_idam/arm64/rerp_auth_idam_impl`
- `build_artifacts/auth_idam/arm7/rerp_auth_idam_impl`

### 3. Build Docker Images

```bash
# Build and push to Docker Hub
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

This creates:
- `rerp/auth-idam:latest-amd64`
- `rerp/auth-idam:latest-arm64`
- `rerp/auth-idam:latest-arm7`
- `rerp/auth-idam:latest` (multi-arch manifest)

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
docker buildx create --use
docker buildx inspect --bootstrap
```

## Docker Hub Configuration

To push to Docker Hub, ensure you're logged in:
```bash
docker login
```

Then use the `push` argument:
```bash
./scripts/build-multiarch-docker.sh auth idam rerp/auth-idam latest push
```

## Architecture Support

| Architecture | Rust Target | Platform | Use Case |
|-------------|-------------|----------|----------|
| amd64 | x86_64-unknown-linux-musl | linux/amd64 | Cloud servers, Intel/AMD |
| arm64 | aarch64-unknown-linux-musl | linux/arm64 | Apple Silicon, ARM64 servers |
| arm7 | armv7-unknown-linux-musleabihf | linux/arm/v7 | Raspberry Pi clusters |
