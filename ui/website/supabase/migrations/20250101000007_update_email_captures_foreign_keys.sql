-- Update email_captures table to use foreign keys for company and plan
-- This migration:
-- 1. Adds new foreign key columns
-- 2. Migrates existing data
-- 3. Drops old TEXT columns
-- 4. Makes foreign keys NOT NULL where appropriate

-- Step 1: Add new foreign key columns (nullable initially for migration)
ALTER TABLE email_captures
  ADD COLUMN IF NOT EXISTS company_id UUID REFERENCES companies(id) ON DELETE SET NULL,
  ADD COLUMN IF NOT EXISTS plan_id UUID REFERENCES plans(id) ON DELETE SET NULL;

-- Step 2: Migrate existing company data
-- Upsert companies from existing email_captures.company values
INSERT INTO companies (name)
SELECT DISTINCT company
FROM email_captures
WHERE company IS NOT NULL
  AND company != ''
ON CONFLICT (name) DO NOTHING;

-- Update email_captures with company_id foreign keys
UPDATE email_captures ec
SET company_id = c.id
FROM companies c
WHERE ec.company = c.name
  AND ec.company IS NOT NULL
  AND ec.company != '';

-- Step 3: Migrate existing plan data
-- Plans should already exist from the plans table creation
-- Update email_captures with plan_id foreign keys
UPDATE email_captures ec
SET plan_id = p.id
FROM plans p
WHERE ec.plan = p.code
  AND ec.plan IS NOT NULL
  AND ec.plan != '';

-- Step 4: Create indexes on foreign keys
CREATE INDEX IF NOT EXISTS idx_email_captures_company_id ON email_captures(company_id);
CREATE INDEX IF NOT EXISTS idx_email_captures_plan_id ON email_captures(plan_id);

-- Step 5: Drop old TEXT columns
ALTER TABLE email_captures
  DROP COLUMN IF EXISTS company,
  DROP COLUMN IF EXISTS plan;

-- Note: We keep the 'name' column as TEXT since names are user-specific
-- and don't need normalization (unlike companies which can be reused)

