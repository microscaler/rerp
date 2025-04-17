-- Update contact_messages table to use foreign key for company
-- This migration:
-- 1. Adds new foreign key column
-- 2. Migrates existing data
-- 3. Drops old TEXT column

-- Step 1: Add new foreign key column (nullable initially for migration)
ALTER TABLE contact_messages
  ADD COLUMN IF NOT EXISTS company_id UUID REFERENCES companies(id) ON DELETE SET NULL;

-- Step 2: Migrate existing company data
-- Upsert companies from existing contact_messages.company values
INSERT INTO companies (name)
SELECT DISTINCT company
FROM contact_messages
WHERE company IS NOT NULL
  AND company != ''
ON CONFLICT (name) DO NOTHING;

-- Update contact_messages with company_id foreign keys
UPDATE contact_messages cm
SET company_id = c.id
FROM companies c
WHERE cm.company = c.name
  AND cm.company IS NOT NULL
  AND cm.company != '';

-- Step 3: Create index on foreign key
CREATE INDEX IF NOT EXISTS idx_contact_messages_company_id ON contact_messages(company_id);

-- Step 4: Drop old TEXT column
ALTER TABLE contact_messages
  DROP COLUMN IF EXISTS company;

