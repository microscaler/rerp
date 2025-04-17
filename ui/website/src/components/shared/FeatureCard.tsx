import { Component, JSX } from 'solid-js';

export interface FeatureCardProps {
  icon: string;
  iconBgColor?: string;
  title: string;
  description: string | JSX.Element;
  variant?: 'default' | 'centered';
}

const FeatureCard: Component<FeatureCardProps> = (props) => {
  const iconBgColor = props.iconBgColor || 'bg-primary';
  const variant = props.variant || 'default';
  const isCentered = variant === 'centered';

  return (
    <div class={isCentered ? 'text-center' : ''}>
      <div class={`w-16 h-16 ${iconBgColor} rounded-full flex items-center justify-center ${isCentered ? 'mx-auto' : ''} mb-4`}>
        <i class={`${props.icon} text-white text-2xl`}></i>
      </div>
      <h3 class={`text-xl font-semibold text-gray-900 mb-2 ${isCentered ? '' : ''}`}>{props.title}</h3>
      <div class="text-gray-600">
        {typeof props.description === 'string' ? (
          <p>{props.description}</p>
        ) : (
          props.description
        )}
      </div>
    </div>
  );
};

export default FeatureCard;

