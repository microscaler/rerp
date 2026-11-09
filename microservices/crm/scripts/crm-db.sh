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
#                Default: the local Tilt Postgres at postgres://postgres:postgres@localhost:5432/postgres
#                (same endpoint both ms02 and a Mac reach against their Tilt/dev cluster).
#   CRM_DEV_PASSWORD  optional; when set, every crm_* role gets this password on up.
#                Default (per role): the role's own name — a DEV-ONLY convention matching the shared local
#                postgres; real deployments provision credentials through SOPS profiles, never through this file.
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
    local service="$1"
    local password="${CRM_DEV_PASSWORD:-crm_${service}}"
    # Single quotes would break the CREATE/ALTER ROLE statements; the dev convention never needs them.
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
    # The admin URL with its database segment swapped (psql silently ignores a URI positional once -d is
    # given, so the database must live IN the URL).
    local database="$1" base="${CRM_PG_URL%%\?*}"
    if [[ "$base" == */* ]]; then
        echo "${base%/*}/${database}"
    else
        echo "${base}/${database}"
    fi
}

service_psql_url() {
    local service="$1"
    local base="${CRM_PG_URL}"
    # Same host/port as the admin URL, credentials swapped to the service role.
    local without_scheme="${base#*://}"
    local authority="${without_scheme%%/*}"
    local hostport="${authority##*@}"
    local password
    password="$(role_password "$service")"
    echo "postgres://crm_${service}:${password}@${hostport}/$(service_database "$service")"
}

usage() {
    sed -n '2,16p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
    exit "${1:-1}"
}

cmd_up() {
    local database
    for database in crm crm_insights crm_intake; do
        if ! psql_admin_exists_query crm_database_exists "$database" >/dev/null; then
            # CREATE DATABASE cannot run inside a transaction or take IF NOT EXISTS; single-runner dev tool, so
            # the query gate only guards the re-run case — concurrent `up` invocations are out of scope.
            admin_psql postgres <<<"CREATE DATABASE ${database} OWNER postgres;" >/dev/null
            echo "created database ${database}"
        fi
    done

    local service
    for service in "${ALL_SERVICES[@]}"; do
        local role="crm_${service}"
        local pw
        pw="$(role_password "$service")"
        admin_psql postgres <<SQL >/dev/null
DO \$\$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = '${role}') THEN
        CREATE ROLE ${role} LOGIN PASSWORD '${pw}'
            NOSUPERUSER NOCREATEDB NOCREATEROLE NOBYPASSRLS NOINHERIT
            VALID UNTIL 'infinity';
    ELSE
        ALTER ROLE ${role} WITH LOGIN PASSWORD '${pw}'
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

    # The audit log (14-data-model.md §1: schema `audit` in `crm`): every write appends a row in its own
    # transaction; only crm/workspace serves it back (CRM-WSP-05). The table is created here so every service's
    # insert path works from the first migration; later stories extend it with migrations, never by hand here.
    admin_psql crm <<'SQL' >/dev/null
CREATE SCHEMA IF NOT EXISTS audit AUTHORIZATION postgres;
REVOKE ALL ON SCHEMA audit FROM PUBLIC;
CREATE TABLE IF NOT EXISTS audit.audit_log (
    id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    workspace_id  uuid NOT NULL,
    actor_kind    text NOT NULL,
    actor_id      varchar(64) NOT NULL,
    entity        text NOT NULL,
    entity_id     uuid NOT NULL,
    action        text NOT NULL,
    changes       jsonb NOT NULL,
    request_id    text,
    occurred_at   timestamptz NOT NULL DEFAULT now()
);
-- Every service role may append; only the workspace service may read (09-api-conventions.md §3, US_55_39).
GRANT USAGE ON SCHEMA audit TO crm_audience, crm_messaging, crm_automation, crm_sales,
                              crm_acquisition, crm_workspace, crm_insights, crm_intake;
GRANT INSERT ON audit.audit_log TO crm_audience, crm_messaging, crm_automation, crm_sales,
                                  crm_acquisition, crm_workspace, crm_insights, crm_intake;
REVOKE SELECT, UPDATE, DELETE, TRUNCATE ON audit.audit_log FROM PUBLIC;
GRANT SELECT ON audit.audit_log TO crm_workspace;
SQL

    echo "up: databases, roles, schemas and grants ready"
}

# Helper for the database-existence gate: psql_admin_exists_query <kind> <name> succeeds when it exists.
psql_admin_exists_query() {
    local kind="$1" name="$2"
    case "$kind" in
        crm_database_exists)
            psql "$(url_for_db postgres)" -v ON_ERROR_STOP=1 -tA \
                -c "SELECT 1 FROM pg_database WHERE datname = '${name}'" | grep -qx 1
            ;;
        *) return 2 ;;
    esac
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
