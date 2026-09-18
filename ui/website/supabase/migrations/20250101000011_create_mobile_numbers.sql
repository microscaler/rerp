-- Create mobile_numbers table for SMS confirmations
-- This table stores mobile phone numbers for users (one-to-one relationship with human_names)
-- Will be used in job application forms for SMS confirmations

CREATE TABLE IF NOT EXISTS mobile_numbers (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL UNIQUE REFERENCES human_names(id) ON DELETE CASCADE,
  phone_number TEXT NOT NULL, -- E.164 format recommended (e.g., +1234567890)
  country_code TEXT, -- ISO 3166-1 alpha-2 country code (e.g., 'US', 'GB')
  is_verified BOOLEAN DEFAULT false,
  verification_code TEXT, -- For SMS verification
  verification_sent_at TIMESTAMP WITH TIME ZONE,
  verified_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_mobile_numbers_human_name_id ON mobile_numbers(human_name_id);
CREATE INDEX IF NOT EXISTS idx_mobile_numbers_phone_number ON mobile_numbers(phone_number);
CREATE INDEX IF NOT EXISTS idx_mobile_numbers_is_verified ON mobile_numbers(is_verified);
CREATE INDEX IF NOT EXISTS idx_mobile_numbers_verification_code ON mobile_numbers(verification_code) WHERE verification_code IS NOT NULL;

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_mobile_numbers_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_mobile_numbers_updated_at
  BEFORE UPDATE ON mobile_numbers
  FOR EACH ROW
  EXECUTE FUNCTION update_mobile_numbers_updated_at();

-- Enable Row Level Security
ALTER TABLE mobile_numbers ENABLE ROW LEVEL SECURITY;

-- Create policy to allow anonymous inserts (for public forms)
CREATE POLICY "Allow public inserts" ON mobile_numbers
  FOR INSERT
  TO anon
  WITH CHECK (true);

-- Create policy to allow anonymous updates (for verification)
CREATE POLICY "Allow public updates" ON mobile_numbers
  FOR UPDATE
  TO anon
  USING (true)
  WITH CHECK (true);

-- Create policy to allow service role full access
CREATE POLICY "Allow service role full access" ON mobile_numbers
  FOR ALL
  TO service_role
  USING (true);

