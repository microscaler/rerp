# Supabase Integration for Email Captures

This directory contains Supabase migrations and configuration for storing email captures from the website forms.

## Setup

### 1. Create Supabase Project

1. Go to [Supabase Dashboard](https://app.supabase.com)
2. Create a new project (or use existing)
3. Note your:
   - **Project URL**: `https://your-project-ref.supabase.co`
   - **Anon Key**: Found in Settings > API

### 2. Add GitHub Secrets

Add these secrets to your GitHub repository:

- `SUPABASE_WEBSITE_PROJECT_URL` - Your Supabase project URL (e.g., `https://someid.supabase.co`)
- `SUPABASE_WEBSITE_ANON_API_KEY` - Your Supabase anon key (for client-side access)
- `SUPABASE_WEBSITE_PASSWORD` - Database password (for migrations)

**Note:** The naming convention uses `WEBSITE` prefix because the PriceWhisperer Supabase org has multiple databases (website, FTE, etc.)

### 3. Run Initial Migration

#### Option A: Using Supabase CLI (Recommended)

```bash
# Install Supabase CLI
npm install -g supabase

# Login
supabase login

# Link to your project
cd ui/website
supabase link --project-ref your-project-ref

# Run migrations
supabase db push
```

#### Option B: Using Supabase Dashboard

1. Go to Supabase Dashboard > SQL Editor
2. Copy the contents of `migrations/20250101000000_create_email_captures.sql`
3. Paste and run in SQL Editor

#### Option C: Using psql (Direct Database Connection)

```bash
# Extract project ID from URL (format: https://someid.supabase.co)
# Project ID is the subdomain before .supabase.co

# Get connection string from Supabase Dashboard > Settings > Database
# Format: postgresql://postgres:[PASSWORD]@db.[PROJECT-ID].supabase.co:5432/postgres
psql "postgresql://postgres:[YOUR-PASSWORD]@db.[YOUR-PROJECT-ID].supabase.co:5432/postgres" < migrations/20250101000000_create_email_captures.sql
```

## Schema

The `email_captures` table stores:

- `id` - UUID primary key
- `email` - Email address (required)
- `source` - Source of capture: 'hero', 'exit_intent', or 'free_trial'
- `name` - Name (optional, for free trial)
- `company` - Company (optional, for free trial)
- `plan` - Plan selected (optional, for free trial)
- `recaptcha_token` - reCAPTCHA v3 token (for verification)
- `created_at` - Timestamp

## Security

- **Row Level Security (RLS)** is enabled
- **Anonymous users** can INSERT (for website forms)
- **Anonymous users** cannot SELECT (emails are private)
- **Service role** can read all records (for backend/admin)

## Local Development

For local development with Tilt, you can:

1. Use your production Supabase project (simplest)
2. Or set up a local Supabase instance

To use production Supabase locally, create `.env` in `ui/website/`:

```env
VITE_SUPABASE_URL=https://your-project-ref.supabase.co
VITE_SUPABASE_ANON_KEY=your-anon-key
```

## GitHub Actions Migrations

Migrations run automatically when:
- Migration files in `ui/website/supabase/migrations/` are changed
- Workflow is manually triggered

The workflow uses `psql` to apply migrations directly to your production database.

**Required GitHub Secrets:**
- `SUPABASE_WEBSITE_PROJECT_URL` - Full project URL (e.g., `https://someid.supabase.co`)
- `SUPABASE_WEBSITE_PASSWORD` - Database password
- `SUPABASE_WEBSITE_ANON_API_KEY` - Anon key (already used for build)

The workflow automatically extracts the project ID from the URL.

## Adding New Migrations

1. Create a new file: `migrations/YYYYMMDDHHMMSS_description.sql`
2. Use timestamp format: `20250101120000_add_new_column.sql`
3. Write your SQL migration
4. Commit and push - GitHub Actions will run it automatically

## Querying Email Captures

From Supabase Dashboard or using service role key:

```sql
-- Get all email captures
SELECT * FROM email_captures ORDER BY created_at DESC;

-- Get captures by source
SELECT * FROM email_captures WHERE source = 'hero';

-- Get recent captures
SELECT * FROM email_captures WHERE created_at > NOW() - INTERVAL '7 days';
```

## Exporting Data

You can export email captures via:
- Supabase Dashboard > Table Editor > Export
- SQL Editor with COPY command
- Supabase API (using service role key)

