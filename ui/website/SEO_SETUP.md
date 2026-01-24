# SEO Setup & Maintenance Guide

This guide explains the SEO implementation for RERP and how to maintain it.

## Overview

RERP uses a **hybrid SEO approach**:
- **Base SEO**: Site-wide meta tags in `index.html`
- **Dynamic SEO**: Per-page meta tags updated via JavaScript for SPA routing

## Files Structure

```
website/
├── public/
│   ├── robots.txt          # Search engine crawling directives
│   └── sitemap.xml         # Generated sitemap (auto-generated)
├── src/
│   ├── utils/
│   │   ├── seo.ts          # SEO utility functions
│   │   └── sitemap.ts      # Sitemap generation utilities
│   └── data/
│       └── seo-data.ts     # SEO metadata for all pages
└── scripts/
    └── generate-sitemap.js # Sitemap generation script
```

## Components

### 1. SEO Utility (`src/utils/seo.ts`)

Main function: `updateSEO(data: SEOData)`

Updates:
- Document title
- Meta description
- Meta keywords
- Open Graph tags
- Twitter Card tags
- Canonical URL
- Structured data (JSON-LD)
- Robots meta tag

**Usage:**
```typescript
import { updateSEO } from './utils/seo';

updateSEO({
  title: 'Page Title - RERP',
  description: 'Page description...',
  keywords: 'keyword1, keyword2',
  canonical: 'https://rerp.microscaler.io/#page',
  ogImage: '/og-image.jpg',
  structuredData: { /* JSON-LD schema */ }
});
```

### 2. SEO Data (`src/data/seo-data.ts`)

Centralized SEO metadata for all pages:
- `homepageSEO` - Homepage
- `faqPageSEO` - FAQ page
- `howItWorksSEO` - How It Works page
- `blogsNewsSEO` - Blog listing page
- `featurePagesSEO` - All 9 feature pages
- `blogPostsSEO` - All blog posts

**Adding a new page:**
1. Add SEO data to `seo-data.ts`
2. Import and use in the component
3. Update sitemap if needed

### 3. Robots.txt (`public/robots.txt`)

Controls search engine crawling:
- Allows all pages
- References sitemap location
- Can disallow specific paths if needed

**Location:** `https://rerp.microscaler.io/robots.txt`

### 4. Sitemap (`public/sitemap.xml`)

XML sitemap listing all pages with:
- URL location
- Last modification date
- Change frequency
- Priority

**Generation:**
```bash
npm run generate-sitemap
```

**Auto-generated during build:**
The sitemap is automatically generated when running `npm run build`

**Manual update:**
1. Edit `scripts/generate-sitemap.js`
2. Add/remove URLs as needed
3. Run `npm run generate-sitemap`

## Adding New Pages

### Step 1: Add SEO Data

Edit `src/data/seo-data.ts`:

```typescript
export const newPageSEO: SEOData = {
  title: 'New Page Title - RERP',
  description: 'Page description for search engines...',
  keywords: 'relevant, keywords, here',
  canonical: `${BASE_URL}/#new-page`,
  ogType: 'website',
  ogImage: '/og-image.jpg',
  structuredData: {
    '@context': 'https://schema.org',
    '@type': 'WebPage',
    // ... schema data
  }
};
```

### Step 2: Integrate in Component

```typescript
import { updateSEO } from '../utils/seo';
import { newPageSEO } from '../data/seo-data';
import { onMount } from 'solid-js';

const NewPage: Component = () => {
  onMount(() => {
    updateSEO(newPageSEO);
  });
  
  return <div>...</div>;
};
```

### Step 3: Update Sitemap

Edit `scripts/generate-sitemap.js`:

```javascript
{
  loc: `${BASE_URL}/#new-page`,
  lastmod: getCurrentDate(),
  changefreq: 'monthly',
  priority: 0.7
}
```

Then run:
```bash
npm run generate-sitemap
```

## Structured Data Schemas

### Homepage
- **SoftwareApplication** - Product information
- **Organization** - Company information

### Blog Posts
- **Article** - Blog post metadata (author, date, category)

### Feature Pages
- **Service** - Feature descriptions (can be added)

### FAQ Page
- **FAQPage** - Question/Answer pairs

### How It Works Page
- **HowTo** - Step-by-step process

## Testing SEO

### 1. Google Search Console
1. Submit sitemap: `https://rerp.microscaler.io/sitemap.xml`
2. Monitor indexing status
3. Check for errors

### 2. Rich Results Test
- URL: https://search.google.com/test/rich-results
- Test structured data validity
- Check for rich snippet eligibility

### 3. Social Media Validators
- **Facebook**: https://developers.facebook.com/tools/debug/
- **Twitter**: https://cards-dev.twitter.com/validator
- **LinkedIn**: https://www.linkedin.com/post-inspector/

### 4. PageSpeed Insights
- URL: https://pagespeed.web.dev/
- Check Core Web Vitals
- Verify mobile optimization

## Maintenance

### Regular Tasks

1. **Update sitemap** when adding new pages
2. **Review meta descriptions** quarterly
3. **Check Google Search Console** monthly
4. **Update structured data** when content changes
5. **Monitor rankings** for target keywords

### When to Update

- **New pages added** → Update sitemap
- **Content changes** → Update descriptions
- **New blog posts** → Add Article schema
- **Feature updates** → Update feature page SEO
- **URL changes** → Update canonical URLs

## Best Practices

1. **Unique Titles**: Each page should have a unique, descriptive title
2. **Descriptive Meta**: Write compelling descriptions (150-160 characters)
3. **Keywords**: Use relevant keywords naturally
4. **Structured Data**: Always include appropriate schema
5. **Canonical URLs**: Set canonical to prevent duplicate content
6. **Images**: Use descriptive alt text and proper OG images
7. **Internal Linking**: Link between related pages
8. **Mobile-First**: Ensure mobile optimization

## Troubleshooting

### Meta tags not updating
- Check browser console for errors
- Verify `updateSEO()` is called
- Check that component is mounted

### Sitemap not found
- Verify `public/sitemap.xml` exists
- Check robots.txt references sitemap
- Ensure sitemap is accessible at `/sitemap.xml`

### Structured data errors
- Validate with Rich Results Test
- Check JSON-LD syntax
- Verify schema.org types are correct

### Pages not indexing
- Submit sitemap to Google Search Console
- Check robots.txt allows crawling
- Verify canonical URLs are correct
- Ensure pages are accessible

## Resources

- [Google Search Central](https://developers.google.com/search)
- [Schema.org Documentation](https://schema.org/)
- [Sitemap Protocol](https://www.sitemaps.org/protocol.html)
- [Open Graph Protocol](https://ogp.me/)
- [Twitter Cards](https://developer.twitter.com/en/docs/twitter-for-websites/cards)

## Support

For SEO questions or issues:
1. Check this documentation
2. Review `docs/SEO_STRATEGY.md` for strategy
3. Check Google Search Console for errors
4. Validate structured data with Rich Results Test

