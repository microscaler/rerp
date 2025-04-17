/**
 * Analytics utility functions for Google Analytics 4
 * 
 * Usage:
 * import { trackEvent, trackPageView } from './utils/analytics';
 * 
 * trackEvent('button_click', { event_category: 'engagement', event_label: 'hero_cta' });
 * trackPageView('/pricing');
 */

import { GA_MEASUREMENT_ID } from '../config/build-config';

declare global {
  interface Window {
    dataLayer: any[];
    gtag: (...args: any[]) => void;
  }
}

/**
 * Check if Google Analytics is initialized
 */
export const isGAInitialized = (): boolean => {
  return typeof window !== 'undefined' && typeof window.gtag === 'function';
};

/**
 * Get the GA4 Measurement ID from build config
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
 * @param path - Page path (e.g., '/pricing', '/features', '/#faq')
 * @param title - Optional page title
 * 
 * @example
 * trackPageView('/pricing', 'Pricing - PriceWhisperer');
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
 * Track email capture
 * 
 * @param source - Where the email was captured (e.g., 'hero', 'exit_intent', 'footer')
 */
export const trackEmailCapture = (source: string): void => {
  trackEvent('email_capture', {
    event_category: 'lead_generation',
    event_label: `${source}_email_capture`
  });
};

/**
 * Track CTA click
 * 
 * @param label - CTA identifier (e.g., 'hero_start_trial', 'pricing_start_trial')
 * @param location - Where the CTA is located (e.g., 'hero', 'pricing', 'footer')
 */
export const trackCTAClick = (label: string, location: string): void => {
  trackEvent('cta_click', {
    event_category: 'conversion',
    event_label: label,
    location: location
  });
};

/**
 * Track pricing interaction
 * 
 * @param action - Action type (e.g., 'toggle_billing', 'plan_selected')
 * @param label - Additional label (e.g., 'monthly', 'annual', 'starter')
 * @param value - Optional numeric value (e.g., plan price)
 */
export const trackPricingInteraction = (
  action: string,
  label?: string,
  value?: number
): void => {
  trackEvent('pricing_interaction', {
    event_category: 'engagement',
    event_label: label,
    action: action,
    value: value
  });
};

/**
 * Track FAQ interaction
 * 
 * @param faqId - FAQ item identifier
 * @param action - Action type ('expand', 'collapse')
 */
export const trackFAQInteraction = (faqId: string, action: 'expand' | 'collapse'): void => {
  trackEvent('faq_interaction', {
    event_category: 'engagement',
    event_label: faqId,
    action: action
  });
};

/**
 * Track ROI calculator interaction
 * 
 * @param action - Action type (e.g., 'slider_change', 'calculate', 'cta_click')
 * @param value - Optional value (e.g., calculated ROI percentage)
 */
export const trackROICalculator = (action: string, value?: number): void => {
  trackEvent('roi_calculator', {
    event_category: 'engagement',
    event_label: action,
    value: value
  });
};

