import { Component } from 'solid-js';

export interface FeatureListItem {
  title?: string;
  description: string;
}

export interface FeatureListProps {
  items: FeatureListItem[];
  iconColor?: string;
  showBold?: boolean;
  variant?: 'default' | 'check' | 'info' | 'shield';
}

const iconMap = {
  check: 'fa-solid fa-check',
  info: 'fa-solid fa-info-circle',
  shield: 'fa-solid fa-shield-alt',
  default: 'fa-solid fa-check'
};

const FeatureList: Component<FeatureListProps> = (props) => {
  const iconColor = props.iconColor || 'text-secondary';
  const showBold = props.showBold !== false; // default true
  const variant = props.variant || 'default';
  const iconClass = iconMap[variant];

  return (
    <ul class="space-y-3 text-gray-700">
      {props.items.map((item) => (
        <li class="flex items-start">
          <i class={`${iconClass} ${iconColor} mr-3 mt-1`}></i>
          <span>
            {item.title && showBold && <strong>{item.title}:</strong>}
            {item.title && showBold && ' '}
            {item.description}
          </span>
        </li>
      ))}
    </ul>
  );
};

export default FeatureList;

