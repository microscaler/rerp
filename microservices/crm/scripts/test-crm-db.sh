#!/usr/bin/env bash
# CRM-DB-01 acceptance tests for crm-db.sh (docs/crm/stories/CRM-DB-01).
#
# Runs against a throwaway Postgres 16. Create one first, e.g.:
#   docker run -d --name crm-db-test -e POSTGRES_HOST_AUTH_METHOD=trust -p 127.0.0.1:55432:5432 postgres:16
#   CRM_PG_URL=postgres://postgres@127.0.0.1:55432/postgres ./test-crm-db.sh
#
# Covers:
#   AC-1  up && apply on an empty server succeeds; applying again changes nothing.
#   AC-2  a service role cannot read another service's schema (nor connect to another service's database).
#   grants  audit: INSERT for every service role, SELECT only for crm_workspace; edge has no role at all.
#   reset  drop and recreate works.
#
# Negative probes assert the REJECTION REASON, never just a nonzero exit.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRM_PG_URL="${CRM_PG_URL:-postgres://postgres@localhost:5432/postgres}"
export CRM_PG_URL

# This suite DROPS and recreates the three CRM databases. Refuse to run unless the caller confirms the
# target is disposable, so a default pytest run pointed at a live development Postgres cannot destroy it.
if [ "${CRM_TEST_DISPOSABLE:-}" != "yes" ]; then
    echo "test-crm-db.sh: this suite drops and recreates the CRM databases on CRM_PG_URL." >&2
    echo "Confirm the target is a throwaway instance with CRM_TEST_DISPOSABLE=yes." >&2
    exit 1
fi
SCRIPT="${SCRIPT_DIR}/crm-db.sh"

FIXTURES="$(mktemp -d)"
trap 'rm -rf "$FIXTURES"' EXIT
export CRM_MIGRATIONS_DIR="$FIXTURES"

PASS_COUNT=0
ok() { PASS_COUNT=$((PASS_COUNT + 1)); echo "  ok: $*"; }
step() { echo; echo "── $*"; }

fail_with_reason() {
    # fail_with_reason <description> <expected-substring-in-stderr> <command...>
    local description="$1" needle="$2"
    shift 2
    local stderr_file rc=0
    stderr_file="$(mktemp)"
    "$@" >/dev/null 2>"$stderr_file" || rc=$?
    if [ "$rc" -eq 0 ]; then
        echo "  FAIL: ${description} — command unexpectedly succeeded" >&2
        rm -f "$stderr_file"; exit 1
    fi
    if ! grep -qF "$needle" "$stderr_file"; then
        echo "  FAIL: ${description} — stderr lacked '${needle}':" >&2
        cat "$stderr_file" >&2
        rm -f "$stderr_file"; exit 1
    fi
    rm -f "$stderr_file"
    ok "${description} (rejected: ${needle})"
}

succeeds() {
    local description="$1"
    shift
    if "$@" >/dev/null 2>&1; then
        ok "$description"
    else
        echo "  FAIL: $description" >&2
        "$@"   # rerun without suppression so the failure is visible
        exit 1
    fi
}

admin() {
    local base="${CRM_PG_URL%%\?*}"
    psql "${base%/*}/$1" -v ON_ERROR_STOP=1 -tA -c "$2"
}

service_db() {
    case "$1" in
        insights) echo crm_insights ;;
        intake)   echo crm_intake ;;
        *)        echo crm ;;
    esac
}

as_service() {
    # as_service <service> <sql> [<sql>...] — psql as the service role, same host/port as the admin URL.
    # (The local dev convention: the admin URL host is 127.0.0.1/localhost.)
    local service="$1"
    shift
    as_service_db "$service" "$(service_db "$service")" "$@"
}

as_service_db() {
    # as_service_db <service> <database> <sql>... — any database, for the negative CONNECT probes.
    local service="$1" database="$2"
    shift 2
    local without_scheme="${CRM_PG_URL#*://}"
    local authority="${without_scheme%%/*}"
    local hostport="${authority##*@}"
    if [[ "$hostport" != 127.0.0.1:* && "$hostport" != localhost:* ]]; then
        echo "test-crm-db: host '${hostport}' not supported; run against a local throwaway Postgres" >&2
        exit 1
    fi
    local args=()
    local sql
    for sql in "$@"; do
        args+=(-c "$sql")
    done
    local pw_env=()
    [ -n "${CRM_DEV_PASSWORD:-}" ] && pw_env=(PGPASSWORD="***")
    env ${pw_env[@]+"${pw_env[@]}"} psql -h 127.0.0.1 -p "${hostport##*:}" \
        -U "crm_${service}" -d "$database" -v ON_ERROR_STOP=1 -tA "${args[@]}"
}

# ── fixtures: generated-style tables + the real RLS template for two services ───────────────────────────
mkdir -p "$FIXTURES/audience" "$FIXTURES/messaging"
cat > "$FIXTURES/audience/0001_generated_entities.sql" <<'SQL'
CREATE TABLE IF NOT EXISTS audience.contact (
    id            uuid PRIMARY KEY,
    workspace_id  uuid NOT NULL,
    email         text NOT NULL
);
SQL
cat > "$FIXTURES/messaging/0001_generated_entities.sql" <<'SQL'
CREATE TABLE IF NOT EXISTS messaging.campaign (
    id            uuid PRIMARY KEY,
    workspace_id  uuid NOT NULL,
    name          text NOT NULL
);
SQL
for service in audience messaging; do
    sed "s/{{schema}}/${service}/g" "${SCRIPT_DIR}/../migrations/templates/rls_and_infrastructure.sql" \
        > "${FIXTURES}/${service}/0002_rls_and_infrastructure.sql"
done
cat > "$FIXTURES/apply_order.txt" <<'TXT'
# CRM-DB-01 fixture order
audience/0001_generated_entities.sql
audience/0002_rls_and_infrastructure.sql
messaging/0001_generated_entities.sql
messaging/0002_rls_and_infrastructure.sql
TXT

step "AC-1: up && apply on an empty server; apply again changes nothing"
succeeds "up" bash "$SCRIPT" up
succeeds "apply" bash "$SCRIPT" apply
succeeds "apply again" bash "$SCRIPT" apply
# A second `up` must also be a no-op.
succeeds "up again" bash "$SCRIPT" up
ok "AC-1"

step "fixtures landed through apply_order.txt"
for schema in audience messaging; do
    count="$(admin crm "SELECT count(*) FROM information_schema.tables WHERE table_schema = '${schema}' AND table_name IN ('outbox','processed_events','idempotency_keys')")"
    if [ "$count" = "3" ]; then
        ok "${schema}: outbox + processed_events + idempotency_keys (from the real RLS template)"
    else
        echo "  FAIL: ${schema} infrastructure tables: ${count} of 3" >&2
        exit 1
    fi
done
role_owns="$(admin crm "SELECT has_table_privilege('crm_audience', 'audience.contact', 'INSERT')")"
[ "$role_owns" = "t" ] && ok "crm_audience has INSERT on audience.contact (grants pre-installed by up)" \
    || { echo "  FAIL: crm_audience lacks INSERT on audience.contact" >&2; exit 1; }

step "service role can use its own schema; rows are RLS-gated"
ws=01940f6e-2f00-7abc-8901-234567890abc
succeeds "crm_audience inserts a contact with the guard set" as_service audience \
    "SET crm.workspace_id='${ws}';" \
    "INSERT INTO audience.contact (id, workspace_id, email) VALUES (gen_random_uuid(), '${ws}', 'a@example');"
# Without the guard the policy must show zero rows (fail closed).
visible_without_guard="$(as_service audience "SELECT count(*) FROM audience.contact")"
[ "$visible_without_guard" = "0" ] && ok "without crm.workspace_id the service role sees 0 of the inserted row (fail closed)" \
    || { echo "  FAIL: rows visible without guard: $visible_without_guard" >&2; exit 1; }

step "AC-2: no service role can read another service's schema"
fail_with_reason "crm_audience reads messaging.campaign" "permission denied for schema messaging" \
    as_service audience "SELECT * FROM messaging.campaign"
fail_with_reason "crm_messaging reads audience.contact" "permission denied for schema audience" \
    as_service messaging "SELECT * FROM audience.contact"
ok "AC-2"

step "database isolation: crm_audience cannot connect to crm_intake"
fail_with_reason "crm_audience connects to crm_intake" 'permission denied for database "crm_intake"' \
    as_service_db audience crm_intake "SELECT 1"
ok "database CONNECT is scoped per service"

step "audit grants"
# audit.audit_entries is created by crm/workspace's own migrations (CRM-WSP-00 owns it); the admin stand-in
# here proves the schema-level default privileges `up` installed behave exactly as workspace.yaml requires.
succeeds "admin creates the audit_entries stand-in (as CRM-WSP-00 would)" admin crm \
    "CREATE TABLE IF NOT EXISTS audit.audit_entries (
         workspace_id uuid NOT NULL,
         id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
         service text NOT NULL,
         entity varchar(64) NOT NULL,
         entity_id text NOT NULL,
         action text NOT NULL,
         actor_kind text NOT NULL,
         actor_id varchar(64) NOT NULL,
         changes jsonb,
         request_id text,
         created_at timestamptz NOT NULL DEFAULT now()
     )"
succeeds "crm_audience INSERTs an audit row (default privilege)" as_service audience \
    "INSERT INTO audit.audit_entries (workspace_id, service, entity, entity_id, action, actor_kind, actor_id, changes)
     VALUES ('01940f6e-2f00-7abc-8901-234567890abc', 'crm_audience', 'contact',
             gen_random_uuid()::text, 'CREATE', 'user', 'crm-db-test', '{}'::jsonb)"
succeeds "crm_workspace SELECTs the audit log" as_service workspace "SELECT count(*) FROM audit.audit_entries"
fail_with_reason "crm_audience SELECTs the audit log" "permission denied for table audit_entries" \
    as_service audience "SELECT * FROM audit.audit_entries"
# The cross-database auditors: insights/intake hold their data elsewhere but write here (needs CONNECT on
# `crm`, and still nothing beyond the audit schema).
succeeds "crm_insights INSERTs an audit row into crm (cross-database auditor)" as_service_db insights crm \
    "INSERT INTO audit.audit_entries (workspace_id, service, entity, entity_id, action, actor_kind, actor_id, changes)
     VALUES ('01940f6e-2f00-7abc-8901-234567890abc', 'crm_insights', 'segment',
             gen_random_uuid()::text, 'CREATE', 'system', 'crm-db-test', '{}'::jsonb)"
fail_with_reason "crm_insights SELECTs from crm outside audit" "permission denied for schema audience" \
    as_service_db insights crm "SELECT count(*) FROM audience.contact"
ok "audit: default privileges give INSERT to services (incl. cross-database auditors), SELECT only to crm_workspace"

step "edge has no database role (ADR 004)"
edge_role="$(admin postgres "SELECT count(*) FROM pg_roles WHERE rolname = 'crm_edge'")"
[ "$edge_role" = "0" ] && ok "no crm_edge role exists" \
    || { echo "  FAIL: crm_edge role exists" >&2; exit 1; }

step "no service role bypasses or escalates"
for svc_flags in "rolbypassrls false" "rolsuper false" "rolcreatedb false" "rolcreaterole false"; do
    flag="${svc_flags%% *}"
    bad="$(admin postgres "SELECT count(*) FROM pg_roles WHERE rolname LIKE 'crm\\_%' AND ${flag}")"
    [ "$bad" = "0" ] && ok "no crm_* role has ${flag}" \
        || { echo "  FAIL: ${bad} crm_* roles have ${flag}" >&2; exit 1; }
done

step "reset: drop and recreate"
succeeds "reset" bash "$SCRIPT" reset
succeeds "apply after reset" bash "$SCRIPT" apply
survivors="$(admin crm "SELECT count(*) FROM audience.contact")"  # runs as admin; RLS off for superuser
[ "$survivors" = "0" ] && ok "audience.contact is empty after reset" \
    || { echo "  FAIL: rows survived reset: $survivors" >&2; exit 1; }

echo
echo "test-crm-db: ${PASS_COUNT} checks passed"
