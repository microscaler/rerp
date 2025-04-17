import { Component } from 'solid-js';
import FloatingSidebar from '../FloatingSidebar';

interface LegalPagesSidebarProps {
  currentPage: 'terms-of-service' | 'privacy-policy' | 'refund-policy';
  title?: string;
}

const LegalPagesSidebar: Component<LegalPagesSidebarProps> = (props) => {
  const legalPages = [
    {
      slug: 'terms-of-service',
      title: 'Terms of Service',
      href: '#terms-of-service',
      icon: 'fa-solid fa-file-contract',
    },
    {
      slug: 'privacy-policy',
      title: 'Privacy Policy',
      href: '#privacy-policy',
      icon: 'fa-solid fa-shield-halved',
    },
    {
      slug: 'refund-policy',
      title: 'Refund Policy',
      href: '#refund-policy',
      icon: 'fa-solid fa-money-bill-wave',
    },
  ];

  const handleClick = (slug: string) => {
    // Track navigation
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'legal_page_navigation', {
        event_category: 'engagement',
        event_label: `legal_sidebar_${slug}`,
        from_page: props.currentPage,
        to_page: slug,
      });
    }
  };

  return (
    <FloatingSidebar title={props.title || 'Legal Pages'}>
      {legalPages.map((page) => {
        const isActive = page.slug === props.currentPage;
        return (
          <a
            href={page.href}
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = page.href;
              window.scrollTo({ top: 0, behavior: 'instant' });
              handleClick(page.slug);
            }}
            class={`block rounded-lg p-4 transition-all border ${
              isActive
                ? 'bg-primary-50 border-primary-300 text-primary-900 shadow-md'
                : 'bg-gray-50 border-gray-200 hover:border-primary hover:bg-primary-50 hover:shadow-md'
            }`}
          >
            <div class="flex items-start space-x-3">
              <div
                class={`text-xl flex-shrink-0 mt-0.5 ${
                  isActive ? 'text-primary-600' : 'text-gray-600'
                }`}
              >
                <i class={page.icon}></i>
              </div>
              <div class="flex-1 min-w-0">
                <h3
                  class={`text-sm font-semibold mb-1 transition-colors ${
                    isActive
                      ? 'text-primary-900'
                      : 'text-gray-900 hover:text-primary-600'
                  }`}
                >
                  {page.title}
                </h3>
                {isActive && (
                  <div class="flex items-center text-primary-600 text-xs font-medium">
                    <span>Current page</span>
                    <i class="fa-solid fa-check-circle ml-2"></i>
                  </div>
                )}
              </div>
            </div>
          </a>
        );
      })}
    </FloatingSidebar>
  );
};

export default LegalPagesSidebar;

