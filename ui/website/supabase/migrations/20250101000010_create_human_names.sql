-- Create human_names table for storing person information
-- This table represents individual users/people in the system
-- Links to email_addresses via foreign key (one-to-one relationship)

CREATE TABLE IF NOT EXISTS human_names (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email_address_id UUID NOT NULL UNIQUE REFERENCES email_addresses(id) ON DELETE CASCADE,
  first_name TEXT NOT NULL,
  surname TEXT NOT NULL,
  username TEXT UNIQUE, -- Optional username (can be null)
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_human_names_email_address_id ON human_names(email_address_id);
CREATE INDEX IF NOT EXISTS idx_human_names_username ON human_names(username) WHERE username IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_human_names_full_name ON human_names(first_name, surname);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_human_names_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_human_names_updated_at
  BEFORE UPDATE ON human_names
  FOR EACH ROW
  EXECUTE FUNCTION update_human_names_updated_at();

-- Enable Row Level Security
ALTER TABLE human_names ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public forms)
CREATE POLICY "Allow public inserts" ON human_names
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON human_names
  FOR ALL
  TO service_role
  USING (true);

-- Now add foreign key constraint to addresses table
ALTER TABLE addresses
  ADD CONSTRAINT addresses_person_id_fkey 
  FOREIGN KEY (person_id) REFERENCES human_names(id) ON DELETE CASCADE;

