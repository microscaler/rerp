/**
 * Supabase utility functions for direct database writes
 * 
 * This module provides functions to save email captures and form submissions
 * directly to Supabase PostgreSQL database.
 * 
 * All email addresses are stored in the email_addresses table and referenced
 * via foreign key in other tables. The upsert_email_address function ensures
 * emails are created if they don't exist, or reused if they do.
 */

import { SUPABASE_URL, SUPABASE_ANON_KEY } from '../config/build-config';

export interface EmailCapture {
  email: string;
  source: 'hero' | 'exit_intent' | 'free_trial';
  name?: string;
  company?: string;
  plan?: string;
  recaptcha_token?: string;
}

export interface ContactMessage {
  email: string;
  name: string;
  company?: string;
  message: string;
  recaptcha_token?: string;
}

/**
 * Upsert email address (create if not exists, return existing if exists)
 * Uses the upsert_email_address database function
 * @param email - Email address to upsert
 * @returns Promise that resolves with the email_address_id UUID
 */
const upsertEmailAddress = async (email: string): Promise<string> => {
  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    throw new Error('Supabase credentials not configured');
  }

  try {
    // Call the database function via RPC
    const response = await fetch(`${SUPABASE_URL}/rest/v1/rpc/upsert_email_address`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({
        email_text: email.toLowerCase().trim(),
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Supabase error: ${response.status} ${errorText}`);
    }

    // RPC returns UUID as JSON string
    const emailId = await response.text();
    // Parse JSON if needed, otherwise return as-is (UUID string)
    try {
      const parsed = JSON.parse(emailId);
      return typeof parsed === 'string' ? parsed : String(parsed);
    } catch {
      // If not JSON, return as-is (should be UUID string)
      return emailId.trim().replace(/^"|"$/g, '');
    }
  } catch (error) {
    console.error('Error upserting email address:', error);
    throw error;
  }
};

/**
 * Upsert company (create if not exists, return existing if exists)
 * Uses the upsert_company database function
 * @param companyName - Company name to upsert
 * @returns Promise that resolves with the company_id UUID, or null if no company provided
 */
const upsertCompany = async (companyName?: string): Promise<string | null> => {
  if (!companyName || !companyName.trim()) {
    return null;
  }

  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    throw new Error('Supabase credentials not configured');
  }

  try {
    // Call the database function via RPC
    const response = await fetch(`${SUPABASE_URL}/rest/v1/rpc/upsert_company`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({
        company_name: companyName.trim(),
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Supabase error: ${response.status} ${errorText}`);
    }

    // RPC returns UUID as JSON string
    const companyId = await response.text();
    // Parse JSON if needed, otherwise return as-is (UUID string)
    try {
      const parsed = JSON.parse(companyId);
      return typeof parsed === 'string' ? parsed : String(parsed);
    } catch {
      // If not JSON, return as-is (should be UUID string)
      return companyId.trim().replace(/^"|"$/g, '');
    }
  } catch (error) {
    console.error('Error upserting company:', error);
    throw error;
  }
};

/**
 * Get plan ID by plan code
 * @param planCode - Plan code (e.g., 'starter', 'pro', 'enterprise')
 * @returns Promise that resolves with the plan_id UUID, or null if plan not found
 */
const getPlanId = async (planCode?: string): Promise<string | null> => {
  if (!planCode || !planCode.trim()) {
    return null;
  }

  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    throw new Error('Supabase credentials not configured');
  }

  try {
    // Query plans table to get plan by code
    const response = await fetch(
      `${SUPABASE_URL}/rest/v1/plans?code=eq.${encodeURIComponent(planCode.trim())}&select=id`,
      {
        method: 'GET',
        headers: {
          'apikey': SUPABASE_ANON_KEY,
          'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        },
      }
    );

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Supabase error: ${response.status} ${errorText}`);
    }

    const plans = await response.json();
    if (plans && plans.length > 0) {
      return plans[0].id;
    }
    return null;
  } catch (error) {
    console.error('Error getting plan ID:', error);
    throw error;
  }
};

/**
 * Save email capture to Supabase
 * This function:
 * 1. Upserts the email address (creates if not exists, uses existing if exists)
 * 2. Creates an email_capture record linked to the email_address
 * 
 * @param data - Email capture data
 * @returns Promise that resolves when email is saved
 */
export const saveEmailCapture = async (data: EmailCapture): Promise<void> => {
  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    console.warn('Supabase credentials not configured. Email capture not saved.');
    return;
  }

  try {
    // Step 1: Upsert email address and get the ID
    const emailAddressId = await upsertEmailAddress(data.email);

    // Step 2: Upsert company if provided and get the ID
    const companyId = await upsertCompany(data.company);

    // Step 3: Get plan ID if provided
    const planId = await getPlanId(data.plan);

    // Step 4: Create email capture record with foreign keys
    const response = await fetch(`${SUPABASE_URL}/rest/v1/email_captures`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        'Prefer': 'return=minimal', // Don't return the inserted row
      },
      body: JSON.stringify({
        email_address_id: emailAddressId,
        source: data.source,
        name: data.name || null,
        company_id: companyId,
        plan_id: planId,
        recaptcha_token: data.recaptcha_token || null,
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Supabase error: ${response.status} ${errorText}`);
    }
  } catch (error) {
    console.error('Error saving email capture to Supabase:', error);
    throw error;
  }
};

/**
 * Save contact message to Supabase
 * This function:
 * 1. Upserts the email address (creates if not exists, uses existing if exists)
 * 2. Creates a contact_message record linked to the email_address
 * 
 * @param data - Contact message data
 * @returns Promise that resolves when message is saved
 */
export const saveContactMessage = async (data: ContactMessage): Promise<void> => {
  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    console.warn('Supabase credentials not configured. Contact message not saved.');
    return;
  }

  try {
    // Step 1: Upsert email address and get the ID
    const emailAddressId = await upsertEmailAddress(data.email);

    // Step 2: Upsert company if provided and get the ID
    const companyId = await upsertCompany(data.company);

    // Step 3: Create contact message record with foreign keys
    const response = await fetch(`${SUPABASE_URL}/rest/v1/contact_messages`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        'Prefer': 'return=minimal', // Don't return the inserted row
      },
      body: JSON.stringify({
        email_address_id: emailAddressId,
        name: data.name,
        company_id: companyId,
        message: data.message,
        recaptcha_token: data.recaptcha_token || null,
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Supabase error: ${response.status} ${errorText}`);
    }
  } catch (error) {
    console.error('Error saving contact message to Supabase:', error);
    throw error;
  }
};

