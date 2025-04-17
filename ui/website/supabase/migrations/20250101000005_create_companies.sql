-- Create companies table for storing company information
-- This table normalizes company names and allows tracking company-related data

CREATE TABLE IF NOT EXISTS companies (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create index on name for faster lookups
CREATE INDEX IF NOT EXISTS idx_companies_name ON companies(name);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_companies_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_companies_updated_at
  BEFORE UPDATE ON companies
  FOR EACH ROW
  EXECUTE FUNCTION update_companies_updated_at();

-- Enable Row Level Security
ALTER TABLE companies ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public website forms)
CREATE POLICY "Allow public inserts" ON companies
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow service role to read all records
CREATE POLICY "Allow service role read" ON companies
  FOR SELECT
  TO service_role
  USING (true);

-- Create upsert function for companies (similar to email_addresses)
CREATE OR REPLACE FUNCTION upsert_company(company_name TEXT)
RETURNS UUID AS $$
DECLARE
  company_id UUID;
BEGIN
  -- Try to get existing company
  SELECT id INTO company_id
  FROM companies
  WHERE name = TRIM(company_name);
  
  -- If not found, insert new company
  IF company_id IS NULL THEN
    INSERT INTO companies (name)
    VALUES (TRIM(company_name))
    RETURNING id INTO company_id;
  END IF;
  
  RETURN company_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permission to anon role
GRANT EXECUTE ON FUNCTION upsert_company(TEXT) TO anon;

