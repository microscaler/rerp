/**
 * Internal Linking Utility
 * 
 * Provides utilities for creating internal links and breadcrumbs
 * Improves SEO through proper internal linking structure
 */

import { BASE_URL } from '../config/constants';
import { EXTERNAL_URLS } from '@shared/config/constants';

export interface BreadcrumbItem {
  name: string;
  url: string;
}

/**
 * Generate breadcrumb structured data (JSON-LD)
 */
export const generateBreadcrumbSchema = (items: BreadcrumbItem[]) => {
  return {
    '@context': 'https://schema.org',
    '@type': 'BreadcrumbList',
    itemListElement: items.map((item, index) => ({
      '@type': 'ListItem',
      position: index + 1,
      name: item.name,
      item: item.url.startsWith('http') ? item.url : `${BASE_URL}${item.url}`
    }))
  };
};

/**
 * Common breadcrumb paths
 */
export const BREADCRUMB_PATHS = {
  home: [{ name: 'Home', url: BASE_URL }],
  features: [
    { name: 'Home', url: BASE_URL },
    { name: 'Features', url: `${BASE_URL}/#features` }
  ],
  feature: (featureName: string, featureSlug: string) => [
    { name: 'Home', url: BASE_URL },
    { name: 'Features', url: `${BASE_URL}/#features` },
    { name: featureName, url: `${BASE_URL}/#feature-${featureSlug}` }
  ],
  blog: [
    { name: 'Home', url: BASE_URL },
    { name: 'Blog', url: `${BASE_URL}/#blogs` }
  ],
  blogPost: (postTitle: string, postSlug: string) => [
    { name: 'Home', url: BASE_URL },
    { name: 'Blog', url: `${BASE_URL}/#blogs` },
    { name: postTitle, url: `${BASE_URL}/#blog-${postSlug}` }
  ],
  faq: [
    { name: 'Home', url: BASE_URL },
    { name: 'FAQ', url: `${BASE_URL}/#faq` }
  ],
  howItWorks: [
    { name: 'Home', url: BASE_URL },
    { name: 'How It Works', url: `${BASE_URL}/#how-it-works-page` }
  ],
  pricing: [
    { name: 'Home', url: BASE_URL },
    { name: 'Pricing', url: `${BASE_URL}/#pricing` }
  ]
};

/**
 * Related pages mapping for internal linking
 */
export const RELATED_PAGES = {
  'smart-trading-alerts': [
    { title: 'Pattern Recognition Guide', url: '#blog-pattern-recognition-guide', description: 'Learn how patterns are detected' },
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Essential risk management for trading alerts' },
    { title: 'Multi-Exchange Trading', url: '#blog-multi-exchange-trading', description: 'Trading across global exchanges' }
  ],
  'options-strategy-finder': [
    { title: 'Risk Management', url: '#feature-built-in-risk-management', description: 'Manage options risk automatically' },
    { title: 'Trading Education', url: '#feature-financial-trading-education', description: 'Learn options strategies' },
    { title: 'Iron Condor Strategy Guide', url: '#blog-iron-condor-strategy', description: 'Complete Iron Condor guide' }
  ],
  'pattern-recognition': [
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get alerts on patterns' },
    { title: 'Pattern Recognition Guide', url: '#blog-pattern-recognition-guide', description: 'Learn pattern detection' },
    { title: 'Trading Dashboard', url: '#feature-live-trading-dashboard', description: 'View patterns in dashboard' }
  ],
  'global-market-coverage': [
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Alerts across all exchanges' },
    { title: 'Multi-Exchange Trading', url: '#blog-multi-exchange-trading', description: 'Trading guide' },
    { title: 'Execution Options', url: '#feature-execution-options', description: 'Trade on multiple exchanges' }
  ],
  'live-trading-dashboard': [
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'View alerts in dashboard' },
    { title: 'Pattern Recognition', url: '#feature-pattern-recognition', description: 'See detected patterns' },
    { title: 'Risk Management', url: '#feature-built-in-risk-management', description: 'Monitor portfolio risk' }
  ],
  'built-in-risk-management': [
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Learn risk management' },
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'Find low-risk strategies' },
    { title: 'Trading Education', url: '#feature-financial-trading-education', description: 'Risk management courses' }
  ],
  'financial-trading-education': [
    { title: 'Trading Education', url: EXTERNAL_URLS.tradingEducation, description: 'Full education platform', external: true },
    { title: 'Options Greeks Explained', url: '#blog-options-greeks-explained', description: 'Learn options Greeks' },
    { title: 'Paper Trading Guide', url: '#blog-paper-trading-guide', description: 'Practice trading safely' }
  ],
  'execution-options': [
    { title: 'Live Trading Dashboard', url: '#feature-live-trading-dashboard', description: 'Execute from dashboard' },
    { title: 'Paper Trading Guide', url: '#blog-paper-trading-guide', description: 'Practice first' },
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get trade signals' }
  ],
  'blogs-and-news': [
    { title: 'Financial Trading Education', url: '#feature-financial-trading-education', description: 'Comprehensive trading education' },
    { title: 'Pattern Recognition Guide', url: '#blog-pattern-recognition-guide', description: 'Learn chart pattern recognition' },
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Essential risk management guide' }
  ],
  // Blog Posts Related Articles
  'iron-condor-strategy': [
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'Automatically find profitable options strategies' },
    { title: 'Options Greeks Explained', url: '#blog-options-greeks-explained', description: 'Understand Delta, Gamma, Theta, and Vega' },
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Learn essential risk management for options' }
  ],
  'pattern-recognition-guide': [
    { title: 'Pattern Recognition Feature', url: '#feature-pattern-recognition', description: 'AI-powered pattern detection system' },
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get alerts when patterns are detected' },
    { title: 'Live Trading Dashboard', url: '#feature-live-trading-dashboard', description: 'View patterns in real-time dashboard' }
  ],
  'risk-management-basics': [
    { title: 'Built-In Risk Management', url: '#feature-built-in-risk-management', description: 'Automated risk management features' },
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'Find low-risk options strategies' },
    { title: 'Paper Trading Guide', url: '#blog-paper-trading-guide', description: 'Practice risk management safely' }
  ],
  'options-greeks-explained': [
    { title: 'Iron Condor Strategy', url: '#blog-iron-condor-strategy', description: 'Apply Greeks to Iron Condor trades' },
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'AI analyzes Greeks automatically' },
    { title: 'Trading Education', url: '#feature-financial-trading-education', description: 'Comprehensive options trading courses' }
  ],
  'paper-trading-guide': [
    { title: 'Execution Options', url: '#feature-execution-options', description: 'Seamless transition from paper to live trading' },
    { title: 'Live Trading Dashboard', url: '#feature-live-trading-dashboard', description: 'Practice with real-time market data' },
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Learn risk management in paper trading' }
  ],
  'multi-exchange-trading': [
    { title: 'Global Market Coverage', url: '#feature-global-market-coverage', description: 'Monitor 25+ exchanges worldwide' },
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get alerts across all exchanges' },
    { title: 'Execution Options', url: '#feature-execution-options', description: 'Trade on multiple exchanges seamlessly' }
  ],
  // Case Studies Related Articles
  'day-trader-increased-win-rate-40': [
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get instant alerts on high-probability setups' },
    { title: 'Pattern Recognition', url: '#feature-pattern-recognition', description: 'AI-powered pattern detection' },
    { title: 'Risk Management Basics', url: '#blog-risk-management-basics', description: 'Learn essential risk management' }
  ],
  'options-trader-finds-50k-opportunities': [
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'Automatically find profitable options strategies' },
    { title: 'Iron Condor Strategy', url: '#blog-iron-condor-strategy', description: 'Complete Iron Condor guide' },
    { title: 'Options Greeks Explained', url: '#blog-options-greeks-explained', description: 'Understand Delta, Gamma, Theta, and Vega' }
  ],
  'swing-trader-saves-15-hours-week': [
    { title: 'Pattern Recognition', url: '#feature-pattern-recognition', description: 'AI-powered pattern detection' },
    { title: 'Global Market Coverage', url: '#feature-global-market-coverage', description: 'Monitor 25+ exchanges worldwide' },
    { title: 'Pattern Recognition Guide', url: '#blog-pattern-recognition-guide', description: 'Learn pattern detection' }
  ],
  'professional-trader-monitors-50k-tickers': [
    { title: 'Global Market Coverage', url: '#feature-global-market-coverage', description: 'Monitor 25+ exchanges worldwide' },
    { title: 'Smart Trading Alerts', url: '#feature-smart-trading-alerts', description: 'Get alerts across all exchanges' },
    { title: 'Multi-Exchange Trading', url: '#blog-multi-exchange-trading', description: 'Trading across global exchanges' }
  ],
  'options-specialist-finds-50k-opportunities': [
    { title: 'Options Strategy Finder', url: '#feature-options-strategy-finder', description: 'Automatically find profitable options strategies' },
    { title: 'Iron Condor Strategy', url: '#blog-iron-condor-strategy', description: 'Complete Iron Condor guide' },
    { title: 'Options Greeks Explained', url: '#blog-options-greeks-explained', description: 'Understand Delta, Gamma, Theta, and Vega' }
  ],
  'retail-trader-becomes-consistently-profitable': [
    { title: 'Financial Trading Education', url: '#feature-financial-trading-education', description: 'Comprehensive trading education' },
    { title: 'Built-In Risk Management', url: '#feature-built-in-risk-management', description: 'Automated risk management features' },
    { title: 'Paper Trading Guide', url: '#blog-paper-trading-guide', description: 'Practice trading safely' }
  ]
};

/**
 * Get related pages for a given page slug
 */
export const getRelatedPages = (slug: string): Array<{ title: string; url: string; description: string; external?: boolean }> => {
  return RELATED_PAGES[slug as keyof typeof RELATED_PAGES] || [];
};

