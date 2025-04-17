import { Component } from 'solid-js';
import { FeatureCard } from './features/components';

const Features: Component = () => {
  const features = [
    {
      id: 'feature-financial',
      slug: 'financial-management',
      icon: 'fa-chart-line',
      iconBg: 'bg-primary',
      title: 'Financial Management',
      description: 'Complete accounting suite with general ledger, accounts payable/receivable, invoicing, budgeting, financial reporting, and bank synchronization.',
      proofPoint: '9 accounting services',
    },
    {
      id: 'feature-sales-crm',
      slug: 'sales-crm',
      icon: 'fa-handshake',
      iconBg: 'bg-secondary',
      title: 'Sales & CRM',
      description: 'End-to-end sales management from lead generation to order fulfillment. Includes CRM, quotations, orders, subscriptions, and loyalty programs.',
      proofPoint: '8 sales & CRM services',
    },
    {
      id: 'feature-inventory',
      slug: 'inventory-logistics',
      icon: 'fa-warehouse',
      iconBg: 'bg-accent',
      title: 'Inventory & Logistics',
      description: 'Comprehensive inventory management with warehouse operations, stock tracking, logistics coordination, and dropshipping support.',
      proofPoint: '4 inventory services',
    },
    {
      id: 'feature-manufacturing',
      slug: 'manufacturing',
      icon: 'fa-industry',
      iconBg: 'bg-purple-600',
      title: 'Manufacturing',
      description: 'Production planning, BOM management, work centers, production tracking, repair management, and subcontracting capabilities.',
      proofPoint: '5 manufacturing services',
    },
    {
      id: 'feature-hr',
      slug: 'human-resources',
      icon: 'fa-users',
      iconBg: 'bg-orange-600',
      title: 'Human Resources',
      description: 'Complete HR management including employee records, payroll, recruitment, attendance, leave management, performance appraisals, and skills tracking.',
      proofPoint: '7 HR services',
    },
    {
      id: 'feature-projects',
      slug: 'project-management',
      icon: 'fa-tasks',
      iconBg: 'bg-indigo-600',
      title: 'Project Management',
      description: 'Project planning, task tracking, resource allocation, timesheet management, and project billing for service-based businesses.',
      proofPoint: '2 project services',
    },
    {
      id: 'feature-marketing',
      slug: 'marketing-ecommerce',
      icon: 'fa-bullhorn',
      iconBg: 'bg-blue-600',
      title: 'Marketing & E-commerce',
      description: 'Marketing automation, email campaigns, social media integration, CMS, online stores, website builder, and e-commerce management.',
      proofPoint: '6 marketing services',
    },
    {
      id: 'feature-analytics',
      slug: 'analytics-bi',
      icon: 'fa-chart-bar',
      iconBg: 'bg-green-600',
      title: 'Analytics & BI',
      description: 'Business intelligence, data warehousing, custom dashboards, reporting tools, and comprehensive analytics across all modules.',
      proofPoint: '3 analytics services',
    },
  ];

  return (
    <section id="features" class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Complete Business Management in One Platform</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto">
            RERP provides 71 integrated services across 6 implementation phases, covering every aspect of enterprise operations.
          </p>
        </div>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {features.map((feature) => (
            <FeatureCard
              id={feature.id}
              slug={feature.slug}
              icon={feature.icon}
              iconBg={feature.iconBg}
              title={feature.title}
              description={feature.description}
              proofPoint={feature.proofPoint}
            />
          ))}
        </div>
      </div>
    </section>
  );
};

export default Features;
