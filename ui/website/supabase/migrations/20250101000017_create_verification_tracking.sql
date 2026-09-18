-- Create verification_tracking table for dual verification (email + phone)
-- This table tracks verification state separately from Supabase Auth sessions
-- Both email and phone must be verified before full account access is granted

CREATE TABLE IF NOT EXISTS verification_tracking (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL UNIQUE REFERENCES human_names(id) ON DELETE CASCADE,
  email_verified BOOLEAN DEFAULT false NOT NULL,
  email_verified_at TIMESTAMP WITH TIME ZONE,
  phone_verified BOOLEAN DEFAULT false NOT NULL,
  phone_verified_at TIMESTAMP WITH TIME ZONE,
  both_verified BOOLEAN DEFAULT false NOT NULL,
  both_verified_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_verification_tracking_human_name_id ON verification_tracking(human_name_id);
CREATE INDEX IF NOT EXISTS idx_verification_tracking_both_verified ON verification_tracking(both_verified);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_verification_tracking_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update updated_at
CREATE TRIGGER trigger_update_verification_tracking_updated_at
  BEFORE UPDATE ON verification_tracking
  FOR EACH ROW
  EXECUTE FUNCTION update_verification_tracking_updated_at();

-- Function to mark email as verified
-- Can be called with either human_name_uuid or email address
CREATE OR REPLACE FUNCTION mark_email_verified(human_name_uuid UUID DEFAULT NULL, email_address TEXT DEFAULT NULL)
RETURNS BOOLEAN AS $$
DECLARE
  tracking_record verification_tracking%ROWTYPE;
  found_human_name_id UUID;
BEGIN
  -- Resolve human_name_id if not provided
  IF human_name_uuid IS NULL AND email_address IS NOT NULL THEN
    SELECT hn.id INTO found_human_name_id
    FROM human_names hn
    JOIN email_addresses ea ON hn.email_address_id = ea.id
    WHERE ea.email = LOWER(TRIM(email_address));
    
    IF found_human_name_id IS NULL THEN
      RAISE EXCEPTION 'Human name not found for email: %', email_address;
    END IF;
  ELSIF human_name_uuid IS NOT NULL THEN
    found_human_name_id := human_name_uuid;
  ELSE
    RAISE EXCEPTION 'Either human_name_uuid or email_address must be provided';
  END IF;
  
  -- Get or create verification tracking record
  SELECT * INTO tracking_record
  FROM verification_tracking
  WHERE human_name_id = found_human_name_id;
  
  IF tracking_record IS NULL THEN
    INSERT INTO verification_tracking (human_name_id, email_verified, email_verified_at)
    VALUES (found_human_name_id, true, NOW())
    RETURNING * INTO tracking_record;
  ELSE
    UPDATE verification_tracking
    SET email_verified = true,
        email_verified_at = COALESCE(email_verified_at, NOW()),
        updated_at = NOW()
    WHERE human_name_id = found_human_name_id
    RETURNING * INTO tracking_record;
  END IF;
  
  -- Check if both are now verified
  IF tracking_record.phone_verified AND tracking_record.email_verified THEN
    UPDATE verification_tracking
    SET both_verified = true,
        both_verified_at = COALESCE(both_verified_at, NOW()),
        updated_at = NOW()
    WHERE human_name_id = found_human_name_id;
    RETURN true;
  END IF;
  
  RETURN false;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to mark phone as verified
-- Can be called with either human_name_uuid or phone number
CREATE OR REPLACE FUNCTION mark_phone_verified(human_name_uuid UUID DEFAULT NULL, phone_number TEXT DEFAULT NULL)
RETURNS BOOLEAN AS $$
DECLARE
  tracking_record verification_tracking%ROWTYPE;
  found_human_name_id UUID;
BEGIN
  -- Resolve human_name_id if not provided
  IF human_name_uuid IS NULL AND phone_number IS NOT NULL THEN
    SELECT mn.human_name_id INTO found_human_name_id
    FROM mobile_numbers mn
    WHERE mn.phone_number = phone_number;
    
    IF found_human_name_id IS NULL THEN
      RAISE EXCEPTION 'Human name not found for phone: %', phone_number;
    END IF;
  ELSIF human_name_uuid IS NOT NULL THEN
    found_human_name_id := human_name_uuid;
  ELSE
    RAISE EXCEPTION 'Either human_name_uuid or phone_number must be provided';
  END IF;
  
  -- Get or create verification tracking record
  SELECT * INTO tracking_record
  FROM verification_tracking
  WHERE human_name_id = found_human_name_id;
  
  IF tracking_record IS NULL THEN
    INSERT INTO verification_tracking (human_name_id, phone_verified, phone_verified_at)
    VALUES (found_human_name_id, true, NOW())
    RETURNING * INTO tracking_record;
  ELSE
    UPDATE verification_tracking
    SET phone_verified = true,
        phone_verified_at = COALESCE(phone_verified_at, NOW()),
        updated_at = NOW()
    WHERE human_name_id = found_human_name_id
    RETURNING * INTO tracking_record;
  END IF;
  
  -- Check if both are now verified
  IF tracking_record.phone_verified AND tracking_record.email_verified THEN
    UPDATE verification_tracking
    SET both_verified = true,
        both_verified_at = COALESCE(both_verified_at, NOW()),
        updated_at = NOW()
    WHERE human_name_id = found_human_name_id;
    RETURN true;
  END IF;
  
  RETURN false;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to check if both are verified
CREATE OR REPLACE FUNCTION is_dual_verified(human_name_uuid UUID)
RETURNS BOOLEAN AS $$
BEGIN
  RETURN EXISTS (
    SELECT 1 FROM verification_tracking
    WHERE human_name_id = human_name_uuid
      AND both_verified = true
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permissions
GRANT EXECUTE ON FUNCTION mark_email_verified(UUID, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION mark_phone_verified(UUID, TEXT) TO anon, authenticated;
GRANT EXECUTE ON FUNCTION is_dual_verified(UUID) TO anon, authenticated;

-- Enable Row Level Security
ALTER TABLE verification_tracking ENABLE ROW LEVEL SECURITY;

-- Create policy to allow users to read their own verification status
CREATE POLICY "Users can read own verification status" ON verification_tracking
  FOR SELECT
  TO authenticated
  USING (
    human_name_id = get_current_human_name_id()
  );

-- Create policy to allow service role full access
CREATE POLICY "Service role full access" ON verification_tracking
  FOR ALL
  TO service_role
  USING (true);

-- Note: Updates are handled via the functions above, not direct table updates

