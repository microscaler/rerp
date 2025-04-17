import { RECAPTCHA_SITE_KEY } from '../config/build-config';

// reCAPTCHA v3 type definitions
declare global {
  interface Window {
    grecaptcha: {
      ready: (callback: () => void) => void;
      execute: (siteKey: string, options: { action: string }) => Promise<string>;
    };
  }
}

let recaptchaLoaded = false;
let recaptchaLoading = false;

/**
 * Load reCAPTCHA v3 script
 */
const loadRecaptcha = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    if (window.grecaptcha) {
      recaptchaLoaded = true;
      resolve();
      return;
    }

    if (recaptchaLoading) {
      // Wait for existing load to complete
      const checkInterval = setInterval(() => {
        if (recaptchaLoaded) {
          clearInterval(checkInterval);
          resolve();
        }
      }, 100);
      return;
    }

    recaptchaLoading = true;
    const script = document.createElement('script');
    script.src = `https://www.google.com/recaptcha/api.js?render=${RECAPTCHA_SITE_KEY}`;
    script.async = true;
    script.defer = true;
    script.onload = () => {
      recaptchaLoaded = true;
      recaptchaLoading = false;
      resolve();
    };
    script.onerror = () => {
      recaptchaLoading = false;
      reject(new Error('Failed to load reCAPTCHA'));
    };
    document.head.appendChild(script);
  });
};

/**
 * Execute reCAPTCHA v3 and get a token
 * @param action - Action name for reCAPTCHA (e.g., 'submit_job_application')
 * @returns Promise that resolves with the reCAPTCHA token
 */
export const executeRecaptcha = async (action: string = 'submit_job_application'): Promise<string> => {
  try {
    // Load reCAPTCHA if not already loaded
    await loadRecaptcha();

    // Wait for grecaptcha to be ready
    return new Promise((resolve, reject) => {
      window.grecaptcha.ready(async () => {
        try {
          const token = await window.grecaptcha.execute(RECAPTCHA_SITE_KEY, { action });
          resolve(token);
        } catch (error) {
          reject(error);
        }
      });
    });
  } catch (error) {
    console.error('reCAPTCHA execution error:', error);
    throw error;
  }
};

