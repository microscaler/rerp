import { Component, Show, onMount, onCleanup, createSignal, createEffect } from 'solid-js';
import { RECAPTCHA_SITE_KEY } from '../../config/build-config';

export interface CaptchaModalProps {
  isOpen: boolean;
  onClose: () => void;
  onVerify: (token: string) => void;
}

// Note: This component is for reCAPTCHA v2 (with modal)
// reCAPTCHA v3 (transparent) is now used instead via recaptcha.ts utility
// This component is kept for potential future use but is not currently used

// Global type is defined in recaptcha.ts to avoid conflicts

const CaptchaModal: Component<CaptchaModalProps> = (props) => {
  const [captchaWidgetId, setCaptchaWidgetId] = createSignal<number | null>(null);
  const [isLoading, setIsLoading] = createSignal(true);
  let captchaContainer: HTMLDivElement | undefined;

  const loadRecaptcha = () => {
    if (document.getElementById('recaptcha-script')) {
      // Script already loaded
      renderCaptcha();
      return;
    }

    const script = document.createElement('script');
    script.id = 'recaptcha-script';
    script.src = 'https://www.google.com/recaptcha/api.js?render=explicit';
    script.async = true;
    script.defer = true;
    script.onload = () => {
      setIsLoading(false);
      renderCaptcha();
    };
    document.head.appendChild(script);
  };

  const renderCaptcha = () => {
    if (!captchaContainer || !window.grecaptcha) {
      // If container not ready, try again after a short delay
      if (captchaContainer && window.grecaptcha) {
        setTimeout(renderCaptcha, 100);
      }
      return;
    }

    // Clear any existing widget
    if (captchaWidgetId() !== null) {
      try {
        // Clear the container
        captchaContainer.innerHTML = '';
        setCaptchaWidgetId(null);
      } catch (e) {
        // Ignore errors when clearing
      }
    }

    try {
      if (!window.grecaptcha?.render) {
        console.error('reCAPTCHA v2 render method not available');
        setIsLoading(false);
        return;
      }
      const widgetId = window.grecaptcha.render(captchaContainer, {
        sitekey: RECAPTCHA_SITE_KEY,
        callback: (token: string) => {
          props.onVerify(token);
        },
        'expired-callback': () => {
          // Token expired, user needs to solve again
          if (captchaWidgetId() !== null && window.grecaptcha?.reset) {
            window.grecaptcha.reset(captchaWidgetId()!);
          }
        },
      });
      setCaptchaWidgetId(widgetId);
      setIsLoading(false);
    } catch (error) {
      console.error('Error rendering reCAPTCHA:', error);
      setIsLoading(false);
    }
  };

  onMount(() => {
    if (props.isOpen) {
      loadRecaptcha();
    }
  });

  // Watch for isOpen changes and reload/reset CAPTCHA
  createEffect(() => {
    if (props.isOpen) {
      if (!window.grecaptcha) {
        loadRecaptcha();
      } else {
        // Wait a bit for the container to be ready, then render
        setTimeout(() => {
          if (captchaWidgetId() !== null) {
            // Reset existing CAPTCHA when modal reopens
            try {
              window.grecaptcha.reset(captchaWidgetId()!);
            } catch (e) {
              // If reset fails, re-render
              renderCaptcha();
            }
          } else {
            // Render CAPTCHA if script is loaded but not rendered
            renderCaptcha();
          }
        }, 50);
      }
    }
  });

  onCleanup(() => {
    // Cleanup is handled by reCAPTCHA
  });

  if (!props.isOpen) return null;

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4">
      <div class="bg-white rounded-xl shadow-2xl max-w-md w-full p-6 relative">
        <button
          onClick={props.onClose}
          class="absolute top-4 right-4 text-gray-400 hover:text-gray-600 transition-colors"
          aria-label="Close CAPTCHA"
        >
          <i class="fa-solid fa-times text-xl"></i>
        </button>

        <div class="text-center mb-6">
          <div class="inline-flex items-center justify-center w-16 h-16 bg-primary/10 rounded-full mb-4">
            <i class="fa-solid fa-shield-alt text-primary text-2xl"></i>
          </div>
          <h3 class="text-2xl font-bold text-gray-900 mb-2">Verify You're Human</h3>
          <p class="text-gray-600 text-sm">
            Please complete the CAPTCHA to submit your application.
          </p>
        </div>

        <div class="flex justify-center mb-4">
          <Show when={!isLoading()} fallback={
            <div class="py-8">
              <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto"></div>
              <p class="text-sm text-gray-500 mt-2">Loading CAPTCHA...</p>
            </div>
          }>
            <div ref={captchaContainer} id="captcha-container"></div>
          </Show>
        </div>

        <p class="text-xs text-center text-gray-500">
          This site is protected by reCAPTCHA and the Google{' '}
          <a href="https://policies.google.com/privacy" target="_blank" rel="noopener noreferrer" class="text-primary hover:underline">
            Privacy Policy
          </a>
          {' '}and{' '}
          <a href="https://policies.google.com/terms" target="_blank" rel="noopener noreferrer" class="text-primary hover:underline">
            Terms of Service
          </a>
          {' '}apply.
        </p>
      </div>
    </div>
  );
};

export default CaptchaModal;

