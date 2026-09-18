-- Create address_type enum for person/company distinction
CREATE TYPE address_type AS ENUM ('person', 'company');

-- Create addresses table for KYC requirements
-- This table will be used in the dashboard for Know Your Customer (KYC) requirements
-- Addresses can be associated with either a person or a company

CREATE TABLE IF NOT EXISTS addresses (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  type address_type NOT NULL,
  person_id UUID, -- Foreign key to human_names (will be added in next migration)
  company_id UUID REFERENCES companies(id) ON DELETE CASCADE,
  street_address TEXT,
  city TEXT,
  state_province TEXT,
  postal_code TEXT,
  country TEXT,
  is_primary BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  -- Ensure either person_id or company_id is set, but not both
  CONSTRAINT addresses_owner_check CHECK (
    (person_id IS NOT NULL AND company_id IS NULL) OR
    (person_id IS NULL AND company_id IS NOT NULL)
  )
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_addresses_person_id ON addresses(person_id);
CREATE INDEX IF NOT EXISTS idx_addresses_company_id ON addresses(company_id);
CREATE INDEX IF NOT EXISTS idx_addresses_type ON addresses(type);
CREATE INDEX IF NOT EXISTS idx_addresses_is_primary ON addresses(is_primary);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_addresses_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_addresses_updated_at
  BEFORE UPDATE ON addresses
  FOR EACH ROW
  EXECUTE FUNCTION update_addresses_updated_at();

-- Enable Row Level Security
ALTER TABLE addresses ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public forms)
CREATE POLICY "Allow public inserts" ON addresses
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON addresses
  FOR ALL
  TO service_role
  USING (true);

-- Note: Foreign key constraint to human_names.person_id is added in the human_names migration
-- (20250101000010_create_human_names.sql) after the human_names table is created

