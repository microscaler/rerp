import { Component, createSignal, onMount, createEffect } from 'solid-js';
import RelatedCaseStudies from './RelatedCaseStudies';
import { updateSEO } from '../utils/seo';
import { getCaseStudyBySlug, caseStudiesData } from '../data/case-studies-data';
import CaseStudyLayout from './case-studies/CaseStudyLayout';
import { BASE_URL } from '../config/constants';
import { generateBreadcrumbSchema } from '../utils/internal-linking';

const CaseStudyPage: Component = () => {
  // Extract slug from hash - works from any entry point
  const extractSlug = () => {
    if (typeof window === 'undefined') return '';
    const hash = window.location.hash;
    if (hash.startsWith('#case-study-')) {
      return hash.replace('#case-study-', '');
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
      setSlug(''); // Clear slug if not a case study
    }
  };

  // Watch for hash changes reactively
  createEffect(() => {
    updateSlugFromHash();
  });

  onMount(() => {
    updateSlugFromHash();

    const handleHashChange = () => {
      updateSlugFromHash();
    };
    
    window.addEventListener('hashchange', handleHashChange);
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
      const caseStudy = getCaseStudyBySlug(currentSlug);
      if (caseStudy) {
        updateSEO({
          title: `${caseStudy.title} | PriceWhisperer Case Study`,
          description: caseStudy.excerpt,
          keywords: `case study, ${caseStudy.role.toLowerCase()}, trading success, ${caseStudy.customer}, ${caseStudy.location}`,
          canonical: `${BASE_URL}/#case-study-${currentSlug}`,
          ogType: 'article',
          ogImage: '/og-image.jpg',
          structuredData: [
            {
              '@context': 'https://schema.org',
              '@type': 'Article',
              headline: caseStudy.title,
              description: caseStudy.excerpt,
              author: {
                '@type': 'Person',
                name: caseStudy.customer,
                jobTitle: caseStudy.role
              },
              publisher: {
                '@type': 'Organization',
                name: 'PriceWhisperer'
              },
              datePublished: new Date().toISOString(),
              dateModified: new Date().toISOString()
            },
            generateBreadcrumbSchema([
              { name: 'Home', url: BASE_URL },
              { name: 'Case Studies', url: `${BASE_URL}/#case-studies` },
              { name: caseStudy.title, url: `${BASE_URL}/#case-study-${currentSlug}` }
            ])
          ]
        });
      }
      // Scroll to top when case study page loads
      window.scrollTo({ top: 0, behavior: 'instant' });
    }
  });

  const renderCaseStudy = () => {
    const currentSlug = slug();
    if (!currentSlug) return null;

    const caseStudy = getCaseStudyBySlug(currentSlug);
    if (!caseStudy) {
      return (
        <div class="bg-white rounded-lg shadow-lg p-8 text-center">
          <h1 class="text-3xl font-bold text-gray-900 mb-4">Case Study Not Found</h1>
          <p class="text-gray-600 mb-6">The case study you're looking for doesn't exist.</p>
          <a
            href="#case-studies"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#case-studies';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
          >
            <i class="fa-solid fa-arrow-left mr-2"></i>
            Back to Case Studies
          </a>
        </div>
      );
    }

    return <CaseStudyLayout data={caseStudy} showBackLink={true} />;
  };

  return (
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
        {/* Main Content */}
        <div class="lg:col-span-8">
          {renderCaseStudy()}
        </div>
        
        {/* Sidebar - Related Case Studies */}
        <div class="lg:col-span-4 lg:sticky lg:top-8 lg:self-start">
          {slug() && <RelatedCaseStudies currentSlug={slug()} />}
        </div>
      </div>
    </div>
  );
};

export default CaseStudyPage;

