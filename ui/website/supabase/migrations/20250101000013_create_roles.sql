-- Create role_type enum for user roles
-- Internal platform roles: admin, customer_success, finance
-- Customer-side roles: billing_admin, user, moderator, viewer

CREATE TYPE role_type AS ENUM (
  -- Internal platform roles
  'admin',
  'customer_success',
  'finance',
  -- Customer-side roles
  'billing_admin',
  'user',
  'moderator',
  'viewer'
);

-- Note: 
-- - Internal roles (admin, customer_success, finance) are for platform staff
-- - Customer roles (billing_admin, user, moderator, viewer) are for account users
-- - billing_admin: Accounts with a single user default to billing_admin
-- - billing_admin can be transferred for company accounts

