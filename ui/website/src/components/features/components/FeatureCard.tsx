import { Component } from 'solid-js';

export interface FeatureCardProps {
  id: string;
  slug: string;
  icon: string;
  iconBg: string;
  title: string;
  description: string;
  proofPoint?: string;
}

const FeatureCard: Component<FeatureCardProps> = (props) => {
  const handleClick = (e: MouseEvent) => {
    e.preventDefault();
    window.location.hash = `#feature-${props.slug}`;
    window.scrollTo({ top: 0, behavior: 'instant' });
  };

  return (
    <a
      href={`#feature-${props.slug}`}
      id={props.id}
      onClick={handleClick}
      class="bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-shadow cursor-pointer block"
    >
      <div class={`w-12 h-12 ${props.iconBg} rounded-lg flex items-center justify-center mb-6`}>
        <i class={`fa-solid ${props.icon} text-white text-xl`}></i>
      </div>
      <h3 class="text-xl font-semibold text-gray-900 mb-4">{props.title}</h3>
      <p class="text-gray-600 leading-relaxed mb-4">{props.description}</p>
      {props.proofPoint && (
        <div class="mb-4">
          <div class="inline-flex items-center space-x-2 bg-primary/10 rounded-lg px-3 py-1">
            <i class="fa-solid fa-chart-line text-primary text-sm"></i>
            <span class="text-sm font-semibold text-primary">{props.proofPoint}</span>
          </div>
        </div>
      )}
      <div class="flex items-center text-primary-600 font-medium text-sm">
        <span>Learn more</span>
        <i class="fa-solid fa-arrow-right ml-2"></i>
      </div>
    </a>
  );
};

export default FeatureCard;

