-- Template for microservices/crm/migrations/<service>/0002_rls_and_infrastructure.sql.
-- Copy it, replace every audience with the service's schema (audience, messaging, automation, sales, acquisition,
-- workspace, insights, intake), and append the service's exceptions at the end. Idempotent: safe to re-apply.
-- Design: openapi/crm/docs/14-data-model.md §1 and §2; ADR 004 (workspaces, RLS on workspace_id).

-- The workspace the current transaction acts in. crm-context runs
--   SELECT set_config('crm.workspace_id', $1, true)
-- at the start of every transaction; without it every policy below sees no rows (fail closed).
CREATE OR REPLACE FUNCTION public.crm_current_workspace_id() RETURNS uuid
    LANGUAGE sql STABLE
AS $$ SELECT nullif(current_setting('crm.workspace_id', true), '')::uuid $$;

CREATE SCHEMA IF NOT EXISTS audience;

-- Events leave through the outbox, in the same transaction as the change (crm-events relays after commit).
CREATE TABLE IF NOT EXISTS audience.outbox (
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
CREATE INDEX IF NOT EXISTS outbox_unpublished ON audience.outbox (occurred_at) WHERE published_at IS NULL;

-- Idempotent consumers: an event is processed at most once per service.
CREATE TABLE IF NOT EXISTS audience.processed_events (
    event_id      uuid PRIMARY KEY,
    event_type    text NOT NULL,
    processed_at  timestamptz NOT NULL DEFAULT now()
);

-- Idempotency-Key replay for side-effecting commands (24 hours).
CREATE TABLE IF NOT EXISTS audience.idempotency_keys (
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
-- Exception (CRM-RT-02): the relay reads unpublished rows for EVERY workspace at once, which no single
-- workspace GUC can satisfy. The outbox is append-only event intent inside the service's OWN schema — the
-- schema is its isolation boundary — so its policy is permissive for reads and still forces every row to
-- carry a workspace tag. processed_events keeps the blanket workspace-isolation policy (the consumer sets
-- the guard before it touches the table).
ALTER TABLE audience.outbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS outbox_relay_access ON audience.outbox;
CREATE POLICY outbox_relay_access ON audience.outbox
    AS PERMISSIVE FOR ALL
    USING (true)
    WITH CHECK (workspace_id IS NOT NULL);

DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT c.table_name FROM information_schema.columns c
        WHERE c.table_schema = 'audience' AND c.column_name = 'workspace_id'
          AND c.table_name <> 'outbox'
    LOOP
        EXECUTE format('ALTER TABLE audience.%I ENABLE ROW LEVEL SECURITY', t);
        EXECUTE format('ALTER TABLE audience.%I FORCE ROW LEVEL SECURITY', t);
        EXECUTE format('DROP POLICY IF EXISTS workspace_isolation ON audience.%I', t);
        EXECUTE format('CREATE POLICY workspace_isolation ON audience.%I '
                       'USING (workspace_id = public.crm_current_workspace_id()) '
                       'WITH CHECK (workspace_id = public.crm_current_workspace_id())', t);
    END LOOP;
END $$;

-- Service-specific exceptions go below (for example crm/workspace: memberships are visible to their own user;
-- crm/intake: rate_limits has no workspace). Keep them explicit and commented.
