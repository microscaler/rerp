/**
 * Sitemap Generation Utility
 * 
 * Generates XML sitemap for all pages in the PriceWhisperer website
 * Works with hash-based routing (SPA)
 */

import { BASE_URL } from '../config/constants';

export interface SitemapUrl {
  loc: string;
  lastmod?: string;
  changefreq?: 'always' | 'hourly' | 'daily' | 'weekly' | 'monthly' | 'yearly' | 'never';
  priority?: number;
}

/**
 * Get current date in ISO format (YYYY-MM-DD)
 */
const getCurrentDate = (): string => {
  return new Date().toISOString().split('T')[0];
};

/**
 * Generate sitemap XML
 */
export const generateSitemap = (urls: SitemapUrl[]): string => {
  const urlEntries = urls.map(url => {
    let entry = `  <url>\n    <loc>${url.loc}</loc>`;
    
    if (url.lastmod) {
      entry += `\n    <lastmod>${url.lastmod}</lastmod>`;
    }
    
    if (url.changefreq) {
      entry += `\n    <changefreq>${url.changefreq}</changefreq>`;
    }
    
    if (url.priority !== undefined) {
      entry += `\n    <priority>${url.priority}</priority>`;
    }
    
    entry += '\n  </url>';
    return entry;
  }).join('\n');

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
        xsi:schemaLocation="http://www.sitemaps.org/schemas/sitemap/0.9
        http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd">
${urlEntries}
</urlset>`;
};

/**
 * Get all sitemap URLs for PriceWhisperer
 */
export const getAllSitemapUrls = (): SitemapUrl[] => {
  const currentDate = getCurrentDate();
  
  return [
    // Homepage
    {
      loc: BASE_URL,
      lastmod: currentDate,
      changefreq: 'daily',
      priority: 1.0
    },
    
    // Main pages
    {
      loc: `${BASE_URL}/#features`,
      lastmod: currentDate,
      changefreq: 'weekly',
      priority: 0.9
    },
    {
      loc: `${BASE_URL}/#pricing`,
      lastmod: currentDate,
      changefreq: 'weekly',
      priority: 0.9
    },
    {
      loc: `${BASE_URL}/#faq`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.8
    },
    {
      loc: `${BASE_URL}/#how-it-works-page`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.8
    },
    {
      loc: `${BASE_URL}/#blogs`,
      lastmod: currentDate,
      changefreq: 'daily',
      priority: 0.8
    },
    
    // Feature pages
    {
      loc: `${BASE_URL}/#feature-smart-trading-alerts`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-options-strategy-finder`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-pattern-recognition`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-global-market-coverage`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-live-trading-dashboard`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-built-in-risk-management`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-financial-trading-education`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#feature-execution-options`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    
    // Blog posts
    {
      loc: `${BASE_URL}/#blog-iron-condor-strategy`,
      lastmod: '2024-11-15',
      changefreq: 'monthly',
      priority: 0.6
    },
    {
      loc: `${BASE_URL}/#blog-pattern-recognition-guide`,
      lastmod: '2024-11-10',
      changefreq: 'monthly',
      priority: 0.6
    },
    {
      loc: `${BASE_URL}/#blog-risk-management-basics`,
      lastmod: '2024-11-05',
      changefreq: 'monthly',
      priority: 0.6
    },
    {
      loc: `${BASE_URL}/#blog-options-greeks-explained`,
      lastmod: '2024-10-28',
      changefreq: 'monthly',
      priority: 0.6
    },
    {
      loc: `${BASE_URL}/#blog-paper-trading-guide`,
      lastmod: '2024-10-20',
      changefreq: 'monthly',
      priority: 0.6
    },
    {
      loc: `${BASE_URL}/#blog-multi-exchange-trading`,
      lastmod: '2024-10-15',
      changefreq: 'monthly',
      priority: 0.6
    },
    
    // Category pages
    {
      loc: `${BASE_URL}/#category-options-trading`,
      lastmod: '2024-11-15',
      changefreq: 'weekly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#category-technical-analysis`,
      lastmod: '2024-11-12',
      changefreq: 'weekly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#category-risk-management`,
      lastmod: '2024-11-10',
      changefreq: 'weekly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#category-trading-basics`,
      lastmod: '2024-11-05',
      changefreq: 'weekly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#category-market-analysis`,
      lastmod: '2024-11-03',
      changefreq: 'weekly',
      priority: 0.7
    },
    
    // Case Studies pages
    {
      loc: `${BASE_URL}/#case-studies`,
      lastmod: currentDate,
      changefreq: 'weekly',
      priority: 0.8
    },
    {
      loc: `${BASE_URL}/#case-study-day-trader-increased-win-rate-40`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-options-trader-finds-50k-opportunities`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-swing-trader-saves-15-hours-week`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-professional-trader-monitors-50k-tickers`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-options-specialist-finds-50k-opportunities`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-retail-trader-becomes-consistently-profitable`,
      lastmod: currentDate,
      changefreq: 'monthly',
      priority: 0.7
    }
  ];
};

/**
 * Generate and return sitemap XML string
 */
export const getSitemapXML = (): string => {
  const urls = getAllSitemapUrls();
  return generateSitemap(urls);
};

