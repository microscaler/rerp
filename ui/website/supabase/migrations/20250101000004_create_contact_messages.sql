-- Create contact_messages table for storing contact form submissions
-- This table references email_addresses via foreign key

CREATE TABLE IF NOT EXISTS contact_messages (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email_address_id UUID NOT NULL REFERENCES email_addresses(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  company TEXT,
  message TEXT NOT NULL,
  recaptcha_token TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create index on email_address_id for faster joins
CREATE INDEX IF NOT EXISTS idx_contact_messages_email_address_id ON contact_messages(email_address_id);

-- Create index on created_at for time-based queries
CREATE INDEX IF NOT EXISTS idx_contact_messages_created_at ON contact_messages(created_at DESC);

-- Enable Row Level Security
ALTER TABLE contact_messages ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public contact form)
CREATE POLICY "Allow public inserts" ON contact_messages
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow service role to read all records
CREATE POLICY "Allow service role read" ON contact_messages
  FOR SELECT
  TO service_role
  USING (true);

-- Note: Anonymous users cannot read messages (no SELECT policy for anon)
-- This ensures contact messages are private

