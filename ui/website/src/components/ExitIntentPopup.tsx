import { Component, createSignal, onMount, onCleanup } from 'solid-js';
import { executeRecaptcha } from '../utils/recaptcha';
import { saveEmailCapture } from '../utils/supabase';

const ExitIntentPopup: Component = () => {
  const [showPopup, setShowPopup] = createSignal(false);
  const [email, setEmail] = createSignal('');
  const [emailSubmitted, setEmailSubmitted] = createSignal(false);
  const [hasShown, setHasShown] = createSignal(false);

  const handleEmailSubmit = async (e: Event) => {
    e.preventDefault();
    
    try {
      // Execute reCAPTCHA v3
      const recaptchaToken = await executeRecaptcha('submit_exit_intent_email');
      
      // Save to Supabase
      await saveEmailCapture({
        email: email(),
        source: 'exit_intent',
        recaptcha_token: recaptchaToken,
      });
      
      console.log('Exit intent email submitted:', email());
      setEmailSubmitted(true);
      
      // Track conversion
      if (typeof window !== 'undefined' && (window as any).gtag) {
        (window as any).gtag('event', 'email_capture', {
          event_category: 'lead_generation',
          event_label: 'exit_intent_popup'
        });
      }
      
      // Hide popup after 3 seconds
      setTimeout(() => {
        setShowPopup(false);
        setHasShown(true);
        // Store in localStorage to prevent showing again for this session
        if (typeof window !== 'undefined') {
          sessionStorage.setItem('exitIntentShown', 'true');
        }
      }, 3000);
    } catch (error) {
      console.error('Email submission error:', error);
      // Optionally show error to user
    }
  };

  const handleClose = () => {
    setShowPopup(false);
    setHasShown(true);
    if (typeof window !== 'undefined') {
      sessionStorage.setItem('exitIntentShown', 'true');
    }
  };

  onMount(() => {
    // Check if we've already shown the popup this session
    if (typeof window !== 'undefined') {
      const shown = sessionStorage.getItem('exitIntentShown');
      if (shown === 'true') {
        setHasShown(true);
        return;
      }
    }

    // Detect exit intent (mouse leaving viewport at top)
    const handleMouseLeave = (e: MouseEvent) => {
      // Only trigger if mouse is moving upward (toward top of screen)
      if (e.clientY <= 0 && !hasShown()) {
        setShowPopup(true);
        
        // Track exit intent
        if (typeof window !== 'undefined' && (window as any).gtag) {
          (window as any).gtag('event', 'exit_intent', {
            event_category: 'engagement',
            event_label: 'mouse_leave'
          });
        }
      }
    };

    // Also detect when user tries to close tab/window
    const handleBeforeUnload = () => {
      if (!hasShown() && !showPopup()) {
        setShowPopup(true);
      }
    };

    document.addEventListener('mouseleave', handleMouseLeave);
    window.addEventListener('beforeunload', handleBeforeUnload);

    onCleanup(() => {
      document.removeEventListener('mouseleave', handleMouseLeave);
      window.removeEventListener('beforeunload', handleBeforeUnload);
    });
  });

  if (!showPopup()) return null;

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50 p-4">
      <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full p-8 relative animate-slide-up">
        <button
          onClick={handleClose}
          class="absolute top-4 right-4 text-gray-400 hover:text-gray-600 transition-colors"
        >
          <i class="fa-solid fa-times text-xl"></i>
        </button>

        {!emailSubmitted() ? (
          <>
            <div class="text-center mb-6">
              <div class="inline-flex items-center justify-center w-16 h-16 bg-primary/10 rounded-full mb-4">
                <i class="fa-solid fa-gift text-primary text-2xl"></i>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 mb-2">Wait! Don't Miss Out</h3>
              <p class="text-gray-600">
                Get <span class="font-semibold text-primary">10 free trading alerts</span> before you go. 
                No spam, unsubscribe anytime.
              </p>
            </div>

            <form onSubmit={handleEmailSubmit} class="space-y-4">
              <input
                type="email"
                placeholder="Enter your email"
                value={email()}
                onInput={(e) => setEmail(e.currentTarget.value)}
                required
                class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary"
                autofocus
              />
              <button
                type="submit"
                class="w-full bg-primary text-white py-3 rounded-lg hover:bg-blue-700 font-semibold transition-colors"
              >
                Get My Free Alerts
              </button>
            </form>

            <p class="text-xs text-center text-gray-500 mt-4">
              Join 10,000+ traders getting smarter every day
            </p>
          </>
        ) : (
          <div class="text-center">
            <div class="inline-flex items-center justify-center w-16 h-16 bg-green-100 rounded-full mb-4">
              <i class="fa-solid fa-check text-green-600 text-2xl"></i>
            </div>
            <h3 class="text-2xl font-bold text-gray-900 mb-2">Thanks! Check Your Email</h3>
            <p class="text-gray-600">
              We've sent your 10 free alerts. Check your inbox to get started!
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ExitIntentPopup;

