import { Component, createSignal } from 'solid-js';

const Header: Component = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = createSignal(false);

  const nav = [
    { label: 'Home', href: '#', onClick: () => { window.location.hash = ''; window.scrollTo({ top: 0, behavior: 'instant' }); } },
    { label: 'About', href: '#about' },
    { label: 'Contact', href: '#contact' },
  ];

  return (
    <header class="bg-white shadow-sm border-b border-gray-200 sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <a
            href="#"
            onClick={(e) => { e.preventDefault(); window.location.hash = ''; window.scrollTo({ top: 0, behavior: 'instant' }); }}
            class="flex items-center space-x-2 text-gray-900 hover:opacity-80"
          >
            <i class="fa-solid fa-cube text-primary text-2xl"></i>
            <span class="text-xl font-bold">RERP</span>
          </a>

          <nav class="hidden md:flex items-center space-x-8">
            {nav.map(({ label, href, onClick }) => (
              <a
                href={href}
                class="text-gray-800 hover:text-primary font-medium"
                onClick={(e) => { if (onClick) { e.preventDefault(); onClick(); } }}
              >
                {label}
              </a>
            ))}
          </nav>

          <button
            class="md:hidden text-gray-800 hover:text-primary"
            onClick={() => setMobileMenuOpen(!mobileMenuOpen())}
          >
            <i class={`fa-solid ${mobileMenuOpen() ? 'fa-times' : 'fa-bars'} text-xl`}></i>
          </button>
        </div>

        <div class={`md:hidden ${mobileMenuOpen() ? 'block' : 'hidden'} pb-4 space-y-2`}>
          {nav.map(({ label, href, onClick }) => (
            <a
              href={href}
              class="block text-gray-800 hover:text-primary font-medium"
              onClick={(e) => { if (onClick) { e.preventDefault(); onClick(); } setMobileMenuOpen(false); }}
            >
              {label}
            </a>
          ))}
        </div>
      </div>
    </header>
  );
};

export default Header;
