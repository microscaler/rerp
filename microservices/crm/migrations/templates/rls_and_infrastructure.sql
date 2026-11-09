-- Template for microservices/crm/migrations/<service>/0002_rls_and_infrastructure.sql.
-- Copy it, replace every {{schema}} with the service's schema (audience, messaging, automation, sales, acquisition,
-- workspace, insights, intake), and append the service's exceptions at the end. Idempotent: safe to re-apply.
-- Design: openapi/crm/docs/14-data-model.md §1 and §2; ADR 004 (workspaces, RLS on workspace_id).

-- The workspace the current transaction acts in. crm-context runs
--   SELECT set_config('crm.workspace_id', $1, true)
-- at the start of every transaction; without it every policy below sees no rows (fail closed).
CREATE OR REPLACE FUNCTION public.crm_current_workspace_id() RETURNS uuid
    LANGUAGE sql STABLE
AS $$ SELECT nullif(current_setting('crm.workspace_id', true), '')::uuid $$;

CREATE SCHEMA IF NOT EXISTS {{schema}};

-- Events leave through the outbox, in the same transaction as the change (crm-events relays after commit).
CREATE TABLE IF NOT EXISTS {{schema}}.outbox (
    id            uuid PRIMARY KEY,
    workspace_id  uuid NOT NULL,
    event_type    text NOT NULL,
    entity_id     text NOT NULL,
    payload       jsonb NOT NULL,
    occurred_at   timestamptz NOT NULL DEFAULT now(),
    published_at  timestamptz,
    attempts      integer NOT NULL DEFAULT 0,
    last_error    text
);
CREATE INDEX IF NOT EXISTS outbox_unpublished ON {{schema}}.outbox (occurred_at) WHERE published_at IS NULL;

-- Idempotent consumers: an event is processed at most once per service.
CREATE TABLE IF NOT EXISTS {{schema}}.processed_events (
    event_id      uuid PRIMARY KEY,
    event_type    text NOT NULL,
    processed_at  timestamptz NOT NULL DEFAULT now()
);

-- Idempotency-Key replay for side-effecting commands (24 hours).
CREATE TABLE IF NOT EXISTS {{schema}}.idempotency_keys (
    workspace_id  uuid NOT NULL,
    key           varchar(128) NOT NULL,
    request_hash  text NOT NULL,
    status        integer NOT NULL,
    response      jsonb,
    created_at    timestamptz NOT NULL DEFAULT now(),
    expires_at    timestamptz NOT NULL DEFAULT now() + interval '24 hours',
    PRIMARY KEY (workspace_id, key)
);

-- Row-level security on every table that has workspace_id (the infrastructure tables too: the relay and the
-- consumer set the workspace before they touch them).
DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT c.table_name FROM information_schema.columns c
        WHERE c.table_schema = '{{schema}}' AND c.column_name = 'workspace_id'
    LOOP
        EXECUTE format('ALTER TABLE {{schema}}.%I ENABLE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE {{schema}}.%I FORCE ROW LEVEL SECURITY', t);
        EXECUTE format('DROP POLICY IF EXISTS workspace_isolation ON {{schema}}.%I', t);
        EXECUTE format('CREATE POLICY workspace_isolation ON {{schema}}.%I '
                       'USING (workspace_id = public.crm_current_workspace_id()) '
                       'WITH CHECK (workspace_id = public.crm_current_workspace_id())', t);
    END LOOP;
END $$;

-- Service-specific exceptions go below (for example crm/workspace: memberships are visible to their own user;
-- crm/intake: rate_limits has no workspace). Keep them explicit and commented.
