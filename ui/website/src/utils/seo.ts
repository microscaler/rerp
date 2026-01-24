/**
 * SEO Utility Functions
 * 
 * Dynamically updates meta tags, titles, and structured data for each page
 * Works with SolidJS SPA architecture and hash-based routing
 */

import { BASE_URL, SITE_NAME, SITE_DESCRIPTION } from '../config/constants';

export interface SEOData {
  title: string;
  description: string;
  keywords?: string;
  ogImage?: string;
  ogType?: string;
  canonical?: string;
  structuredData?: object | object[];
  noindex?: boolean;
}

/**
 * Update or create a meta tag
 */
const updateMetaTag = (name: string, content: string, isProperty = false): void => {
  if (typeof document === 'undefined') return;

  const selector = isProperty 
    ? `meta[property="${name}"]` 
    : `meta[name="${name}"]`;
  
  let meta = document.querySelector(selector) as HTMLMetaElement;
  
  if (!meta) {
    meta = document.createElement('meta');
    if (isProperty) {
      meta.setAttribute('property', name);
    } else {
      meta.setAttribute('name', name);
    }
    document.head.appendChild(meta);
  }
  
  meta.setAttribute('content', content);
};

/**
 * Update or create Open Graph tag
 */
const updateOGTag = (property: string, content: string): void => {
  updateMetaTag(property, content, true);
};

/**
 * Update or create Twitter Card tag
 */
const updateTwitterTag = (name: string, content: string): void => {
  updateMetaTag(`twitter:${name}`, content);
};

/**
 * Update canonical URL
 */
const updateCanonical = (url: string): void => {
  if (typeof document === 'undefined') return;

  let canonical = document.querySelector('link[rel="canonical"]') as HTMLLinkElement;
  
  if (!canonical) {
    canonical = document.createElement('link');
    canonical.setAttribute('rel', 'canonical');
    document.head.appendChild(canonical);
  }
  
  canonical.setAttribute('href', url);
};

/**
 * Remove structured data script tags
 */
const removeStructuredData = (): void => {
  if (typeof document === 'undefined') return;

  const scripts = document.querySelectorAll('script[type="application/ld+json"][data-seo-dynamic]');
  scripts.forEach(script => script.remove());
};

/**
 * Add structured data (JSON-LD)
 */
const addStructuredData = (data: object | object[]): void => {
  if (typeof document === 'undefined') return;

  removeStructuredData();

  const dataArray = Array.isArray(data) ? data : [data];
  
  dataArray.forEach((item, index) => {
    const script = document.createElement('script');
    script.type = 'application/ld+json';
    script.setAttribute('data-seo-dynamic', 'true');
    script.textContent = JSON.stringify(item, null, 2);
    document.head.appendChild(script);
  });
};

/**
 * Update robots meta tag
 */
const updateRobots = (noindex: boolean): void => {
  if (typeof document === 'undefined') return;

  const content = noindex ? 'noindex, nofollow' : 'index, follow';
  updateMetaTag('robots', content);
};

/**
 * Main function to update SEO for a page
 */
export const updateSEO = (data: SEOData): void => {
  if (typeof document === 'undefined') return;

  // Update document title
  document.title = data.title;

  // Update meta description
  updateMetaTag('description', data.description);

  // Update keywords if provided
  if (data.keywords) {
    updateMetaTag('keywords', Array.isArray(data.keywords) ? data.keywords.join(', ') : data.keywords);
  }

  // Update Open Graph tags
  updateOGTag('og:title', data.title);
  updateOGTag('og:description', data.description);
  updateOGTag('og:type', data.ogType || 'website');
  updateOGTag('og:url', data.canonical || `${BASE_URL}${window.location.pathname}${window.location.hash}`);
  
  if (data.ogImage) {
    updateOGTag('og:image', data.ogImage.startsWith('http') ? data.ogImage : `${BASE_URL}${data.ogImage}`);
  }

  // Update Twitter Card tags
  updateTwitterTag('title', data.title);
  updateTwitterTag('description', data.description);
  if (data.ogImage) {
    const imageUrl = data.ogImage.startsWith('http') ? data.ogImage : `${BASE_URL}${data.ogImage}`;
    updateTwitterTag('image', imageUrl);
  }

  // Update canonical URL
  if (data.canonical) {
    const canonicalUrl = data.canonical.startsWith('http') ? data.canonical : `${BASE_URL}${data.canonical}`;
    updateCanonical(canonicalUrl);
  } else {
    const canonicalUrl = `${BASE_URL}${window.location.pathname}${window.location.hash || ''}`;
    updateCanonical(canonicalUrl);
  }

  // Update robots meta tag
  if (data.noindex !== undefined) {
    updateRobots(data.noindex);
  }

  // Add structured data
  if (data.structuredData) {
    addStructuredData(data.structuredData);
  }
};

/**
 * Reset SEO to default homepage values
 */
export const resetSEO = (): void => {
  updateSEO({
    title: `${SITE_NAME} - ${SITE_DESCRIPTION}`,
    description: SITE_DESCRIPTION,
    keywords: 'ERP, enterprise resource planning, open source, modular, cloud-native',
    canonical: BASE_URL,
    ogType: 'website',
  });
};

