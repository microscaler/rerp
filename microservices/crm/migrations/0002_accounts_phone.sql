-- The customer's phone, as Stripe collects it at checkout or staff record it on conversion (ADR 003). Idempotent.
ALTER TABLE crm_accounts ADD COLUMN IF NOT EXISTS phone varchar(40);
