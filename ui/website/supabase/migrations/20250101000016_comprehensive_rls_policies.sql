-- Comprehensive Row Level Security (RLS) Policies
-- This migration adds row-based access control for platform roles and customer roles
--
-- Platform Roles:
-- - admin: Full access to all customer data
-- - customer_success: Read customer data, update support-related fields
-- - finance: Read billing data, update payment/subscription info
--
-- Customer Roles:
-- - billing_admin: Manage their account billing, read account info
-- - user: Access their own data
-- - moderator: Access team data (for company accounts)
-- - viewer: Read-only access to their data

-- Helper function to get current user's human_name_id from JWT claim
-- This assumes Supabase Auth is being used and human_name_id is stored in JWT
-- Alternative: Use auth.uid() if human_names.id matches Supabase Auth user ID
CREATE OR REPLACE FUNCTION get_current_human_name_id()
RETURNS UUID AS $$
BEGIN
  -- Try to get from JWT claim first (if custom claim is set)
  -- Otherwise fall back to auth.uid() if human_names.id matches auth user ID
  RETURN COALESCE(
    (current_setting('request.jwt.claims', true)::json->>'human_name_id')::uuid,
    auth.uid()
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to get current platform user ID from JWT or auth
-- This assumes platform users authenticate via Supabase Auth and we can identify them
CREATE OR REPLACE FUNCTION get_current_platform_user_id()
RETURNS UUID AS $$
DECLARE
  user_email TEXT;
BEGIN
  -- Try to get from JWT claim first (if custom claim is set)
  -- Otherwise try to get from auth.uid() and lookup
  user_email := COALESCE(
    (current_setting('request.jwt.claims', true)::json->>'email')::text,
    (SELECT email FROM auth.users WHERE id = auth.uid() LIMIT 1)
  );
  
  IF user_email IS NULL THEN
    RETURN NULL;
  END IF;
  
  -- Lookup platform_user_id from email
  RETURN (
    SELECT id FROM platform_users
    WHERE email = LOWER(TRIM(user_email))
      AND is_active = true
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to check if current user is a platform admin
CREATE OR REPLACE FUNCTION is_platform_admin()
RETURNS BOOLEAN AS $$
DECLARE
  platform_user_uuid UUID;
BEGIN
  platform_user_uuid := get_current_platform_user_id();
  
  IF platform_user_uuid IS NULL THEN
    RETURN false;
  END IF;
  
  RETURN EXISTS (
    SELECT 1 FROM admins
    WHERE platform_user_id = platform_user_uuid
      AND role = 'admin'
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to check if current user is customer_success
CREATE OR REPLACE FUNCTION is_customer_success()
RETURNS BOOLEAN AS $$
DECLARE
  platform_user_uuid UUID;
BEGIN
  platform_user_uuid := get_current_platform_user_id();
  
  IF platform_user_uuid IS NULL THEN
    RETURN false;
  END IF;
  
  RETURN EXISTS (
    SELECT 1 FROM admins
    WHERE platform_user_id = platform_user_uuid
      AND role = 'customer_success'
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to check if current user is finance
CREATE OR REPLACE FUNCTION is_finance()
RETURNS BOOLEAN AS $$
DECLARE
  platform_user_uuid UUID;
BEGIN
  platform_user_uuid := get_current_platform_user_id();
  
  IF platform_user_uuid IS NULL THEN
    RETURN false;
  END IF;
  
  RETURN EXISTS (
    SELECT 1 FROM admins
    WHERE platform_user_id = platform_user_uuid
      AND role = 'finance'
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to get current user's account_id
CREATE OR REPLACE FUNCTION get_user_account_id()
RETURNS UUID AS $$
DECLARE
  current_user_id UUID;
BEGIN
  current_user_id := get_current_human_name_id();
  RETURN (
    SELECT id FROM accounts
    WHERE human_name_id = current_user_id
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Helper function to check if user has role in account
CREATE OR REPLACE FUNCTION has_account_role(check_account_id UUID, check_role role_type)
RETURNS BOOLEAN AS $$
DECLARE
  current_user_id UUID;
BEGIN
  current_user_id := get_current_human_name_id();
  RETURN EXISTS (
    SELECT 1 FROM account_roles
    WHERE account_id = check_account_id
      AND human_name_id = current_user_id
      AND role = check_role
  );
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ============================================================================
-- EMAIL_ADDRESSES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read email_addresses" ON email_addresses
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own email address
CREATE POLICY "Users can read own email_address" ON email_addresses
  FOR SELECT
  TO authenticated
  USING (
    id IN (
      SELECT email_address_id FROM human_names
      WHERE id = get_current_human_name_id()
    )
  );

-- ============================================================================
-- HUMAN_NAMES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read human_names" ON human_names
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own name
CREATE POLICY "Users can read own human_name" ON human_names
  FOR SELECT
  TO authenticated
  USING (id = get_current_human_name_id());

-- Customers: Can update their own name
CREATE POLICY "Users can update own human_name" ON human_names
  FOR UPDATE
  TO authenticated
  USING (id = get_current_human_name_id())
  WITH CHECK (id = get_current_human_name_id());

-- ============================================================================
-- ACCOUNTS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read accounts" ON accounts
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Platform admin: Full access
CREATE POLICY "Platform admin can update accounts" ON accounts
  FOR UPDATE
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Finance: Can update plan_id (billing changes)
CREATE POLICY "Finance can update account plans" ON accounts
  FOR UPDATE
  TO authenticated
  USING (is_finance())
  WITH CHECK (is_finance());

-- Customers: Can read their own account
CREATE POLICY "Users can read own account" ON accounts
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

-- Billing admin: Can read account (for billing management)
CREATE POLICY "Billing admin can read account" ON accounts
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT account_id FROM account_roles
      WHERE human_name_id = auth.uid()::uuid
        AND role = 'billing_admin'
    )
  );

-- ============================================================================
-- ACCOUNT_ROLES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read account_roles" ON account_roles
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Platform admin: Full access
CREATE POLICY "Platform admin can manage account_roles" ON account_roles
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Customers: Can read roles in their account
CREATE POLICY "Users can read roles in own account" ON account_roles
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

-- Billing admin: Can read roles in their account
CREATE POLICY "Billing admin can read account roles" ON account_roles
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT account_id FROM account_roles
      WHERE human_name_id = auth.uid()::uuid
        AND role = 'billing_admin'
    )
  );

-- ============================================================================
-- MOBILE_NUMBERS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read mobile_numbers" ON mobile_numbers
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read/update their own mobile number
CREATE POLICY "Users can manage own mobile_number" ON mobile_numbers
  FOR ALL
  TO authenticated
  USING (human_name_id = get_current_human_name_id())
  WITH CHECK (human_name_id = get_current_human_name_id());

-- ============================================================================
-- ADDRESSES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read addresses" ON addresses
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Platform admin: Full access
CREATE POLICY "Platform admin can manage addresses" ON addresses
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Customers: Can read/update their own addresses
CREATE POLICY "Users can manage own addresses" ON addresses
  FOR ALL
  TO authenticated
  USING (
    person_id = get_current_human_name_id()
  )
  WITH CHECK (
    person_id = get_current_human_name_id()
  );

-- ============================================================================
-- COMPANIES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read companies" ON companies
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Platform admin: Full access
CREATE POLICY "Platform admin can manage companies" ON companies
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- ============================================================================
-- EMAIL_CAPTURES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read email_captures" ON email_captures
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Cannot read email_captures (marketing data, not user data)

-- ============================================================================
-- CONTACT_MESSAGES TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read contact_messages" ON contact_messages
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success()
  );

-- Customer success: Can update contact_messages (mark as resolved, add notes)
CREATE POLICY "Customer success can update contact_messages" ON contact_messages
  FOR UPDATE
  TO authenticated
  USING (is_customer_success())
  WITH CHECK (is_customer_success());

-- Customers: Can read their own contact messages
CREATE POLICY "Users can read own contact_messages" ON contact_messages
  FOR SELECT
  TO authenticated
  USING (
    email_address_id IN (
      SELECT email_address_id FROM human_names
      WHERE id = get_current_human_name_id()
    )
  );

-- ============================================================================
-- PLANS TABLE
-- ============================================================================

-- Already has public read access for anonymous users (for form dropdowns)
-- Platform roles: Full access
CREATE POLICY "Platform roles can manage plans" ON plans
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Customers: Can read plans (already allowed via anonymous policy)

-- ============================================================================
-- ADMINS TABLE
-- ============================================================================

-- Platform admin: Full access
CREATE POLICY "Platform admin can manage admins" ON admins
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Platform roles: Can read admins (to see who has platform access)
CREATE POLICY "Platform roles can read admins" ON admins
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- ============================================================================
-- PLATFORM_USERS TABLE
-- ============================================================================

-- Platform admin: Full access to platform users
CREATE POLICY "Platform admin can manage platform_users" ON platform_users
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Platform users: Can read their own record
CREATE POLICY "Platform users can read own record" ON platform_users
  FOR SELECT
  TO authenticated
  USING (
    id = get_current_platform_user_id()
  );

-- Platform users: Can update their own record
CREATE POLICY "Platform users can update own record" ON platform_users
  FOR UPDATE
  TO authenticated
  USING (id = get_current_platform_user_id())
  WITH CHECK (id = get_current_platform_user_id());

-- ============================================================================
-- PLATFORM_USER_ADDRESSES TABLE
-- ============================================================================

-- Platform admin: Full access
CREATE POLICY "Platform admin can manage platform_user_addresses" ON platform_user_addresses
  FOR ALL
  TO authenticated
  USING (is_platform_admin())
  WITH CHECK (is_platform_admin());

-- Platform users: Can manage their own addresses
CREATE POLICY "Platform users can manage own addresses" ON platform_user_addresses
  FOR ALL
  TO authenticated
  USING (
    platform_user_id = get_current_platform_user_id()
  )
  WITH CHECK (
    platform_user_id = get_current_platform_user_id()
  );

-- Note: Customers cannot access admins table (internal platform roles)

-- ============================================================================
-- VERIFICATION_TRACKING TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read verification_tracking" ON verification_tracking
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own verification status
CREATE POLICY "Users can read own verification status" ON verification_tracking
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

-- ============================================================================
-- STRIPE_PAYMENT_METHODS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read stripe_payment_methods" ON stripe_payment_methods
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own payment methods
CREATE POLICY "Users can read own payment methods" ON stripe_payment_methods
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

-- ============================================================================
-- STRIPE_CUSTOMERS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read stripe_customers" ON stripe_customers
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own stripe customer
CREATE POLICY "Users can read own stripe customer" ON stripe_customers
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

-- ============================================================================
-- STRIPE_SUBSCRIPTIONS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read stripe_subscriptions" ON stripe_subscriptions
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Finance: Can update subscriptions (billing changes)
CREATE POLICY "Finance can update subscriptions" ON stripe_subscriptions
  FOR UPDATE
  TO authenticated
  USING (is_finance())
  WITH CHECK (is_finance());

-- Customers: Can read their own subscriptions
CREATE POLICY "Users can read own subscriptions" ON stripe_subscriptions
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

-- Billing admin: Can read account subscriptions
CREATE POLICY "Billing admin can read account subscriptions" ON stripe_subscriptions
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT account_id FROM account_roles
      WHERE human_name_id = get_current_human_name_id()
        AND role = 'billing_admin'
    )
  );

-- ============================================================================
-- STRIPE_PAYMENT_INTENTS TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read stripe_payment_intents" ON stripe_payment_intents
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Customers: Can read their own payment intents
CREATE POLICY "Users can read own payment intents" ON stripe_payment_intents
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

-- ============================================================================
-- KYC_BILLING_VERIFICATION TABLE
-- ============================================================================

-- Platform roles: Full read access
CREATE POLICY "Platform roles can read kyc_billing_verification" ON kyc_billing_verification
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() OR is_customer_success() OR is_finance()
  );

-- Finance: Can update KYC verification status
CREATE POLICY "Finance can update KYC verification" ON kyc_billing_verification
  FOR UPDATE
  TO authenticated
  USING (is_finance())
  WITH CHECK (is_finance());

-- Customers: Can read their own KYC verification status
CREATE POLICY "Users can read own KYC verification" ON kyc_billing_verification
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

-- Billing admin: Can read account KYC verification
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

