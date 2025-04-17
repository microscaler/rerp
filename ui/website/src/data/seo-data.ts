/**
 * SEO Data Definitions
 * 
 * Centralized SEO metadata for all pages
 */

import { SEOData } from '../utils/seo';
import { BASE_URL } from '../config/constants';
import { generateBreadcrumbSchema, BREADCRUMB_PATHS } from '../utils/internal-linking';

// Homepage SEO
export const homepageSEO: SEOData = {
  title: "RERP - Enterprise Resource Planning, Reimagined | Open-Source ERP",
  description: "RERP is a comprehensive, modular ERP system with 71 integrated services. Cloud-native, open-source, and designed for businesses of all sizes. Deploy only what you need.",
  keywords: "ERP, enterprise resource planning, open source ERP, modular ERP, cloud-native ERP, business management software, ERP system, open source business software",
  canonical: BASE_URL,
  ogType: 'website',
  ogImage: '/og-image.jpg',
  structuredData: {
    '@context': 'https://schema.org',
    '@type': 'SoftwareApplication',
    name: 'RERP',
    applicationCategory: 'BusinessApplication',
    operatingSystem: 'Web',
    offers: {
      '@type': 'Offer',
      price: '0',
      priceCurrency: 'USD',
      availability: 'https://schema.org/InStock'
    },
    description: "RERP is a comprehensive, modular ERP system with 71 integrated services. Cloud-native, open-source, and designed for businesses of all sizes. Deploy only what you need.",
    featureList: [
      '71 independent services',
      '6 implementation phases',
      'Cloud-native architecture',
      'Open-source license',
      'Modular design',
      'API-first approach',
      'Self-hostable',
      'Enterprise-ready'
    ]
  }
};

// Modules Page SEO
export const modulesPageSEO: SEOData = {
  title: "RERP Modules - 71 Services Across 6 Implementation Phases",
  description: "Explore RERP's 71 services organized into 6 implementation phases. From core foundation to advanced operations, build the perfect ERP for your business.",
  keywords: "ERP modules, ERP services, business modules, ERP phases, enterprise modules, modular business software",
  canonical: `${BASE_URL}/#modules`,
  ogType: 'website',
  ogImage: '/og-image.jpg'
};

// How It Works Page SEO
export const howItWorksSEO: SEOData = {
  title: "How RERP Works - Modular ERP Architecture Explained",
  description: "Learn how RERP's modular architecture works. Independent services, cloud-native deployment, API-first design, and flexible scaling options.",
  keywords: "how ERP works, modular architecture, cloud-native ERP, API-first ERP, ERP deployment, microservices ERP",
  canonical: `${BASE_URL}/#how-it-works-page`,
  ogType: 'website',
  ogImage: '/og-image.jpg',
  structuredData: [
    {
      '@context': 'https://schema.org',
      '@type': 'HowTo',
      name: 'How RERP Works',
      description: 'Guide to RERP modular architecture and deployment',
      step: [
        {
          '@type': 'HowToStep',
          position: 1,
          name: 'Choose Your Modules',
          text: 'Select from 71 services across 6 implementation phases'
        },
        {
          '@type': 'HowToStep',
          position: 2,
          name: 'Deploy & Configure',
          text: 'Cloud-native deployment with independent scaling'
        },
        {
          '@type': 'HowToStep',
          position: 3,
          name: 'Scale & Extend',
          text: 'Add modules as your business grows'
        }
      ]
    },
    generateBreadcrumbSchema(BREADCRUMB_PATHS.howItWorks)
  ]
};

// FAQ Page SEO
export const faqPageSEO: SEOData = {
  title: "RERP FAQ - Frequently Asked Questions About Open-Source ERP",
  description: "Get answers to common questions about RERP, modules, deployment, licensing, support, and contribution. Everything you need to know about the open-source ERP system.",
  keywords: "RERP FAQ, ERP questions, open source ERP FAQ, ERP help, ERP support, ERP documentation",
  canonical: `${BASE_URL}/#faq`,
  ogType: 'website',
  ogImage: '/og-image.jpg',
  structuredData: [
    {
      '@context': 'https://schema.org',
      '@type': 'FAQPage',
      mainEntity: []
    },
    generateBreadcrumbSchema(BREADCRUMB_PATHS.faq)
  ]
};

// Blog Listing Page SEO (placeholder)
export const blogsNewsSEO: SEOData = {
  title: 'RERP Blog - News & Updates',
  description: 'Read the latest news, updates, and insights about RERP open-source ERP system.',
  keywords: 'RERP blog, ERP news, open source ERP updates, ERP insights',
  canonical: `${BASE_URL}/#blogs`,
  ogType: 'website',
  ogImage: '/og-image.jpg'
};
