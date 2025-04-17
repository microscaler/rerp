import { Component, createSignal, onMount, onCleanup, createEffect } from 'solid-js';

const StickyCTABar: Component = () => {
  const [isVisible, setIsVisible] = createSignal(false);
  const [currentHash, setCurrentHash] = createSignal(
    typeof window !== 'undefined' ? window.location.hash : ''
  );

  const handleScroll = () => {
    if (typeof window === 'undefined') return;
    // Get scroll position from multiple sources for compatibility
    const scrollY = window.scrollY 
      || window.pageYOffset 
      || document.documentElement.scrollTop 
      || document.body.scrollTop 
      || 0;
    const shouldShow = scrollY > 300; // Show after scrolling 300px
    setIsVisible(shouldShow);
  };

  const handleHashChange = () => {
    if (typeof window !== 'undefined') {
      setCurrentHash(window.location.hash);
    }
  };

  const handleCTAClick = (action: 'trial' | 'demo') => {
    // Track CTA click
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'cta_click', {
        event_category: 'engagement',
        event_label: `sticky_bar_${action}`,
        event_location: 'sticky_cta_bar'
      });
    }

    if (action === 'trial') {
      // Navigate to free trial form
      window.location.hash = '#free-trial';
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else {
      // Navigate to contact section
      window.location.hash = '#contact';
      window.scrollTo({ top: 0, behavior: 'smooth' });
    }
  };

  // Don't show on certain pages (like pricing, contact, etc.)
  const shouldHide = () => {
    const hash = currentHash();
    // Hide on pricing, contact, case studies, blog posts, and free trial form
    // But show on homepage and other pages
    return hash === '#pricing' || hash === '#contact' || hash === '#free-trial' || hash.startsWith('#case-study') || hash.startsWith('#blog-');
  };

  onMount(() => {
    if (typeof window === 'undefined') return;
    
    // Listen to scroll on window
    window.addEventListener('scroll', handleScroll, { passive: true });
    window.addEventListener('hashchange', handleHashChange);
    
    // Check initial state
    handleScroll();
    handleHashChange();
    
    // Also check on delays to catch any initial render issues
    setTimeout(handleScroll, 100);
    setTimeout(handleScroll, 500);
  });

  onCleanup(() => {
    if (typeof window === 'undefined') return;
    window.removeEventListener('scroll', handleScroll);
    window.removeEventListener('hashchange', handleHashChange);
  });

  // Update visibility based on hash changes
  createEffect(() => {
    const hash = currentHash();
    // Re-check scroll position when hash changes
    if (typeof window !== 'undefined') {
      setTimeout(handleScroll, 50);
    }
  });

  // Always render, but control visibility with classes
  const showBar = () => isVisible() && !shouldHide();

  return (
    <div
      class="fixed bottom-0 left-0 right-0 z-50 bg-white border-t border-gray-200 shadow-lg transition-all duration-300 ease-in-out"
      classList={{
        'translate-y-0 opacity-100': showBar(),
        'translate-y-full opacity-0 pointer-events-none': !showBar()
      }}
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3">
        <div class="flex items-center justify-between flex-wrap gap-3">
          <div class="flex-1 min-w-0">
            <p class="text-sm sm:text-base font-semibold text-gray-900">
              Start your free trial
            </p>
            <p class="text-xs text-gray-600">
              14-day free trial
            </p>
          </div>
          <div class="flex items-center space-x-3">
            <button
              onClick={() => handleCTAClick('trial')}
              class="bg-primary text-white px-4 sm:px-6 py-2 rounded-lg hover:bg-blue-700 font-semibold text-sm sm:text-base transition-colors whitespace-nowrap"
            >
              Start Free Trial
            </button>
            <button
              onClick={() => handleCTAClick('demo')}
              class="border border-gray-300 text-gray-700 px-4 sm:px-6 py-2 rounded-lg hover:bg-gray-50 font-semibold text-sm sm:text-base transition-colors whitespace-nowrap"
            >
              Schedule Demo
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StickyCTABar;

