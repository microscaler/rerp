# Supabase Edge Functions

This directory contains Supabase Edge Functions for server-side operations.

## Functions

### `send-email`

Sends emails via Resend API. This keeps the Resend API key secure on the server side.

**Location:** `supabase/functions/send-email/index.ts`

**Usage:**
```typescript
import { sendEmail } from '../utils/send-email';

await sendEmail({
  to: 'user@example.com',
  subject: 'Welcome!',
  html: '<p>Welcome to PriceWhisperer!</p>',
});
```

## Setup

### 1. Install Supabase CLI

```bash
npm install -g supabase
```

### 2. Login to Supabase

```bash
supabase login
```

### 3. Link to Your Project

```bash
cd ui/website
supabase link --project-ref your-project-ref
```

### 4. Set Environment Variables (Edge Function Secrets)

**IMPORTANT:** Edge Function secrets are **NOT** in `build-config.ts`. They are runtime secrets set in Supabase Dashboard.

Set these secrets in **Supabase Dashboard > Edge Functions > Secrets**:

- `RESEND_API_KEY` - Your Resend API key (get from [Resend Dashboard](https://resend.com/api-keys))
- `FROM_EMAIL` - Your verified sender email (e.g., `PriceWhisperer <noreply@pricewhisperer.ai>`)

**Why not in build-config.ts?**
- Edge Functions run on Supabase servers (Deno runtime), not in the client build
- Secrets are accessed at runtime via `Deno.env.get()` in the Edge Function
- They are server-side only and never exposed to the client

**To set secrets via CLI:**
```bash
supabase secrets set RESEND_API_KEY=re_xxxxxxxxx
supabase secrets set FROM_EMAIL="PriceWhisperer <noreply@pricewhisperer.ai>"
```

**To set secrets via Supabase Dashboard:**
1. Go to your Supabase project
2. Navigate to **Edge Functions** > **Secrets**
3. Click **Add Secret**
4. Enter secret name and value
5. Save

### 5. Deploy Function

```bash
# Deploy from ui/website directory
supabase functions deploy send-email
```

### 6. Test Locally (Optional)

```bash
# Start Supabase locally
supabase start

# Serve function locally
supabase functions serve send-email --no-verify-jwt --env-file .env.local
```

## Resend Setup

1. **Create Resend Account**: Go to [resend.com](https://resend.com) and sign up
2. **Get API Key**: Go to [API Keys](https://resend.com/api-keys) and create a new key
3. **Verify Domain** (for production):
   - Go to [Domains](https://resend.com/domains)
   - Add your domain (e.g., `pricewhisperer.ai`)
   - Add the DNS records provided
   - Wait for verification

## Environment Variables

### Required
- `RESEND_API_KEY` - Resend API key

### Optional
- `FROM_EMAIL` - Default sender email (defaults to `PriceWhisperer <onboarding@resend.dev>`)

## Function Endpoint

Once deployed, the function is available at:
```
https://your-project-ref.supabase.co/functions/v1/send-email
```

## Request Format

```json
{
  "to": "user@example.com",
  "subject": "Email Subject",
  "html": "<p>HTML content</p>",
  "text": "Plain text content"
}
```

Or with template:
```json
{
  "to": "user@example.com",
  "subject": "Email Subject",
  "template": {
    "id": "template-id",
    "variables": {
      "name": "John",
      "link": "https://example.com"
    }
  }
}
```

## Response Format

**Success:**
```json
{
  "success": true,
  "messageId": "49a3999c-0ce1-4ea6-ab68-afcd6dc2e794"
}
```

**Error:**
```json
{
  "error": "Error message",
  "details": {}
}
```

## CORS

The function includes CORS headers to allow calls from the website. The function accepts:
- `POST` requests for sending emails
- `OPTIONS` requests for CORS preflight

## Security

- Resend API key is stored as a Supabase secret (never exposed to client)
- Function requires Supabase anon key for authentication
- CORS is enabled for the website domain

## GitHub Actions Deployment

To deploy Edge Functions via GitHub Actions, add a step to your workflow:

```yaml
- name: Deploy Edge Functions
  env:
    SUPABASE_ACCESS_TOKEN: ${{ secrets.SUPABASE_ACCESS_TOKEN }}
  run: |
    cd ui/website
    supabase functions deploy send-email
```

Note: You'll need to set `SUPABASE_ACCESS_TOKEN` in GitHub Secrets.

