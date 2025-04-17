import { Component, JSX } from 'solid-js';

export interface KeyTakeawayBoxProps {
  color?: 'primary' | 'orange' | 'indigo' | 'secondary' | 'green' | 'accent' | 'blue' | 'purple';
  icon?: string;
  title?: string;
  children: JSX.Element;
}

const colorClasses = {
  primary: {
    bg: 'bg-primary-50',
    border: 'border-primary-500',
    icon: 'text-primary-600',
    title: 'text-primary-900',
    text: 'text-primary-700'
  },
  orange: {
    bg: 'bg-orange-50',
    border: 'border-orange-500',
    icon: 'text-orange-600',
    title: 'text-orange-900',
    text: 'text-orange-700'
  },
  indigo: {
    bg: 'bg-indigo-50',
    border: 'border-indigo-500',
    icon: 'text-indigo-600',
    title: 'text-indigo-900',
    text: 'text-indigo-700'
  },
  secondary: {
    bg: 'bg-secondary-50',
    border: 'border-secondary-500',
    icon: 'text-secondary-600',
    title: 'text-secondary-900',
    text: 'text-secondary-700'
  },
  green: {
    bg: 'bg-green-50',
    border: 'border-green-500',
    icon: 'text-green-600',
    title: 'text-green-900',
    text: 'text-green-700'
  },
  accent: {
    bg: 'bg-accent-50',
    border: 'border-accent-500',
    icon: 'text-accent-600',
    title: 'text-accent-900',
    text: 'text-accent-700'
  },
  blue: {
    bg: 'bg-blue-50',
    border: 'border-blue-500',
    icon: 'text-blue-600',
    title: 'text-blue-900',
    text: 'text-blue-700'
  },
  purple: {
    bg: 'bg-purple-50',
    border: 'border-purple-500',
    icon: 'text-purple-600',
    title: 'text-purple-900',
    text: 'text-purple-700'
  }
};

const defaultIcons = {
  primary: 'fa-solid fa-lightbulb',
  orange: 'fa-solid fa-brain',
  indigo: 'fa-solid fa-shield',
  secondary: 'fa-solid fa-chart-line',
  green: 'fa-solid fa-graduation-cap',
  accent: 'fa-solid fa-globe-americas',
  blue: 'fa-solid fa-book',
  purple: 'fa-solid fa-desktop'
};

const KeyTakeawayBox: Component<KeyTakeawayBoxProps> = (props) => {
  const color = props.color || 'primary';
  const classes = colorClasses[color];
  const icon = props.icon || defaultIcons[color];
  const title = props.title || 'Key Takeaway';

  return (
    <div class={`${classes.bg} border-l-4 ${classes.border} p-6 mb-8`}>
      <div class="flex">
        <div class="flex-shrink-0">
          <i class={`${icon} ${classes.icon} text-2xl`}></i>
        </div>
        <div class="ml-4">
          <h3 class={`text-lg font-medium ${classes.title}`}>{title}</h3>
          <p class={`mt-2 ${classes.text}`}>
            {props.children}
          </p>
        </div>
      </div>
    </div>
  );
};

export default KeyTakeawayBox;

