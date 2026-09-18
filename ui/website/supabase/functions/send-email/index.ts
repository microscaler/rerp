// Supabase Edge Function: Send Email via Resend
// This function sends emails using the Resend API
// Called from the website to send welcome emails, verification emails, etc.

const RESEND_API_KEY = Deno.env.get('RESEND_API_KEY');
const FROM_EMAIL = Deno.env.get('FROM_EMAIL') || 'RERP <onboarding@resend.dev>';

interface EmailRequest {
  to: string | string[];
  subject: string;
  html?: string;
  text?: string;
  template?: {
    id: string;
    variables?: Record<string, string>;
  };
}

const handler = async (request: Request): Promise<Response> => {
  // Handle CORS
  if (request.method === 'OPTIONS') {
    return new Response(null, {
      status: 204,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'POST, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type, Authorization',
      },
    });
  }

  // Only allow POST requests
  if (request.method !== 'POST') {
    return new Response(
      JSON.stringify({ error: 'Method not allowed' }),
      {
        status: 405,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  }

  // Check for API key
  if (!RESEND_API_KEY) {
    console.error('RESEND_API_KEY not configured');
    return new Response(
      JSON.stringify({ error: 'Email service not configured' }),
      {
        status: 500,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  }

  try {
    // Parse request body
    const emailData: EmailRequest = await request.json();

    // Validate required fields
    if (!emailData.to || !emailData.subject) {
      return new Response(
        JSON.stringify({ error: 'Missing required fields: to, subject' }),
        {
          status: 400,
          headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*',
          },
        }
      );
    }

    // Prepare Resend API request
    const resendPayload: any = {
      from: FROM_EMAIL,
      to: emailData.to,
      subject: emailData.subject,
    };

    // Add content (html, text, or template)
    if (emailData.template) {
      resendPayload.template = emailData.template;
    } else {
      if (emailData.html) {
        resendPayload.html = emailData.html;
      }
      if (emailData.text) {
        resendPayload.text = emailData.text;
      }
    }

    // Call Resend API
    const resendResponse = await fetch('https://api.resend.com/emails', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${RESEND_API_KEY}`,
      },
      body: JSON.stringify(resendPayload),
    });

    const resendData = await resendResponse.json();

    if (!resendResponse.ok) {
      console.error('Resend API error:', resendData);
      return new Response(
        JSON.stringify({ 
          error: 'Failed to send email',
          details: resendData 
        }),
        {
          status: resendResponse.status,
          headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*',
          },
        }
      );
    }

    // Success
    return new Response(
      JSON.stringify({ 
        success: true,
        messageId: resendData.id 
      }),
      {
        status: 200,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  } catch (error) {
    console.error('Error sending email:', error);
    return new Response(
      JSON.stringify({ 
        error: 'Internal server error',
        message: error instanceof Error ? error.message : 'Unknown error'
      }),
      {
        status: 500,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  }
};

Deno.serve(handler);

