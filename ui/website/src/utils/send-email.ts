/**
 * Email sending utility via Supabase Edge Functions
 * 
 * This module provides functions to send emails through Supabase Edge Functions
 * which use Resend API. This keeps the Resend API key secure on the server side.
 */

import { SUPABASE_URL, SUPABASE_ANON_KEY } from '../config/build-config';

export interface SendEmailOptions {
  to: string | string[];
  subject: string;
  html?: string;
  text?: string;
  template?: {
    id: string;
    variables?: Record<string, string>;
  };
}

/**
 * Send email via Supabase Edge Function
 * @param options - Email options
 * @returns Promise that resolves with the email message ID
 */
export const sendEmail = async (options: SendEmailOptions): Promise<string> => {
  if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
    throw new Error('Supabase credentials not configured');
  }

  try {
    const response = await fetch(`${SUPABASE_URL}/functions/v1/send-email`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${SUPABASE_ANON_KEY}`,
      },
      body: JSON.stringify(options),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ error: 'Unknown error' }));
      throw new Error(errorData.error || `Failed to send email: ${response.status}`);
    }

    const data = await response.json();
    return data.messageId;
  } catch (error) {
    console.error('Error sending email:', error);
    throw error;
  }
};

/**
 * Send welcome email for "Get Free Daily Market Insights"
 * @param email - Recipient email address
 * @returns Promise that resolves when email is sent
 */
export const sendWelcomeEmail = async (email: string): Promise<void> => {
  await sendEmail({
    to: email,
    subject: 'Welcome to PriceWhisperer - Your Free Daily Market Insights',
    html: `
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
        </head>
        <body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
          <div style="text-align: center; margin-bottom: 30px;">
            <h1 style="color: #2563eb; margin: 0;">PriceWhisperer</h1>
          </div>
          
          <div style="background: #f8fafc; border-radius: 8px; padding: 30px; margin-bottom: 20px;">
            <h2 style="color: #1e293b; margin-top: 0;">Welcome! 🎉</h2>
            <p style="font-size: 16px; color: #475569;">
              Thank you for subscribing to PriceWhisperer's free daily market insights!
            </p>
            <p style="font-size: 16px; color: #475569;">
              You'll now receive our daily newsletter with:
            </p>
            <ul style="font-size: 16px; color: #475569; padding-left: 20px;">
              <li>AI-powered trading alerts</li>
              <li>Market analysis and insights</li>
              <li>Options opportunities</li>
              <li>Risk management tips</li>
            </ul>
          </div>
          
          <div style="text-align: center; margin: 30px 0;">
            <a href="https://pricewhisperer.ai" 
               style="display: inline-block; background: #2563eb; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; font-weight: 600;">
              Explore PriceWhisperer
            </a>
          </div>
          
          <div style="border-top: 1px solid #e2e8f0; padding-top: 20px; margin-top: 30px; font-size: 14px; color: #64748b; text-align: center;">
            <p>You're receiving this because you signed up for free market insights at PriceWhisperer.</p>
            <p style="margin-top: 10px;">
              <a href="#" style="color: #2563eb; text-decoration: none;">Unsubscribe</a> | 
              <a href="https://pricewhisperer.ai/privacy-policy" style="color:rgb(5, 5, 5); text-decoration: none;">Privacy Policy</a>
            </p>
          </div>
        </body>
      </html>
    `,
    text: `
Welcome to PriceWhisperer!

Thank you for subscribing to our free daily market insights!

You'll now receive our daily newsletter with:
- AI-powered trading alerts
- Market analysis and insights
- Options opportunities
- Risk management tips

Explore PriceWhisperer: https://pricewhisperer.ai

---
You're receiving this because you signed up for free market insights at PriceWhisperer.
Unsubscribe: [link] | Privacy Policy: https://pricewhisperer.ai/privacy-policy
    `.trim(),
  });
};

