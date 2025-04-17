import { Component, JSX } from 'solid-js';

export interface InfoBoxProps {
  title?: string;
  variant?: 'gray' | 'blue' | 'red' | 'yellow' | 'green' | 'purple' | 'orange';
  children: JSX.Element;
}

const variantClasses = {
  gray: 'bg-gray-50 border-gray-200',
  blue: 'bg-blue-50 border-blue-200',
  red: 'bg-red-50 border-red-200',
  yellow: 'bg-yellow-50 border-yellow-200',
  green: 'bg-green-50 border-green-200',
  purple: 'bg-purple-50 border-purple-200',
  orange: 'bg-orange-50 border-orange-200'
};

const InfoBox: Component<InfoBoxProps> = (props) => {
  const variant = props.variant || 'gray';
  const bgClass = variantClasses[variant];

  return (
    <div class={`${bgClass} border rounded-lg p-4 mb-6`}>
      {props.title && (
        <h3 class="text-lg font-semibold text-gray-900 mb-3">{props.title}</h3>
      )}
      {props.children}
    </div>
  );
};

export default InfoBox;

