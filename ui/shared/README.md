# Shared Components

Shared components, styles, and configuration used across PriceWhisperer applications.

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

### In FTE Site (`ui/fte`)

Import using the `@shared` alias:

```typescript
import Header from '@shared/header/Header';
import Footer from '@shared/footer/Footer';
import { EXTERNAL_URLS } from '@shared/config/constants';
```

## Configuration

Both `website` and `fte` have path aliases configured:

- **TypeScript**: `tsconfig.json` includes `@shared/*` path mapping
- **Vite**: `vite.config.ts` includes `@shared` alias resolution

## Migration Notes

- ✅ Header component migrated from `ui/website/src/components/Header.tsx`
- ✅ Footer component migrated from `ui/website/src/components/Footer.tsx`
- ✅ Footer subcomponents migrated from `ui/website/src/components/footer/components/`
- ✅ Shared constants extracted from `ui/website/src/config/constants.ts` (EXTERNAL_URLS)
- ✅ Website updated to import from shared (build verified ✅)
- ✅ FTE site configured to import from shared
- ✅ Old duplicate files removed from website directory (migration complete)

## Adding New Shared Components

1. Create component in appropriate directory under `shared/`
2. Export from `shared/index.ts` if needed
3. Update consuming applications to import using `@shared` alias
4. Test build in both website and FTE site

