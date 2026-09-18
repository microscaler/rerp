-- Create email_addresses table
-- This is the central table for all email addresses used across the application
-- Other tables will reference this table via foreign key

CREATE TABLE IF NOT EXISTS email_addresses (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  verified BOOLEAN DEFAULT FALSE NOT NULL,
  verification_token TEXT,
  verification_sent_at TIMESTAMP WITH TIME ZONE,
  verified_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_email_addresses_email ON email_addresses(email);

-- Create index on verified status
CREATE INDEX IF NOT EXISTS idx_email_addresses_verified ON email_addresses(verified);

-- Create index on verification_token for verification lookups
CREATE INDEX IF NOT EXISTS idx_email_addresses_verification_token ON email_addresses(verification_token) WHERE verification_token IS NOT NULL;

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_email_addresses_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_email_addresses_updated_at
  BEFORE UPDATE ON email_addresses
  FOR EACH ROW
  EXECUTE FUNCTION update_email_addresses_updated_at();

-- Enable Row Level Security
ALTER TABLE email_addresses ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public website forms)
CREATE POLICY "Allow public inserts" ON email_addresses
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow anonymous updates (for email verification)
CREATE POLICY "Allow public updates for verification" ON email_addresses
  FOR UPDATE
  TO anon
  USING (true)
  WITH CHECK (true);

-- Create policy to allow service role to read all records
CREATE POLICY "Allow service role read" ON email_addresses
  FOR SELECT
  TO service_role
  USING (true);

-- Create function to upsert email address and return ID
-- This function creates an email address if it doesn't exist, or returns the existing one
CREATE OR REPLACE FUNCTION upsert_email_address(email_text TEXT)
RETURNS UUID AS $$
DECLARE
  email_id UUID;
BEGIN
  -- Try to get existing email address
  SELECT id INTO email_id
  FROM email_addresses
  WHERE email = LOWER(TRIM(email_text));
  
  -- If not found, insert new email address
  IF email_id IS NULL THEN
    INSERT INTO email_addresses (email, verification_token)
    VALUES (LOWER(TRIM(email_text)), encode(gen_random_bytes(24), 'base64'))
    RETURNING id INTO email_id;
  END IF;
  
  RETURN email_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permission to anonymous users for upsert function
GRANT EXECUTE ON FUNCTION upsert_email_address(TEXT) TO anon;

-- Note: Anonymous users cannot SELECT email addresses (no SELECT policy for anon)
-- This ensures email addresses are not exposed to the public

