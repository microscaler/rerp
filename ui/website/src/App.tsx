import { Component, onMount } from 'solid-js';
import Header from '@shared/header/Header';
import Hero from './components/Hero';
import About from './components/About';
import Contact from './components/Contact';
import Footer from '@shared/footer/Footer';
import { resetSEO } from './utils/seo';
import { initializeGA, trackPageView } from '@shared/utils/analytics';

const App: Component = () => {
  onMount(() => {
    const path = window.location.pathname + window.location.search + window.location.hash;
    initializeGA(path);
    trackPageView(path || '/');

    const sync = () => {
      const h = window.location.hash || '';
      if (!h || h === '#') {
        resetSEO();
        window.scrollTo({ top: 0, behavior: 'instant' });
      } else if (h === '#about' || h === '#contact') {
        resetSEO();
        const el = document.querySelector(h);
        if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }
    };

    sync();
    window.addEventListener('hashchange', () => {
      trackPageView(window.location.pathname + window.location.search + window.location.hash);
      sync();
    });
  });

  return (
    <div class="bg-gray-50">
      <Header />
      <main>
        <Hero />
        <About />
        <Contact />
      </main>
      <Footer />
    </div>
  );
};

export default App;
