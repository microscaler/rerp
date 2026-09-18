-- Update admins table to reference platform_users instead of human_names
-- This ensures platform admins are completely separate from customer users
-- 
-- Migration steps:
-- 1. Add new platform_user_id column
-- 2. Migrate existing data (if any) - create platform_users from existing admins
-- 3. Drop old human_name_id column
-- 4. Add constraint to ensure platform_user_id is set

-- Step 1: Add new platform_user_id column
ALTER TABLE admins
  ADD COLUMN IF NOT EXISTS platform_user_id UUID REFERENCES platform_users(id) ON DELETE CASCADE;

-- Step 2: Create platform_users from existing admins (if any exist)
-- This migration assumes admins.human_name_id exists and we need to migrate
-- For new installations, this will be empty
DO $$
DECLARE
  admin_record RECORD;
  new_platform_user_id UUID;
BEGIN
  FOR admin_record IN 
    SELECT a.id, a.human_name_id, a.role, hn.first_name, hn.surname, ea.email
    FROM admins a
    JOIN human_names hn ON a.human_name_id = hn.id
    JOIN email_addresses ea ON hn.email_address_id = ea.id
    WHERE a.platform_user_id IS NULL
  LOOP
    -- Create platform_user from human_name data
    INSERT INTO platform_users (email, first_name, surname, is_active)
    VALUES (
      admin_record.email,
      admin_record.first_name,
      admin_record.surname,
      true
    )
    ON CONFLICT (email) DO UPDATE
    SET first_name = EXCLUDED.first_name,
        surname = EXCLUDED.surname
    RETURNING id INTO new_platform_user_id;
    
    -- Update admin record with platform_user_id
    UPDATE admins
    SET platform_user_id = new_platform_user_id
    WHERE id = admin_record.id;
  END LOOP;
END $$;

-- Step 3: Make platform_user_id NOT NULL and UNIQUE
ALTER TABLE admins
  ALTER COLUMN platform_user_id SET NOT NULL,
  ADD CONSTRAINT admins_platform_user_id_unique UNIQUE (platform_user_id);

-- Step 4: Drop old human_name_id column (after ensuring all data is migrated)
ALTER TABLE admins
  DROP COLUMN IF EXISTS human_name_id;

-- Step 5: Update indexes
DROP INDEX IF EXISTS idx_admins_human_name_id;
CREATE INDEX IF NOT EXISTS idx_admins_platform_user_id ON admins(platform_user_id);

-- Note: After this migration, platform admins are completely isolated from customer users
-- They exist only in platform_users table and are referenced by admins table

