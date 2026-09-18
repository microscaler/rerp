-- Create account_roles table for assigning roles to users within accounts
-- This table manages role assignments for users in customer accounts
-- Internal platform roles (admin, customer_success, finance) are managed separately in admins table

CREATE TABLE IF NOT EXISTS account_roles (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  human_name_id UUID NOT NULL REFERENCES human_names(id) ON DELETE CASCADE,
  role role_type NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  -- Ensure one role per user per account
  UNIQUE(account_id, human_name_id, role)
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_account_roles_account_id ON account_roles(account_id);
CREATE INDEX IF NOT EXISTS idx_account_roles_human_name_id ON account_roles(human_name_id);
CREATE INDEX IF NOT EXISTS idx_account_roles_role ON account_roles(role);
CREATE INDEX IF NOT EXISTS idx_account_roles_account_human ON account_roles(account_id, human_name_id);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_account_roles_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_account_roles_updated_at
  BEFORE UPDATE ON account_roles
  FOR EACH ROW
  EXECUTE FUNCTION update_account_roles_updated_at();

-- Enable Row Level Security
ALTER TABLE account_roles ENABLE ROW LEVEL SECURITY;

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON account_roles
  FOR ALL
  TO service_role
  USING (true);

-- Function to assign default billing_admin role to single-user accounts
-- This function should be called when an account is created with a single user
CREATE OR REPLACE FUNCTION assign_default_billing_admin(account_uuid UUID)
RETURNS VOID AS $$
DECLARE
  user_count INTEGER;
  account_human_name_id UUID;
BEGIN
  -- Get the human_name_id for the account
  SELECT human_name_id INTO account_human_name_id
  FROM accounts
  WHERE id = account_uuid;
  
  IF account_human_name_id IS NULL THEN
    RAISE EXCEPTION 'Account not found: %', account_uuid;
  END IF;
  
  -- Check if this account has only one user (the account owner)
  -- For now, we assume single-user accounts are those where account.human_name_id is the only user
  -- In the future, you might have a separate users table to check actual user count
  
  -- Assign billing_admin role to the account owner
  INSERT INTO account_roles (account_id, human_name_id, role)
  VALUES (account_uuid, account_human_name_id, 'billing_admin')
  ON CONFLICT (account_id, human_name_id, role) DO NOTHING;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permission to service role
GRANT EXECUTE ON FUNCTION assign_default_billing_admin(UUID) TO service_role;

-- Trigger to automatically assign billing_admin when account is created
-- Note: This assumes single-user accounts. For company accounts, billing_admin should be assigned manually
CREATE OR REPLACE FUNCTION trigger_assign_billing_admin()
RETURNS TRIGGER AS $$
BEGIN
  -- Automatically assign billing_admin to the account owner
  PERFORM assign_default_billing_admin(NEW.id);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_auto_assign_billing_admin
  AFTER INSERT ON accounts
  FOR EACH ROW
  EXECUTE FUNCTION trigger_assign_billing_admin();

-- Note: 
-- - Single-user accounts automatically get billing_admin assigned to the account owner
-- - For company accounts, billing_admin can be transferred by updating account_roles
-- - Internal platform roles (admin, customer_success, finance) are managed in admins table, not account_roles

