#!/usr/bin/env bash
# Setup script for RERP PostgreSQL database + schema (in-cluster; PostgreSQL listens on 127.0.0.1:5432 in the pod).
# Connects to shared-k8s managed by microscaler/shared-k8s-cluster.
#
# Layout:
#   - Database `rerp` — app data only.
#   - Schema `rerp` — all RERP tables (search_path default for this database).
#   - Role `rerp` — login role matching helm app.config.database (password from env below).
#   - After ./migrations (apply_order.txt), optional microservices/*/impl/seeds/*.sql.
#
# Shared infrastructure namespace: `data` (postgres, redis, minio, observability).
# RERP deploys to namespace `rerp`.
#
# Optional:
#   RERP_DB_INIT_TIMEOUT (default 600s), RERP_DB_PASSWORD (must match helm dev password).
#   RERP_APPLY_MIGRATIONS_ONLY=1 — skip role/DB creation; only wait for postgres, apply ./migrations, then GRANTs.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
SUITE_DIR="${REPO_ROOT}/microservices/accounting"
cd "${REPO_ROOT}"

NS=data
DEPLOY=postgres-primary
WAIT_TIMEOUT="${RERP_DB_INIT_TIMEOUT:-600s}"
RERP_DB_PASSWORD="${RERP_DB_PASSWORD:-dev_password_change_in_prod}"

sql_escape() { printf '%s' "$1" | sed "s/'/''/g"; }
PW_SQL=$(sql_escape "${RERP_DB_PASSWORD}")

wait_for_postgres() {
  echo "⏳ Waiting for ${DEPLOY} rollout (${WAIT_TIMEOUT})..."
  kubectl rollout status "deployment/${DEPLOY}" -n "${NS}" --timeout="${WAIT_TIMEOUT}"

  echo "⏳ Waiting for postgres pod Ready (${WAIT_TIMEOUT})..."
  kubectl wait --for=condition=ready pod -l 'app in (postgres, postgres-primary)' -n "${NS}" --timeout="${WAIT_TIMEOUT}" >/dev/null
}

bootstrap_rerp_role_and_db() {
  echo "⏳ Creating role rerp, database rerp, schema rerp (if missing)..."
  kubectl exec -i -n "${NS}" "deployment/${DEPLOY}" -c postgres -- \
    sh -c 'env PGPASSWORD="${POSTGRESQL_PASSWORD:-${POSTGRES_PASSWORD:-}}" psql -h 127.0.0.1 -p 5432 -U "${POSTGRESQL_USERNAME:-postgres}" -d postgres -v ON_ERROR_STOP=1' <<EOF
-- Cluster login role for RERP apps (matches Helm database.user)
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'rerp') THEN
    EXECUTE format('CREATE ROLE rerp LOGIN PASSWORD %L', '${PW_SQL}');
  ELSE
    EXECUTE format('ALTER ROLE rerp PASSWORD %L', '${PW_SQL}');
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
      cat "${migration_file}" | kubectl exec -i -n "${NS}" "deployment/${DEPLOY}" -c postgres -- \
        sh -c 'env PGPASSWORD="${POSTGRESQL_PASSWORD:-${POSTGRES_PASSWORD:-}}" psql -h 127.0.0.1 -p 5432 -U "${POSTGRESQL_USERNAME:-postgres}" -d rerp -v ON_ERROR_STOP=1'
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
  count="$(find ./microservices -path '*/impl/seeds/*.sql' 2>/dev/null | wc -l | tr -d ' ')"
  if [ -z "${count}" ] || [ "${count}" = "0" ]; then
    return 0
  fi
  apply_one_seed() {
    local seed_file="$1"
    echo "  -> Applying ${seed_file}..."
    cat "${seed_file}" | kubectl exec -i -n "${NS}" "deployment/${DEPLOY}" -c postgres -- \
      sh -c 'env PGPASSWORD="${POSTGRESQL_PASSWORD:-${POSTGRES_PASSWORD:-}}" psql -h 127.0.0.1 -p 5432 -U "${POSTGRESQL_USERNAME:-postgres}" -d rerp -v ON_ERROR_STOP=1'
  }
  if [ -f ./microservices/seed_order.txt ]; then
    echo "📥 Applying per-microservice seed SQL (microservices/seed_order.txt, FK-ordered)..."
    while IFS= read -r rel || [ -n "${rel}" ]; do
      [[ -z "${rel}" || "${rel}" =~ ^# ]] && continue
      seed_file="./microservices/${rel}"
      if [ -f "${seed_file}" ]; then
        apply_one_seed "${seed_file}"
      else
        echo "  ⚠️  seed_order.txt lists missing file: ${seed_file}" >&2
      fi
    done < ./microservices/seed_order.txt
  else
    echo "📥 Applying per-microservice seed SQL (microservices/*/impl/seeds/*.sql, alphabetical)..."
    while IFS= read -r -d '' seed_file; do
      apply_one_seed "${seed_file}"
    done < <(find ./microservices -path '*/impl/seeds/*.sql' -print0 2>/dev/null | sort -z)
  fi
}

grant_rerp_dml() {
  # Migrations run as superuser (postgres); tables are owned by postgres. The app role rerp
  # has schema USAGE/CREATE but not automatic DML on those tables — without GRANT, microservices
  # get Postgres errors that tokio-postgres surfaces as Display "db error".
  echo "🔐 GRANT DML on rerp schema objects to role rerp..."
  kubectl exec -i -n "${NS}" "deployment/${DEPLOY}" -c postgres -- \
    sh -c 'env PGPASSWORD="${POSTGRESQL_PASSWORD:-${POSTGRES_PASSWORD:-}}" psql -h 127.0.0.1 -p 5432 -U "${POSTGRESQL_USERNAME:-postgres}" -d rerp -v ON_ERROR_STOP=1' <<'EOF'
SET search_path TO rerp;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA rerp TO rerp;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA rerp TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp GRANT USAGE, SELECT ON SEQUENCES TO rerp;

GRANT EXECUTE ON FUNCTION public.sesame_rls_contract_version() TO rerp;
GRANT EXECUTE ON FUNCTION public.rls_set_session(text, uuid, uuid, text, jsonb, jsonb, text, text) TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_tenant_id() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_subject_id() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_organization_id() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_session_id() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_roles() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_permissions() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_user_type() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_current_org_type() TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_has_role(text) TO rerp;
GRANT EXECUTE ON FUNCTION public.sesame_has_permission(text) TO rerp;
EOF
}

if [ "${RERP_APPLY_MIGRATIONS_ONLY:-0}" = "1" ]; then
  echo "📌 RERP_APPLY_MIGRATIONS_ONLY=1 — apply migration SQL files to cluster + GRANTs (no role/DB bootstrap)."
  echo "    Run once: rerp-db-init. After lifeguard_migrator regenerates SQL, run this resource or re-run with the env set."
  wait_for_postgres
  apply_migrations_from_disk
  apply_seeds_from_disk
  grant_rerp_dml
  echo "✅ Migrations applied to database rerp."
  exit 0
fi

echo "🚀 Initializing RERP database, role, and schema..."
wait_for_postgres
bootstrap_rerp_role_and_db
apply_migrations_from_disk
apply_seeds_from_disk
grant_rerp_dml

echo "✅ RERP database + schema setup complete."
