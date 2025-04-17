import { Component, JSX } from 'solid-js';

export interface ExampleBoxProps {
  title: string;
  variant?: 'blue' | 'yellow' | 'green' | 'red';
  children: JSX.Element;
}

const variantClasses = {
  blue: 'bg-blue-50',
  yellow: 'bg-yellow-50',
  green: 'bg-green-50',
  red: 'bg-red-50'
};

const ExampleBox: Component<ExampleBoxProps> = (props) => {
  const variant = props.variant || 'blue';
  const bgClass = variantClasses[variant];

  return (
    <div class={`${bgClass} rounded-lg p-6 mb-6`}>
      <h3 class="font-semibold text-gray-900 mb-2">{props.title}</h3>
      <div class="text-gray-700 text-sm">
        {props.children}
      </div>
    </div>
  );
};

export default ExampleBox;

