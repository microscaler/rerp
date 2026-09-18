-- Create Stripe payment-related tables
-- These tables store Stripe payment methods, subscriptions, and billing information
-- Used for KYC credit card billing verification after the 14-day trial

-- Stripe payment methods (credit cards, etc.)
CREATE TABLE IF NOT EXISTS stripe_payment_methods (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL REFERENCES human_names(id) ON DELETE CASCADE,
  stripe_payment_method_id TEXT NOT NULL UNIQUE, -- Stripe payment method ID (pm_xxxxx)
  stripe_customer_id TEXT NOT NULL, -- Stripe customer ID (cus_xxxxx)
  type TEXT NOT NULL, -- 'card', 'bank_account', etc.
  card_brand TEXT, -- 'visa', 'mastercard', 'amex', etc.
  card_last4 TEXT, -- Last 4 digits of card
  card_exp_month INTEGER,
  card_exp_year INTEGER,
  is_default BOOLEAN DEFAULT false,
  is_verified BOOLEAN DEFAULT false, -- KYC verification status
  verified_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_stripe_payment_methods_human_name_id ON stripe_payment_methods(human_name_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_methods_stripe_customer_id ON stripe_payment_methods(stripe_customer_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_methods_stripe_payment_method_id ON stripe_payment_methods(stripe_payment_method_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_methods_is_default ON stripe_payment_methods(is_default);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_methods_is_verified ON stripe_payment_methods(is_verified);

-- Stripe customers (links Supabase users to Stripe customers)
CREATE TABLE IF NOT EXISTS stripe_customers (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  human_name_id UUID NOT NULL UNIQUE REFERENCES human_names(id) ON DELETE CASCADE,
  account_id UUID REFERENCES accounts(id) ON DELETE SET NULL,
  stripe_customer_id TEXT NOT NULL UNIQUE, -- Stripe customer ID (cus_xxxxx)
  email TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_stripe_customers_human_name_id ON stripe_customers(human_name_id);
CREATE INDEX IF NOT EXISTS idx_stripe_customers_account_id ON stripe_customers(account_id);
CREATE INDEX IF NOT EXISTS idx_stripe_customers_stripe_customer_id ON stripe_customers(stripe_customer_id);

-- Stripe subscriptions
CREATE TABLE IF NOT EXISTS stripe_subscriptions (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  stripe_subscription_id TEXT NOT NULL UNIQUE, -- Stripe subscription ID (sub_xxxxx)
  stripe_customer_id TEXT NOT NULL,
  plan_id UUID REFERENCES plans(id) ON DELETE SET NULL,
  status TEXT NOT NULL, -- 'active', 'canceled', 'past_due', 'trialing', etc.
  current_period_start TIMESTAMP WITH TIME ZONE,
  current_period_end TIMESTAMP WITH TIME ZONE,
  trial_start TIMESTAMP WITH TIME ZONE,
  trial_end TIMESTAMP WITH TIME ZONE,
  canceled_at TIMESTAMP WITH TIME ZONE,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_stripe_subscriptions_account_id ON stripe_subscriptions(account_id);
CREATE INDEX IF NOT EXISTS idx_stripe_subscriptions_stripe_subscription_id ON stripe_subscriptions(stripe_subscription_id);
CREATE INDEX IF NOT EXISTS idx_stripe_subscriptions_stripe_customer_id ON stripe_subscriptions(stripe_customer_id);
CREATE INDEX IF NOT EXISTS idx_stripe_subscriptions_status ON stripe_subscriptions(status);

-- Stripe payment intents (for tracking payment attempts)
CREATE TABLE IF NOT EXISTS stripe_payment_intents (
  id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
  account_id UUID REFERENCES accounts(id) ON DELETE SET NULL,
  stripe_payment_intent_id TEXT NOT NULL UNIQUE, -- Stripe payment intent ID (pi_xxxxx)
  stripe_customer_id TEXT NOT NULL,
  amount INTEGER NOT NULL, -- Amount in cents
  currency TEXT NOT NULL DEFAULT 'usd',
  status TEXT NOT NULL, -- 'succeeded', 'processing', 'requires_payment_method', etc.
  payment_method_id UUID REFERENCES stripe_payment_methods(id) ON DELETE SET NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_stripe_payment_intents_account_id ON stripe_payment_intents(account_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_intents_stripe_payment_intent_id ON stripe_payment_intents(stripe_payment_intent_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_intents_stripe_customer_id ON stripe_payment_intents(stripe_customer_id);
CREATE INDEX IF NOT EXISTS idx_stripe_payment_intents_status ON stripe_payment_intents(status);

-- Create functions to update updated_at timestamps
CREATE OR REPLACE FUNCTION update_stripe_payment_methods_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_stripe_customers_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_stripe_subscriptions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_stripe_payment_intents_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers
CREATE TRIGGER trigger_update_stripe_payment_methods_updated_at
  BEFORE UPDATE ON stripe_payment_methods
  FOR EACH ROW
  EXECUTE FUNCTION update_stripe_payment_methods_updated_at();

CREATE TRIGGER trigger_update_stripe_customers_updated_at
  BEFORE UPDATE ON stripe_customers
  FOR EACH ROW
  EXECUTE FUNCTION update_stripe_customers_updated_at();

CREATE TRIGGER trigger_update_stripe_subscriptions_updated_at
  BEFORE UPDATE ON stripe_subscriptions
  FOR EACH ROW
  EXECUTE FUNCTION update_stripe_subscriptions_updated_at();

CREATE TRIGGER trigger_update_stripe_payment_intents_updated_at
  BEFORE UPDATE ON stripe_payment_intents
  FOR EACH ROW
  EXECUTE FUNCTION update_stripe_payment_intents_updated_at();

-- Enable Row Level Security
ALTER TABLE stripe_payment_methods ENABLE ROW LEVEL SECURITY;
ALTER TABLE stripe_customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE stripe_subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE stripe_payment_intents ENABLE ROW LEVEL SECURITY;

-- RLS Policies for stripe_payment_methods
CREATE POLICY "Users can read own payment methods" ON stripe_payment_methods
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

CREATE POLICY "Service role full access" ON stripe_payment_methods
  FOR ALL
  TO service_role
  USING (true);

-- RLS Policies for stripe_customers
CREATE POLICY "Users can read own stripe customer" ON stripe_customers
  FOR SELECT
  TO authenticated
  USING (human_name_id = get_current_human_name_id());

CREATE POLICY "Service role full access" ON stripe_customers
  FOR ALL
  TO service_role
  USING (true);

-- RLS Policies for stripe_subscriptions
CREATE POLICY "Users can read own subscriptions" ON stripe_subscriptions
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

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

CREATE POLICY "Service role full access" ON stripe_subscriptions
  FOR ALL
  TO service_role
  USING (true);

-- RLS Policies for stripe_payment_intents
CREATE POLICY "Users can read own payment intents" ON stripe_payment_intents
  FOR SELECT
  TO authenticated
  USING (
    account_id IN (
      SELECT id FROM accounts
      WHERE human_name_id = get_current_human_name_id()
    )
  );

CREATE POLICY "Service role full access" ON stripe_payment_intents
  FOR ALL
  TO service_role
  USING (true);

