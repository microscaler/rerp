-- Create email_captures table with foreign key to email_addresses
-- This table stores email captures from website forms
-- All email addresses are stored in email_addresses table and referenced here

CREATE TABLE IF NOT EXISTS email_captures (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email_address_id UUID NOT NULL REFERENCES email_addresses(id) ON DELETE CASCADE,
  source TEXT NOT NULL CHECK (source IN ('hero', 'exit_intent', 'free_trial')),
  name TEXT,
  company TEXT,
  plan TEXT,
  recaptcha_token TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create index on email_address_id for faster joins
CREATE INDEX IF NOT EXISTS idx_email_captures_email_address_id ON email_captures(email_address_id);

-- Create index on source for analytics queries
CREATE INDEX IF NOT EXISTS idx_email_captures_source ON email_captures(source);

-- Create index on created_at for time-based queries
CREATE INDEX IF NOT EXISTS idx_email_captures_created_at ON email_captures(created_at DESC);

-- Enable Row Level Security
ALTER TABLE email_captures ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public website forms)
-- This allows the website to insert records using the anon key
CREATE POLICY "Allow public inserts" ON email_captures
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow service role to read all records
-- This is for backend/admin access using service role key
CREATE POLICY "Allow service role read" ON email_captures
  FOR SELECT
  TO service_role
  USING (true);

-- Note: Anonymous users cannot read records (no SELECT policy for anon)
-- This ensures email addresses are not exposed to the public

