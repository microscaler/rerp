# Hauliage Tiltfile Architecture — Complete Technical Reference

> Purpose: Full extraction and documentation of every function, resource, and pattern in Hauliage's Tiltfile, justfile, and Docker setup — so it can be replicated for RERP's accounting suite.

> Historical reference: RERP adopted Hauliage's build-chain semantics, not its
> flat service inventory or rendered Dockerfiles. The authoritative RERP path is
> the descriptor-driven `Tiltfile`, `rerp_tooling.runtime`, and the single
> `docker/microservices/Dockerfile`.

---

## 1. Python Helper Functions (Full Source Code)

### 1.1 `create_microservice_lint(name, spec_file)`

```python
def create_microservice_lint(name, spec_file):
    local_resource(
        '%s-lint' % name,
        cmd='''
            set -e
            echo "Linting %s OpenAPI spec..."
            # Use the built debug binary directly for speed
            %s/target/debug/brrtrouter-gen lint \
                --spec ./openapi/%s \
                --fail-on-error || \
            cargo run --manifest-path %s/Cargo.toml --bin brrtrouter-gen -- \
                lint \
                --spec ./openapi/%s \
                --fail-on-error
            echo "%s OpenAPI spec linting passed"
        ''' % (name, brrtrouter_root, spec_file, brrtrouter_root, spec_file, name),
        deps=[
            './openapi/%s' % spec_file,
        ],
        resource_deps=[],
        labels=[name],
        allow_parallel=True,
    )
```

**Purpose:** Creates a `local_resource` that lints a microservice's OpenAPI spec using `brrtrouter-gen`.
- Strategy: First tries pre-built debug binary (`$brrtrouter_root/target/debug/brrtrouter-gen lint`), falls back to `cargo run` if binary is not found.
- Dependencies: Only the OpenAPI spec file.
- Labels: Service name (e.g., `identity`, `fleet`).
- Allows parallel execution.

### 1.2 `create_microservice_gen(name, spec_file, output_dir)`

```python
def create_microservice_gen(name, spec_file, output_dir):
    local_resource(
        '%s-service-gen' % name,
        cmd='%s gen suite hauliage --service %s' % (hauliage_bin, name),
        deps=[
            './openapi/%s' % spec_file,
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/%s/gen/src' % output_dir,
            './microservices/%s/gen/doc' % output_dir,
            './microservices/%s/impl/config' % output_dir,
            './microservices/%s/gen/static_site' % output_dir,
        ],
        resource_deps=['%s-lint' % name],
        labels=[name],
        allow_parallel=True,
    )
```

**Purpose:** Code generation resource that delegates to `hauliage gen suite hauliage --service <name>`.
- `deps` tracks OpenAPI spec and `tooling/pyproject.toml` (tooling changes trigger regeneration).
- `ignore` prevents watch loops on generated directories (`gen/src`, `gen/doc`, `impl/config`, `gen/static_site`).
- `resource_deps=['<name>-lint']` creates a dependency chain: lint must pass before generation runs.

### 1.3 `create_microservice_build_resource(name)`

```python
def create_microservice_build_resource(name):
    # Build the service binary. hauliage build microservice maps name -> Cargo [package] and
    # emits to microservices/target/x86_64-unknown-linux-musl/debug/<package_name>.
    local_resource(
        'build-%s' % name,
        '%s build microservice %s' % (hauliage_bin, name),
        deps=[
            './microservices/%s/gen/Cargo.toml' % name,
            './microservices/%s/impl/Cargo.toml' % name,
            './microservices/%s/gen/src' % name,
            './microservices/%s/impl/src' % name,
            'tooling/pyproject.toml',
        ],
        ignore=[
            './microservices/target',
            './build_artifacts',
        ],
        resource_deps=['hauliage-all-gens'],
        labels=[name],
        allow_parallel=True,
    )
```

**Purpose:** Builds a single microservice binary via `hauliage build microservice <name>`.
- Depends on both `gen/` and `impl/` Cargo.toml and source dirs.
- `resource_deps=['hauliage-all-gens']` ensures all services' codegen is complete before any build (so Cargo workspace members are registered in `microservices/Cargo.toml`).
- Ignores `microservices/target` and `build_artifacts` to avoid build storms.

### 1.4 `create_microservice_deployment(name)`

```python
def create_microservice_deployment(name):
    package_name = PACKAGE_NAMES.get(name, 'hauliage_%s' % name.replace('-', '_'))
    binary_name = name.replace('-', '_')
    target_path = 'microservices/target/%s/debug/%s' % (TARGET_RUST_TRIPLE, package_name)
    artifact_path = 'build_artifacts/%s/%s' % (TARGET_ARCH_NAME, binary_name)
    dockerfile_template = 'docker/microservices/Dockerfile.template'
    image_name = 'localhost:5001/hauliage-%s' % name

    # 1. Copy binary from workspace build to artifacts and create SHA256 hash
    hash_path = 'build_artifacts/%s/%s.sha256' % (TARGET_ARCH_NAME, binary_name)
    local_resource(
        'copy-%s' % name,
        '%s docker copy-binary %s %s %s' % (hauliage_bin, target_path, artifact_path, binary_name),
        deps=[target_path, 'tooling/pyproject.toml'],
        resource_deps=['build-%s' % name],
        labels=[name],
        allow_parallel=True,
    )

    # 2. Build and push Docker image (template rendered on the fly with --service)
    local_resource(
        'docker-%s' % name,
        '%s docker build-image-simple %s %s %s %s --service %s' % (hauliage_bin, image_name, dockerfile_template, hash_path, artifact_path, name),
        deps=[hash_path, artifact_path, dockerfile_template, 'tooling/pyproject.toml'],
        resource_deps=['build-base-image', 'copy-%s' % name],
        labels=[name],
        allow_parallel=False,
    )

    # 3. Custom build for Tilt live updates
    custom_build(
        image_name,
        ('%s docker build-image-simple %s %s %s %s --service %s' % (hauliage_bin, image_name, dockerfile_template, hash_path, artifact_path, name)
         + ' && (docker push %s:tilt 2>/dev/null || kind load docker-image %s:tilt --name hauliage)' % (image_name, image_name)),
        deps=[artifact_path, hash_path, dockerfile_template, 'microservices/%s/impl/config' % name, 'microservices/%s/gen/doc' % name, 'microservices/%s/gen/static_site' % name],
        tag='tilt',
        live_update=[
            sync(artifact_path, '/app/%s' % binary_name),
            sync('microservices/%s/impl/config/' % name, '/app/config/'),
            sync('microservices/%s/gen/doc/' % name, '/app/doc/'),
            sync('microservices/%s/gen/static_site/' % name, '/app/static_site/'),
            run('kill -HUP 1', trigger=[artifact_path]),
        ],
    )

    # 4. Deploy using Helm (Postgres in data; Redis in data when not bundling hauliage redis.yaml)
    _helm_values = [
        './helm/hauliage-microservice/values/%s.yaml' % name,
        './helm/hauliage-microservice/values/_database-kubernetes.yaml',
    ]
    if use_shared_kind_infra:
        _helm_values.append('./helm/hauliage-microservice/values/_redis-shared-kind.yaml')
    k8s_yaml(helm('./helm/hauliage-microservice', name=name, namespace='hauliage', values=_helm_values))

    # 5. Kubernetes resource configuration
    k8s_resource(
        name,
        port_forwards=['%s:%s' % (get_service_port(name), get_service_port(name))],
        resource_deps=['hauliage-database-env', 'docker-%s' % name],
        labels=[name],
        auto_init=True,
        trigger_mode=TRIGGER_MODE_AUTO,
    )
```

**Purpose:** Creates the full deployment pipeline for a microservice with 5 Tilt resources + a `custom_build`:

- **Stage 1 — `copy-<name>`**: Copies the compiled binary from Cargo build target to `build_artifacts/` and generates a SHA256 hash. Depends on `build-<name>`.
- **Stage 2 — `docker-<name>`**: Builds the Docker image using the template and pushes to `localhost:5001`. Depends on `build-base-image` and `copy-<name>`. Not parallel (to avoid Docker build contention).
- **Stage 3 — `custom_build`**: Tilt `custom_build` for live updates. Rebuilds image and sets up live_update rules:
  - Sync binary to `/app/<binary_name>`
  - Sync config, doc, static_site from service dirs
  - On binary change, `kill -HUP 1` triggers graceful restart via dev-entrypoint.sh trap
- **Stage 4 — Helm deploy**: Renders Helm chart with per-service values + shared DB config + optional shared-Kind Redis config.
- **Stage 5 — `k8s_resource`**: Port forwards the service port, depends on database env and docker build.

### 1.5 `create_microservice_test_resource(name)`

```python
def create_microservice_test_resource(name):
    package_name = PACKAGE_NAMES.get(name, 'hauliage_%s' % name.replace('-', '_'))
    db_pass = os.environ.get('HAULIAGE_DB_PASSWORD', 'dev_password_change_in_prod')
    local_resource(
        'test-%s' % name,
        'cd microservices && cargo test -p %s' % package_name,
        env={'HAULIAGE_DB_PASSWORD': db_pass},
        deps=[
            './microservices/%s/impl/src' % name,
            './microservices/%s/impl/tests' % name,
            './microservices/%s/gen/src' % name,
            './microservices/Cargo.toml',
            'tooling/pyproject.toml',
        ],
        labels=[name],
        trigger_mode=TRIGGER_MODE_MANUAL,
        allow_parallel=True,
    )
```

**Purpose:** Test resource for a microservice. Runs `cargo test -p <package_name>` on the microservices workspace. Only triggered manually from the Tilt UI.

### 1.6 `create_microservice_stubs_resource(name, spec_file)`

```python
def create_microservice_stubs_resource(name, spec_file):
    local_resource(
        'stubs-%s' % name,
        '%s gen stubs hauliage %s --force' % (hauliage_bin, name),
        deps=[
            './openapi/%s' % spec_file,
            './microservices/%s/gen' % name,
            'tooling/pyproject.toml',
        ],
        resource_deps=[],
        labels=[name],
        allow_parallel=True,
        trigger_mode=TRIGGER_MODE_MANUAL,
    )
```

**Purpose:** Manual resource to regenerate impl stubs for a microservice. Run from Tilt UI: triggers `hauliage gen stubs hauliage <name> --force`.

### 1.7 `get_service_port(name)`

```python
def get_service_port(name):
    ports = {
        'analytics': '8007',
        'bff': '8010',
        'bidding': '8004',
        'company': '8009',
        'consignments': '8003',
        'fleet': '8002',
        'gics': '8016',
        'identity': '8001',
        'inbox': '8005',
        'locations': '8014',
        'marketing': '8011',
        'notifications': '8006',
        'reviews': '8013',
        'telemetry': '8008',
    }
    return ports.get(name, '8080')
```

**Purpose:** Returns the HTTP port for a service. Default fallback is `8080`.

---

## 2. PACKAGE_NAMES Map and Port Mappings

### 2.1 PACKAGE_NAMES

```python
PACKAGE_NAMES = {
    'identity': 'hauliage_identity',
    'company': 'hauliage_company',
    'locations': 'hauliage_locations',
    'customs': 'hauliage_customs',
    'fleet': 'hauliage_fleet',
    'consignments': 'hauliage_consignments',
    'bidding': 'hauliage_bidding',
    'inbox': 'hauliage_inbox',
    'notifications': 'hauliage_notifications',
    'analytics': 'hauliage_analytics',
    'telemetry': 'hauliage_telemetry',
    'marketing': 'hauliage_marketing',
    'reviews': 'hauliage_reviews',
    'gics': 'hauliage_gics',
    'bff': 'hauliage_bff',
}
```

**How it maps services to specs/ports:**
- Keys are service directory names under `openapi/<name>/openapi.yaml`
- Values are Cargo `[package].name` for `cargo build -p <name>`
- Ports are defined separately in `get_service_port()` — no automatic derivation from PACKAGE_NAMES
- The map is used in path construction (e.g., `microservices/target/<arch>/debug/<package_name>`)

### 2.2 Dynamic Architecture Selection

```python
host_machine = str(local('uname -m', quiet=True)).strip()
if host_machine in ['arm64', 'aarch64']:
    TARGET_ARCH_NAME = 'arm64'
    TARGET_RUST_TRIPLE = 'aarch64-unknown-linux-musl'
else:
    TARGET_ARCH_NAME = 'amd64'
    TARGET_RUST_TRIPLE = 'x86_64-unknown-linux-musl'
```

Binary paths use the correct triple: `microservices/target/<TARGET_RUST_TRIPLE>/debug/<package_name>`.

---

## 3. Shared Venv Management Pattern

### 3.1 Venv Configuration

```python
# Override via env var; default to ~/.local/share/brrtrouter/venv
_brrtrouter_venv_env = os.getenv('BRRTROUTER_VENV', '').strip().rstrip('/')
_home = os.getenv('HOME', '') or os.getenv('USERPROFILE', '')
brrtrouter_venv = _brrtrouter_venv_env if _brrtrouter_venv_env else (
    _home + '/.local/share/brrtrouter/venv' if _home else '.local/share/brrtrouter/venv'
)
hauliage_bin = '%s/bin/hauliage' % brrtrouter_venv
```

**Key design decisions:**
- Single shared venv for both `brrtrouter-tooling` and `hauliage-tooling`
- Override via `BRRTROUTER_VENV` env var
- Default path: `~/.local/share/brrtrouter/venv`

### 3.2 Tooling Ignore Patterns

```python
TOOLING_IGNORE = [
    '**/*.pyc', '**/*.pyo', '**/__pycache__', '**/.pytest_cache',
    '**/.coverage', '**/.coverage.*', '**/htmlcov', '**/coverage.xml',
    '**/.ruff_cache', '**/.mypy_cache', '**/*.egg', '**/*.egg-info',
    '**/*.egg-info/**', '**/brrtrouter_tooling.egg-info/**',
    '**/hauliage_tooling.egg-info/**', '**/*tooling.egg-info',
    '**/*tooling.egg-info/**', '**/.eggs', '**/dist',
    '**/build', '**/build/**', '**/.hypothesis', '**/.DS_Store',
]
```

**Why these patterns matter:**
- `ignore` prevents Tilt from watching generated/cache files that would trigger build storms
- `*.egg-info` and `*.egg-info/**` exclude pip editable install metadata — these are constantly refreshed by `pip install -e` and would cause infinite Tilt loops
- Editable installs pick up `.py` changes without re-running pip, so only `pyproject.toml` needs to be tracked as a dependency

### 3.3 build-tooling Resource

```python
local_resource(
    'build-tooling',
    '''set -e
VENV="%s"
BRRROOT="%s"
test -d "$VENV" || python3 -m venv "$VENV"
"$VENV/bin/pip" install -U pip
"$VENV/bin/pip" install -e "$BRRROOT/tooling[dev]"
cd tooling && "$VENV/bin/pip" install -e '.[dev]'
''' % (brrtrouter_venv, brrtrouter_root),
    deps=[
        './tooling/pyproject.toml',
        '%s/tooling/pyproject.toml' % brrtrouter_watch_root,
    ],
    ignore=TOOLING_IGNORE,
    labels=['tooling'],
    allow_parallel=True,
)
```

**Purpose:** Creates the venv and installs both tooling packages as editable.

**Why only `pyproject.toml` is in `deps`:**
- Editable installs already pick up `.py` edits without re-running pip
- Listing `tooling/src` as deps would cause Tilt loops because pip constantly refreshes `*.egg-info` inside `src/`

### 3.4 CLI Architecture

```python
# tooling/src/hauliage_tooling/cli/main.py
from brrtrouter_tooling.workspace.cli.main import main
```

The `hauliage` CLI is a thin shim that delegates to `brrtrouter_tooling.workspace.cli.main`. The `pyproject.toml` declares this dependency:

```toml
[project]
dependencies = ["brrtrouter-tooling @ file:../../BRRTRouter/tooling"]

[project.scripts]
hauliage = "hauliage_tooling.cli:main"
```

---

## 4. Three-Stage Pipeline (copy -> docker -> k8s) Per Service

The full pipeline with strict resource_deps ordering:

```
Spec File (openapi/<name>/openapi.yaml)
    |
    v
  lint-<name>          (brrtrouter-gen lint)
    |
    v
  <name>-service-gen   (hauliage gen suite hauliage --service <name>)
    |
    v
  hauliage-all-gens    (synchronization point — all gens must complete)
    |
    v
  build-<name>         (hauliage build microservice <name> -> Cargo)
    |
    v
  copy-<name>          (copy binary + SHA256 to build_artifacts/)
    |
    v
  docker-<name>        (build Docker image, push to localhost:5001)
    |
    v
  custom_build         (Tilt image + live_update rules)
    |
    v
  Helm deploy + k8s_resource (kubectl apply via Helm, port forward)
```

### Synchronization Resource

```python
local_resource(
    'hauliage-all-gens',
    'echo "All hauliage codegen complete"',
    resource_deps=['%s-service-gen' % name for name in HAULIAGE_SERVICES] + ['bff-service-gen'],
    labels=['all_gens'],
    allow_parallel=False,
)
```

### Service Registration

```python
HAULIAGE_SERVICES = [
    'identity', 'company', 'fleet', 'consignments', 'bidding',
    'inbox', 'notifications', 'analytics', 'telemetry', 'marketing',
    'reviews', 'locations', 'customs', 'gics',
]

for name in HAULIAGE_SERVICES:
    create_microservice_lint(name, '%s/openapi.yaml' % name)
    create_microservice_gen(name, '%s/openapi.yaml' % name, name)
    create_microservice_stubs_resource(name, '%s/openapi.yaml' % name)

for name in HAULIAGE_SERVICES:
    create_microservice_build_resource(name)
    create_microservice_deployment(name)
```

---

## 5. Live Update Configuration

### 5.1 Live Update Rules

```python
live_update=[
    sync(artifact_path, '/app/%s' % binary_name),
    sync('microservices/%s/impl/config/' % name, '/app/config/'),
    sync('microservices/%s/gen/doc/' % name, '/app/doc/'),
    sync('microservices/%s/gen/static_site/' % name, '/app/static_site/'),
    run('kill -HUP 1', trigger=[artifact_path]),
]
```

### 5.2 dev-entrypoint.sh — Hot-Reload Wrapper

```sh
#!/bin/sh
# dev-entrypoint.sh
#
# A lightweight wrapper script executing the target application.
# For local Tilt development, the binary might not be present immediately when the
# container starts (as it is copied asynchronously via live_update sync).
# This script stalls loop execution safely instead of allowing Kubernetes
# to trigger CrashLoopBackOff instantly.

if [ "$#" -lt 1 ]; then
    echo "Usage: $0 <binary_path> [args...]"
    exit 1
fi

BINARY="$1"
PID=""

# Trap SIGHUP from Tilt (kill -HUP 1) to restart the child process gracefully
trap 'echo "Received SIGHUP from Tilt. Hot-reloading..."; [ -n "$PID" ] && kill -TERM "$PID"' HUP

while true; do
    if [ ! -x "$BINARY" ]; then
        echo "Waiting for binary $BINARY to be synced by Tilt..."
        while [ ! -x "$BINARY" ]; do
            sleep 0.5
        done
        echo "Binary $BINARY found!"
    fi

    echo "Booting $BINARY..."
    "$@" &
    PID=$!

    # Wait for child process; wait is interrupted when trap executes
    wait $PID
    EXIT_CODE=$?

    echo "Process exited (code $EXIT_CODE). Restarting in 1s..."
    sleep 1
done
```

**Key features:**
- Waits for binary to be synced before starting (prevents CrashLoopBackOff)
- Traps SIGHUP to gracefully restart child process (hot reload without container restart)
- Loops: if binary missing -> sleep 0.5 -> retry; if SIGHUP received -> terminate child and restart; if process exits -> sleep 1 and restart

### 5.3 custom_build Image Build Command

```python
custom_build(
    image_name,
    ('%s docker build-image-simple %s %s %s %s --service %s' % (hauliage_bin, image_name, dockerfile_template, hash_path, artifact_path, name)
     + ' && (docker push %s:tilt 2>/dev/null || kind load docker-image %s:tilt --name hauliage)' % (image_name, image_name)),
    deps=[...],
    tag='tilt',
    live_update=[...],
)
```

**Fallback strategy:** After building the image, tries to push to `localhost:5001`; if registry is not running, falls back to `kind load docker-image` (no registry needed).

---

## 6. Dual Infra Modes (Shared vs Bundled)

### 6.1 Environment Variable Resolution

```python
_explicit = os.environ.get('TILT_USE_SHARED_KIND_INFRA', '').strip().lower()
_bundled_explicit = os.environ.get('TILT_USE_BUNDLED_DATA_STACK', '').strip().lower()
if _explicit in ('0', 'false', 'no'):
    use_shared_kind_infra = False
elif _explicit in ('1', 'true', 'yes'):
    use_shared_kind_infra = True
elif _bundled_explicit in ('1', 'true', 'yes'):
    use_shared_kind_infra = False
elif _bundled_explicit in ('0', 'false', 'no'):
    use_shared_kind_infra = True
else:
    use_shared_kind_infra = True  # DEFAULT
bundled_data_stack = not use_shared_kind_infra
```

### 6.2 Shared Kind Mode (DEFAULT)

Postgres, Redis, and other data services run in the sibling repo `shared-kind-cluster`'s Tilt environment (context `kind-kind`, namespace `data`). Hauliage's Tilt does NOT apply those manifests.

**Behavior:**
- `k8s_yaml(kustomize('k8s/data/supabase'))` — SKIPPED
- `k8s_yaml('k8s/data/redis.yaml')` — SKIPPED
- `k8s_yaml('k8s/microservices/data-stack-aliases.yaml')` — APPLIED (Mosquitto cross-namespace alias)
- `k8s_yaml('k8s/data/mosquitto.yaml')` — ALWAYS APPLIED (Hauliage-owned, not from shared-kind)
- Helm: adds `_redis-shared-kind.yaml` values for `redis.data.svc.cluster.local`

### 6.3 Bundled Mode (Offline/Isolated)

All data infrastructure is applied from Hauliage's own kustomize manifests.

**Behavior:**
- `k8s_yaml(kustomize('k8s/data/supabase'))` — APPLIED (Postgres in namespace `data`)
- `k8s_yaml('k8s/data/redis.yaml')` — APPLIED (Redis in namespace `hauliage`)
- `k8s_yaml('k8s/data/mosquitto.yaml')` — APPLIED
- No shared-Kind Redis values file used
- `resource_deps=['postgres']` on DB init

### 6.4 Justfile Infrastructure Commands

```makefile
# Shared Kind (default)
dev-up:
    just dev-registry
    # Create Kind cluster + connect registry
    kubectl apply -f ${SHARED_KIND_CLUSTER_ROOT}/k8s/platform-namespaces.yaml
    tilt up --port 10352 --host 0.0.0.0

# Bundled (offline)
dev-up-bundled-infra:
    TILT_USE_SHARED_KIND_INFRA=0 just dev-up

dev-up-shared-infra:
    TILT_USE_SHARED_KIND_INFRA=1 just dev-up

dev-down:
    tilt down || true
    kind delete cluster --name kind
```

---

## 7. Migration Tilt Resources

### 7.1 Database Init (Manual)

```python
local_resource(
    'hauliage-db-init',
    'chmod +x ./scripts/setup-db.sh && ./scripts/setup-db.sh',
    deps=['./scripts/setup-db.sh'],
    resource_deps=_db_init_deps,  # ['postgres'] if bundled, else []
    labels=['data'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
)
```

Runs `setup-db.sh` which waits for PostgreSQL to be ready and bootstraps the database/schema.

### 7.2 Migrate (Manual)

```python
local_resource(
    'hauliage-migrate',
    'cd microservices && cargo run -p hauliage_migrator',
    deps=[
        './microservices/migrator',
        './microservices/consignments/impl/src/models',
        './microservices/fleet/impl/src/models',
        './microservices/telemetry/impl/src/models',
        './microservices/marketing/impl/src/models',
        './microservices/Cargo.toml',
    ],
    ignore=['./microservices/target'],
    labels=['data'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)
```

Regenerates SQL migration files from Lifeguard entity registries in each service's `models/` directory. Does NOT connect to PostgreSQL — only writes files. Run `hauliage-apply-migrations` after.

### 7.3 Apply Migrations (Manual)

```python
local_resource(
    'hauliage-apply-migrations',
    'HAULIAGE_APPLY_MIGRATIONS_ONLY=1 chmod +x ./scripts/setup-db.sh && ./scripts/setup-db.sh',
    deps=['./scripts/setup-db.sh', './migrations/apply_order.txt', './microservices/company/impl/seeds', './microservices/gics/impl/seeds'],
    resource_deps=_apply_migrations_deps,
    labels=['data'],
    trigger_mode=TRIGGER_MODE_MANUAL,
    auto_init=False,
    allow_parallel=True,
)
```

Applies `migrations/*.sql` in `apply_order.txt` sequence via `kubectl exec` into the Postgres deployment.

---

## 8. BFF Pipeline

### 8.1 BFF Spec Generation (Automatic)

```python
local_resource(
    'bff-spec-gen',
    cmd='''
        set -e
        echo "Regenerating Hauliage BFF OpenAPI spec..."
        %s bff generate-system
        echo "Hauliage BFF spec regeneration complete"
    ''' % (hauliage_bin,),
    deps=[
        './openapi/bff-suite-config.yaml',
    ] + ['./openapi/%s/openapi.yaml' % name for name in HAULIAGE_SERVICES],
    ignore=[
        './openapi/openapi_bff.yaml',  # Don't watch the generated file
    ],
    resource_deps=[],
    labels=['bff'],
    allow_parallel=True,
)
```

Aggregates all 14 hauliage microservice OpenAPI specs into `openapi/openapi_bff.yaml`.

### 8.2 BFF Lint

```python
local_resource(
    'bff-lint',
    cmd='''
        set -e
        echo "Linting BFF OpenAPI spec..."
        %s/target/debug/brrtrouter-gen lint \
            --spec ./openapi/openapi_bff.yaml \
            --fail-on-error || \
        cargo run --manifest-path %s/Cargo.toml --bin brrtrouter-gen -- \
            lint \
            --spec ./openapi/openapi_bff.yaml \
            --fail-on-error
        echo "BFF OpenAPI spec linting passed"
    ''' % (brrtrouter_root, brrtrouter_root),
    deps=['./openapi/openapi_bff.yaml'],
    resource_deps=['bff-spec-gen'],
    labels=['bff'],
    allow_parallel=True,
)
```

### 8.3 BFF Service Pipeline

The BFF follows the same pipeline as regular services but with a flat OpenAPI path:

```python
create_microservice_gen('bff', 'openapi_bff.yaml', 'bff')
create_microservice_stubs_resource('bff', 'openapi_bff.yaml')
create_microservice_build_resource('bff')
create_microservice_deployment('bff')
```

Port: `bff` uses port `8010`.

### 8.4 BFF in the all-gens Resource

```python
local_resource(
    'hauliage-all-gens',
    'echo "All hauliage codegen complete"',
    resource_deps=['%s-service-gen' % name for name in HAULIAGE_SERVICES] + ['bff-service-gen'],
    labels=['all_gens'],
    allow_parallel=False,
)
```

BFF service-gen is included in the synchronization resource, so all services (including BFF) must complete codegen before builds start.

---

## 9. Base Image Lifecycle

### 9.1 Base Image Build (Manual/Triggered)

```python
local_resource(
    'build-base-image',
    '%s docker build-base' % hauliage_bin,
    deps=[
        'docker/base/Dockerfile',
        'docker/base/dev-entrypoint.sh',
    ],
    labels=['docker'],
    allow_parallel=True,
)
```

Calls `hauliage docker build-base` which builds `docker/base/Dockerfile` as `hauliage-base:latest` (and optionally pushes to `ghcr.io/microscaler/hauliage-base:latest`).

### 9.2 Base Dockerfile

```dockerfile
ARG TARGETPLATFORM=linux/amd64
ARG BUILDPLATFORM
FROM --platform=${TARGETPLATFORM} alpine:3.23

RUN apk add --no-cache ca-certificates libgcc tzdata

WORKDIR /app

RUN mkdir -p /app/config /app/doc /app/static_site && \
    chmod -R 777 /app

ENV TZ=UTC
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

COPY docker/base/dev-entrypoint.sh /dev-entrypoint.sh
RUN chmod +x /dev-entrypoint.sh

LABEL org.opencontainers.image.title="Hauliage Base Runtime"
LABEL org.opencontainers.image.description="Base runtime image for Hauliage microservices"
LABEL org.opencontainers.image.vendor="Microscaler"
```

**Key design decisions:**
- `chmod -R 777 /app` — Allows Tilt's `sync()` to write files as root in the container
- `dev-entrypoint.sh` — Hot-reload wrapper with SIGHUP handling and binary-wait loop
- `RUST_LOG=info` — Verbose by default for development debugging

### 9.3 How Services Use the Base

The Dockerfile.template uses `ARG BASE_IMAGE=hauliage-base:latest`:

```dockerfile
ARG BASE_IMAGE=hauliage-base:latest
FROM ${BASE_IMAGE}

# TARGETARCH injected via python template processing
COPY ./build_artifacts/{{target_arch}}/{{binary_name}} /app/{{binary_name}}

# Create directories for configuration and assets with write permissions
RUN mkdir -p /app/config /app/doc /app/static_site && chmod -R 777 /app

# Copy configuration and assets (gen: doc/static_site; impl: config)
COPY ./microservices/{{module}}/impl/config /app/config
COPY ./microservices/{{module}}/gen/doc /app/doc
COPY ./microservices/{{module}}/gen/static_site /app/static_site

# Expose HTTP port
EXPOSE {{port}}

# Runtime environment
ENV RUST_BACKTRACE=1
ENV RUST_LOG=debug

# Run the service
ENTRYPOINT ["/dev-entrypoint.sh", "/app/{{binary_name}}", \
    "--spec", "/app/doc/openapi.yaml", \
    "--doc-dir", "/app/doc", \
    "--static-dir", "/app/static_site", \
    "--config", "/app/config/config.yaml"]
```

---

## 10. Dynamic Microservice Discovery

### 10.1 The Service Registry List

```python
HAULIAGE_SERVICES = [
    'identity', 'company', 'fleet', 'consignments', 'bidding',
    'inbox', 'notifications', 'analytics', 'telemetry', 'marketing',
    'reviews', 'locations', 'customs', 'gics',
]
```

### 10.2 For-Loop Expansion Pattern

```python
for name in HAULIAGE_SERVICES:
    create_microservice_lint(name, '%s/openapi.yaml' % name)
    create_microservice_gen(name, '%s/openapi.yaml' % name, name)
    create_microservice_stubs_resource(name, '%s/openapi.yaml' % name)

for name in HAULIAGE_SERVICES:
    create_microservice_build_resource(name)
    create_microservice_deployment(name)
```

Each service gets:
- `<name>-lint`
- `<name>-service-gen`
- `stubs-<name>` (manual trigger)
- `build-<name>`
- `copy-<name>`
- `docker-<name>`
- `custom_build` (Tilt-managed image)
- `k8s_resource` with Helm

### 10.3 Spec File Paths

- **Regular services:** `openapi/<name>/openapi.yaml` (14 services)
- **BFF service:** `openapi/openapi_bff.yaml` (flat, generated by BFF spec gen)

---

## 11. Additional Resources

### 11.1 IoT Worker (Standalone, Not in Loop)

```python
custom_build(
    'localhost:5001/hauliage-iot-worker',
    'docker build -f docker/microservices/Dockerfile.iot_worker -t localhost:5001/hauliage-iot-worker:tilt . && (docker push ... || kind load ...)',
    deps=['./microservices/hauliage_iot_worker', 'docker/microservices/Dockerfile.iot_worker'],
    tag='tilt',
)
k8s_yaml('k8s/microservices/hauliage-iot-worker.yaml')
k8s_resource('hauliage-iot-worker', resource_deps=['hauliage-database-env'], labels=['workers'])
```

### 11.2 Test Resources (All Manual Trigger)

```python
def create_microservice_test_resource(name):
    local_resource(
        'test-%s' % name,
        'cd microservices && cargo test -p %s' % package_name,
        env={'HAULIAGE_DB_PASSWORD': db_pass},
        deps=[
            './microservices/%s/impl/src' % name,
            './microservices/%s/impl/tests' % name,
            './microservices/%s/gen/src' % name,
            './microservices/Cargo.toml',
            'tooling/pyproject.toml',
        ],
        labels=[name],
        trigger_mode=TRIGGER_MODE_MANUAL,
        allow_parallel=True,
    )
```

Frontend tests:
- `test-frontend-bdd` (Playwright BDD)
- `test-frontend-e2e` (Playwright E2E)
- `api-load-test` (microservices load test)
- `ui-load-test` (frontend load test)

### 11.3 Observability Dashboards (Manual)

```python
local_resource(
    'hauliage-observability-dashboards',
    'kubectl apply -k ./k8s/observability/',
    deps=[
        './k8s/observability/kustomization.yaml',
        './k8s/observability/README.md',
        './k8s/observability/dashboards/hauliage-overview.json',
        './k8s/observability/dashboards/hauliage-bff.json',
        './k8s/observability/dashboards/hauliage-postgres.json',
        './k8s/observability/dashboards/hauliage-lifeguard.json',
        './k8s/observability/dashboards/hauliage-cluster-logs.json',
    ],
    labels=['observability'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
    allow_parallel=True,
)
```

### 11.4 Docker Prune Settings

```python
docker_prune_settings(
    disable=False,
    max_age_mins=30,     # Clean up images older than 1 hour
    keep_recent=1,       # Keep only the single most recent build per image
    interval_hrs=1       # Run prune every hour automatically
)
```

### 11.5 Mock Telemetry Injector (Manual)

```python
local_resource(
    'mock-telemetry-injector',
    serve_cmd='cd frontend && node scripts/inject-telemetry.js --broker "%s"' % _mock_telemetry_broker,
    deps=[
        './frontend/scripts/inject-telemetry.js',
        './frontend/package.json',
    ],
    labels=['data'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
    allow_parallel=True,
)
```

Uses `serve_cmd` (not `cmd`) so the script runs until stopped without blocking Tilt.

### 11.6 Mosquitto (Always Applied)

```python
k8s_yaml('k8s/microservices/data-stack-aliases.yaml')
k8s_yaml('k8s/data/mosquitto.yaml')
k8s_resource('mosquitto', port_forwards=['1883:1883', '9002:9001'], labels=['data'])
```

Mosquitto is Hauliage-specific dev tooling — always applied from this repo (not from shared-kind-cluster).

---

## 12. Docker Files Reference

### 12.1 docker/base/Dockerfile
- Alpine 3.23 + ca-certificates, libgcc, tzdata
- Creates `/app`, `/app/config`, `/app/doc`, `/app/static_site` with 777 perms
- Copies `dev-entrypoint.sh` as `/dev-entrypoint.sh`

### 12.2 docker/base/dev-entrypoint.sh
- Shell script that waits for binary, runs it, handles SIGHUP for hot reload
- Loops: if binary missing -> sleep -> retry; else run in background, wait, restart on exit

### 12.3 docker/microservices/Dockerfile.template
- `ARG BASE_IMAGE=hauliage-base:latest` -> `FROM ${BASE_IMAGE}`
- COPY binary from `build_artifacts/{arch}/{binary_name}`
- COPY config/doc/static_site from service dirs
- EXPOSE {{port}}
- ENTRYPOINT with /dev-entrypoint.sh + runtime args (--spec, --doc-dir, --static-dir, --config)

### 12.4 docker/build/Dockerfile.self-contained
- CI-only: full build inside a container with Rust, Python, cross
- Produces `build_artifacts/` zip for extraction
- Three arch variants: arm7, amd64, arm64

### 12.5 docker/build/Dockerfile
- Legacy CI-like: Ubuntu 24.04 + Rust + Python + Docker socket + cross
- Used via `docker run --rm -v $(pwd):/work -v /var/run/docker.sock:/var/run/docker.sock`

### 12.6 docker/microservices/Dockerfile.iot_worker
- Standalone Rust build: `rust:1.85-bookworm` builder -> `debian:bookworm-slim` runtime
- NOT part of the main microservice pipeline

### 12.7 docker/website/Dockerfile
- Multi-stage: `node:25-alpine` builder (yarn build SolidJS) -> `nginx:alpine` server
- Port 8080

### 12.8 docker/base/Dockerfile.multiarch
- Same as base/Dockerfile but with `TARGETPLATFORM` build arg for `docker buildx`

---

## 13. justfile Task Definitions

### 13.1 Infrastructure

| Command | Purpose |
|---------|---------|
| `dev-up` | Full stack: registry + Kind cluster + namespaces + env + Tilt |
| `dev-down` | Tilt down + delete Kind cluster |
| `dev-up-shared-infra` | `TILT_USE_SHARED_KIND_INFRA=1 just dev-up` |
| `dev-up-bundled-infra` | `TILT_USE_SHARED_KIND_INFRA=0 just dev-up` (offline) |
| `tilt-up` | Tilt only (for shells without direnv) on port 10352 |
| `dev-registry` | Start local Docker registry on `localhost:5001` |
| `apply-platform-namespaces` | Apply platform namespaces from shared-kind-cluster |
| `sanitize-stuck-supabase-volumes` | Force-release stuck PVs |

### 13.2 Tooling

| Command | Purpose |
|---------|---------|
| `init-tooling` | Create venv, install BRRTRouter tooling + hauliage tooling |
| `build-tooling` | Reinstall tooling from venv |
| `generate-bff` | Run `hauliage bff generate-system` |

### 13.3 Frontend/Mobile

| Command | Purpose |
|---------|---------|
| `install-frontend` | `cd frontend && yarn install` |
| `dev` | `cd frontend && yarn dev` (SolidJS HMR) |
| `build` | `cd frontend && yarn build` |
| `clean` | Clean frontend + mobile |
| `install-mobile` | `cd mobile && flutter pub get` |
| `dev-mobile` | Flutter run on macOS |
| `inject-maps-keys` | Inject Google Maps API keys into mobile configs |

### 13.4 Testing

| Command | Purpose |
|---------|---------|
| `check-contracts` | Verify consignments OpenAPI two-file parity |
| `test-frontend` | Frontend unit tests |
| `test-frontend-e2e` | Frontend Playwright E2E |
| `test-microservices` | `cd microservices && cargo test` |
| `test-iot` | `cd microservices/hauliage_iot_worker && cargo test` |

---

## 14. Kubernetes Configuration Pattern

### 14.1 Namespace Strategy
- `data` — Shared platform services (postgres, redis, minio, etc.) from `shared-kind-cluster`
- `hauliage` — Hauliage microservices and IoT worker
- `observability` — Grafana dashboards (applied manually)

### 14.2 Database Connection Pattern

All microservices connect to Postgres via `postgres.data.svc.cluster.local:5432`:
- ConfigMap `hauliage-database-config` in `hauliage` namespace: `DB_HOST=postgres.data.svc.cluster.local`
- Secret `hauliage-db-credentials` in `hauliage` namespace: `DB_PASS`, `HAULIAGE_DB_PASSWORD`
- Mosquitto: `ExternalName` service `mosquitto` in `hauliage` namespace -> `mosquitto.data.svc.cluster.local`

### 14.3 Helm Values Pattern

Each service has:
- `helm/hauliage-microservice/values/<name>.yaml` — Service-specific values
- `helm/hauliage-microservice/values/_database-kubernetes.yaml` — Shared DB settings
- `helm/hauliage-microservice/values/_redis-shared-kind.yaml` — Shared Kind Redis (optional)

---

## 15. Tiltfile Configuration Summary

| Setting | Value |
|---------|-------|
| k8s contexts | `kind-kind`, `kind-hauliage` |
| k8s_upsert_timeout | 60 seconds |
| docker_prune | Every 1h, keep 1 recent, age > 30min |
| UI port | 10352 (via .envrc or --port flag) |
| Docker registry | localhost:5001 (kind-registry) |
| Default infra | Shared Kind (TILT_USE_SHARED_KIND_INFRA=1) |
| Base image | hauliage-base:latest |
| Registry | ghcr.io/microscaler/hauliage-base:latest (publish) |
| Namespace | hauliage |

---

## 16. Key Patterns for Replication in RERP

1. **Service Discovery**: Maintain a `PACKAGE_NAMES` map and service list; iterate with `for` loop to create all resources. For RERP, replace `PACKAGE_NAMES` with RERP service names and update port mappings.

2. **Tooling Delegation**: The Python CLI (`hauliage`) delegates to a shared tooling repo (BRRTRouter). For RERP, create a `rerp-tooling` shim that delegates to the shared tooling or a dedicated RERP tooling package.

3. **Single Dockerfile Template**: Use `Dockerfile.template` with `{{placeholders}}` rendered by the build tooling. No per-service Dockerfiles needed.

4. **Live Update via Entrypoint Wrapper**: The `dev-entrypoint.sh` script enables hot-reload by waiting for synced binary and handling SIGHUP. Replicate this for RERP services.

5. **Dual Infra Modes**: Support both `shared` (infrastructure from sibling repo) and `bundled` (all infrastructure from this repo) via `TILT_USE_SHARED_KIND_INFRA` env var.

6. **Three-Stage Pipeline**: Spec -> Gen -> Build -> Copy -> Docker -> Helm. Keep the strict `resource_deps` chain for correct build ordering.

7. **Shared Venv**: Single Python venv for all tooling CLIs. `ignore` patterns must exclude `*.egg-info` to prevent Tilt loops.

8. **BFF Aggregation**: If RERP has a BFF, the `bff-spec-gen` step aggregates all microservice OpenAPI specs into one. This is driven by a suite config file.

9. **Dynamic Architecture**: `TARGET_RUST_TRIPLE` adapts to host architecture (amd64 vs arm64). Binary paths change accordingly.

10. **Helm Template Reuse**: The microservice Helm chart is a template used for all services, differentiated by per-service values files and the `name` parameter.
