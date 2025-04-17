-- Create plans table as a lookup table for subscription plans
-- This table stores predefined plan options

CREATE TABLE IF NOT EXISTS plans (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  code TEXT NOT NULL UNIQUE, -- 'starter', 'growth', 'pro', 'enterprise'
  name TEXT NOT NULL, -- 'Starter', 'Growth', 'Professional', 'Enterprise'
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Insert predefined plans
INSERT INTO plans (code, name) VALUES
  ('starter', 'Starter'),
  ('growth', 'Growth'),
  ('pro', 'Professional'),
  ('enterprise', 'Enterprise')
ON CONFLICT (code) DO NOTHING;

-- Create index on code for faster lookups
CREATE INDEX IF NOT EXISTS idx_plans_code ON plans(code);

-- Enable Row Level Security
ALTER TABLE plans ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous reads (for form dropdowns, etc.)
CREATE POLICY "Allow public reads" ON plans
  FOR SELECT
  TO anon
  USING (true);

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON plans
  FOR ALL
  TO service_role
  USING (true);

