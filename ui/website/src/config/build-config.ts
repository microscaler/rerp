/**
 * Build configuration. Values from VITE_* env vars.
 */

export const GA_MEASUREMENT_ID = import.meta.env.VITE_GA_MEASUREMENT_ID || '';
export const BASE_URL = import.meta.env.VITE_BASE_URL || 'https://github.com/microscaler/rerp';
