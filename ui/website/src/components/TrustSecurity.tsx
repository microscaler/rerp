import { Component } from 'solid-js';

const TrustSecurity: Component = () => {
  const trustBadges = [
    {
      icon: 'fa-code-branch',
      title: 'Open Source Transparency',
      description: 'Full source code access enables security audits and community verification',
      color: 'text-primary',
    },
    {
      icon: 'fa-user-shield',
      title: 'Role-Based Access Control',
      description: 'Comprehensive RBAC system with fine-grained permissions and access policies',
      color: 'text-secondary',
    },
    {
      icon: 'fa-shield-alt',
      title: 'Data Privacy Compliance Ready',
      description: 'Architecture designed to support GDPR, CCPA, and other data protection regulations',
      color: 'text-accent',
    },
    {
      icon: 'fa-clipboard-list',
      title: 'Audit Trails & Logging',
      description: 'Complete audit trails for all operations with comprehensive logging capabilities',
      color: 'text-purple-600',
    },
    {
      icon: 'fa-lock',
      title: 'Secure API Architecture',
      description: 'API-first design with authentication, authorization, and secure communication protocols',
      color: 'text-orange-600',
    },
    {
      icon: 'fa-check-circle',
      title: 'Community Audited',
      description: 'Open-source codebase reviewed by the community for security and quality',
      color: 'text-green-600',
    },
  ];

  const securityFeatures = [
    {
      title: 'Security by Design',
      items: [
        'Open-source transparency allows security audits',
        'API-first architecture with built-in security',
        'Regular security reviews by the community',
      ],
    },
    {
      title: 'Access Control',
      items: [
        'Role-based access control (RBAC)',
        'Identity and access management (IDAM)',
        'Fine-grained permission system',
      ],
    },
    {
      title: 'Compliance Ready',
      items: [
        'Data privacy compliance architecture',
        'Audit trail capabilities',
        'Secure data handling practices',
      ],
    },
  ];

  return (
    <section id="trust-security" class="py-20 bg-gray-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Enterprise-Grade Security & Compliance</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto">
            RERP is built with security and compliance as core principles. Open-source transparency enables community verification and security audits.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-16">
          {trustBadges.map((badge) => (
            <div class="bg-white rounded-xl p-6 shadow-md hover:shadow-lg transition-shadow border border-gray-100">
              <div class={`${badge.color} text-3xl mb-4`}>
                <i class={`fa-solid ${badge.icon}`}></i>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">{badge.title}</h3>
              <p class="text-gray-600 text-sm leading-relaxed">{badge.description}</p>
            </div>
          ))}
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-12">
          {securityFeatures.map((feature) => (
            <div class="bg-white rounded-xl p-8 shadow-md">
              <h3 class="text-xl font-semibold text-gray-900 mb-4">{feature.title}</h3>
              <ul class="space-y-3">
                {feature.items.map((item) => (
                  <li class="flex items-start">
                    <i class="fa-solid fa-check-circle text-secondary mr-3 mt-1"></i>
                    <span class="text-gray-600">{item}</span>
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div class="bg-gradient-to-r from-primary to-blue-600 rounded-2xl p-8 text-white text-center">
          <h3 class="text-2xl font-bold mb-4">Security Through Transparency</h3>
          <p class="text-blue-100 mb-6 max-w-2xl mx-auto">
            RERP's open-source model enables security through transparency. The community can review, audit, and improve the codebase continuously.
          </p>
          <div class="flex items-center justify-center space-x-8 flex-wrap gap-4">
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-code-branch"></i>
              <span class="font-medium">Open Source</span>
            </div>
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-users"></i>
              <span class="font-medium">Community Audited</span>
            </div>
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-shield-alt"></i>
              <span class="font-medium">Security First</span>
            </div>
            <div class="flex items-center space-x-2">
              <i class="fa-solid fa-check-circle"></i>
              <span class="font-medium">Compliance Ready</span>
            </div>
          </div>
        </div>

        <div class="text-center mt-8">
          <a
            href="https://github.com/microscaler/rerp"
            target="_blank"
            rel="noopener noreferrer"
            class="text-primary hover:text-blue-700 font-medium inline-flex items-center"
          >
            Review the Source Code
            <i class="fa-solid fa-arrow-right ml-2"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default TrustSecurity;
