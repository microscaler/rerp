#!/usr/bin/env bash
# Local CRM database administration helper (CRM-DB-01). Flux does not run this script.
#
# One command creates the three CRM databases with a login role per service, applies the CRM migrations in
# `apply_order.txt` order, resets them, and opens psql as a service role — so every data-model story verifies
# isolation the same way (openapi/crm/docs/14-data-model.md §1–§2, ADR 004).
#
#   ./crm-db.sh up                create databases, roles, schemas, grants, audit schema (idempotent)
#   ./crm-db.sh apply [service]   apply migrations/<service>/ files in apply_order.txt order
#   ./crm-db.sh reset             drop the three databases and the crm_* roles, then up
#   ./crm-db.sh psql <service>    a psql shell as that service's role
#
# Layout (14-data-model.md §1):
#   - database `crm`         — schemas audience, messaging, automation, sales, acquisition, workspace, audit.
#   - database `crm_insights`— schema insights.
#   - database `crm_intake`  — schema intake.
#   - one login role per service (crm_audience … crm_intake): no superuser, no BYPASSRLS, CONNECT only on its
#     database, USAGE only on its schema. The `audit` schema grants INSERT to every service role and SELECT to
#     crm_workspace only. crm/edge gets no role at all (ADR 004: the edge has no database credentials).
#   - migrations are applied by the $CRM_PG_URL admin role into the service's schema; up pre-installs schema
#     grants and default privileges so the service role owns DML on the tables and row-level security
#     (enabled per table by each service's 0002_rls_and_infrastructure.sql) applies to it.
#
# Connection:
#   CRM_PG_URL   admin URL used for bootstrap and apply.
#                Default: the local Tilt Postgres at postgres://postgres:***@localhost:5432/postgres
#                (same endpoint both ms02 and a Mac reach against their Tilt/dev cluster).
#   CRM_DEV_PASSWORD  optional. When set, every crm_* role authenticates with this password. When unset,
#                roles are created WITHOUT a password (LOGIN only) and connect through the trust pg_hba
#                of the local throwaway/Tilt Postgres — no credential ever appears in this repository.
#                Real deployments provision roles through SOPS profiles, never through this file.
#   CRM_MIGRATIONS_DIR  optional override of microservices/crm/migrations (tests use fixtures).
#
# The admin URL role must be the `postgres` superuser that owns the databases and schemas (the ALTER DEFAULT
# PRIVILEGES statements above are pinned to it); real deployments provision their own roles via SOPS profiles.
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
CRM_PG_URL="${CRM_PG_URL:-postgres://postgres:postgres@localhost:5432/postgres}"
CRM_MIGRATIONS_DIR="${CRM_MIGRATIONS_DIR:-${SCRIPT_DIR}/../migrations}"

# Services in database `crm` (14-data-model.md §1). insights and intake live in their own databases.
CRM_DB_SERVICES=(audience messaging automation sales acquisition workspace)
ALL_SERVICES=(audience messaging automation sales acquisition insights workspace intake)

service_database() {
    case "$1" in
        insights) echo crm_insights ;;
        intake)   echo crm_intake ;;
        *)        echo crm ;;
    esac
}

role_password() {
    # Empty output = roles are created LOGIN-without-PASSWORD and rely on the dev cluster's trust pg_hba.
    local password="${CRM_DEV_PASSWORD:-}"
    if [[ "$password" == *"'"* ]]; then
        echo "crm-db.sh: CRM_DEV_PASSWORD must not contain a single quote" >&2
        exit 1
    fi
    echo "$password"
}

admin_psql() {
    # admin_psql <database> [extra psql args...] — runs SQL from stdin with ON_ERROR_STOP.
    local database="$1"
    shift
    psql "$(url_for_db "$database")" -v ON_ERROR_STOP=1 "$@" -f -
}

url_for_db() {
    # The admin URL with its database segment swapped, query parameters (sslmode etc.) preserved (psql
    # silently ignores a URI positional once -d is given, so the database must live IN the URL).
    local database="$1" base="${CRM_PG_URL%%\?*}"
    local query=""
    [ "$base" != "$CRM_PG_URL" ] && query="?${CRM_PG_URL#*\?}"
    if [[ "$base" == */* ]]; then
        echo "${base%/*}/${database}${query}"
    else
        echo "${base}/${database}${query}"
    fi
}

service_psql_url() {
    local service="$1"
    # Same host/port and query parameters as the admin URL, credentials swapped to the service role.
    local base="${CRM_PG_URL%%\?*}" query=""
    [ "$base" != "$CRM_PG_URL" ] && query="?${CRM_PG_URL#*\?}"
    local without_scheme="${base#*://}"
    local authority="${without_scheme%%/*}"
    local hostport="${authority##*@}"
    local password creds
    password="$(role_password)"
    creds="crm_${service}${password:+:${password}}"
    echo "postgres://${creds}@${hostport}/$(service_database "$service")${query}"
}

usage() {
    sed -n '2,16p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
    exit "${1:-1}"
}

cmd_up() {
    local database
    for database in crm crm_insights crm_intake; do
        if ! crm_database_exists "$database"; then
            # CREATE DATABASE cannot run inside a transaction or take IF NOT EXISTS; single-runner dev tool, so
            # the query gate only guards the re-run case — concurrent `up` invocations are out of scope.
            admin_psql postgres <<<"CREATE DATABASE ${database} OWNER postgres;" >/dev/null
            echo "created database ${database}"
        fi
    done

    local service
    for service in "${ALL_SERVICES[@]}"; do
        local role="crm_${service}"
        local pw pw_clause
        pw="$(role_password)"
        pw_clause="${pw:+PASSWORD '${pw}'}"
        admin_psql postgres <<SQL >/dev/null
DO \$\$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = '${role}') THEN
        CREATE ROLE ${role} LOGIN ${pw_clause}
            NOSUPERUSER NOCREATEDB NOCREATEROLE NOBYPASSRLS NOINHERIT
            VALID UNTIL 'infinity';
    ELSE
        ALTER ROLE ${role} WITH LOGIN ${pw_clause}
            NOSUPERUSER NOCREATEDB NOCREATEROLE NOBYPASSRLS NOINHERIT;
    END IF;
END
\$\$;
SQL
    done

    for service in "${ALL_SERVICES[@]}"; do
        database="$(service_database "$service")"
        local role="crm_${service}"
        admin_psql "$database" <<SQL >/dev/null
-- Only this service's role (and the admin) may reach this database.
REVOKE CONNECT ON DATABASE ${database} FROM PUBLIC;
GRANT CONNECT ON DATABASE ${database} TO ${role};

-- The workspace guard the RLS policies call; every transaction sets crm.workspace_id first.
CREATE SCHEMA IF NOT EXISTS public;
CREATE OR REPLACE FUNCTION public.crm_current_workspace_id() RETURNS uuid
    LANGUAGE sql STABLE
AS \$\$ SELECT nullif(current_setting('crm.workspace_id', true), '')::uuid \$\$;

-- The service's schema; the migrations (applied by the admin) create its tables, the service role does DML.
CREATE SCHEMA IF NOT EXISTS ${service} AUTHORIZATION postgres;
GRANT USAGE ON SCHEMA ${service} TO ${role};
GRANT EXECUTE ON FUNCTION public.crm_current_workspace_id() TO ${role};

-- Tables the admin creates in this schema are immediately usable by the service role (and not by anyone else).
REVOKE ALL ON SCHEMA ${service} FROM PUBLIC;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA ${service}
    GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO ${role};
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA ${service}
    GRANT SELECT, UPDATE ON SEQUENCES TO ${role};
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA ${service}
    GRANT EXECUTE ON FUNCTIONS TO ${role};
SQL
    done

    # The audit schema (14-data-model.md §1, workspace.yaml `audit_entries`): the table itself belongs to the
    # crm/workspace migrations (CRM-WSP-00 owns that database), so `up` only provisions the schema and its
    # grants. Default privileges make every table the admin later creates in this schema INSERT-able by every
    # service role and SELECT-able by crm_workspace only — append-only audit from the first write path
    # (09-api-conventions.md §3, US_55_39), without crm-db.sh ever owning the table.
    admin_psql crm <<'SQL' >/dev/null
CREATE SCHEMA IF NOT EXISTS audit AUTHORIZATION postgres;
REVOKE ALL ON SCHEMA audit FROM PUBLIC;
-- insights and intake keep their data in their own databases but still write audit entries here (the single
-- audit store in `crm`), so they need CONNECT on `crm` — and nothing else in it (no schema grants).
GRANT CONNECT ON DATABASE crm TO crm_insights, crm_intake;
GRANT USAGE ON SCHEMA audit TO crm_audience, crm_messaging, crm_automation, crm_sales,
                              crm_acquisition, crm_workspace, crm_insights, crm_intake;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA audit
    GRANT INSERT ON TABLES TO crm_audience, crm_messaging, crm_automation, crm_sales,
                               crm_acquisition, crm_workspace, crm_insights, crm_intake;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA audit
    GRANT SELECT ON TABLES TO crm_workspace;
SQL

    echo "up: databases, roles, schemas and grants ready"
}

# Gate for CREATE DATABASE: succeeds only when the database provably does not exist; a failed QUERY aborts
# (exit 3) instead of being mistaken for a missing database, so a transient connection problem cannot make
# `up` try to create an existing database.
crm_database_exists() {
    local name="$1" out
    if ! out="$(psql "$(url_for_db postgres)" -v ON_ERROR_STOP=1 -tA \
        -c "SELECT 1 FROM pg_database WHERE datname = '${name}'" 2>&1)"; then
        echo "crm-db.sh: cannot query pg_database for '${name}': ${out}" >&2
        return 3
    fi
    [ "$out" = "1" ]
}

migration_files() {
    # Echo, in apply order, the migration files for one service (relative to $CRM_MIGRATIONS_DIR).
    local service="$1"
    local dir="${CRM_MIGRATIONS_DIR}/${service}"
    [ -d "$dir" ] || return 0
    if [ -f "${CRM_MIGRATIONS_DIR}/apply_order.txt" ]; then
        # The migrator's suite-level order file; lines are `<service>/<file>.sql`, comments start with #.
        grep -E "^[[:space:]]*${service}/" "${CRM_MIGRATIONS_DIR}/apply_order.txt" | sed 's/^[[:space:]]*//' || true
        return 0
    fi
    if [ -f "${dir}/apply_order.txt" ]; then
        # A per-service order file; lines are file names inside the service directory.
        grep -vE '^[[:space:]]*(#|$)' "${dir}/apply_order.txt" | sed "s|^|${service}/|"
        return 0
    fi
    # No order file yet (the migrator refuses until the first LifeModel exists): sorted file names.
    find "$dir" -maxdepth 1 -name '*.sql' -print | sort | sed "s|^${CRM_MIGRATIONS_DIR}/||"
}

cmd_apply() {
    # ${@+"$"} keeps empty "$@" legal under set -u on bash 3.2 (a Mac's /bin/bash).
    local services=(${@+"$@"})
    if [ "${#services[@]}" -eq 0 ]; then
        services=("${ALL_SERVICES[@]}")
    fi
    local service
    for service in "${services[@]}"; do
        if [[ ! " ${ALL_SERVICES[*]} " == *" ${service} "* ]]; then
            echo "apply: unknown service '${service}' (known: ${ALL_SERVICES[*]})" >&2
            exit 1
        fi
        local database files file count=0
        database="$(service_database "$service")"
        files="$(migration_files "$service")"
        if [ -z "$files" ]; then
            echo "apply ${service}: no migrations yet (skipped)"
            continue
        fi
        while IFS= read -r file; do
            [ -n "$file" ] || continue
            echo "apply ${service}: ${file} -> ${database}"
            admin_psql "$database" < "${CRM_MIGRATIONS_DIR}/${file}"
            count=$((count + 1))
        done <<<"$files"
        echo "apply ${service}: ${count} file(s) applied"
    done
}

cmd_reset() {
    local database service role
    # Non-destructive pre-check: DROP ROLE fails if any crm_* role still owns or holds grants on objects in
    # a database outside the three CRM ones. Detect that BEFORE dropping anything, so a failed reset never
    # leaves the CRM databases recreated without their roles and grants.
    local external
    external="$(admin_psql postgres -tA <<'SQL'
SELECT coalesce(string_agg(d.datname, ', '), '')
    FROM pg_database d
    WHERE d.datname NOT IN ('template0','template1','postgres','crm','crm_insights','crm_intake')
      AND EXISTS (SELECT 1 FROM pg_shdepend sd
                  WHERE sd.dbid = d.oid AND sd.refclassid = 'pg_authid'::regclass
                    AND sd.refobjid IN (SELECT oid FROM pg_roles WHERE rolname LIKE 'crm\_%'));
SQL
)"
    if [ -n "$external" ]; then
        echo "reset: crm_* roles hold privileges in other database(s): ${external} — REVOKE there first; nothing dropped" >&2
        exit 1
    fi
    for database in crm crm_insights crm_intake; do
        admin_psql postgres <<SQL >/dev/null
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '${database}' AND pid <> pg_backend_pid();
DROP DATABASE IF EXISTS ${database};
SQL
        admin_psql postgres <<<"CREATE DATABASE ${database} OWNER postgres;" >/dev/null
        echo "reset: recreated database ${database}"
    done
    for service in "${ALL_SERVICES[@]}"; do
        role="crm_${service}"
        admin_psql postgres <<<"DROP ROLE IF EXISTS ${role};" >/dev/null
    done
    cmd_up
}

cmd_psql() {
    local service="${1:-}"
    if [[ ! " ${ALL_SERVICES[*]} " == *" ${service} "* ]]; then
        echo "psql: unknown service '${service}' (known: ${ALL_SERVICES[*]})" >&2
        exit 1
    fi
    exec psql "$(service_psql_url "$service")"
}

main() {
    local command="${1:-}"
    shift || true
    case "$command" in
        up)    cmd_up ;;
        apply) cmd_apply "$@" ;;
        reset) cmd_reset ;;
        psql)  cmd_psql "$@" ;;
        "")    usage 1 ;;
        *)     echo "crm-db.sh: unknown command '${command}'" >&2; usage 1 ;;
    esac
}

main "$@"
