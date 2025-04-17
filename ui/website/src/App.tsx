import { Component, createSignal, onMount, onCleanup, createEffect } from 'solid-js';
import Header from '@shared/header/Header';
import Hero from './components/Hero';
import Features from './components/Features';
import HowItWorksPreview from './components/HowItWorksPreview';
import Stats from './components/Stats';
import Testimonials from './components/Testimonials';
import About from './components/About';
import Pricing from './components/Pricing';
import TrustSecurity from './components/TrustSecurity';
import CTA from './components/CTA';
import Contact from './components/Contact';
import Footer from '@shared/footer/Footer';
import ExitIntentPopup from './components/ExitIntentPopup';
import StickyCTABar from './components/StickyCTABar';
import FeaturePage from './components/FeaturePage';
import BlogsAndNewsPage from './components/BlogsAndNewsPage';
import BlogPostPage from './components/BlogPostPage';
import FAQPage from './components/FAQPage';
import HowItWorksPage from './components/HowItWorksPage';
import ModulesPage from './components/ModulesPage';
import CommunityPage from './components/CommunityPage';
import BlogCategoryPage from './components/BlogCategoryPage';
import CaseStudyPage from './components/CaseStudyPage';
import CaseStudiesPage from './components/CaseStudiesPage';
import SignInPage from './components/SignInPage';
import FreeTrialForm from './components/FreeTrialForm';
import TermsOfService from './components/TermsOfService';
import PrivacyPolicy from './components/PrivacyPolicy';
import RefundPolicy from './components/RefundPolicy';
import CareersPage from './components/CareersPage';
import JobApplicationPage from './components/JobApplicationPage';
import StatusPage from './components/StatusPage';
import { updateSEO, resetSEO } from './utils/seo';
import { homepageSEO, faqPageSEO, howItWorksSEO, blogsNewsSEO } from './data/seo-data';
import { initializeGA, trackPageView } from '@shared/utils/analytics';

const App: Component = () => {
  const [currentView, setCurrentView] = createSignal<'home' | 'feature' | 'blogs' | 'blog-post' | 'faq' | 'how-it-works' | 'blog-category' | 'case-study' | 'case-studies' | 'sign-in' | 'free-trial' | 'terms-of-service' | 'privacy-policy' | 'refund-policy' | 'careers' | 'job-application' | 'status' | 'modules' | 'community'>('home');
  
  // Track if hash change was manual (user click) vs automatic (scroll)
  let isManualHashChange = false;
  let scrollUpdateTimeout: number | null = null;

  onMount(() => {
    // Initialize Google Analytics
    const initialPath = window.location.pathname + window.location.search + window.location.hash;
    initializeGA(initialPath);
    trackPageView(initialPath);

    const hash = window.location.hash;
    if (!hash || hash === '#') {
      setCurrentView('home');
      resetSEO();
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash.startsWith('#feature-')) {
      setCurrentView('feature');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash.startsWith('#blog-')) {
      setCurrentView('blog-post');
    } else if (hash === '#blogs') {
      setCurrentView('blogs');
      updateSEO(blogsNewsSEO);
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#sign-in') {
      setCurrentView('sign-in');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#free-trial') {
      setCurrentView('free-trial');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#terms-of-service') {
      setCurrentView('terms-of-service');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#privacy-policy') {
      setCurrentView('privacy-policy');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#refund-policy') {
      setCurrentView('refund-policy');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#careers') {
      setCurrentView('careers');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash.startsWith('#job-application')) {
      setCurrentView('job-application');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#faq' || hash === '#faq-page') {
      setCurrentView('faq');
      updateSEO(faqPageSEO);
    } else if (hash === '#how-it-works-page') {
      setCurrentView('how-it-works');
      updateSEO(howItWorksSEO);
    } else if (hash === '#modules') {
      setCurrentView('modules');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#community') {
      setCurrentView('community');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash.startsWith('#category-')) {
      setCurrentView('blog-category');
      // SEO will be handled by BlogCategoryPage component
    } else if (hash.startsWith('#case-study-')) {
      setCurrentView('case-study');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else if (hash === '#case-studies') {
      setCurrentView('case-studies');
    } else if (hash === '#status') {
      setCurrentView('status');
      window.scrollTo({ top: 0, behavior: 'instant' });
    } else {
      setCurrentView('home');
      // Set homepage SEO
      updateSEO(homepageSEO);
      // Scroll to section if hash is present (e.g., #features, #pricing)
      if (hash && hash !== '#') {
        setTimeout(() => {
          const element = document.querySelector(hash);
          if (element) {
            element.scrollIntoView({ behavior: 'smooth', block: 'start' });
          }
        }, 100);
      }
    }

    // Setup scroll-based hash updates for homepage sections
    const setupScrollHashUpdates = (): (() => void) => {
      // Only enable on homepage
      if (currentView() !== 'home') return () => {};
      
      // Check if IntersectionObserver is supported
      if (typeof window === 'undefined' || !('IntersectionObserver' in window)) {
        return () => {}; // Return empty cleanup function
      }
      
      // Section IDs in order of appearance on homepage
      const sections = [
        'hero',
        'features',
        'how-it-works',
        'stats',
        'testimonials',
        'about',
        'pricing',
        'roi',
        'trust-security',
        'cta',
        'contact'
      ];
      
      // Use Intersection Observer to detect which section is in view
      const observerOptions = {
        root: null,
        rootMargin: '-20% 0px -50% 0px', // Trigger when section is 20% from top
        threshold: 0
      };
      
      let observer: IntersectionObserver | null = null;
      
      try {
        observer = new IntersectionObserver((entries) => {
          // Clear any pending scroll updates
          if (scrollUpdateTimeout) {
            clearTimeout(scrollUpdateTimeout);
          }
          
          // Don't update hash if user just clicked a link
          if (isManualHashChange) {
            isManualHashChange = false;
            return;
          }
          
          // Find the section that's most visible
          let mostVisibleSection: string | null = null;
          let maxVisibility = 0;
          
          entries.forEach((entry) => {
            if (entry.isIntersecting && entry.target instanceof Element) {
              const rect = entry.boundingClientRect;
              const visibility = Math.min(rect.height, window.innerHeight - rect.top) / window.innerHeight;
              if (visibility > maxVisibility) {
                maxVisibility = visibility;
                mostVisibleSection = entry.target.id;
              }
            }
          });
          
          // Update hash if we found a visible section
          if (mostVisibleSection && mostVisibleSection !== window.location.hash.replace('#', '')) {
            scrollUpdateTimeout = window.setTimeout(() => {
              // Use replaceState to avoid adding to history and triggering scroll
              const newHash = `#${mostVisibleSection}`;
              window.history.replaceState(null, '', newHash);
              
              // Track page view for analytics (scroll-based hash updates)
              const scrollPath = window.location.pathname + window.location.search + newHash;
              trackPageView(scrollPath);
            }, 100); // Small delay to debounce rapid scroll events
          }
        }, observerOptions);
        
        // Observe all homepage sections - with validation
        sections.forEach((sectionId) => {
          try {
            const element = document.getElementById(sectionId);
            // Validate element exists and is a valid Node
            if (element && element instanceof Element && element.nodeType === Node.ELEMENT_NODE) {
              observer!.observe(element);
            }
          } catch (error) {
            // Silently skip invalid elements (may not exist yet or may be invalid)
            console.debug(`Skipping observation of section: ${sectionId}`, error);
          }
        });
      } catch (error) {
        console.error('Failed to setup scroll hash updates:', error);
        return () => {}; // Return empty cleanup function
      }
      
      return () => {
        if (observer) {
          try {
            observer.disconnect();
          } catch (error) {
            console.debug('Error disconnecting observer:', error);
          }
        }
        if (scrollUpdateTimeout) {
          clearTimeout(scrollUpdateTimeout);
        }
      };
    };
    
    // Setup scroll hash updates when on homepage
    let scrollCleanup: (() => void) | null = null;
    createEffect(() => {
      // Cleanup previous observer if exists
      if (scrollCleanup) {
        scrollCleanup();
        scrollCleanup = null;
      }
      
      if (currentView() === 'home') {
        // Wait for DOM to be ready and all sections to be rendered
        // Use requestAnimationFrame to ensure DOM is fully rendered
        const setupObserver = () => {
          requestAnimationFrame(() => {
            scrollCleanup = setupScrollHashUpdates();
          });
        };
        
        // Try immediately, then again after a short delay to catch late-rendering sections
        setupObserver();
        setTimeout(setupObserver, 1000);
      }
    });

    const handleHashChange = () => {
      const newHash = window.location.hash;
      
      // Mark as manual hash change (user clicked a link)
      isManualHashChange = true;
      
      // Track page view for hash changes (SPA navigation)
      // This ensures all navigation (clicks, scroll, back/forward) is tracked
      const fullPath = window.location.pathname + window.location.search + newHash;
      trackPageView(fullPath);
      
      if (!newHash || newHash === '#') {
        setCurrentView('home');
        resetSEO();
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash.startsWith('#feature-')) {
        setCurrentView('feature');
        window.scrollTo({ top: 0, behavior: 'instant' });
        // SEO will be handled by FeaturePage component
      } else if (newHash.startsWith('#blog-')) {
        setCurrentView('blog-post');
        // SEO will be handled by BlogPostPage component
      } else if (newHash === '#blogs') {
        setCurrentView('blogs');
        updateSEO(blogsNewsSEO);
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#sign-in') {
        setCurrentView('sign-in');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#free-trial') {
        setCurrentView('free-trial');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#terms-of-service') {
        setCurrentView('terms-of-service');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#privacy-policy') {
        setCurrentView('privacy-policy');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#refund-policy') {
        setCurrentView('refund-policy');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#careers') {
        setCurrentView('careers');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash.startsWith('#job-application')) {
        setCurrentView('job-application');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#faq' || newHash === '#faq-page') {
        setCurrentView('faq');
        updateSEO(faqPageSEO);
      } else if (newHash === '#how-it-works-page') {
        setCurrentView('how-it-works');
        updateSEO(howItWorksSEO);
      } else if (newHash === '#modules') {
        setCurrentView('modules');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash === '#community') {
        setCurrentView('community');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (newHash.startsWith('#category-')) {
        setCurrentView('blog-category');
        // SEO will be handled by BlogCategoryPage component
      } else if (newHash.startsWith('#case-study-')) {
        setCurrentView('case-study');
        window.scrollTo({ top: 0, behavior: 'instant' });
        // SEO will be handled by CaseStudyPage component
      } else if (newHash === '#case-studies') {
        setCurrentView('case-studies');
      } else if (newHash === '#status') {
        setCurrentView('status');
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else {
        setCurrentView('home');
        updateSEO(homepageSEO);
        // Scroll to section when navigating back
        if (newHash && newHash !== '#') {
          setTimeout(() => {
            const element = document.querySelector(newHash);
            if (element) {
              element.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
          }, 100);
        } else {
          // Scroll to top if no hash
          window.scrollTo({ top: 0, behavior: 'smooth' });
        }
      }
    };
    
    // Handle browser back/forward navigation
    const handlePopState = () => {
      isManualHashChange = true;
      handleHashChange();
    };
    
    window.addEventListener('hashchange', handleHashChange);
    window.addEventListener('popstate', handlePopState);
    
    return () => {
      window.removeEventListener('hashchange', handleHashChange);
      window.removeEventListener('popstate', handlePopState);
      if (scrollCleanup) {
        scrollCleanup();
      }
      if (scrollUpdateTimeout) {
        clearTimeout(scrollUpdateTimeout);
      }
    };
  });

  return (
    <div class="bg-gray-50">
      <Header />
      {currentView() === 'home' && (
        <main>
          <Hero />
          <Features />
          <HowItWorksPreview />
          <Stats />
          <Testimonials />
          <About />
          <Pricing />
          <TrustSecurity />
          <CTA />
          <Contact />
        </main>
      )}
      {currentView() === 'feature' && <FeaturePage />}
      {currentView() === 'blogs' && <BlogsAndNewsPage />}
      {currentView() === 'blog-post' && <BlogPostPage />}
      {currentView() === 'blog-category' && <BlogCategoryPage />}
      {currentView() === 'case-study' && <CaseStudyPage />}
      {currentView() === 'case-studies' && <CaseStudiesPage />}
      {currentView() === 'faq' && <FAQPage />}
      {currentView() === 'how-it-works' && <HowItWorksPage />}
      {currentView() === 'modules' && <ModulesPage />}
      {currentView() === 'community' && <CommunityPage />}
      {currentView() === 'sign-in' && <SignInPage />}
      {currentView() === 'free-trial' && <FreeTrialForm />}
      {currentView() === 'terms-of-service' && <TermsOfService />}
      {currentView() === 'privacy-policy' && <PrivacyPolicy />}
      {currentView() === 'refund-policy' && <RefundPolicy />}
      {currentView() === 'careers' && <CareersPage />}
      {currentView() === 'job-application' && <JobApplicationPage />}
      {currentView() === 'status' && <StatusPage />}
      <ExitIntentPopup />
      <StickyCTABar />
      <Footer />
    </div>
  );
};

export default App;
