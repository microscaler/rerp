import { Component, JSXElement } from 'solid-js';

export interface FeatureCard {
  icon: string;
  iconColor: string;
  title: string;
  description: string | JSXElement;
}

export interface FeatureCardGridProps {
  items: FeatureCard[];
  columns?: 2 | 3 | 4;
}

const FeatureCardGrid: Component<FeatureCardGridProps> = (props) => {
  const columns = props.columns || 2;
  const gridClass = columns === 2 
    ? 'grid-cols-1 md:grid-cols-2' 
    : columns === 3 
    ? 'grid-cols-1 md:grid-cols-3'
    : 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4';

  return (
    <div class={`grid ${gridClass} gap-6 mb-6`}>
      {props.items.map((item) => (
        <div class="bg-gray-50 rounded-lg p-6">
          <div class="flex items-center mb-3">
            <i class={`${item.icon} ${item.iconColor} text-xl mr-3`}></i>
            <h3 class="text-lg font-semibold text-gray-900">{item.title}</h3>
          </div>
          <div class="text-gray-600 text-sm">{item.description}</div>
        </div>
      ))}
    </div>
  );
};

export default FeatureCardGrid;

