# SEO Contributing Guide

This guide ensures all new pages and components follow our established SEO standards and best practices.

## Table of Contents

1. [Quick Checklist](#quick-checklist)
2. [Page Structure Requirements](#page-structure-requirements)
3. [SEO Data Configuration](#seo-data-configuration)
4. [Structured Data](#structured-data)
5. [Images & Alt Text](#images--alt-text)
6. [Heading Structure](#heading-structure)
7. [Internal Linking](#internal-linking)
8. [Examples & Templates](#examples--templates)

---

## Quick Checklist

Before submitting a new page, ensure:

- [ ] Page has unique H1 heading
- [ ] SEO data added to `src/data/seo-data.ts`
- [ ] Structured data (Schema.org) implemented
- [ ] All images have descriptive alt text
- [ ] Heading hierarchy is logical (H1 → H2 → H3)
- [ ] Internal links added to related pages
- [ ] Breadcrumb schema included (if standalone page)
- [ ] Meta tags configured (title, description, keywords)
- [ ] Canonical URL set
- [ ] Open Graph tags configured
- [ ] Page added to sitemap (`src/utils/sitemap.ts`)

---

## Page Structure Requirements

### 1. Component Structure

Every page component should:

```typescript
import { Component, createEffect } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { getPageSEO } from '../data/seo-data';

const NewPage: Component = () => {
  // Update SEO on mount
  createEffect(() => {
    const seoData = getPageSEO('page-slug');
    updateSEO(seoData);
  });

  return (
    <article class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
        <h1 class="text-4xl font-extrabold text-gray-900">Page Title</h1>
        {/* Content */}
      </div>
    </article>
  );
};

export default NewPage;
```

### 2. Standalone vs Embedded Pages

**Standalone Pages** (own route):
- Must have H1 heading
- Must include breadcrumb schema
- Must have full SEO data

**Embedded Pages** (section on homepage):
- Use H2 for section title
- Can accept `standalone` prop to conditionally show/hide H2

Example:
```typescript
interface ComponentProps {
  standalone?: boolean;
}

const Component: Component<ComponentProps> = (props) => {
  return (
    <section>
      {!props.standalone && (
        <h2>Section Title</h2>
      )}
      {/* Content */}
    </section>
  );
};
```

---

## SEO Data Configuration

### 1. Add SEO Data

Add your page's SEO data to `src/data/seo-data.ts`:

```typescript
export const newPageSEO: SEOData = {
  title: 'Page Title - PriceWhisperer',
  description: 'Compelling 150-160 character description with primary keywords.',
  keywords: 'keyword1, keyword2, keyword3, long-tail keyword',
  canonical: `${BASE_URL}/#new-page-slug`,
  ogType: 'website', // or 'article' for blog posts
  ogImage: '/og-image.jpg', // or specific image path
  structuredData: {
    // See Structured Data section
  }
};
```

### 2. Required Fields

- **title**: 50-60 characters, include brand name
- **description**: 150-160 characters, include primary keywords
- **keywords**: 5-10 relevant keywords, comma-separated
- **canonical**: Full URL with hash for SPA routing
- **ogType**: 'website' or 'article'
- **ogImage**: Path to Open Graph image

### 3. Update SEO in Component

```typescript
import { updateSEO } from '../utils/seo';
import { newPageSEO } from '../data/seo-data';

createEffect(() => {
  updateSEO(newPageSEO);
});
```

---

## Structured Data

### 1. Service Schema (Feature Pages)

```typescript
structuredData: {
  '@context': 'https://schema.org',
  '@type': 'Service',
  name: 'Service Name',
  description: 'Detailed service description',
  provider: {
    '@type': 'Organization',
    name: 'PriceWhisperer'
  },
  areaServed: 'Worldwide',
  serviceType: 'Service Type Name'
}
```

### 2. Article Schema (Blog Posts)

```typescript
structuredData: [
  {
    '@context': 'https://schema.org',
    '@type': 'Article',
    headline: 'Article Title',
    description: 'Article description',
    author: {
      '@type': 'Organization',
      name: 'PriceWhisperer'
    },
    publisher: {
      '@type': 'Organization',
      name: 'PriceWhisperer',
      logo: {
        '@type': 'ImageObject',
        url: `${BASE_URL}/logo.png`
      }
    },
    datePublished: '2024-11-15',
    dateModified: '2024-11-15',
    image: `${BASE_URL}/article-image.jpg`
  },
  generateBreadcrumbSchema(BREADCRUMB_PATHS.blogPost('Article Title', 'article-slug'))
]
```

### 3. Breadcrumb Schema (All Standalone Pages)

```typescript
import { generateBreadcrumbSchema, BREADCRUMB_PATHS } from '../utils/internal-linking';

structuredData: [
  // Your main schema (Service, Article, etc.),
  generateBreadcrumbSchema(BREADCRUMB_PATHS.feature('Feature Name', 'feature-slug'))
]
```

### 4. FAQPage Schema

```typescript
structuredData: {
  '@context': 'https://schema.org',
  '@type': 'FAQPage',
  mainEntity: [
    {
      '@type': 'Question',
      name: 'Question text?',
      acceptedAnswer: {
        '@type': 'Answer',
        text: 'Answer text'
      }
    }
  ]
}
```

### 5. HowTo Schema

```typescript
structuredData: {
  '@context': 'https://schema.org',
  '@type': 'HowTo',
  name: 'How To Title',
  description: 'Process description',
  step: [
    {
      '@type': 'HowToStep',
      position: 1,
      name: 'Step Name',
      text: 'Step description'
    }
  ]
}
```

---

## Images & Alt Text

### 1. Image Requirements

**All images MUST have alt text:**

```typescript
// Good
<img 
  src="/image.jpg" 
  alt="Descriptive text explaining what the image shows"
/>

// Bad
<img src="/image.jpg" />
```

### 2. Alt Text Guidelines

- **Descriptive**: Describe what's in the image
- **Concise**: 125 characters or less
- **Relevant**: Include keywords when natural
- **Contextual**: Consider surrounding content

**Examples:**

```typescript
// Good
alt="PriceWhisperer trading dashboard showing real-time stock charts and alerts"

// Good
alt="AI pattern recognition detecting bullish candlestick patterns on stock chart"

// Bad
alt="image"
alt="chart"
alt="screenshot"
```

### 3. Decorative Images

For decorative images (icons, emojis, decorative graphics):

```typescript
// Use aria-label for emojis
<div role="img" aria-label="Chart showing growth">📈</div>

// Or use empty alt for purely decorative images
<img src="/decorative-icon.svg" alt="" />
```

### 4. Icon Fonts

Font Awesome icons don't need alt text (they're decorative):

```typescript
<i class="fa-solid fa-chart-line"></i> // No alt needed
```

---

## Heading Structure

### 1. Heading Hierarchy

**Required hierarchy:**
```
H1 (Page Title - Only ONE per page)
  └─ H2 (Main Sections)
      └─ H3 (Subsections)
          └─ H4 (Sub-subsections)
```

### 2. Rules

- **One H1 per page** - The main page title
- **No skipped levels** - Don't go from H1 to H3
- **Logical order** - H1 → H2 → H3 → H4
- **Semantic meaning** - Use headings for structure, not styling

### 3. Examples

**Homepage:**
```typescript
// Hero section
<h1>Never Miss a Profitable Trade Again</h1>

// Sections
<h2>Everything You Need to Trade Smarter</h2>
<h2>How PriceWhisperer Works</h2>
<h2>Trusted by Thousands of Traders</h2>

// Subsections
<h3>Smart Trading Alerts</h3>
<h3>Pattern Recognition</h3>
```

**Standalone Page:**
```typescript
<h1>Page Title</h1>
  <h2>Main Section</h2>
    <h3>Subsection</h3>
```

**Feature Page:**
```typescript
<h1>Feature Name</h1>
  <h2>How It Works</h2>
    <h3>Key Feature</h3>
  <h2>Benefits</h2>
    <h3>Benefit 1</h3>
```

### 4. Common Mistakes

❌ **Don't:**
- Use multiple H1s on same page
- Skip heading levels (H1 → H3)
- Use headings for styling only
- Use H4+ without H3

✅ **Do:**
- One H1 per page
- Follow logical hierarchy
- Use headings for content structure
- Keep hierarchy consistent

---

## Internal Linking

### 1. Related Pages

Use the internal linking utility to add related pages:

```typescript
import { getRelatedPages } from '../utils/internal-linking';

const relatedPages = getRelatedPages('page-slug');
// Returns array of related pages with title, url, description
```

### 2. Adding Related Pages

Update `src/utils/internal-linking.ts`:

```typescript
export const RELATED_PAGES = {
  'new-page-slug': [
    { 
      title: 'Related Page Title', 
      url: '#related-page', 
      description: 'Why it's related' 
    }
  ]
};
```

### 3. Breadcrumb Paths

Add breadcrumb path if needed:

```typescript
export const BREADCRUMB_PATHS = {
  newPage: [
    { name: 'Home', url: BASE_URL },
    { name: 'New Page', url: `${BASE_URL}/#new-page` }
  ]
};
```

---

## Examples & Templates

### Template: New Feature Page

```typescript
import { Component, createEffect } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { featurePagesSEO } from '../data/seo-data';
import { generateBreadcrumbSchema, BREADCRUMB_PATHS } from '../utils/internal-linking';

const NewFeature: Component = () => {
  const slug = 'new-feature';
  
  createEffect(() => {
    const seoData = featurePagesSEO[slug];
    if (seoData) {
      updateSEO(seoData);
    }
  });

  return (
    <article class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
        <div class="mb-8">
          <h1 class="text-4xl font-extrabold text-gray-900">New Feature Name</h1>
          <p class="text-lg text-gray-600 mt-2">Feature tagline</p>
        </div>

        <div class="prose prose-lg max-w-none">
          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">How It Works</h2>
          <p class="text-gray-700 mb-4">Description...</p>

          <h3 class="text-xl font-semibold text-gray-900 mb-3">Key Feature</h3>
          <p class="text-gray-600">Details...</p>
        </div>
      </div>
    </article>
  );
};

export default NewFeature;
```

### Template: New Blog Post

```typescript
import { Component, createEffect } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { blogPostsSEO } from '../data/seo-data';

const NewBlogPost: Component = () => {
  const slug = 'new-blog-post';
  
  createEffect(() => {
    const seoData = blogPostsSEO[slug];
    if (seoData) {
      updateSEO(seoData);
    }
  });

  return (
    <article class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
        <div class="mb-8">
          <div class="text-sm text-gray-500 mb-2">Category</div>
          <h1 class="text-4xl font-extrabold text-gray-900">Blog Post Title</h1>
          <div class="flex items-center text-gray-500 text-sm mt-2">
            <span>November 15, 2024</span>
            <span class="mx-2">•</span>
            <span>8 min read</span>
          </div>
        </div>

        <div class="prose prose-lg max-w-none">
          <img 
            src="/blog/image.jpg" 
            alt="Descriptive alt text for blog image"
            class="w-full rounded-lg mb-8"
          />
          
          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Main Section</h2>
          <p class="text-gray-700 mb-4">Content...</p>

          <h3 class="text-xl font-semibold text-gray-900 mb-3">Subsection</h3>
          <p class="text-gray-600">Details...</p>
        </div>
      </div>
    </article>
  );
};

export default NewBlogPost;
```

---

## Sitemap Updates

When creating a new page, add it to `src/utils/sitemap.ts`:

```typescript
export const sitemapUrls: SitemapUrl[] = [
  // ... existing URLs
  {
    loc: `${BASE_URL}/#new-page-slug`,
    lastmod: '2024-11-15',
    changefreq: 'monthly',
    priority: 0.8
  }
];
```

Then regenerate sitemap:
```bash
npm run generate-sitemap
```

---

## Constants

Always use constants from `src/config/constants.ts`:

```typescript
import { BASE_URL, SITE_NAME, EXTERNAL_URLS } from '../config/constants';

// Use BASE_URL for canonical URLs
canonical: `${BASE_URL}/#page`

// Use EXTERNAL_URLS for external links
href={EXTERNAL_URLS.tradingEducation}
```

---

## Testing Checklist

Before submitting:

1. **SEO Validation**
   - [ ] Run `npm run generate-sitemap` and verify page is included
   - [ ] Check meta tags in browser DevTools
   - [ ] Validate structured data with [Google Rich Results Test](https://search.google.com/test/rich-results)
   - [ ] Verify canonical URL is correct

2. **Accessibility**
   - [ ] All images have alt text
   - [ ] Heading hierarchy is logical
   - [ ] Test with screen reader (optional but recommended)

3. **Code Quality**
   - [ ] No TypeScript errors
   - [ ] No linting errors
   - [ ] Follows existing code patterns
   - [ ] Uses constants from `config/constants.ts`

4. **Content**
   - [ ] Title is unique and descriptive
   - [ ] Description is 150-160 characters
   - [ ] Keywords are relevant
   - [ ] Content is well-structured

---

## Resources

- [Schema.org Documentation](https://schema.org/)
- [Google Search Central](https://developers.google.com/search)
- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Google Rich Results Test](https://search.google.com/test/rich-results)
- [PageSpeed Insights](https://pagespeed.web.dev/)

---

## Questions?

If you're unsure about SEO requirements:
1. Check existing pages for examples
2. Review `src/data/seo-data.ts` for patterns
3. Consult the main SEO strategy document: `docs/SEO_STRATEGY.md`

---

**Last Updated:** November 2024  
**Maintained By:** Development Team

