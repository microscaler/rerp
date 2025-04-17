import { Component, createSignal, onMount, createEffect } from 'solid-js';
import FinancialManagement from './features/FinancialManagement';
import SalesCRM from './features/SalesCRM';
import InventoryLogistics from './features/InventoryLogistics';
import Manufacturing from './features/Manufacturing';
import HumanResources from './features/HumanResources';
import ProjectManagement from './features/ProjectManagement';
import MarketingEcommerce from './features/MarketingEcommerce';
import AnalyticsBI from './features/AnalyticsBI';
import { updateSEO } from '../utils/seo';

const FeaturePage: Component = () => {
  // Extract slug from hash - works from any entry point
  const extractSlug = () => {
    if (typeof window === 'undefined') return '';
    const hash = window.location.hash;
    if (hash.startsWith('#feature-')) {
      return hash.replace('#feature-', '');
    }
    return '';
  };

  // Initialize slug immediately from hash (synchronous check)
  const [slug, setSlug] = createSignal<string>(extractSlug());

  // Update slug whenever hash changes (works from any entry point)
  const updateSlugFromHash = () => {
    const newSlug = extractSlug();
    if (newSlug && newSlug !== slug()) {
      setSlug(newSlug);
    } else if (!newSlug && slug()) {
      setSlug(''); // Clear slug if not a feature page
    }
  };

  // Watch for hash changes reactively - runs whenever component is rendered
  createEffect(() => {
    // Check hash whenever effect runs (component mount, re-render, etc.)
    updateSlugFromHash();
  });

  onMount(() => {
    // Set initial slug immediately (handles direct navigation, bookmarks, etc.)
    updateSlugFromHash();

    // Listen for hash changes (browser navigation, back/forward, direct links)
    const handleHashChange = () => {
      updateSlugFromHash();
    };
    
    window.addEventListener('hashchange', handleHashChange);
    
    // Also listen for popstate (browser back/forward buttons)
    window.addEventListener('popstate', handleHashChange);
    
    return () => {
      window.removeEventListener('hashchange', handleHashChange);
      window.removeEventListener('popstate', handleHashChange);
    };
  });

  // Update SEO when slug changes
  createEffect(() => {
    const currentSlug = slug();
    if (currentSlug) {
      const seoData = getFeatureSEO(currentSlug);
      if (seoData) {
        updateSEO(seoData);
      }
      // Scroll to top when feature page loads
      window.scrollTo({ top: 0, behavior: 'instant' });
    }
  });

  const renderFeature = () => {
    const currentSlug = slug();
    switch (currentSlug) {
      case 'smart-trading-alerts':
        return <SmartTradingAlerts />;
      case 'options-strategy-finder':
        return <OptionsStrategyFinder />;
      case 'global-market-coverage':
        return <GlobalMarketCoverage />;
      case 'live-trading-dashboard':
        return <LiveTradingDashboard />;
      case 'pattern-recognition':
        return <PatternRecognition />;
      case 'built-in-risk-management':
        return <BuiltInRiskManagement />;
      case 'financial-trading-education':
        return <FinancialTradingEducation />;
      case 'execution-options':
        return <ExecutionOptions />;
      case 'blogs-and-news':
        return <BlogsAndNews />;
      default:
        return (
          <div class="bg-white rounded-lg shadow-lg p-8 text-center">
            <h1 class="text-3xl font-bold text-gray-900 mb-4">Feature Not Found</h1>
            <p class="text-gray-600 mb-6">The feature page you're looking for doesn't exist.</p>
            <a
              href="#features"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#features';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
            >
              <i class="fa-solid fa-arrow-left mr-2"></i>
              Back to Features
            </a>
          </div>
        );
    }
  };

  return (
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {renderFeature()}
    </div>
  );
};

export default FeaturePage;
