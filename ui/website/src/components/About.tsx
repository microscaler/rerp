import { Component } from 'solid-js';

const About: Component = () => {
  return (
    <section id="about" class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Built for Modern Enterprises</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto">
            RERP addresses the limitations of traditional monolithic ERPs with a modular, cloud-native architecture designed for today's businesses.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-12 items-center mb-16">
          <div>
            <h3 class="text-2xl font-bold text-gray-900 mb-4">The Problem with Traditional ERPs</h3>
            <p class="text-gray-600 mb-4 leading-relaxed">
              Traditional ERP systems suffer from monolithic architectures that make updates risky and slow, vendor lock-in that creates dependency, 
              and high costs that limit accessibility. They struggle with modern workloads and complex integrations.
            </p>
            <p class="text-gray-600 leading-relaxed">
              RERP changes that. Our modular architecture enables independent scaling and updates. The open-source model ensures no vendor lock-in. 
              Cloud-native design adapts to modern infrastructure needs.
            </p>
          </div>
          <div class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-xl p-8">
            <div class="space-y-6">
              <div class="flex items-start space-x-4">
                <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center flex-shrink-0">
                  <i class="fa-solid fa-puzzle-piece text-white"></i>
                </div>
                <div>
                  <h4 class="font-semibold text-gray-900 mb-2">Modular vs Monolithic</h4>
                  <p class="text-gray-600 text-sm">71 independent services vs single codebase. Deploy only what you need, update independently.</p>
                </div>
              </div>
              <div class="flex items-start space-x-4">
                <div class="w-10 h-10 bg-secondary rounded-lg flex items-center justify-center flex-shrink-0">
                  <i class="fa-solid fa-code-branch text-white"></i>
                </div>
                <div>
                  <h4 class="font-semibold text-gray-900 mb-2">Open Standards vs Proprietary</h4>
                  <p class="text-gray-600 text-sm">OpenAPI-first design with open standards. No proprietary APIs or vendor lock-in.</p>
                </div>
              </div>
              <div class="flex items-start space-x-4">
                <div class="w-10 h-10 bg-accent rounded-lg flex items-center justify-center flex-shrink-0">
                  <i class="fa-solid fa-cloud text-white"></i>
                </div>
                <div>
                  <h4 class="font-semibold text-gray-900 mb-2">Independent Deployment</h4>
                  <p class="text-gray-600 text-sm">Each service can be deployed, scaled, and updated independently. No all-or-nothing deployments.</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="mt-16 text-center">
          <h3 class="text-2xl font-bold text-gray-900 mb-4">Community-Driven Development</h3>
          <p class="text-gray-600 mb-6 max-w-2xl mx-auto">
            RERP is built by the community, for the community. Open-source transparency, collaborative development, and shared innovation 
            drive continuous improvement.
          </p>
          <a
            href="https://github.com/microscaler/rerp"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 font-semibold"
          >
            Join the Community
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default About;
