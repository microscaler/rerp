import { Component } from 'solid-js';

const Pricing: Component = () => {
  return (
    <section id="pricing" class="py-20 bg-gray-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Open Source. Enterprise Ready.</h2>
          <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
            RERP is 100% open-source. No licensing fees. No per-user costs. Deploy on your infrastructure with full control.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto mb-12">
          <div class="bg-white rounded-xl p-8 border-2 border-primary shadow-lg">
            <div class="text-center mb-6">
              <div class="w-16 h-16 bg-primary rounded-full flex items-center justify-center mx-auto mb-4">
                <i class="fa-solid fa-server text-white text-2xl"></i>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 mb-2">Self-Hosted</h3>
              <div class="text-4xl font-bold text-primary mb-2">Free</div>
              <p class="text-gray-600">Full control, no restrictions</p>
            </div>
            <ul class="space-y-3 mb-6">
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Complete source code access</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">All 71 services included</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Deploy on your infrastructure</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">No user limits</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Full customization rights</span>
              </li>
            </ul>
            <a
              href="https://github.com/microscaler/rerp"
              target="_blank"
              rel="noopener noreferrer"
              class="w-full bg-primary text-white py-3 rounded-lg hover:bg-blue-700 font-semibold transition-colors inline-flex items-center justify-center"
            >
              <i class="fa-brands fa-github mr-2"></i>
              Get Started
            </a>
          </div>

          <div class="bg-white rounded-xl p-8 border border-gray-200 shadow-md">
            <div class="text-center mb-6">
              <div class="w-16 h-16 bg-secondary rounded-full flex items-center justify-center mx-auto mb-4">
                <i class="fa-solid fa-users text-white text-2xl"></i>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 mb-2">Community Support</h3>
              <div class="text-4xl font-bold text-gray-900 mb-2">Free</div>
              <p class="text-gray-600">Community-driven help</p>
            </div>
            <ul class="space-y-3 mb-6">
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">GitHub Discussions</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Comprehensive documentation</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Community forums</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Issue tracking</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-secondary mr-3 mt-1"></i>
                <span class="text-gray-600">Contributor resources</span>
              </li>
            </ul>
            <a
              href="https://github.com/microscaler/rerp/discussions"
              target="_blank"
              rel="noopener noreferrer"
              class="w-full border-2 border-primary text-primary py-3 rounded-lg hover:bg-blue-50 font-semibold transition-colors inline-flex items-center justify-center"
            >
              Join Community
            </a>
          </div>

          <div class="bg-white rounded-xl p-8 border border-gray-200 shadow-md opacity-75">
            <div class="text-center mb-6">
              <div class="w-16 h-16 bg-gray-400 rounded-full flex items-center justify-center mx-auto mb-4">
                <i class="fa-solid fa-building text-white text-2xl"></i>
              </div>
              <h3 class="text-2xl font-bold text-gray-900 mb-2">Enterprise Support</h3>
              <div class="text-4xl font-bold text-gray-400 mb-2">Coming Soon</div>
              <p class="text-gray-500">Commercial support options</p>
            </div>
            <ul class="space-y-3 mb-6">
              <li class="flex items-start">
                <i class="fa-solid fa-check text-gray-400 mr-3 mt-1"></i>
                <span class="text-gray-500">Priority support</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-gray-400 mr-3 mt-1"></i>
                <span class="text-gray-500">SLA guarantees</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-gray-400 mr-3 mt-1"></i>
                <span class="text-gray-500">Dedicated account manager</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-gray-400 mr-3 mt-1"></i>
                <span class="text-gray-500">Custom development</span>
              </li>
              <li class="flex items-start">
                <i class="fa-solid fa-check text-gray-400 mr-3 mt-1"></i>
                <span class="text-gray-500">Training & onboarding</span>
              </li>
            </ul>
            <a
              href="#contact"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#contact';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="w-full bg-gray-300 text-gray-600 py-3 rounded-lg font-semibold inline-flex items-center justify-center cursor-not-allowed"
            >
              Contact for Info
            </a>
          </div>
        </div>

        <div class="bg-gradient-to-r from-primary to-blue-600 rounded-2xl p-8 text-white text-center max-w-4xl mx-auto">
          <h3 class="text-2xl font-bold mb-4">Dual Licensed</h3>
          <p class="text-blue-100 mb-6 text-lg">
            RERP is licensed under Apache 2.0 and MIT licenses. Use it freely for any purpose, including commercial use.
          </p>
          <div class="flex items-center justify-center space-x-8 flex-wrap gap-4">
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-file-code"></i>
              <span class="font-medium">Apache 2.0</span>
            </div>
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-file-code"></i>
              <span class="font-medium">MIT License</span>
            </div>
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-check-circle"></i>
              <span class="font-medium">Commercial Use Allowed</span>
            </div>
          </div>
        </div>

        <div class="text-center mt-12">
          <a
            href="https://github.com/microscaler/rerp#readme"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center text-primary hover:text-blue-700 font-semibold"
          >
            <i class="fa-solid fa-book mr-2"></i>
            View Documentation
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default Pricing;
