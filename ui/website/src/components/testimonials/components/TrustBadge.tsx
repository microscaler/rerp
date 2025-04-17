import { Component, JSX } from 'solid-js';

export interface TrustBadgeProps {
  icon: string;
  children: JSX.Element;
  bgColor?: string;
}

const TrustBadge: Component<TrustBadgeProps> = (props) => {
  const bgColor = props.bgColor || 'bg-gray-100';

  return (
    <div class="text-center mt-12">
      <div class={`inline-flex items-center space-x-2 ${bgColor} rounded-full px-6 py-3`}>
        <i class={props.icon}></i>
        <span class="text-gray-700 font-medium">
          {props.children}
        </span>
      </div>
    </div>
  );
};

export default TrustBadge;

