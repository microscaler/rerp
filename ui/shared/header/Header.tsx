import { Component, createSignal, createMemo } from 'solid-js';

// Detect if we're on the FTE site based on the current path
const isFTESite = (): boolean => {
  if (typeof window === 'undefined') return false;
  return window.location.pathname.startsWith('/fte/') || window.location.pathname === '/fte';
};

const Header: Component = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = createSignal(false);
  const isFTE = createMemo(() => isFTESite());

  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen());
  };

  return (
    <header id="header" class="bg-white shadow-sm border-b border-gray-200 sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center space-x-6">
            <a 
              href={isFTE() ? "https://pricewhisperer.ai" : "#"}
              onClick={(e) => {
                if (!isFTE()) {
                  e.preventDefault();
                  window.location.hash = '';
                  window.scrollTo({ top: 0, behavior: 'instant' });
                }
              }}
              class="flex items-center space-x-2 cursor-pointer hover:opacity-80 transition-opacity"
            >
              <i class="fa-solid fa-chart-line text-primary text-2xl"></i>
              <span class="text-xl font-bold text-gray-900">PriceWhisperer</span>
            </a>
            
            {/* Desktop Navigation */}
            <nav class="hidden md:flex items-center space-x-8">
              {isFTE() ? (
                // FTE Site: Full navigation menu pointing to main website
                <>
                  <a 
                    href="https://pricewhisperer.ai"
                    class="text-gray-800 hover:text-primary font-medium"
                  >
                    Home
                  </a>
                  <a href="https://pricewhisperer.ai/#features" class="text-gray-800 hover:text-primary font-medium">Features</a>
                  <a href="https://pricewhisperer.ai/#how-it-works" class="text-gray-800 hover:text-primary font-medium">How It Works</a>
                  <a href="https://pricewhisperer.ai/#testimonials" class="text-gray-800 hover:text-primary font-medium">Testimonials</a>
                  <a href="https://pricewhisperer.ai/#case-studies" class="text-gray-800 hover:text-primary font-medium">
                    Case Studies
                  </a>
                  <a href="https://pricewhisperer.ai/#pricing" class="text-gray-800 hover:text-primary font-medium">Pricing</a>
                  <a href="https://pricewhisperer.ai/#blogs" class="text-gray-800 hover:text-primary font-medium">
                    Blog
                  </a>
                </>
              ) : (
                // Website: Full navigation menu
                <>
                  <a 
                    href="#"
                    onClick={(e) => {
                      e.preventDefault();
                      window.location.hash = '';
                      window.scrollTo({ top: 0, behavior: 'instant' });
                    }}
                    class="text-gray-800 hover:text-primary font-medium"
                  >
                    Home
                  </a>
                  <a href="#features" class="text-gray-800 hover:text-primary font-medium">Features</a>
                  <a href="#how-it-works" class="text-gray-800 hover:text-primary font-medium">How It Works</a>
                  <a href="#testimonials" class="text-gray-800 hover:text-primary font-medium">Testimonials</a>
                  <a 
                    href="#case-studies" 
                    onClick={(e) => {
                      e.preventDefault();
                      window.location.hash = '#case-studies';
                      window.scrollTo({ top: 0, behavior: 'instant' });
                    }}
                    class="text-gray-800 hover:text-primary font-medium"
                  >
                    Case Studies
                  </a>
                  <a href="#pricing" class="text-gray-800 hover:text-primary font-medium">Pricing</a>
                  <a 
                    href="#blogs" 
                    onClick={(e) => {
                      e.preventDefault();
                      window.location.hash = '#blogs';
                      window.scrollTo({ top: 0, behavior: 'instant' });
                    }}
                    class="text-gray-800 hover:text-primary font-medium"
                  >
                    Blog
                  </a>
                </>
              )}
            </nav>
          </div>
          
          <div class="flex items-center space-x-4">
            <a
              href={isFTE() ? "https://pricewhisperer.ai/#sign-in" : "#sign-in"}
              onClick={(e) => {
                if (!isFTE()) {
                  e.preventDefault();
                  window.location.hash = '#sign-in';
                  window.scrollTo({ top: 0, behavior: 'instant' });
                }
              }}
              class="text-gray-800 hover:text-primary font-medium"
            >
              Sign In
            </a>
            
            {/* Mobile menu button */}
            <button
              class="md:hidden text-gray-800 hover:text-primary"
              onClick={toggleMobileMenu}
            >
              <i class={`fa-solid ${mobileMenuOpen() ? 'fa-times' : 'fa-bars'} text-xl`}></i>
            </button>
          </div>
        </div>
        
        {/* Mobile Menu */}
        <div class={`md:hidden ${mobileMenuOpen() ? 'block' : 'hidden'} pb-4`}>
          <div class="flex flex-col space-y-4">
            {isFTE() ? (
              // FTE Site: Full navigation menu pointing to main website
              <>
                <a href="https://pricewhisperer.ai" class="text-gray-800 hover:text-primary font-medium">
                  Home
                </a>
                <a href="https://pricewhisperer.ai/#features" class="text-gray-800 hover:text-primary font-medium">Features</a>
                <a href="https://pricewhisperer.ai/#how-it-works" class="text-gray-800 hover:text-primary font-medium">How It Works</a>
                <a href="https://pricewhisperer.ai/#testimonials" class="text-gray-800 hover:text-primary font-medium">Testimonials</a>
                <a href="https://pricewhisperer.ai/#case-studies" class="text-gray-800 hover:text-primary font-medium">
                  Case Studies
                </a>
                <a href="https://pricewhisperer.ai/#pricing" class="text-gray-800 hover:text-primary font-medium">Pricing</a>
                <a href="https://pricewhisperer.ai/#blogs" class="text-gray-800 hover:text-primary font-medium">
                  Blog
                </a>
              </>
            ) : (
              // Website: Full navigation menu
              <>
                <a 
                  href="#"
                  onClick={(e) => {
                    e.preventDefault();
                    window.location.hash = '';
                    window.scrollTo({ top: 0, behavior: 'instant' });
                    setMobileMenuOpen(false);
                  }}
                  class="text-gray-800 hover:text-primary font-medium"
                >
                  Home
                </a>
                <a href="#features" class="text-gray-800 hover:text-primary font-medium">Features</a>
                <a href="#how-it-works" class="text-gray-800 hover:text-primary font-medium">How It Works</a>
                <a href="#testimonials" class="text-gray-800 hover:text-primary font-medium">Testimonials</a>
                <a 
                  href="#case-studies" 
                  onClick={(e) => {
                    e.preventDefault();
                    window.location.hash = '#case-studies';
                    window.scrollTo({ top: 0, behavior: 'instant' });
                    setMobileMenuOpen(false); // Close mobile menu after navigation
                  }}
                  class="text-gray-800 hover:text-primary font-medium"
                >
                  Case Studies
                </a>
                <a href="#pricing" class="text-gray-800 hover:text-primary font-medium">Pricing</a>
                <a 
                  href="#blogs" 
                  onClick={(e) => {
                    e.preventDefault();
                    window.location.hash = '#blogs';
                    window.scrollTo({ top: 0, behavior: 'instant' });
                    setMobileMenuOpen(false); // Close mobile menu after navigation
                  }}
                  class="text-gray-800 hover:text-primary font-medium"
                >
                  Blog
                </a>
              </>
            )}
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;

