/**
 * Analytics utility functions for Google Analytics 4
 * 
 * Shared utility for RERP UI applications
 * 
 * Usage:
 * import { trackEvent, trackPageView, initializeGA } from '@shared/utils/analytics';
 * 
 * // Initialize GA in App.tsx onMount
 * initializeGA();
 * 
 * // Track events
 * trackEvent('button_click', { event_category: 'engagement', event_label: 'hero_cta' });
 * trackPageView('/dashboard');
 */

// Get GA Measurement ID directly from environment variable
const GA_MEASUREMENT_ID = import.meta.env.VITE_GA_MEASUREMENT_ID || '';

declare global {
  interface Window {
    dataLayer: any[];
    gtag: (...args: any[]) => void;
  }
}

/**
 * Initialize Google Analytics 4
 * Call this in App.tsx onMount() to set up GA tracking
 * 
 * @param initialPath - Initial page path to track (optional)
 */
export const initializeGA = (initialPath?: string): void => {
  if (typeof window === 'undefined') return;
  
  const gaId = GA_MEASUREMENT_ID;
  if (!gaId) {
    if (import.meta.env.DEV) {
      console.warn('⚠️ VITE_GA_MEASUREMENT_ID not set. Analytics will be disabled.');
    }
    return;
  }

  // Initialize dataLayer if not already initialized
  window.dataLayer = window.dataLayer || [];
  
  // Define gtag function if not already defined
  if (!window.gtag) {
    function gtag(...args: any[]) {
      window.dataLayer.push(args);
    }
    window.gtag = gtag;
    gtag('js', new Date());
  }

  // Load Google Analytics script if not already loaded
  if (!document.querySelector(`script[src*="gtag/js?id=${gaId}"]`)) {
    const script = document.createElement('script');
    script.async = true;
    script.src = `https://www.googletagmanager.com/gtag/js?id=${gaId}`;
    document.head.appendChild(script);
  }

  // Configure GA4 with initial path
  const path = initialPath || window.location.pathname + window.location.search + window.location.hash;
  window.gtag('config', gaId, {
    page_path: path,
    page_title: document.title,
    send_page_view: true
  });
};

/**
 * Check if Google Analytics is initialized
 */
export const isGAInitialized = (): boolean => {
  return typeof window !== 'undefined' && typeof window.gtag === 'function';
};

/**
 * Get the GA4 Measurement ID from environment variable
 */
export const getGAId = (): string => {
  return GA_MEASUREMENT_ID || '';
};

/**
 * Track a custom event
 * 
 * @param eventName - Name of the event (e.g., 'button_click', 'form_submit')
 * @param params - Event parameters (event_category, event_label, value, etc.)
 * 
 * @example
 * trackEvent('cta_click', {
 *   event_category: 'conversion',
 *   event_label: 'hero_start_trial',
 *   value: 99
 * });
 */
export const trackEvent = (
  eventName: string,
  params?: {
    event_category?: string;
    event_label?: string;
    value?: number;
    [key: string]: any;
  }
): void => {
  if (!isGAInitialized()) {
    console.warn('Google Analytics not initialized. Event not tracked:', eventName);
    return;
  }

  try {
    window.gtag('event', eventName, params || {});
  } catch (error) {
    console.error('Error tracking event:', error);
  }
};

/**
 * Track a page view
 * 
 * @param path - Page path (e.g., '/pricing', '/features', '/#faq', '/#/dashboard')
 * @param title - Optional page title
 * 
 * @example
 * trackPageView('/dashboard', 'Dashboard - Trader Portal');
 */
export const trackPageView = (path: string, title?: string): void => {
  if (!isGAInitialized()) {
    console.warn('Google Analytics not initialized. Page view not tracked:', path);
    return;
  }

  try {
    const gaId = getGAId();
    if (gaId) {
      window.gtag('config', gaId, {
        page_path: path,
        page_title: title || document.title,
        send_page_view: true
      });
    }
  } catch (error) {
    console.error('Error tracking page view:', error);
  }
};

/**
 * Track login events
 * 
 * @param method - Login method ('email-phone', 'google', 'github', 'saml', 'phone')
 * @param success - Whether login was successful
 */
export const trackLogin = (method: string, success: boolean): void => {
  trackEvent('login', {
    event_category: 'authentication',
    event_label: method,
    success: success
  });
};

/**
 * Track navigation events
 * 
 * @param section - Section/page identifier (e.g., 'dashboard', 'alerts', 'portfolio')
 * @param portal - Portal type ('trader', 'platform', 'fte', 'website')
 */
export const trackNavigation = (section: string, portal?: string): void => {
  trackEvent('navigation', {
    event_category: 'navigation',
    event_label: section,
    portal: portal
  });
};

/**
 * Track lesson navigation (FTE-specific)
 * 
 * @param lessonId - Lesson identifier
 * @param moduleId - Module identifier
 * @param action - Action type ('view', 'complete', 'start')
 */
export const trackLessonNavigation = (
  lessonId: string,
  moduleId: string,
  action: 'view' | 'complete' | 'start' = 'view'
): void => {
  trackEvent('lesson_navigation', {
    event_category: 'education',
    event_label: lessonId,
    module_id: moduleId,
    action: action
  });
};

/**
 * Track module navigation (FTE-specific)
 * 
 * @param moduleId - Module identifier
 * @param action - Action type ('view', 'start', 'complete')
 */
export const trackModuleNavigation = (
  moduleId: string,
  action: 'view' | 'start' | 'complete' = 'view'
): void => {
  trackEvent('module_navigation', {
    event_category: 'education',
    event_label: moduleId,
    action: action
  });
};

/**
 * Track admin/management actions (Platform Portal-specific)
 * 
 * @param action - Action type (e.g., 'user_created', 'settings_updated', 'org_switched')
 * @param resource - Resource type (e.g., 'user', 'organization', 'settings')
 */
export const trackAdminAction = (action: string, resource: string): void => {
  trackEvent('admin_action', {
    event_category: 'administration',
    event_label: action,
    resource: resource
  });
};

