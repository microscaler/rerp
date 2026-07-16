#!/usr/bin/env bash
# Local/Tilt database administration helper. Flux does not run this script.
# Its migration-only mode is the rapid-development path for application
# migrations, RLS, seeds, and post-migration grants.
#
# The shared dev cluster runs Bitnami PostgreSQL HA. This script discovers the
# elected primary and connects directly to it for privileged bootstrap work;
# application traffic continues to use Pgpool through the `postgres` Service.
#
# Layout:
#   - Database `rerp` — app data only.
#   - Schema `rerp` — all RERP tables (search_path default for this database).
#   - Role `rerp` — login role matching helm app.config.database (password from env below).
#   - After suite-local migrations (apply_order.txt), optional
#     microservices/accounting/*/impl/seeds/*.sql.
#
# Shared infrastructure namespace: `data` (postgres, redis, minio, observability).
# RERP deploys to namespace `rerp`.
#
# Credentials:
#   - deployment-configuration/profiles/dev/rerp/accounting supplies
#     rerp/rerp-db-credentials.
#   - deployment-configuration/profiles/dev/postgres-ha supplies Pgpool's
#     matching custom-user entry.
#   - RERP_DB_PASSWORD is a break-glass override only; no plaintext default is
#     embedded in this repository.
#
# Optional:
#   RERP_DB_INIT_TIMEOUT (default 600s).
#   RERP_APPLY_MIGRATIONS_ONLY=1 — skip role/DB creation; only wait for postgres, apply ./migrations, then GRANTs.
#
# NOTE: setup-db.sh has no apply ledger — every rollout re-applies every migration.
# This works because every DDL statement in migrations/**/*.sql must be strictly
# idempotent (safe to run against a fresh DB and against a DB where it already ran).
# See ADR 0015 (idempotent schema retrofit pattern) for the canonical pattern.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
SUITE_DIR="${REPO_ROOT}/microservices/accounting"
cd "${REPO_ROOT}"

DATA_NAMESPACE="${RERP_DB_DATA_NAMESPACE:-data}"
APP_NAMESPACE="${RERP_DB_APP_NAMESPACE:-rerp}"
POSTGRES_STATEFULSET="${RERP_POSTGRES_STATEFULSET:-postgres-ha-postgresql}"
POSTGRES_LABEL="${RERP_POSTGRES_LABEL:-app.kubernetes.io/instance=postgres-ha,app.kubernetes.io/component=postgresql}"
POSTGRES_CONTAINER="${RERP_POSTGRES_CONTAINER:-postgresql}"
PGPOOL_SECRET="${RERP_PGPOOL_SECRET:-postgres-credentials}"
PGPOOL_LABEL="${RERP_PGPOOL_LABEL:-app.kubernetes.io/instance=postgres-ha,app.kubernetes.io/component=pgpool}"
PGPOOL_SERVICE="${RERP_PGPOOL_SERVICE:-postgres.data.svc.cluster.local}"
APP_DB_SECRET="${RERP_DB_SECRET:-rerp-db-credentials}"
WAIT_TIMEOUT="${RERP_DB_INIT_TIMEOUT:-600s}"
POSTGRES_POD=""
RERP_DB_PASSWORD="${RERP_DB_PASSWORD:-}"

sql_escape() { printf '%s' "$1" | sed "s/'/''/g"; }

load_rerp_password() {
  if [ -n "${RERP_DB_PASSWORD}" ]; then
    echo "⚠️  Using explicit RERP_DB_PASSWORD override; ensure the SOPS profiles match." >&2
    return 0
  fi

  if ! kubectl get secret "${APP_DB_SECRET}" -n "${APP_NAMESPACE}" >/dev/null 2>&1; then
    echo "❌ Missing ${APP_NAMESPACE}/${APP_DB_SECRET}." >&2
    echo "   Apply RERP's dev/rerp/accounting SOPS profile first." >&2
    return 1
  fi

  RERP_DB_PASSWORD="$(
    kubectl get secret "${APP_DB_SECRET}" -n "${APP_NAMESPACE}" \
      -o jsonpath='{.data.RERP_DB_PASSWORD}' | base64 --decode
  )"
  if [ -z "${RERP_DB_PASSWORD}" ]; then
    echo "❌ ${APP_NAMESPACE}/${APP_DB_SECRET} has no non-empty RERP_DB_PASSWORD key." >&2
    return 1
  fi
}

validate_pgpool_credentials() {
  local usernames passwords
  local -a username_list password_list
  local index

  if ! kubectl get secret "${PGPOOL_SECRET}" -n "${DATA_NAMESPACE}" >/dev/null 2>&1; then
    echo "❌ Missing ${DATA_NAMESPACE}/${PGPOOL_SECRET}; PostgreSQL HA is not ready." >&2
    return 1
  fi

  usernames="$(kubectl get secret "${PGPOOL_SECRET}" -n "${DATA_NAMESPACE}" -o jsonpath='{.data.usernames}' | base64 --decode)"
  passwords="$(kubectl get secret "${PGPOOL_SECRET}" -n "${DATA_NAMESPACE}" -o jsonpath='{.data.passwords}' | base64 --decode)"
  IFS=';' read -r -a username_list <<<"${usernames}"
  IFS=';' read -r -a password_list <<<"${passwords}"

  for index in "${!username_list[@]}"; do
    if [ "${username_list[$index]}" = "rerp" ]; then
      if [ "${password_list[$index]:-}" != "${RERP_DB_PASSWORD}" ]; then
        echo "❌ Pgpool's rerp credential does not match ${APP_NAMESPACE}/${APP_DB_SECRET}." >&2
        echo "   Reconcile the postgres-ha and rerp SOPS profiles together." >&2
        return 1
      fi
      return 0
    fi
  done

  echo "❌ Pgpool does not contain the rerp custom user." >&2
  echo "   Reconcile the postgres-ha SOPS profile and HelmRelease before database initialization." >&2
  return 1
}

postgres_psql() {
  local database="$1"
  kubectl exec -i -n "${DATA_NAMESPACE}" "pod/${POSTGRES_POD}" -c "${POSTGRES_CONTAINER}" -- \
    sh -c 'PGPASSWORD="$(cat "$POSTGRES_PASSWORD_FILE")" exec psql -h 127.0.0.1 -p 5432 -U "${POSTGRES_USER:-postgres}" -d "$1" -v ON_ERROR_STOP=1' sh "${database}"
}

wait_for_postgres() {
  local pod
  local is_primary

  echo "⏳ Waiting for statefulset/${POSTGRES_STATEFULSET} rollout (${WAIT_TIMEOUT})..."
  kubectl rollout status "statefulset/${POSTGRES_STATEFULSET}" -n "${DATA_NAMESPACE}" --timeout="${WAIT_TIMEOUT}"

  echo "⏳ Waiting for PostgreSQL HA pods Ready (${WAIT_TIMEOUT})..."
  kubectl wait --for=condition=ready pod -l "${POSTGRES_LABEL}" -n "${DATA_NAMESPACE}" --timeout="${WAIT_TIMEOUT}" >/dev/null

  echo "🔎 Discovering the elected PostgreSQL primary..."
  while IFS= read -r pod; do
    [ -z "${pod}" ] && continue
    is_primary="$(
      kubectl exec -n "${DATA_NAMESPACE}" "pod/${pod}" -c "${POSTGRES_CONTAINER}" -- \
        sh -c 'PGPASSWORD="$(cat "$POSTGRES_PASSWORD_FILE")" psql -h 127.0.0.1 -U "${POSTGRES_USER:-postgres}" -d postgres -Atqc "SELECT NOT pg_is_in_recovery()"' \
        2>/dev/null || true
    )"
    if [ "${is_primary}" = "t" ]; then
      POSTGRES_POD="${pod}"
      echo "✅ PostgreSQL primary: ${POSTGRES_POD}"
      return 0
    fi
  done < <(kubectl get pods -n "${DATA_NAMESPACE}" -l "${POSTGRES_LABEL}" -o jsonpath='{range .items[*]}{.metadata.name}{"\n"}{end}')

  echo "❌ No elected PostgreSQL primary was discoverable." >&2
  return 1
}

bootstrap_rerp_role_and_db() {
  echo "⏳ Creating role rerp, database rerp, schema rerp (if missing)..."
  local password_sql
  password_sql="$(sql_escape "${RERP_DB_PASSWORD}")"
  postgres_psql postgres <<EOF
-- Cluster login role for RERP apps (matches Helm database.user)
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'rerp') THEN
    EXECUTE format('CREATE ROLE rerp LOGIN PASSWORD %L', '${password_sql}');
  ELSE
    EXECUTE format('ALTER ROLE rerp PASSWORD %L', '${password_sql}');
  END IF;
END \$\$;

SELECT 'CREATE DATABASE rerp OWNER rerp'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'rerp')\gexec

\c rerp

CREATE SCHEMA IF NOT EXISTS rerp;
GRANT CONNECT ON DATABASE rerp TO rerp;
GRANT ALL PRIVILEGES ON SCHEMA rerp TO rerp;
ALTER SCHEMA rerp OWNER TO rerp;
-- Allow extensions / shared objects that still use public if needed
GRANT USAGE ON SCHEMA public TO rerp;
GRANT CREATE ON SCHEMA public TO rerp;

ALTER DATABASE rerp SET search_path TO rerp, public;
EOF
}

apply_migrations_from_disk() {
  if [ -d "${SUITE_DIR}/migrations" ]; then
    echo "📥 Applying Lifeguard migration SQL (search_path=rerp from ALTER DATABASE)..."
    apply_one() {
      local migration_file="$1"
      echo "  -> Applying ${migration_file}..."
      postgres_psql rerp < "${migration_file}"
    }
    if [ -f "${SUITE_DIR}/sql/rls/v1/install.sql" ]; then
      echo "  -> Installing the vendored Sesame RLS v1 contract first..."
      apply_one "${SUITE_DIR}/sql/rls/v1/install.sql"
    else
      echo "  ❌ Missing required Sesame RLS contract: ./sql/rls/v1/install.sql" >&2
      return 1
    fi
    if [ -f "${SUITE_DIR}/migrations/apply_order.txt" ]; then
      while IFS= read -r rel || [ -n "${rel}" ]; do
        [[ -z "${rel}" || "${rel}" =~ ^# ]] && continue
        migration_file="${SUITE_DIR}/migrations/${rel}"
        if [ -f "${migration_file}" ]; then
          apply_one "${migration_file}"
        else
          echo "  ⚠️  apply_order.txt lists missing file: ${migration_file}" >&2
        fi
      done < "${SUITE_DIR}/migrations/apply_order.txt"
    else
      echo "  (no apply_order.txt — falling back to lexicographic path sort; run lifeguard_migrator to generate)"
      while IFS= read -r -d '' migration_file; do
        apply_one "${migration_file}"
      done < <(find "${SUITE_DIR}/migrations" -name "*.sql" -print0 | sort -z)
    fi
  else
    echo "📥 No ./migrations directory; skipping SQL file ingest."
  fi
}

apply_seeds_from_disk() {
  local count
  count="$(find "${SUITE_DIR}" -path '*/impl/seeds/*.sql' 2>/dev/null | wc -l | tr -d ' ')"
  if [ -z "${count}" ] || [ "${count}" = "0" ]; then
    return 0
  fi
  apply_one_seed() {
    local seed_file="$1"
    echo "  -> Applying ${seed_file}..."
    postgres_psql rerp < "${seed_file}"
  }
  if [ -f "${SUITE_DIR}/seed_order.txt" ]; then
    echo "📥 Applying Accounting seed SQL (accounting/seed_order.txt, FK-ordered)..."
    while IFS= read -r rel || [ -n "${rel}" ]; do
      [[ -z "${rel}" || "${rel}" =~ ^# ]] && continue
      seed_file="${SUITE_DIR}/${rel}"
      if [ -f "${seed_file}" ]; then
        apply_one_seed "${seed_file}"
      else
        echo "  ⚠️  seed_order.txt lists missing file: ${seed_file}" >&2
      fi
    done < "${SUITE_DIR}/seed_order.txt"
  else
    echo "📥 Applying Accounting seed SQL (accounting/*/impl/seeds/*.sql, alphabetical)..."
    while IFS= read -r -d '' seed_file; do
      apply_one_seed "${seed_file}"
    done < <(find "${SUITE_DIR}" -path '*/impl/seeds/*.sql' -print0 2>/dev/null | sort -z)
  fi
}

grant_rerp_dml() {
  # Migrations run as superuser (postgres); tables are owned by postgres. The app role rerp
  # has schema USAGE/CREATE but not automatic DML on those tables — without GRANT, microservices
  # get Postgres errors that tokio-postgres surfaces as Display "db error".
  echo "🔐 GRANT DML on rerp schema objects to role rerp..."
  postgres_psql rerp <<'EOF'
SET search_path TO rerp;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA rerp TO rerp;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA rerp TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp GRANT USAGE, SELECT ON SEQUENCES TO rerp;
EOF
}

verify_pgpool_connection() {
  local pgpool_pod
  local result

  pgpool_pod="$(
    kubectl get pods -n "${DATA_NAMESPACE}" -l "${PGPOOL_LABEL}" \
      --field-selector=status.phase=Running \
      -o jsonpath='{.items[0].metadata.name}'
  )"
  if [ -z "${pgpool_pod}" ]; then
    echo "❌ No running Pgpool pod was found." >&2
    return 1
  fi

  echo "🔐 Verifying the RERP login through Pgpool..."
  if ! result="$(
    printf '%s\n' "${RERP_DB_PASSWORD}" | \
      kubectl exec -i -n "${DATA_NAMESPACE}" "pod/${pgpool_pod}" -c pgpool -- \
        sh -c 'IFS= read -r PGPASSWORD; export PGPASSWORD; psql -h "$1" -p 5432 -U rerp -d rerp -Atqc "SELECT 1"' sh "${PGPOOL_SERVICE}"
  )"; then
    echo "❌ RERP cannot authenticate through Pgpool." >&2
    echo "   Reconcile the postgres-ha HelmRelease customUsersSecret, then retry rerp-db-init." >&2
    return 1
  fi
  if [ "${result}" != "1" ]; then
    echo "❌ Pgpool verification returned an unexpected result." >&2
    return 1
  fi
  echo "✅ RERP login verified through Pgpool."
}

load_rerp_password
validate_pgpool_credentials

if [ "${RERP_APPLY_MIGRATIONS_ONLY:-0}" = "1" ]; then
  echo "📌 RERP_APPLY_MIGRATIONS_ONLY=1 — apply migration SQL files to cluster + GRANTs (no role/DB bootstrap)."
  echo "    Run once: rerp-db-init. After lifeguard_migrator regenerates SQL, run this resource or re-run with the env set."
  wait_for_postgres
  apply_migrations_from_disk
  apply_seeds_from_disk
  grant_rerp_dml
  verify_pgpool_connection
  echo "✅ Migrations applied to database rerp."
  exit 0
fi

echo "🚀 Initializing RERP database, role, and schema..."
wait_for_postgres
bootstrap_rerp_role_and_db
apply_migrations_from_disk
apply_seeds_from_disk
grant_rerp_dml
verify_pgpool_connection

echo "✅ RERP database + schema setup complete."
