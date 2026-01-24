import { Component } from 'solid-js';

const Hero: Component = () => {
  return (
    <section id="hero" class="bg-gradient-to-br from-blue-50 to-indigo-100 py-20 flex items-center">
      <div class="max-w-7xl mx-auto px-6 lg:px-8 grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
        <div class="space-y-6">
          <h1 class="text-5xl font-bold text-gray-900 leading-tight">
            Enterprise Resource Planning,{' '}
            <span class="text-primary">Reimagined</span>
          </h1>
          <p class="text-xl text-gray-600 leading-relaxed">
            A comprehensive, modular ERP system with 71 integrated services designed for businesses of all sizes. Build the ERP you need, not the ERP you're forced to use.
          </p>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mt-6">
            <div class="flex items-start space-x-3">
              <i class="fa-solid fa-cloud text-primary text-xl mt-1"></i>
              <div>
                <div class="font-semibold text-gray-900">Cloud-Native</div>
                <div class="text-sm text-gray-600">Modern architecture</div>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <i class="fa-solid fa-puzzle-piece text-primary text-xl mt-1"></i>
              <div>
                <div class="font-semibold text-gray-900">Modular Design</div>
                <div class="text-sm text-gray-600">Use only what you need</div>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <i class="fa-solid fa-code-branch text-primary text-xl mt-1"></i>
              <div>
                <div class="font-semibold text-gray-900">Open Source</div>
                <div class="text-sm text-gray-600">No vendor lock-in</div>
              </div>
            </div>
            <div class="flex items-start space-x-3">
              <i class="fa-solid fa-chart-line text-primary text-xl mt-1"></i>
              <div>
                <div class="font-semibold text-gray-900">Enterprise Scale</div>
                <div class="text-sm text-gray-600">Startup to Fortune 500</div>
              </div>
            </div>
          </div>

          <div class="flex flex-wrap gap-4 pt-4">
            <a
              href="#about"
              class="bg-primary text-white px-6 py-3 rounded-lg hover:bg-blue-700 font-semibold transition-colors inline-flex items-center"
            >
              About
              <i class="fa-solid fa-arrow-right ml-2"></i>
            </a>
            <a
              href="#contact"
              class="bg-white text-primary border-2 border-primary px-6 py-3 rounded-lg hover:bg-blue-50 font-semibold transition-colors inline-flex items-center"
            >
              Contact
            </a>
            <a
              href="https://github.com/microscaler/rerp"
              target="_blank"
              rel="noopener noreferrer"
              class="text-primary hover:text-blue-700 font-semibold inline-flex items-center"
            >
              <i class="fa-brands fa-github mr-2"></i>
              View on GitHub
            </a>
          </div>
        </div>
        <div class="relative">
          <div class="bg-white rounded-2xl shadow-2xl p-6">
            <div class="h-80 overflow-hidden rounded-lg bg-gradient-to-br from-blue-100 to-indigo-200 flex items-center justify-center">
              <div class="text-center text-gray-600">
                <i class="fa-solid fa-sitemap text-6xl mb-4"></i>
                <p class="text-lg font-semibold">Modular ERP Architecture</p>
                <p class="text-sm mt-2">71 Services • 6 Phases</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;
