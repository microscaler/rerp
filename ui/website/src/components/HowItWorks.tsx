import { Component } from 'solid-js';

interface HowItWorksProps {
  standalone?: boolean;
}

const HowItWorks: Component<HowItWorksProps> = (props) => {
  const sections = [
    {
      id: 'modular-architecture',
      title: 'Built for Flexibility',
      icon: 'fa-puzzle-piece',
      color: 'bg-primary',
      content: 'RERP\'s modular architecture means each of the 71 services is independent. Services can be deployed separately, scaled independently, and updated without affecting other modules. This gives you the flexibility to build exactly the ERP system your business needs, without being forced to deploy everything at once.',
    },
    {
      id: 'implementation-phases',
      title: 'Start Small, Scale Big',
      icon: 'fa-chart-line',
      color: 'bg-secondary',
      content: 'RERP is organized into 6 implementation phases. Start with Phase 1 (Core Foundation) for essential user management and product catalogs. As your business grows, add Phase 2 (Business Operations) for sales and inventory. Continue through Phase 3 (Financial & HR), Phase 4 (Advanced Operations), Phase 5 (Customer-Facing), and Phase 6 (Extensions & Analytics) as needed. Each phase builds on the previous ones, but you can deploy them independently.',
    },
    {
      id: 'api-first',
      title: 'API-First Design',
      icon: 'fa-code',
      color: 'bg-accent',
      content: 'Every RERP service is defined with OpenAPI 3.1.0 specifications. This API-first approach enables easy integrations with existing systems, webhooks for event-driven workflows, and seamless data exchange. Whether you need to connect to e-commerce platforms, payment gateways, or custom applications, RERP\'s standardized APIs make integration straightforward.',
    },
    {
      id: 'deployment-options',
      title: 'Deploy Your Way',
      icon: 'fa-cloud',
      color: 'bg-purple-600',
      content: 'RERP supports multiple deployment options to fit your infrastructure needs. Deploy on-premise for complete control, use cloud services for scalability, or choose a hybrid approach. The cloud-native architecture works seamlessly with Kubernetes, Docker, and modern container orchestration platforms. Each service can be deployed to different environments as needed.',
    },
  ];

  return (
    <section id="how-it-works-page" class="py-20 bg-gradient-to-br from-gray-50 to-blue-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        {!props.standalone && (
          <div class="text-center mb-16">
            <h2 class="text-4xl font-bold text-gray-900 mb-4">How RERP Works</h2>
            <p class="text-xl text-gray-600 max-w-3xl mx-auto">
              A modular, cloud-native ERP system designed for modern businesses. Deploy what you need, when you need it.
            </p>
          </div>
        )}

        <div class="space-y-12 mb-16">
          {sections.map((section) => (
            <div class="bg-white rounded-xl p-8 shadow-lg">
              <div class="flex items-start space-x-6">
                <div class={`w-16 h-16 ${section.color} rounded-lg flex items-center justify-center flex-shrink-0`}>
                  <i class={`fa-solid ${section.icon} text-white text-2xl`}></i>
                </div>
                <div class="flex-1">
                  <h3 class="text-2xl font-bold text-gray-900 mb-4">{section.title}</h3>
                  <p class="text-gray-600 leading-relaxed text-lg">{section.content}</p>
                </div>
              </div>
            </div>
          ))}
        </div>

        <div class="bg-white rounded-xl shadow-lg p-8 max-w-4xl mx-auto">
          <h3 class="text-2xl font-bold text-gray-900 mb-6 text-center">
            RERP vs. Traditional ERPs
          </h3>
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-gray-200">
                  <th class="text-left py-4 px-4 font-semibold text-gray-900">Feature</th>
                  <th class="text-center py-4 px-4 font-semibold text-gray-600">Traditional ERP</th>
                  <th class="text-center py-4 px-4 font-semibold text-primary">RERP</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100">
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">Architecture</td>
                  <td class="py-4 px-4 text-center text-gray-600">Monolithic</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">71 Independent Services</td>
                </tr>
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">Deployment</td>
                  <td class="py-4 px-4 text-center text-gray-600">All-or-nothing</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">Deploy only what you need</td>
                </tr>
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">Scaling</td>
                  <td class="py-4 px-4 text-center text-gray-600">Scale entire system</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">Scale services independently</td>
                </tr>
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">Updates</td>
                  <td class="py-4 px-4 text-center text-gray-600">Risky, system-wide</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">Update modules independently</td>
                </tr>
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">Licensing</td>
                  <td class="py-4 px-4 text-center text-gray-600">Proprietary, costly</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">Open-source, free</td>
                </tr>
                <tr>
                  <td class="py-4 px-4 font-medium text-gray-900">API Standards</td>
                  <td class="py-4 px-4 text-center text-gray-600">Proprietary APIs</td>
                  <td class="py-4 px-4 text-center font-semibold text-primary">OpenAPI 3.1.0</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <div class="text-center mt-12">
          <a
            href="https://github.com/microscaler/rerp#readme"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 font-semibold transition-colors"
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

export default HowItWorks;
