# Constants Consolidation Summary

## Overview

All application constants have been consolidated into a single location to ensure a single source of truth and easier maintenance.

## Centralized Constants Location

**Primary Location:** `website/src/config/constants.ts`

This file contains all application-wide constants that are used across the codebase.

## Constants Defined

### BASE_URL
- **Location:** `src/config/constants.ts`
- **Value:** `https://pricewhisperer.microscaler.io`
- **Override:** Can be set via `VITE_BASE_URL` environment variable
- **Used in:**
  - SEO utilities (`src/utils/seo.ts`)
  - SEO data (`src/data/seo-data.ts`)
  - Sitemap utilities (`src/utils/sitemap.ts`)
  - Sitemap generation script (`scripts/generate-sitemap.js` - reads from package.json)

### SITE_NAME
- **Location:** `src/config/constants.ts`
- **Value:** `'PriceWhisperer'`
- **Usage:** Site name for structured data and meta tags

### SITE_DESCRIPTION
- **Location:** `src/config/constants.ts`
- **Value:** `'AI-Powered Stock Trading Alerts & Options Analytics'`
- **Usage:** Default site description

### DEFAULT_OG_IMAGE
- **Location:** `src/config/constants.ts`
- **Value:** `'/og-image.jpg'`
- **Usage:** Default Open Graph image path

### DEFAULT_TWITTER_IMAGE
- **Location:** `src/config/constants.ts`
- **Value:** `'/twitter-image.jpg'`
- **Usage:** Default Twitter Card image path

### ORGANIZATION_NAME
- **Location:** `src/config/constants.ts`
- **Value:** `'PriceWhisperer'`
- **Usage:** Organization name for structured data

### SUPPORT_EMAIL
- **Location:** `src/config/constants.ts`
- **Value:** `'support@pricewhisperer.microscaler.io'`
- **Usage:** Support contact email

### SOCIAL_URLS
- **Location:** `src/config/constants.ts`
- **Value:** Object with Twitter, LinkedIn, GitHub URLs
- **Usage:** Social media links

### EXTERNAL_URLS
- **Location:** `src/config/constants.ts`
- **Value:** Object with external URLs (e.g., trading education)
- **Usage:** External links used in components

## Package.json Configuration

**Location:** `website/package.json`

The `config` section contains constants for Node.js scripts:

```json
{
  "config": {
    "baseUrl": "https://pricewhisperer.microscaler.io",
    "siteName": "PriceWhisperer",
    "siteDescription": "AI-Powered Stock Trading Alerts & Options Analytics"
  }
}
```

**Used by:**
- `scripts/generate-sitemap.js` - Reads from `packageJson.config.baseUrl`

## Files Updated

### TypeScript/SolidJS Files
- ✅ `src/utils/seo.ts` - Now imports `BASE_URL` from constants
- ✅ `src/data/seo-data.ts` - Now imports `BASE_URL` from constants
- ✅ `src/utils/sitemap.ts` - Now imports `BASE_URL` from constants
- ✅ `src/components/Footer.tsx` - Now imports `EXTERNAL_URLS` from constants
- ✅ `src/components/About.tsx` - Now imports `EXTERNAL_URLS` from constants
- ✅ `src/components/features/FinancialTradingEducation.tsx` - Now imports `EXTERNAL_URLS` from constants

### Node.js Scripts
- ✅ `scripts/generate-sitemap.js` - Now reads `BASE_URL` from `package.json` config

## Usage Examples

### Importing Constants

```typescript
// Import single constant
import { BASE_URL } from '../config/constants';

// Import multiple constants
import { BASE_URL, SITE_NAME, EXTERNAL_URLS } from '../config/constants';

// Use in code
const canonicalUrl = `${BASE_URL}/#page`;
const educationLink = EXTERNAL_URLS.tradingEducation;
```

### Environment Variable Override

Set `VITE_BASE_URL` in `.env` file to override BASE_URL:

```bash
VITE_BASE_URL=https://staging.pricewhisperer.microscaler.io
```

## Benefits

1. **Single Source of Truth** - All constants in one location
2. **Easy Updates** - Change once, affects entire application
3. **Environment Support** - Can override via environment variables
4. **Type Safety** - TypeScript ensures correct usage
5. **Maintainability** - Easier to find and update constants

## Adding New Constants

1. Add to `src/config/constants.ts`:
```typescript
export const NEW_CONSTANT = 'value';
```

2. Import where needed:
```typescript
import { NEW_CONSTANT } from '../config/constants';
```

3. For Node.js scripts, also add to `package.json` config if needed.

## Verification

✅ All BASE_URL definitions consolidated
✅ All external URLs consolidated
✅ All files updated to import from constants
✅ Sitemap generation script works correctly
✅ No duplicate constant definitions

