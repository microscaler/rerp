/**
 * Build Configuration
 * 
 * Centralized location for all build-time configuration values.
 * These values are injected via environment variables during the build process.
 * 
 * For Vite, environment variables must be prefixed with VITE_ to be accessible.
 * 
 * GitHub Actions Secrets:
 * - RECAPTCHA_SITE_KEY -> VITE_RECAPTCHA_SITE_KEY
 * - RECAPTCHA_SECRET_KEY -> (server-side only, not exposed to client)
 * - GA_MEASUREMENT_ID -> VITE_GA_MEASUREMENT_ID
 * - BASE_URL -> VITE_BASE_URL
 * - SUPABASE_WEBSITE_ANON_API_KEY -> VITE_SUPABASE_ANON_KEY
 * - SUPABASE_WEBSITE_PROJECT_URL -> VITE_SUPABASE_URL
 * 
 * 
 * IMPORTANT: Edge Function Secrets (NOT in build-config.ts)
 * ==========================================================
 * Edge Functions run on Supabase servers (not in the client build).
 * Their secrets are set in Supabase Dashboard > Edge Functions > Secrets.
 * These secrets are accessed at runtime via Deno.env.get() in the Edge Function.
 * 
 * Required Edge Function Secrets (set in Supabase Dashboard):
 * - RESEND_API_KEY -> Resend API key for sending emails (server-side only)
 * - FROM_EMAIL -> Default sender email (e.g., "PriceWhisperer <noreply@pricewhisperer.ai>")
 * 
 * To set via Supabase CLI:
 *   supabase secrets set RESEND_API_KEY=re_xxxxxxxxx
 *   supabase secrets set FROM_EMAIL="PriceWhisperer <noreply@pricewhisperer.ai>"
 * 
 * These secrets are NEVER exposed to the client and are NOT in build-config.ts
 */

/**
 * reCAPTCHA v3 Site Key
 * Public key used for client-side reCAPTCHA verification
 */
export const RECAPTCHA_SITE_KEY = import.meta.env.VITE_RECAPTCHA_SITE_KEY || '6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI'; // Fallback to test key

/**
 * Google Analytics Measurement ID
 */
export const GA_MEASUREMENT_ID = import.meta.env.VITE_GA_MEASUREMENT_ID || '';


/**
 * Base URL for the application
 * Used for canonical URLs, sitemap generation, and SEO
 */
export const BASE_URL = import.meta.env.VITE_BASE_URL || 'https://pricewhisperer.ai';

/**
 * Supabase Configuration
 * Used for direct database writes (email captures, form submissions)
 */
export const SUPABASE_URL = import.meta.env.VITE_SUPABASE_URL || '';
export const SUPABASE_ANON_KEY = import.meta.env.VITE_SUPABASE_ANON_KEY || '';

/**
 * Build configuration validation
 * Logs warnings in development if required configs are missing
 */
if (import.meta.env.DEV) {
  if (!import.meta.env.VITE_RECAPTCHA_SITE_KEY) {
    console.warn('⚠️ VITE_RECAPTCHA_SITE_KEY not set. Using test key.');
  }
  if (!import.meta.env.VITE_GA_MEASUREMENT_ID) {
    console.warn('⚠️ VITE_GA_MEASUREMENT_ID not set. Analytics will be disabled.');
  }
  if (!import.meta.env.VITE_SUPABASE_URL || !import.meta.env.VITE_SUPABASE_ANON_KEY) {
    console.warn('⚠️ Supabase credentials not set. Email captures will not be saved.');
  }
}

