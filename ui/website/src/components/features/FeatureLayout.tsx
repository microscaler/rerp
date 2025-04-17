import { Component, JSX } from 'solid-js';

interface FeatureLayoutProps {
  title: string;
  description: string;
  icon: string;
  iconColor: string;
  services: string[];
  useCases: string[];
  children?: JSX.Element;
}

const FeatureLayout: Component<FeatureLayoutProps> = (props) => {
  return (
    <div class="bg-white rounded-lg shadow-lg overflow-hidden">
      <div class="bg-gradient-to-r from-primary to-blue-600 p-8 text-white">
        <div class="flex items-center space-x-4 mb-4">
          <div class={`w-16 h-16 ${props.iconColor} rounded-lg flex items-center justify-center`}>
            <i class={`fa-solid ${props.icon} text-white text-2xl`}></i>
          </div>
          <div>
            <h1 class="text-4xl font-bold">{props.title}</h1>
            <p class="text-blue-100 text-lg mt-2">{props.description}</p>
          </div>
        </div>
      </div>

      <div class="p-8">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
          <div>
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Services Included</h2>
            <ul class="space-y-3">
              {props.services.map((service) => (
                <li class="flex items-start">
                  <i class="fa-solid fa-check-circle text-secondary mr-3 mt-1"></i>
                  <span class="text-gray-600">{service}</span>
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h2 class="text-2xl font-bold text-gray-900 mb-4">Use Cases</h2>
            <ul class="space-y-3">
              {props.useCases.map((useCase) => (
                <li class="flex items-start">
                  <i class="fa-solid fa-lightbulb text-primary mr-3 mt-1"></i>
                  <span class="text-gray-600">{useCase}</span>
                </li>
              ))}
            </ul>
          </div>
        </div>

        {props.children}

        <div class="mt-8 pt-8 border-t border-gray-200">
          <h2 class="text-2xl font-bold text-gray-900 mb-4">Integration Points</h2>
          <p class="text-gray-600 mb-4">
            This module integrates seamlessly with other RERP services through standardized OpenAPI interfaces. 
            Data flows automatically between related modules, ensuring consistency and eliminating manual data entry.
          </p>
          <div class="bg-gray-50 rounded-lg p-6">
            <p class="text-sm text-gray-600">
              <i class="fa-solid fa-info-circle text-primary mr-2"></i>
              All RERP services communicate via OpenAPI 3.1.0 specifications, enabling easy integration with external systems and custom applications.
            </p>
          </div>
        </div>

        <div class="mt-8 text-center">
          <a
            href="#modules"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#modules';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="inline-flex items-center px-6 py-3 bg-primary text-white rounded-lg hover:bg-blue-700 font-semibold transition-colors"
          >
            <i class="fa-solid fa-arrow-left mr-2"></i>
            Explore All Modules
          </a>
        </div>
      </div>
    </div>
  );
};

export default FeatureLayout;
