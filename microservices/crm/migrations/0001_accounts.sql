-- CRM accounts: the customer record, and the ledger of conversions that created them (ADR 003).
--
-- A customer exists in RERP only because a lead converted: crm_conversions records every conversion once (its id is
-- deterministic per funnel lead or Stripe checkout) and points at the account it created or matched. Conversions are
-- immutable; accounts are edited in RERP after they cross.
--
-- Prerequisite: the Sesame RLS contract (accounting/sql/rls/v1/install.sql), as for accounting. Idempotent: safe to
-- re-run.

BEGIN;

CREATE TABLE IF NOT EXISTS crm_accounts (
    id                 uuid PRIMARY KEY,
    tenant_id          varchar(200) NOT NULL,
    organization_id    uuid NOT NULL,
    name               varchar(200) NOT NULL,
    contact_name       varchar(200),
    email              varchar(254) NOT NULL,
    street             varchar(200) NOT NULL,
    street2            varchar(200),
    city               varchar(100) NOT NULL,
    zip                varchar(20),
    region             varchar(100),
    country_code       char(2) NOT NULL CHECK (country_code ~ '^[A-Z]{2}$'),
    tax_id             varchar(50),
    active             boolean NOT NULL DEFAULT true,
    funnel_lead_id     uuid,
    stripe_customer_id varchar(255),
    created_at         timestamptz NOT NULL DEFAULT now(),
    updated_at         timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT crm_accounts_tenant_id_unique UNIQUE (tenant_id, id),
    CONSTRAINT crm_accounts_funnel_lead_unique UNIQUE (tenant_id, funnel_lead_id),
    CONSTRAINT crm_accounts_stripe_customer_unique UNIQUE (tenant_id, stripe_customer_id)
);

CREATE INDEX IF NOT EXISTS idx_crm_accounts_name ON crm_accounts (tenant_id, lower(name));

CREATE TABLE IF NOT EXISTS crm_conversions (
    conversion_id      uuid PRIMARY KEY,
    tenant_id          varchar(200) NOT NULL,
    account_id         uuid NOT NULL,
    reason             varchar(20) NOT NULL CHECK (reason IN ('stripe_checkout', 'order', 'contract')),
    reference          varchar(200) NOT NULL,
    funnel_lead_id     uuid,
    stripe_customer_id varchar(255),
    converted_by       varchar(200),
    converted_at       timestamptz NOT NULL,
    signed_by          varchar(32),
    payload            jsonb NOT NULL,
    received_at        timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT crm_conversions_account_fk FOREIGN KEY (tenant_id, account_id)
        REFERENCES crm_accounts (tenant_id, id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_crm_conversions_account ON crm_conversions (tenant_id, account_id);

CREATE OR REPLACE FUNCTION public.crm_reject_conversion_mutation()
RETURNS trigger
LANGUAGE plpgsql
SECURITY INVOKER
SET search_path = pg_catalog
AS $$
BEGIN
    RAISE EXCEPTION 'crm conversions are immutable' USING ERRCODE = '55000';
END;
$$;

DROP TRIGGER IF EXISTS crm_conversions_immutable ON crm_conversions;
CREATE TRIGGER crm_conversions_immutable
BEFORE UPDATE OR DELETE ON crm_conversions
FOR EACH ROW EXECUTE FUNCTION public.crm_reject_conversion_mutation();

ALTER TABLE crm_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE crm_accounts FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS crm_accounts_tenant ON crm_accounts;
CREATE POLICY crm_accounts_tenant ON crm_accounts
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

ALTER TABLE crm_conversions ENABLE ROW LEVEL SECURITY;
ALTER TABLE crm_conversions FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS crm_conversions_tenant ON crm_conversions;
CREATE POLICY crm_conversions_tenant ON crm_conversions
FOR ALL TO PUBLIC
USING (tenant_id = public.sesame_current_tenant_id())
WITH CHECK (tenant_id = public.sesame_current_tenant_id());

COMMIT;
