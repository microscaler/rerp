-- Create platform_users table for PriceWhisperer platform admin users
-- This table is COMPLETELY SEPARATE from customer users (human_names)
-- Platform users are internal staff and must never be mixed with customer data
-- They have their own email, phone, and name fields to ensure complete isolation

CREATE TABLE IF NOT EXISTS platform_users (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  phone TEXT UNIQUE, -- Optional phone number
  first_name TEXT NOT NULL,
  surname TEXT NOT NULL,
  username TEXT UNIQUE, -- Optional username
  is_active BOOLEAN DEFAULT true NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  last_login_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_platform_users_email ON platform_users(email);
CREATE INDEX IF NOT EXISTS idx_platform_users_phone ON platform_users(phone) WHERE phone IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_platform_users_username ON platform_users(username) WHERE username IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_platform_users_is_active ON platform_users(is_active);

-- Create function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_platform_users_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger
CREATE TRIGGER trigger_update_platform_users_updated_at
  BEFORE UPDATE ON platform_users
  FOR EACH ROW
  EXECUTE FUNCTION update_platform_users_updated_at();

-- Create platform_user_addresses table (separate from customer addresses)
CREATE TABLE IF NOT EXISTS platform_user_addresses (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  platform_user_id UUID NOT NULL REFERENCES platform_users(id) ON DELETE CASCADE,
  street_address TEXT,
  city TEXT,
  state_province TEXT,
  postal_code TEXT,
  country TEXT,
  is_primary BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_platform_user_addresses_platform_user_id ON platform_user_addresses(platform_user_id);
CREATE INDEX IF NOT EXISTS idx_platform_user_addresses_is_primary ON platform_user_addresses(is_primary);

-- Create function to update updated_at
CREATE OR REPLACE FUNCTION update_platform_user_addresses_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger
CREATE TRIGGER trigger_update_platform_user_addresses_updated_at
  BEFORE UPDATE ON platform_user_addresses
  FOR EACH ROW
  EXECUTE FUNCTION update_platform_user_addresses_updated_at();

-- Enable Row Level Security
ALTER TABLE platform_users ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform_user_addresses ENABLE ROW LEVEL SECURITY;

-- RLS Policies for platform_users
-- Only platform admins can read platform users
CREATE POLICY "Platform admin can read platform_users" ON platform_users
  FOR SELECT
  TO authenticated
  USING (is_platform_admin());

-- Platform admin can update their own record
CREATE POLICY "Platform admin can update own record" ON platform_users
  FOR UPDATE
  TO authenticated
  USING (
    is_platform_admin() AND
    id IN (
      SELECT human_name_id FROM admins
      WHERE human_name_id = get_current_human_name_id()
        AND role = 'admin'
    )
  )
  WITH CHECK (
    is_platform_admin() AND
    id IN (
      SELECT human_name_id FROM admins
      WHERE human_name_id = get_current_human_name_id()
        AND role = 'admin'
    )
  );

-- Service role full access
CREATE POLICY "Service role full access platform_users" ON platform_users
  FOR ALL
  TO service_role
  USING (true);

-- RLS Policies for platform_user_addresses
CREATE POLICY "Platform admin can read own addresses" ON platform_user_addresses
  FOR SELECT
  TO authenticated
  USING (
    is_platform_admin() AND
    platform_user_id = get_current_platform_user_id()
  );

-- Service role full access
CREATE POLICY "Service role full access platform_user_addresses" ON platform_user_addresses
  FOR ALL
  TO service_role
  USING (true);

-- Note: Platform users are completely isolated from customer users
-- They do NOT appear in human_names, accounts, or any customer-related tables
-- This ensures complete separation between platform staff and customers

