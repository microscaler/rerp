/**
 * Generate sitemap.xml for RERP website
 * Run during build: node scripts/generate-sitemap.js
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const pkg = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'));
const BASE = process.env.VITE_BASE_URL || pkg.config?.baseUrl || 'https://github.com/microscaler/rerp';
const today = new Date().toISOString().split('T')[0];

const urls = [
  { loc: BASE, lastmod: today, changefreq: 'weekly', priority: 1.0 },
  { loc: `${BASE}/#about`, lastmod: today, changefreq: 'monthly', priority: 0.8 },
  { loc: `${BASE}/#contact`, lastmod: today, changefreq: 'monthly', priority: 0.8 },
];

const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.map(u => `  <url>
    <loc>${u.loc}</loc>
    <lastmod>${u.lastmod}</lastmod>
    <changefreq>${u.changefreq}</changefreq>
    <priority>${u.priority}</priority>
  </url>`).join('\n')}
</urlset>`;

const out = path.join(__dirname, '..', 'public', 'sitemap.xml');
fs.mkdirSync(path.dirname(out), { recursive: true });
fs.writeFileSync(out, xml, 'utf8');
console.log('Sitemap written:', out);
