import { Component, createSignal, onMount, createEffect } from 'solid-js';
import IronCondorStrategy from './blog/IronCondorStrategy';
import PatternRecognitionGuide from './blog/PatternRecognitionGuide';
import RiskManagementBasics from './blog/RiskManagementBasics';
import OptionsGreeksExplained from './blog/OptionsGreeksExplained';
import PaperTradingGuide from './blog/PaperTradingGuide';
import MultiExchangeTrading from './blog/MultiExchangeTrading';
import RelatedPosts from './RelatedPosts';
import { updateSEO } from '../utils/seo';
import { getBlogPostSEO } from '../data/seo-data';

const BlogPostPage: Component = () => {
  // Extract slug from hash - works from any entry point
  const extractSlug = () => {
    if (typeof window === 'undefined') return '';
    const hash = window.location.hash;
    if (hash.startsWith('#blog-')) {
      return hash.replace('#blog-', '');
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
      setSlug(''); // Clear slug if not a blog post
    }
  };

  // Watch for hash changes reactively - runs whenever component is rendered
  // This ensures slug is set even if component mounts before hash is available
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
      const seoData = getBlogPostSEO(currentSlug);
      if (seoData) {
        updateSEO(seoData);
      }
    }
  });

  const renderPost = () => {
    const currentSlug = slug();
    switch (currentSlug) {
      case 'iron-condor-strategy':
        return <IronCondorStrategy />;
      case 'pattern-recognition-guide':
        return <PatternRecognitionGuide />;
      case 'risk-management-basics':
        return <RiskManagementBasics />;
      case 'options-greeks-explained':
        return <OptionsGreeksExplained />;
      case 'paper-trading-guide':
        return <PaperTradingGuide />;
      case 'multi-exchange-trading':
        return <MultiExchangeTrading />;
      default:
        return (
          <div class="bg-white rounded-lg shadow-lg p-8 text-center">
            <h1 class="text-3xl font-bold text-gray-900 mb-4">Blog Post Not Found</h1>
            <p class="text-gray-600 mb-6">The blog post you're looking for doesn't exist.</p>
            <a
              href="#blogs"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#blogs';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
            >
              <i class="fa-solid fa-arrow-left mr-2"></i>
              Back to Blog
            </a>
          </div>
        );
    }
  };

  return (
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Main Content */}
        <div class="lg:col-span-8">
          {renderPost()}
        </div>
        
        {/* Sidebar - Related Posts */}
        <div class="lg:col-span-4 lg:sticky lg:top-8 lg:self-start">
          {slug() && <RelatedPosts currentSlug={slug()} />}
        </div>
      </div>
    </div>
  );
};

export default BlogPostPage;
