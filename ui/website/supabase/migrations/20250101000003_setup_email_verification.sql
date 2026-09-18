-- Setup email verification system
-- This migration creates triggers and functions for email verification
-- Supabase Auth can be used, but this provides a custom verification flow

-- Create function to generate verification token
CREATE OR REPLACE FUNCTION generate_verification_token()
RETURNS TEXT AS $$
BEGIN
  -- Generate a secure random token (32 characters, URL-safe)
  RETURN encode(gen_random_bytes(24), 'base64')
    || encode(gen_random_bytes(8), 'base64');
END;
$$ LANGUAGE plpgsql;

-- Create function to send verification email
-- This will be called by a trigger when a new email is inserted
CREATE OR REPLACE FUNCTION send_verification_email()
RETURNS TRIGGER AS $$
BEGIN
  -- Generate verification token if not provided
  IF NEW.verification_token IS NULL THEN
    NEW.verification_token := generate_verification_token();
  END IF;
  
  -- Set verification_sent_at timestamp
  NEW.verification_sent_at := NOW();
  
  -- TODO: Integrate with Supabase Edge Functions or external email service
  -- For now, the token is generated and stored
  -- You can create a Supabase Edge Function to send the email:
  -- - Create edge function: supabase/functions/send-verification-email
  -- - Call it via HTTP or database trigger
  
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically generate verification token on insert
CREATE TRIGGER trigger_send_verification_email
  BEFORE INSERT ON email_addresses
  FOR EACH ROW
  WHEN (NEW.verified = FALSE)
  EXECUTE FUNCTION send_verification_email();

-- Create function to verify email address
CREATE OR REPLACE FUNCTION verify_email_address(token TEXT)
RETURNS BOOLEAN AS $$
DECLARE
  email_record RECORD;
BEGIN
  -- Find email address by verification token
  SELECT * INTO email_record
  FROM email_addresses
  WHERE verification_token = token
    AND verified = FALSE;
  
  -- If not found, return false
  IF NOT FOUND THEN
    RETURN FALSE;
  END IF;
  
  -- Verify the email
  UPDATE email_addresses
  SET 
    verified = TRUE,
    verified_at = NOW(),
    verification_token = NULL -- Clear token after verification
  WHERE id = email_record.id;
  
  RETURN TRUE;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant execute permission to anonymous users for verification
GRANT EXECUTE ON FUNCTION verify_email_address(TEXT) TO anon;

-- Note: To send actual verification emails, you'll need to:
-- 1. Create a Supabase Edge Function for sending emails
-- 2. Or use Supabase's built-in Auth email templates
-- 3. Or integrate with an external email service (SendGrid, Resend, etc.)
-- 
-- Example Edge Function call (from trigger or application):
-- SELECT net.http_post(
--   url := 'https://your-project.supabase.co/functions/v1/send-verification-email',
--   headers := '{"Content-Type": "application/json"}'::jsonb,
--   body := json_build_object('email', NEW.email, 'token', NEW.verification_token)::text
-- );

