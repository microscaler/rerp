# CRM-DB-01: Local CRM databases and the migration pipeline

- **Status:** done
- **Commit:** 286e318e
- **Acceptance criteria:**
  - AC-1 ✅ `up && apply` on an empty server succeeds and `apply` again changes nothing — proven on a fresh
    disposable Postgres 16 (docker) twice in a row; `up` is also idempotent (`up again` in the suite).
    All statements are `IF NOT EXISTS` / `CREATE OR REPLACE` / DO-block-gated.
  - AC-2 ✅ A service role cannot read another service's schema: `crm_audience` → `messaging.campaign` and
    `crm_messaging` → `audience.contact` are both rejected with `permission denied for schema …` (reason
    asserted, not just nonzero exit). Also covered: `CONNECT` is scoped per database (`crm_audience` →
    `crm_intake` rejected with `permission denied for database`), and no `crm_*` role is superuser,
    BYPASSRLS, CREATEDB or CREATEROLE.
  - AC-3 ✅ Works on ms02 (verified here against a throwaway `postgres:16` via `CRM_PG_URL`; the cluster
    Postgres itself is reached the same way once its LB is up). Mac compatibility is by construction, not
    assumption: bash-3.2-safe constructs only (`${@+"$@"}` empty-array guard, no bash-4 features), plain
    `psql`/`sed`/`grep`/`find -maxdepth`, and the default URL is the same localhost Tilt Postgres endpoint
    both machines use. A Mac run is still worth one real confirmation by someone on a Mac (not available here).
- **Files changed:** `microservices/crm/scripts/crm-db.sh` (new), `microservices/crm/scripts/test-crm-db.sh`
  (new), `tooling/tests/test_crm_db.py` (new), this report, `docs/llmwiki/log.md`.
- **Gates:**
  - `bash microservices/crm/scripts/test-crm-db.sh` against `postgres:16` → `test-crm-db: 27 checks passed`
    (fresh container and re-run)
  - `just qa` → `237 passed, 1 skipped` (the skip is `test_crm_db.py` without `CRM_PG_URL`; it runs and
    passes when the URL is set)
  - `python3 openapi/crm/tools/check_specs.py` → contracts untouched and OK
- **Contract or design problems found:** none in the contracts. Two design notes for the supervisor:
  - **Apply runs as the admin, not the service role.** `up` pre-installs schema USAGE + `ALTER DEFAULT
    PRIVILEGES` so every table the admin creates in a service schema is immediately DML-usable by that
    service role and subject to the template's RLS policies (the service role is a non-owner, so
    `ENABLE ROW LEVEL SECURITY` already applies; `FORCE` covers the owner case). The alternative
    (applying as the service role) breaks the template's `CREATE OR REPLACE FUNCTION public.crm_...`
    once a second service re-applies a function owned by the first.
  - **`audit.audit_log` is created by `up`**, not by a service migration: every service's write path needs
    it from its first story and no single service owns the audit store. CRM-RT-03/CRM-WSP-05 extend it
    through migrations; `up` only guarantees it exists with the conventions §3 columns.
- **Needs approval:** none. No new dependencies (pure bash + psql). Service roles carry no repository-visible
  credential: without `CRM_DEV_PASSWORD` they are created LOGIN-without-PASSWORD for the trust-auth dev
  containers (docker `POSTGRES_HOST_AUTH_METHOD=trust`, and the local Tilt Postgres), and with it they take
  that single dev password supplied at run time. Real deployments come from SOPS profiles. This keeps
  GitGuardian's hardcoded-secret scan green instead of arguing the finding away.
- **Notes for the next story:**
  - The migrator's order file is suite-level: `microservices/crm/migrations/apply_order.txt` with
    `<service>/<file>.sql` lines; `crm-db.sh apply` consumes exactly that and falls back to sorted globs
    per service directory while the migrator still refuses (no CRM LifeModel yet).
  - `crm-db.sh apply audience` re-applies a service's migrations to the running dev database only;
    Flux/Tilt rollouts keep using their own bootstrap Jobs — this script is dev tooling, same stance as
    `accounting/scripts/setup-db.sh`.
  - CRM-DB-01 was validated against `postgres:16` per the README's data-model note.
  - CRM-RT-02's outbox relay consumer should call `psql ... <service>` or connect with the service role
    and start every transaction with `rerp_crm_runtime::context::workspace_guard_sql()`.
