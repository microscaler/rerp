-- Create admins table
-- The admin role has one user (one-to-one relationship with human_names)
-- An account can have one admin (one-to-one relationship)

CREATE TABLE IF NOT EXISTS admins (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL UNIQUE REFERENCES human_names(id) ON DELETE CASCADE,
  account_id UUID UNIQUE REFERENCES accounts(id) ON DELETE SET NULL,
  role role_type DEFAULT 'admin' NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_admins_human_name_id ON admins(human_name_id);
CREATE INDEX IF NOT EXISTS idx_admins_account_id ON admins(account_id);
CREATE INDEX IF NOT EXISTS idx_admins_role ON admins(role);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_admins_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_admins_updated_at
  BEFORE UPDATE ON admins
  FOR EACH ROW
  EXECUTE FUNCTION update_admins_updated_at();

-- Enable Row Level Security
ALTER TABLE admins ENABLE ROW LEVEL SECURITY;

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON admins
  FOR ALL
  TO service_role
  USING (true);

-- Note: Admin accounts are created through secure admin flows, not anonymous inserts

