import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';

const MarketingEcommerce: Component = () => {
  return (
    <FeatureLayout
      title="Marketing & E-commerce"
      description="Marketing automation, e-commerce, and customer engagement"
      icon="fa-bullhorn"
      iconColor="bg-blue-600"
      services=[
              "Email Marketing - Campaigns, templates, email analytics",
              "Marketing Automation - Lead nurturing, campaign management",
              "Social Media - Social integration, publishing, analytics",
              "CMS - Content management, page builder, media library",
              "E-commerce - Online store, shopping cart, checkout",
              "Website Builder - Theme management, SEO tools"
]
      useCases=[
              "Digital marketing campaigns",
              "E-commerce store management",
              "Content marketing and CMS",
              "Social media marketing",
              "Customer engagement and retention"
]
    />
  );
};

export default MarketingEcommerce;
