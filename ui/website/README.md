# RERP Website

SolidJS marketing site for RERP (Rust ERP). Minimal setup: Home, About, Contact.

## Development

```bash
cd ui/website
yarn install
yarn dev
```

## Build

```bash
yarn build
```

## Structure

- `src/App.tsx` – app shell, hash routing
- `src/components/` – Hero, About, Contact
- `@shared` – header, footer, analytics (from `ui/shared`)
