/**
 * Supabase Authentication Utilities
 * 
 * This module provides authentication functions using Supabase Auth
 * Supports: Email, Phone, Google, Apple, Facebook, and SAML
 */

import { createClient, SupabaseClient } from '@supabase/supabase-js';
import { SUPABASE_URL, SUPABASE_ANON_KEY } from '../config/build-config';

// Create Supabase client singleton
let supabaseClient: SupabaseClient | null = null;

/**
 * Get or create Supabase client
 */
export const getSupabaseClient = (): SupabaseClient => {
  if (!supabaseClient) {
    if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
      throw new Error('Supabase credentials not configured');
    }
    supabaseClient = createClient(SUPABASE_URL, SUPABASE_ANON_KEY, {
      auth: {
        autoRefreshToken: true,
        persistSession: true,
        detectSessionInUrl: true,
      },
    });
  }
  return supabaseClient;
};

/**
 * Sign in with email and password
 */
export const signInWithEmail = async (email: string, password: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signInWithPassword({
    email,
    password,
  });

  if (error) throw error;
  return data;
};

/**
 * Sign up with email and password
 */
export const signUpWithEmail = async (email: string, password: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signUp({
    email,
    password,
  });

  if (error) throw error;
  return data;
};

/**
 * Send OTP to phone number
 */
export const sendPhoneOTP = async (phone: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signInWithOtp({
    phone,
  });

  if (error) throw error;
  return data;
};

/**
 * Verify phone OTP
 */
export const verifyPhoneOTP = async (phone: string, token: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.verifyOtp({
    phone,
    token,
    type: 'sms',
  });

  if (error) throw error;
  return data;
};

/**
 * Send OTP to email address
 */
export const sendEmailOTP = async (email: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signInWithOtp({
    email,
  });

  if (error) throw error;
  return data;
};

/**
 * Verify email OTP
 */
export const verifyEmailOTP = async (email: string, token: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.verifyOtp({
    email,
    token,
    type: 'email',
  });

  if (error) throw error;
  return data;
};

/**
 * Dual verification: Send OTP to both email and phone
 * Returns promises for both OTP sends
 */
export const sendDualOTP = async (email: string, phone: string) => {
  const supabase = getSupabaseClient();
  
  // Send OTP to both email and phone simultaneously
  const [emailResult, phoneResult] = await Promise.allSettled([
    supabase.auth.signInWithOtp({ email }),
    supabase.auth.signInWithOtp({ phone }),
  ]);

  // Check for errors
  if (emailResult.status === 'rejected' || (emailResult.status === 'fulfilled' && emailResult.value.error)) {
    throw emailResult.status === 'rejected' 
      ? emailResult.reason 
      : emailResult.value.error;
  }

  if (phoneResult.status === 'rejected' || (phoneResult.status === 'fulfilled' && phoneResult.value.error)) {
    throw phoneResult.status === 'rejected' 
      ? phoneResult.reason 
      : phoneResult.value.error;
  }

  return {
    email: emailResult.status === 'fulfilled' ? emailResult.value.data : null,
    phone: phoneResult.status === 'fulfilled' ? phoneResult.value.data : null,
  };
};

/**
 * Dual verification: Verify both email and phone OTPs
 * Both must be verified successfully for authentication
 * Uses database tracking to ensure both are verified before granting access
 */
export const verifyDualOTP = async (
  email: string,
  emailToken: string,
  phone: string,
  phoneToken: string
) => {
  const supabase = getSupabaseClient();
  
  // Step 1: Verify email OTP with Supabase Auth
  const emailResult = await supabase.auth.verifyOtp({
    email,
    token: emailToken,
    type: 'email',
  });

  if (emailResult.error) {
    throw new Error(`Email verification failed: ${emailResult.error.message}`);
  }

  // Step 2: Get the user's human_name_id from the email verification
  // We need to find the human_name_id associated with this email
  const { data: { user } } = await supabase.auth.getUser();
  if (!user) {
    throw new Error('User not found after email verification');
  }

  // Step 3: Get human_name_id from email_addresses -> human_names
  const emailAddressResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/email_addresses?email=eq.${encodeURIComponent(email.toLowerCase().trim())}&select=id`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!emailAddressResponse.ok) {
    throw new Error('Failed to lookup email address');
  }

  const emailAddresses = await emailAddressResponse.json();
  if (!emailAddresses || emailAddresses.length === 0) {
    throw new Error('Email address not found');
  }

  const emailAddressId = emailAddresses[0].id;

  // Get human_name_id
  const humanNameResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/human_names?email_address_id=eq.${emailAddressId}&select=id`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!humanNameResponse.ok) {
    throw new Error('Failed to lookup human name');
  }

  const humanNames = await humanNameResponse.json();
  if (!humanNames || humanNames.length === 0) {
    throw new Error('Human name not found for email');
  }

  const humanNameId = humanNames[0].id;

  // Step 4: Mark email as verified in database
  const markEmailResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/rpc/mark_email_verified`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({ email_address: email.toLowerCase().trim() }),
    }
  );

  if (!markEmailResponse.ok) {
    const errorText = await markEmailResponse.text();
    throw new Error(`Failed to mark email as verified: ${errorText}`);
  }

  const emailBothVerified = await markEmailResponse.json();

  // Step 5: Verify phone OTP with Supabase Auth
  const phoneResult = await supabase.auth.verifyOtp({
    phone,
    token: phoneToken,
    type: 'sms',
  });

  if (phoneResult.error) {
    throw new Error(`Phone verification failed: ${phoneResult.error.message}`);
  }

  // Step 6: Mark phone as verified in database
  const markPhoneResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/rpc/mark_phone_verified`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({ phone_number: phone }),
    }
  );

  if (!markPhoneResponse.ok) {
    const errorText = await markPhoneResponse.text();
    throw new Error(`Failed to mark phone as verified: ${errorText}`);
  }

  const phoneBothVerified = await markPhoneResponse.json();

  // Step 7: Check if both are verified
  if (!emailBothVerified && !phoneBothVerified) {
    // Neither function returned true, so both weren't verified before
    // Check again to see if both are now verified
    const checkBothResponse = await fetch(
      `${SUPABASE_URL}/rest/v1/rpc/is_dual_verified`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'apikey': SUPABASE_ANON_KEY,
          'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        },
        body: JSON.stringify({ human_name_uuid: humanNameId }),
      }
    );

    if (!checkBothResponse.ok) {
      throw new Error('Failed to check dual verification status');
    }

    const bothVerified = await checkBothResponse.json();
    if (!bothVerified) {
      throw new Error('Both email and phone must be verified');
    }
  }

  // Both verified successfully
  // The phone verification will be the active session
  return phoneResult.data;
};

/**
 * Verify email OTP and mark as verified in database
 * Returns true if both email and phone are now verified
 */
export const verifyEmailOTPAndTrack = async (email: string, token: string): Promise<boolean> => {
  const supabase = getSupabaseClient();
  
  // Verify with Supabase Auth
  const result = await supabase.auth.verifyOtp({
    email,
    token,
    type: 'email',
  });

  if (result.error) {
    throw result.error;
  }

  // Get human_name_id
  const { data: { user } } = await supabase.auth.getUser();
  if (!user) {
    throw new Error('User not found after email verification');
  }

  // Get email_address_id and then human_name_id
  const emailAddressResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/email_addresses?email=eq.${encodeURIComponent(email.toLowerCase().trim())}&select=id`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!emailAddressResponse.ok) {
    throw new Error('Failed to lookup email address');
  }

  const emailAddresses = await emailAddressResponse.json();
  if (!emailAddresses || emailAddresses.length === 0) {
    throw new Error('Email address not found');
  }

  const emailAddressId = emailAddresses[0].id;

  const humanNameResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/human_names?email_address_id=eq.${emailAddressId}&select=id`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!humanNameResponse.ok) {
    throw new Error('Failed to lookup human name');
  }

  const humanNames = await humanNameResponse.json();
  if (!humanNames || humanNames.length === 0) {
    throw new Error('Human name not found for email');
  }

  const humanNameId = humanNames[0].id;

  // Mark email as verified in database
  const markResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/rpc/mark_email_verified`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({ email_address: email.toLowerCase().trim() }),
    }
  );

  if (!markResponse.ok) {
    const errorText = await markResponse.text();
    throw new Error(`Failed to mark email as verified: ${errorText}`);
  }

  const bothVerified = await markResponse.json();
  return bothVerified === true;
};

/**
 * Verify phone OTP and mark as verified in database
 * Returns true if both email and phone are now verified
 */
export const verifyPhoneOTPAndTrack = async (phone: string, token: string): Promise<boolean> => {
  const supabase = getSupabaseClient();
  
  // Verify with Supabase Auth
  const result = await supabase.auth.verifyOtp({
    phone,
    token,
    type: 'sms',
  });

  if (result.error) {
    throw result.error;
  }

  // Get human_name_id from mobile_numbers
  const mobileNumberResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/mobile_numbers?phone_number=eq.${encodeURIComponent(phone)}&select=human_name_id`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!mobileNumberResponse.ok) {
    throw new Error('Failed to lookup mobile number');
  }

  const mobileNumbers = await mobileNumberResponse.json();
  if (!mobileNumbers || mobileNumbers.length === 0) {
    throw new Error('Mobile number not found');
  }

  const humanNameId = mobileNumbers[0].human_name_id;

  // Mark phone as verified in database
  const markResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/rpc/mark_phone_verified`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify({ phone_number: phone }),
    }
  );

  if (!markResponse.ok) {
    const errorText = await markResponse.text();
    throw new Error(`Failed to mark phone as verified: ${errorText}`);
  }

  const bothVerified = await markResponse.json();
  return bothVerified === true;
};

/**
 * Check if dual verification is complete for current user
 */
export const checkDualVerificationStatus = async (): Promise<{
  emailVerified: boolean;
  phoneVerified: boolean;
  bothVerified: boolean;
}> => {
  const supabase = getSupabaseClient();
  const { data: { user } } = await supabase.auth.getUser();
  
  if (!user) {
    return { emailVerified: false, phoneVerified: false, bothVerified: false };
  }

  // Get human_name_id from user metadata or lookup
  // This is a simplified version - you may need to adjust based on your auth setup
  const humanNameId = user.user_metadata?.human_name_id;
  
  if (!humanNameId) {
    // Try to lookup from email
    const emailAddressResponse = await fetch(
      `${SUPABASE_URL}/rest/v1/email_addresses?email=eq.${encodeURIComponent(user.email || '')}&select=id`,
      {
        method: 'GET',
        headers: {
          'apikey': SUPABASE_ANON_KEY,
          'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        },
      }
    );

    if (!emailAddressResponse.ok) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const emailAddresses = await emailAddressResponse.json();
    if (!emailAddresses || emailAddresses.length === 0) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const emailAddressId = emailAddresses[0].id;

    const humanNameResponse = await fetch(
      `${SUPABASE_URL}/rest/v1/human_names?email_address_id=eq.${emailAddressId}&select=id`,
      {
        method: 'GET',
        headers: {
          'apikey': SUPABASE_ANON_KEY,
          'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        },
      }
    );

    if (!humanNameResponse.ok) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const humanNames = await humanNameResponse.json();
    if (!humanNames || humanNames.length === 0) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const foundHumanNameId = humanNames[0].id;

    // Get verification status
    const verificationResponse = await fetch(
      `${SUPABASE_URL}/rest/v1/verification_tracking?human_name_id=eq.${foundHumanNameId}&select=email_verified,phone_verified,both_verified`,
      {
        method: 'GET',
        headers: {
          'apikey': SUPABASE_ANON_KEY,
          'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
        },
      }
    );

    if (!verificationResponse.ok) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const verifications = await verificationResponse.json();
    if (!verifications || verifications.length === 0) {
      return { emailVerified: false, phoneVerified: false, bothVerified: false };
    }

    const verification = verifications[0];
    return {
      emailVerified: verification.email_verified || false,
      phoneVerified: verification.phone_verified || false,
      bothVerified: verification.both_verified || false,
    };
  }

  // Get verification status
  const verificationResponse = await fetch(
    `${SUPABASE_URL}/rest/v1/verification_tracking?human_name_id=eq.${humanNameId}&select=email_verified,phone_verified,both_verified`,
    {
      method: 'GET',
      headers: {
        'apikey': SUPABASE_ANON_KEY,
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
    }
  );

  if (!verificationResponse.ok) {
    return { emailVerified: false, phoneVerified: false, bothVerified: false };
  }

  const verifications = await verificationResponse.json();
  if (!verifications || verifications.length === 0) {
    return { emailVerified: false, phoneVerified: false, bothVerified: false };
  }

  const verification = verifications[0];
  return {
    emailVerified: verification.email_verified || false,
    phoneVerified: verification.phone_verified || false,
    bothVerified: verification.both_verified || false,
  };
};

/**
 * Sign in with OAuth provider
 */
export const signInWithOAuth = async (provider: 'google' | 'apple' | 'facebook') => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signInWithOAuth({
    provider,
    options: {
      redirectTo: `${window.location.origin}${window.location.pathname}#sign-in`,
    },
  });

  if (error) throw error;
  return data;
};

/**
 * Sign in with SAML (Enterprise SSO)
 * Note: Currently inactive, but code is ready
 */
export const signInWithSAML = async (domain?: string) => {
  const supabase = getSupabaseClient();
  const { data, error } = await supabase.auth.signInWithSSO({
    provider: 'saml',
    options: {
      domain,
      redirectTo: `${window.location.origin}${window.location.pathname}#sign-in`,
    },
  });

  if (error) throw error;
  return data;
};

/**
 * Sign out
 */
export const signOut = async () => {
  const supabase = getSupabaseClient();
  const { error } = await supabase.auth.signOut();
  if (error) throw error;
};

/**
 * Get current session
 */
export const getSession = async () => {
  const supabase = getSupabaseClient();
  const { data: { session }, error } = await supabase.auth.getSession();
  if (error) throw error;
  return session;
};

/**
 * Get current user
 */
export const getCurrentUser = async () => {
  const supabase = getSupabaseClient();
  const { data: { user }, error } = await supabase.auth.getUser();
  if (error) throw error;
  return user;
};

/**
 * Listen to auth state changes
 */
export const onAuthStateChange = (callback: (event: string, session: any) => void) => {
  const supabase = getSupabaseClient();
  return supabase.auth.onAuthStateChange(callback);
};

