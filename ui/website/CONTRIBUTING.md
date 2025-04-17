# Contributing to PriceWhisperer Website

This document provides guidelines for contributing to the PriceWhisperer website project.

## Table of Contents

- [Build Configuration](#build-configuration)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)

## Build Configuration

### Overview

The website uses a centralized build configuration system located at `src/config/build-config.ts`. This file manages all **build-time** configuration values that are injected via environment variables during the build process.

**Important Distinction:**
- **Build-time secrets** → `build-config.ts` (injected during `yarn build`)
- **Runtime secrets** → Supabase Edge Function secrets (set in Supabase Dashboard, accessed at runtime)

### Why Build Config?

- **Security**: Secrets are injected at build time, not exposed in source code
- **Centralization**: Single source of truth for all configuration values
- **Type Safety**: TypeScript ensures correct usage across the codebase
- **Environment Awareness**: Different values for development vs production

### Configuration Values

All configuration values are exported from `src/config/build-config.ts`:

```typescript
import { 
  RECAPTCHA_SITE_KEY,
  GA_MEASUREMENT_ID,
  BASE_URL,
  SUPABASE_URL,
  SUPABASE_ANON_KEY
} from './config/build-config';
```

#### Available Configurations

| Configuration | Environment Variable | Description | Required |
|--------------|---------------------|-------------|----------|
| `RECAPTCHA_SITE_KEY` | `VITE_RECAPTCHA_SITE_KEY` | reCAPTCHA v3 site key for client-side verification | Yes |
| `GA_MEASUREMENT_ID` | `VITE_GA_MEASUREMENT_ID` | Google Analytics 4 measurement ID | No |
| `BASE_URL` | `VITE_BASE_URL` | Base URL for the application (defaults to `https://pricewhisperer.ai`) | No |

### Using Build Config

**✅ DO:**
```typescript
import { RECAPTCHA_SITE_KEY } from '../config/build-config';

const siteKey = RECAPTCHA_SITE_KEY;
```

**❌ DON'T:**
```typescript
// Don't access environment variables directly
const siteKey = import.meta.env.VITE_RECAPTCHA_SITE_KEY;
```

### Local Development

For local development, you can:

1. **Use fallback values**: The build config includes fallback values for development
2. **Create a `.env` file**: Create a `.env` file in `ui/website/` with:
   ```env
   VITE_RECAPTCHA_SITE_KEY=your-test-key
   VITE_GA_MEASUREMENT_ID=your-ga-id
   VITE_SUPABASE_URL=https://your-project-ref.supabase.co
   VITE_SUPABASE_ANON_KEY=your-anon-key
   VITE_BASE_URL=http://localhost:3000
   ```
3. **Check console warnings**: In development mode, the build config will warn you if required values are missing

### Production Builds

For production builds (GitHub Actions), secrets are injected via GitHub Actions secrets:

```yaml
env:
   VITE_RECAPTCHA_SITE_KEY: ${{ secrets.RECAPTCHA_SITE_KEY }}
   VITE_GA_MEASUREMENT_ID: ${{ secrets.GA_MEASUREMENT_ID }}
   VITE_SUPABASE_URL: ${{ secrets.SUPABASE_WEBSITE_PROJECT_URL }}
   VITE_SUPABASE_ANON_KEY: ${{ secrets.SUPABASE_WEBSITE_ANON_API_KEY }}
```

**Important**: 
- GitHub Pages serves static files and cannot access secrets at runtime
- Secrets MUST be injected at build time (which happens in GitHub Actions)
- The built JavaScript files contain the actual secret values (baked in during build)

### Adding New Configuration

#### Build-Time Configuration (Client-Side)

To add a new **build-time** configuration value (exposed to client):

1. **Add to `build-config.ts`**:
   ```typescript
   export const NEW_CONFIG = import.meta.env.VITE_NEW_CONFIG || 'default-value';
   ```

2. **Update this documentation** with the new configuration

3. **Add to GitHub Actions workflow** (if needed):
   ```yaml
   env:
     VITE_NEW_CONFIG: ${{ secrets.NEW_CONFIG }}
   ```

4. **Use in code**:
   ```typescript
   import { NEW_CONFIG } from './config/build-config';
   ```

#### Runtime Configuration (Server-Side Edge Functions)

For **server-side secrets** used by Supabase Edge Functions:

1. **Set in Supabase Dashboard**:
   - Go to Supabase Dashboard > Edge Functions > Secrets
   - Add the secret (e.g., `RESEND_API_KEY`)

2. **Or via Supabase CLI**:
   ```bash
   supabase secrets set SECRET_NAME=secret-value
   ```

3. **Access in Edge Function**:
   ```typescript
   const secret = Deno.env.get('SECRET_NAME');
   ```

**Note:** Edge Function secrets are NEVER in `build-config.ts` because they run on Supabase servers, not in the client build.

### Security Notes

- **Site Keys** (like `RECAPTCHA_SITE_KEY`) are public and safe to include in client-side code
- **Secret Keys** (like `RECAPTCHA_SECRET_KEY`) must NEVER be exposed to the client
- Secret keys should only be used in server-side API code that verifies tokens
- Never commit secrets to the repository

## Development Setup

### Prerequisites

- Node.js 22+
- Yarn 1.22.22+

### Installation

```bash
cd ui/website
yarn install
```

### Running Locally

```bash
# Start development server
yarn dev

# Build for production
yarn build

# Preview production build
yarn preview
```

## Code Style

- Follow TypeScript best practices
- Use SolidJS patterns and conventions
- Prefer functional components
- Use TypeScript strict mode
- Follow existing code structure and naming conventions

## Testing

- Test your changes locally before submitting
- Ensure the build completes successfully
- Verify functionality in the browser

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes
3. Test locally
4. Submit a pull request with a clear description
5. Ensure all CI checks pass

## Questions?

If you have questions about contributing, please open an issue or contact the maintainers.

