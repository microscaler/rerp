import { Component } from 'solid-js';

const CTA: Component = () => {
  return (
    <section id="cta" class="py-20 bg-gradient-to-r from-primary to-blue-600">
      <div class="max-w-7xl mx-auto px-6 lg:px-8 text-center">
        <h2 class="text-4xl font-bold text-white mb-4">Get Started with RERP</h2>
        <p class="text-xl text-blue-100 mb-8 max-w-3xl mx-auto">
          Join the open-source ERP revolution. Deploy on your infrastructure, customize to your needs, and scale as you grow.
        </p>
        <div class="flex flex-wrap justify-center gap-4">
          <a
            href="https://github.com/microscaler/rerp"
            target="_blank"
            rel="noopener noreferrer"
            class="bg-white text-primary px-8 py-3 rounded-lg hover:bg-gray-100 font-semibold transition-colors inline-flex items-center"
          >
            <i class="fa-brands fa-github mr-2"></i>
            View on GitHub
          </a>
          <a
            href="#modules"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#modules';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="bg-blue-700 text-white px-8 py-3 rounded-lg hover:bg-blue-800 font-semibold transition-colors inline-flex items-center border-2 border-white"
          >
            Explore Modules
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
          <a
            href="https://github.com/microscaler/rerp#readme"
            target="_blank"
            rel="noopener noreferrer"
            class="bg-transparent text-white px-8 py-3 rounded-lg hover:bg-white/10 font-semibold transition-colors inline-flex items-center border-2 border-white"
          >
            View Documentation
            <i class="fa-solid fa-book ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default CTA;
