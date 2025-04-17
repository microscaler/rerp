/**
 * Application Constants
 * 
 * Centralized location for all application-wide constants
 * This ensures single source of truth for configuration values
 */

import { BASE_URL } from './build-config';

// Re-export BASE_URL from build-config for backwards compatibility
export { BASE_URL };

/**
 * Site name
 */
export const SITE_NAME = 'RERP';

/**
 * Site description
 */
export const SITE_DESCRIPTION = 'Enterprise Resource Planning, Reimagined - Modular, Open-Source ERP System';

/**
 * Default Open Graph image
 */
export const DEFAULT_OG_IMAGE = '/og-image.jpg';

/**
 * Default Twitter Card image
 */
export const DEFAULT_TWITTER_IMAGE = '/twitter-image.jpg';

/**
 * Organization name for structured data
 */
export const ORGANIZATION_NAME = 'RERP';

/**
 * Support email
 */
export const SUPPORT_EMAIL = 'support@rerp.dev';

/**
 * Social media URLs
 */
export const SOCIAL_URLS = {
  twitter: 'https://twitter.com/rerp_erp',
  linkedin: 'https://www.linkedin.com/company/rerp',
  github: 'https://github.com/microscaler/rerp'
};

/**
 * External URLs
 */
export const EXTERNAL_URLS = {
  documentation: 'https://github.com/microscaler/rerp#readme',
  github: 'https://github.com/microscaler/rerp',
  discussions: 'https://github.com/microscaler/rerp/discussions',
  issues: 'https://github.com/microscaler/rerp/issues'
};

