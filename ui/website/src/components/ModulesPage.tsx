import { Component } from 'solid-js';
import { modulesData, selectionGuides, additionalServices } from '../data/modules-data';

const ModulesPage: Component = () => {
  return (
    <div class="bg-gray-50 min-h-screen">
      <section id="modules" class="py-20 bg-white">
        <div class="max-w-7xl mx-auto px-6 lg:px-8">
          <div class="text-center mb-16">
            <h1 class="text-5xl font-bold text-gray-900 mb-4">71 Services. 6 Implementation Phases. Infinite Possibilities.</h1>
            <p class="text-xl text-gray-600 max-w-3xl mx-auto">
              RERP's modular architecture lets you build the perfect ERP for your business. Start with core modules and add functionality as you grow.
            </p>
          </div>

          {modulesData.phases.map((phase) => (
            <div class="mb-16 bg-white rounded-xl shadow-lg p-8 border border-gray-200">
              <div class="flex items-center justify-between mb-6">
                <div>
                  <h2 class="text-3xl font-bold text-gray-900 mb-2">{phase.title}</h2>
                  <p class="text-gray-600">{phase.description}</p>
                </div>
                <div class="bg-primary/10 rounded-lg px-4 py-2">
                  <span class="text-primary font-semibold">{phase.serviceCount} Services</span>
                </div>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {phase.categories.map((category) => (
                  <div class="bg-gray-50 rounded-lg p-6 border border-gray-200">
                    <h3 class="text-lg font-semibold text-gray-900 mb-4">{category.name}</h3>
                    <ul class="space-y-2">
                      {category.services.map((service) => (
                        <li class="flex items-center text-gray-600">
                          <i class="fa-solid fa-check text-secondary mr-2"></i>
                          <span>{service}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                ))}
              </div>
            </div>
          ))}

          <div class="mb-16 bg-gradient-to-br from-blue-50 to-indigo-100 rounded-xl p-8 border border-blue-200">
            <h2 class="text-3xl font-bold text-gray-900 mb-4">{additionalServices.title}</h2>
            <p class="text-gray-600 mb-6">{additionalServices.description}</p>
            <div class="flex flex-wrap gap-3">
              {additionalServices.services.map((service) => (
                <span class="bg-white rounded-lg px-4 py-2 border border-gray-200 text-gray-700 font-medium">
                  {service}
                </span>
              ))}
            </div>
          </div>

          <div class="bg-white rounded-xl shadow-lg p-8 border border-gray-200">
            <h2 class="text-3xl font-bold text-gray-900 mb-6 text-center">Which Modules Do You Need?</h2>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              {selectionGuides.map((guide) => (
                <div class="bg-gray-50 rounded-lg p-6 border border-gray-200">
                  <h3 class="text-xl font-semibold text-gray-900 mb-2">{guide.title}</h3>
                  <p class="text-gray-600 text-sm mb-4">{guide.description}</p>
                  <div class="text-sm text-gray-500">
                    Includes {guide.phases.length} phase{guide.phases.length > 1 ? 's' : ''}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};

export default ModulesPage;
