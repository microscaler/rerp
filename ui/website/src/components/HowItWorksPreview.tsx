import { Component } from 'solid-js';

const HowItWorksPreview: Component = () => {
  const steps = [
    {
      number: '01',
      title: 'Choose Your Modules',
      description: 'Select from 71 services across 6 implementation phases',
      icon: 'fa-puzzle-piece',
      color: 'bg-primary',
    },
    {
      number: '02',
      title: 'Deploy & Configure',
      description: 'Cloud-native deployment with independent scaling',
      icon: 'fa-cloud',
      color: 'bg-secondary',
    },
    {
      number: '03',
      title: 'Scale & Extend',
      description: 'Add modules as your business grows',
      icon: 'fa-chart-line',
      color: 'bg-accent',
    },
  ];

  return (
    <section id="how-it-works" class="py-20 bg-gradient-to-br from-gray-50 to-blue-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-12">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Modular Architecture for Maximum Flexibility</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-6">
            Start with core modules and add functionality as your business grows. Each service is independent and can be deployed, scaled, and updated separately.
          </p>
          <a
            href="#how-it-works-page"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#how-it-works-page';
            }}
            class="inline-flex items-center space-x-2 text-primary font-semibold hover:text-blue-700 transition-colors"
          >
            <span>View Full Details</span>
            <i class="fa-solid fa-arrow-right"></i>
          </a>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          {steps.map((step) => (
            <div class="bg-white rounded-xl p-6 shadow-md hover:shadow-lg transition-shadow">
              <div class="flex items-center justify-between mb-4">
                <div class={`w-12 h-12 ${step.color} rounded-lg flex items-center justify-center`}>
                  <i class={`fa-solid ${step.icon} text-white text-xl`}></i>
                </div>
                <span class="text-3xl font-bold text-gray-200">{step.number}</span>
              </div>
              
              <h3 class="text-lg font-semibold text-gray-900 mb-2">{step.title}</h3>
              <p class="text-gray-600 text-sm leading-relaxed">{step.description}</p>
            </div>
          ))}
        </div>

        <div class="text-center">
          <a
            href="#modules"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#modules';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="bg-primary text-white px-8 py-3 rounded-lg hover:bg-blue-700 font-semibold transition-colors inline-flex items-center"
          >
            Explore All Modules
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default HowItWorksPreview;
