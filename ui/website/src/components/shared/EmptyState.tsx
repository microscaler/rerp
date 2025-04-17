import { Component, JSX } from 'solid-js';

export interface EmptyStateProps {
  icon: string;
  title: string;
  description: string | JSX.Element;
  action?: {
    text: string;
    onClick: () => void;
  };
}

const EmptyState: Component<EmptyStateProps> = (props) => {
  return (
    <div class="bg-white rounded-lg border border-gray-200 p-12 text-center">
      <i class={`${props.icon} text-4xl text-gray-400 mb-4`}></i>
      <h3 class="text-xl font-semibold text-gray-900 mb-2">{props.title}</h3>
      <div class="text-gray-600 mb-6">
        {typeof props.description === 'string' ? (
          <p>{props.description}</p>
        ) : (
          props.description
        )}
      </div>
      {props.action && (
        <button
          onClick={props.action.onClick}
          class="bg-primary text-white px-6 py-2 rounded-lg hover:bg-blue-700 font-semibold transition-colors"
        >
          {props.action.text}
        </button>
      )}
    </div>
  );
};

export default EmptyState;

