# Shared Components

Shared components, styles, and configuration for RERP UI applications.

## Structure

```
shared/
├── header/
│   └── Header.tsx          # Main navigation header
├── footer/
│   ├── Footer.tsx          # Main footer component
│   └── components/         # Footer subcomponents
│       ├── FooterBrand.tsx
│       ├── FooterLink.tsx
│       ├── FooterLinkSection.tsx
│       ├── FooterCopyright.tsx
│       └── index.ts
├── config/
│   └── constants.ts        # Shared constants (EXTERNAL_URLS, etc.)
├── styles/
│   └── index.css           # Shared base styles
└── index.ts                # Central export point
```

## Usage

### In Website (`ui/website`)

Import using the `@shared` alias:

```typescript
import Header from '@shared/header/Header';
import Footer from '@shared/footer/Footer';
import { EXTERNAL_URLS } from '@shared/config/constants';
```

## Configuration

The website uses the `@shared` path alias:

- **TypeScript**: `tsconfig.json` includes `@shared/*` path mapping
- **Vite**: `vite.config.ts` includes `@shared` alias resolution

## Adding New Shared Components

1. Create component in appropriate directory under `shared/`
2. Export from `shared/index.ts` if needed
3. Update consuming applications to import using `@shared` alias
4. Test build in the website

