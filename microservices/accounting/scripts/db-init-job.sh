#!/usr/bin/env bash
# Flux database bootstrap entrypoint. Unlike setup-db.sh (the local break-glass
# helper), this runs inside the cluster and needs no kubectl or Kubernetes RBAC.
set -euo pipefail

SUITE_DIR="${RERP_SUITE_DIR:-/opt/rerp/suite}"
PGHOST="${PGHOST:-postgres-ha-pgpool.data.svc.cluster.local}"
PGPORT="${PGPORT:-5432}"
PGUSER="${PGUSER:-postgres}"
PGDATABASE="${PGDATABASE:-postgres}"
WAIT_SECONDS="${RERP_DB_INIT_WAIT_SECONDS:-300}"

: "${POSTGRES_ADMIN_PASSWORD:?POSTGRES_ADMIN_PASSWORD is required}"
: "${RERP_DB_PASSWORD:?RERP_DB_PASSWORD is required}"
: "${PGPOOL_CUSTOM_USERS:?PGPOOL_CUSTOM_USERS is required}"
: "${PGPOOL_CUSTOM_PASSWORDS:?PGPOOL_CUSTOM_PASSWORDS is required}"

sql_escape() { printf '%s' "$1" | sed "s/'/''/g"; }

validate_pgpool_contract() {
  local -a users passwords
  local index
  IFS=';' read -r -a users <<<"${PGPOOL_CUSTOM_USERS}"
  IFS=';' read -r -a passwords <<<"${PGPOOL_CUSTOM_PASSWORDS}"
  for index in "${!users[@]}"; do
    if [ "${users[$index]}" = "rerp" ]; then
      if [ "${passwords[$index]:-}" != "${RERP_DB_PASSWORD}" ]; then
        echo "Pgpool and RERP application credentials do not match" >&2
        return 1
      fi
      return 0
    fi
  done
  echo "Pgpool custom users do not contain rerp" >&2
  return 1
}

admin_psql() {
  local database="$1"
  shift
  PGPASSWORD="${POSTGRES_ADMIN_PASSWORD}" \
    psql -X -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" \
      -d "${database}" -v ON_ERROR_STOP=1 "$@"
}

wait_for_postgres() {
  local elapsed=0
  echo "Waiting for PostgreSQL HA through Pgpool..."
  until PGPASSWORD="${POSTGRES_ADMIN_PASSWORD}" pg_isready \
    -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d postgres >/dev/null 2>&1; do
    if [ "${elapsed}" -ge "${WAIT_SECONDS}" ]; then
      echo "PostgreSQL did not become ready within ${WAIT_SECONDS}s" >&2
      return 1
    fi
    sleep 2
    elapsed=$((elapsed + 2))
  done
}

bootstrap_database() {
  local password_sql
  password_sql="$(sql_escape "${RERP_DB_PASSWORD}")"
  admin_psql postgres <<SQL
DO \$\$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'rerp') THEN
    CREATE ROLE rerp LOGIN;
  END IF;
END \$\$;
ALTER ROLE rerp PASSWORD '${password_sql}';
SELECT 'CREATE DATABASE rerp OWNER rerp'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'rerp')\gexec
SQL

  admin_psql rerp <<'SQL'
CREATE SCHEMA IF NOT EXISTS rerp AUTHORIZATION rerp;
GRANT CONNECT ON DATABASE rerp TO rerp;
GRANT ALL PRIVILEGES ON SCHEMA rerp TO rerp;
GRANT USAGE, CREATE ON SCHEMA public TO rerp;
ALTER DATABASE rerp SET search_path TO rerp, public;
SQL
}

apply_sql_file() {
  local file="$1"
  echo "Applying ${file#${SUITE_DIR}/}"
  admin_psql rerp <"${file}"
}

apply_migrations() {
  local relative file
  file="${SUITE_DIR}/sql/rls/v1/install.sql"
  [ -f "${file}" ] || { echo "Missing Sesame RLS contract: ${file}" >&2; return 1; }
  apply_sql_file "${file}"

  while IFS= read -r relative || [ -n "${relative}" ]; do
    [[ -z "${relative}" || "${relative}" =~ ^# ]] && continue
    file="${SUITE_DIR}/migrations/${relative}"
    [ -f "${file}" ] || { echo "Migration order lists missing file: ${file}" >&2; return 1; }
    apply_sql_file "${file}"
  done <"${SUITE_DIR}/migrations/apply_order.txt"
}

apply_seeds() {
  local seed
  [ -d "${SUITE_DIR}/seeds" ] || return 0
  while IFS= read -r -d '' seed; do
    apply_sql_file "${seed}"
  done < <(find "${SUITE_DIR}/seeds" -type f -name '*.sql' -print0 | sort -z)
}

grant_application_access() {
  admin_psql rerp <<'SQL'
SET search_path TO rerp;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA rerp TO rerp;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA rerp TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO rerp;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA rerp
  GRANT USAGE, SELECT ON SEQUENCES TO rerp;
SQL
}

verify_application_login() {
  local result
  result="$(PGPASSWORD="${RERP_DB_PASSWORD}" psql -X -h "${PGHOST}" -p "${PGPORT}" \
    -U rerp -d rerp -Atqc 'SELECT 1')"
  [ "${result}" = "1" ] || { echo "RERP database login verification failed" >&2; return 1; }
}

validate_pgpool_contract
wait_for_postgres
bootstrap_database
apply_migrations
apply_seeds
grant_application_access
verify_application_login
echo "RERP Accounting database bootstrap complete"
