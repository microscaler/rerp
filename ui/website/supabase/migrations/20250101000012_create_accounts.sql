-- Create accounts table
-- An account represents a user account in the system
-- An account can have one plan (one account = one plan, but multiple accounts can share the same plan)

CREATE TABLE IF NOT EXISTS accounts (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL UNIQUE REFERENCES human_names(id) ON DELETE CASCADE,
  plan_id UUID REFERENCES plans(id) ON DELETE SET NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_accounts_human_name_id ON accounts(human_name_id);
CREATE INDEX IF NOT EXISTS idx_accounts_plan_id ON accounts(plan_id);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_accounts_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_accounts_updated_at
  BEFORE UPDATE ON accounts
  FOR EACH ROW
  EXECUTE FUNCTION update_accounts_updated_at();

-- Enable Row Level Security
ALTER TABLE accounts ENABLE ROW LEVEL SECURITY;

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON accounts
  FOR ALL
  TO service_role
  USING (true);

-- Note: Accounts are typically created through authenticated flows, not anonymous inserts

