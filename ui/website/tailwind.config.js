/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "../shared/**/*.{js,ts,jsx,tsx}",
  ],
  // Theme configuration moved to CSS using @theme directive in index.css
  // Tailwind v4 uses CSS-first configuration
};
