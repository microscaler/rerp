/**
 * Generate sitemap.xml for PriceWhisperer
 * 
 * Run this script during build: node scripts/generate-sitemap.js
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Read constants from package.json config
const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'));
const BASE_URL = process.env.VITE_BASE_URL || packageJson.config?.baseUrl || 'https://pricewhisperer.microscaler.io';

const getCurrentDate = () => {
  return new Date().toISOString().split('T')[0];
};

const urls = [
  // Homepage
  {
    loc: BASE_URL,
    lastmod: getCurrentDate(),
    changefreq: 'daily',
    priority: 1.0
  },
  
  // Main pages
  {
    loc: `${BASE_URL}/#features`,
    lastmod: getCurrentDate(),
    changefreq: 'weekly',
    priority: 0.9
  },
  {
    loc: `${BASE_URL}/#pricing`,
    lastmod: getCurrentDate(),
    changefreq: 'weekly',
    priority: 0.9
  },
  {
    loc: `${BASE_URL}/#faq`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.8
  },
  {
    loc: `${BASE_URL}/#how-it-works-page`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.8
  },
  {
    loc: `${BASE_URL}/#blogs`,
    lastmod: getCurrentDate(),
    changefreq: 'daily',
    priority: 0.8
  },
  
  // Feature pages
  {
    loc: `${BASE_URL}/#feature-smart-trading-alerts`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-options-strategy-finder`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-pattern-recognition`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-global-market-coverage`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-live-trading-dashboard`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-built-in-risk-management`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-financial-trading-education`,
    lastmod: getCurrentDate(),
    changefreq: 'monthly',
    priority: 0.7
  },
  {
    loc: `${BASE_URL}/#feature-execution-options`,
    lastmod: getCurrentDate(),
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
      lastmod: getCurrentDate(),
      changefreq: 'weekly',
      priority: 0.8
    },
    {
      loc: `${BASE_URL}/#case-study-day-trader-increased-win-rate-40`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-options-trader-finds-50k-opportunities`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-swing-trader-saves-15-hours-week`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-professional-trader-monitors-50k-tickers`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-options-specialist-finds-50k-opportunities`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    },
    {
      loc: `${BASE_URL}/#case-study-retail-trader-becomes-consistently-profitable`,
      lastmod: getCurrentDate(),
      changefreq: 'monthly',
      priority: 0.7
    }
  ];

const generateSitemap = (urls) => {
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

// Generate sitemap
const sitemapXML = generateSitemap(urls);

// Write to public directory (will be copied to dist during build)
const publicDir = path.join(__dirname, '..', 'public');
if (!fs.existsSync(publicDir)) {
  fs.mkdirSync(publicDir, { recursive: true });
}

const sitemapPath = path.join(publicDir, 'sitemap.xml');
fs.writeFileSync(sitemapPath, sitemapXML, 'utf8');

console.log(`✅ Sitemap generated: ${sitemapPath}`);
console.log(`   Total URLs: ${urls.length}`);

