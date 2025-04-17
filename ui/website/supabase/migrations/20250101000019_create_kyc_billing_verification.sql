-- Create KYC billing verification table
-- This table tracks credit card billing verification for KYC requirements
-- Verification is required after the 14-day trial period ends

CREATE TABLE IF NOT EXISTS kyc_billing_verification (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id UUID NOT NULL UNIQUE REFERENCES accounts(id) ON DELETE CASCADE,
  human_name_id UUID NOT NULL REFERENCES human_names(id) ON DELETE CASCADE,
  stripe_payment_method_id UUID REFERENCES stripe_payment_methods(id) ON DELETE SET NULL,
  verification_status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'in_progress', 'verified', 'failed', 'expired'
  verification_attempts INTEGER DEFAULT 0 NOT NULL,
  last_verification_attempt_at TIMESTAMP WITH TIME ZONE,
  verified_at TIMESTAMP WITH TIME ZONE,
  verification_failed_at TIMESTAMP WITH TIME ZONE,
  failure_reason TEXT,
  trial_ended_at TIMESTAMP WITH TIME ZONE, -- When the 14-day trial ended
  verification_required_at TIMESTAMP WITH TIME ZONE, -- When verification becomes required
  verification_expires_at TIMESTAMP WITH TIME ZONE, -- When verification expires (if applicable)
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  CONSTRAINT kyc_verification_status_check CHECK (
    verification_status IN ('pending', 'in_progress', 'verified', 'failed', 'expired')
  )
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_kyc_billing_verification_account_id ON kyc_billing_verification(account_id);
CREATE INDEX IF NOT EXISTS idx_kyc_billing_verification_human_name_id ON kyc_billing_verification(human_name_id);
CREATE INDEX IF NOT EXISTS idx_kyc_billing_verification_status ON kyc_billing_verification(verification_status);
CREATE INDEX IF NOT EXISTS idx_kyc_billing_verification_verification_required_at ON kyc_billing_verification(verification_required_at);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_kyc_billing_verification_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger
CREATE TRIGGER trigger_update_kyc_billing_verification_updated_at
  BEFORE UPDATE ON kyc_billing_verification
  FOR EACH ROW
  EXECUTE FUNCTION update_kyc_billing_verification_updated_at();

-- Function to create KYC verification record when trial ends
-- This should be called by a trigger or application logic when trial period ends
CREATE OR REPLACE FUNCTION create_kyc_verification_on_trial_end(account_uuid UUID)
RETURNS UUID AS $$
DECLARE
  verification_id UUID;
  account_human_name_id UUID;
BEGIN
  -- Get human_name_id from account
  SELECT human_name_id INTO account_human_name_id
  FROM accounts
  WHERE id = account_uuid;
  
  IF account_human_name_id IS NULL THEN
    RAISE EXCEPTION 'Account not found: %', account_uuid;
  END IF;
  
  -- Create verification record
  INSERT INTO kyc_billing_verification (
    account_id,
    human_name_id,
    verification_status,
    trial_ended_at,
    verification_required_at
  )
  VALUES (
    account_uuid,
    account_human_name_id,
    'pending',
    NOW(),
    NOW() + INTERVAL '7 days' -- Give user 7 days to complete verification after trial ends
  )
  RETURNING id INTO verification_id;
  
  RETURN verification_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to mark KYC verification as verified
CREATE OR REPLACE FUNCTION mark_kyc_verified(account_uuid UUID, payment_method_uuid UUID)
RETURNS VOID AS $$
BEGIN
  UPDATE kyc_billing_verification
  SET verification_status = 'verified',
      verified_at = NOW(),
      stripe_payment_method_id = payment_method_uuid,
      updated_at = NOW()
  WHERE account_id = account_uuid;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to mark KYC verification as failed
CREATE OR REPLACE FUNCTION mark_kyc_failed(account_uuid UUID, reason TEXT)
RETURNS VOID AS $$
BEGIN
  UPDATE kyc_billing_verification
  SET verification_status = 'failed',
      verification_failed_at = NOW(),
      failure_reason = reason,
      verification_attempts = verification_attempts + 1,
      last_verification_attempt_at = NOW(),
      updated_at = NOW()
  WHERE account_id = account_uuid;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to check if KYC verification is required
CREATE OR REPLACE FUNCTION is_kyc_verification_required(account_uuid UUID)
RETURNS BOOLEAN AS $$
BEGIN
  RETURN EXISTS (
    SELECT 1 FROM kyc_billing_verification
    WHERE account_id = account_uuid
      AND verification_status IN ('pending', 'in_progress', 'failed')
      AND (verification_required_at IS NULL OR verification_required_at <= NOW())
      AND (verification_expires_at IS NULL OR verification_expires_at > NOW())
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to check if KYC verification is complete
CREATE OR REPLACE FUNCTION is_kyc_verified(account_uuid UUID)
RETURNS BOOLEAN AS $$
BEGIN
  RETURN EXISTS (
    SELECT 1 FROM kyc_billing_verification
    WHERE account_id = account_uuid
      AND verification_status = 'verified'
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permissions
GRANT EXECUTE ON FUNCTION create_kyc_verification_on_trial_end(UUID) TO service_role;
GRANT EXECUTE ON FUNCTION mark_kyc_verified(UUID, UUID) TO service_role;
GRANT EXECUTE ON FUNCTION mark_kyc_failed(UUID, TEXT) TO service_role;
GRANT EXECUTE ON FUNCTION is_kyc_verification_required(UUID) TO authenticated, service_role;
GRANT EXECUTE ON FUNCTION is_kyc_verified(UUID) TO authenticated, service_role;

-- Enable Row Level Security
ALTER TABLE kyc_billing_verification ENABLE ROW LEVEL SECURITY;

-- Create policy to allow users to read their own KYC verification status
CREATE POLICY "Users can read own KYC verification" ON kyc_billing_verification
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

-- Create policy to allow billing admin to read account KYC verification
CREATE POLICY "Billing admin can read account KYC verification" ON kyc_billing_verification
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT account_id FROM account_roles
      WHERE human_name_id = get_current_human_name_id()
        AND role = 'billing_admin'
    )
  );

-- Create policy to allow platform roles to read all KYC verifications
CREATE POLICY "Platform roles can read KYC verifications" ON kyc_billing_verification
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Create policy to allow service role full access
CREATE POLICY "Service role full access" ON kyc_billing_verification
  FOR ALL
  TO service_role
  USING (true);

