import { Component, JSX } from 'solid-js';

export interface CTASectionProps {
  title: string;
  description: string | JSX.Element;
  primaryButton: {
    text: string;
    onClick: () => void;
    variant?: 'primary' | 'secondary';
  };
  secondaryButton?: {
    text: string;
    onClick: () => void;
  };
  backgroundClass?: string;
  textColor?: string;
}

const CTASection: Component<CTASectionProps> = (props) => {
  const backgroundClass = props.backgroundClass || 'bg-gray-900';
  const textColor = props.textColor || 'text-white';
  const descriptionColor = props.textColor === 'text-white' ? 'text-gray-300' : 'text-gray-600';
  const primaryButtonClass = props.primaryButton.variant === 'secondary'
    ? 'border border-gray-600 text-gray-300 px-8 py-3 rounded-lg hover:bg-gray-800 font-semibold text-lg transition-colors'
    : 'bg-primary text-white px-8 py-3 rounded-lg hover:bg-blue-700 font-semibold text-lg transition-colors';

  return (
    <section class={`${backgroundClass} ${textColor} py-16`}>
      <div class="max-w-4xl mx-auto px-6 lg:px-8 text-center">
        <h2 class="text-3xl font-bold mb-4">{props.title}</h2>
        <div class={`text-xl ${descriptionColor} mb-8`}>
          {typeof props.description === 'string' ? (
            <p>{props.description}</p>
          ) : (
            props.description
          )}
        </div>
        <div class="flex items-center justify-center space-x-4">
          <button
            onClick={props.primaryButton.onClick}
            class={primaryButtonClass}
          >
            {props.primaryButton.text}
          </button>
          {props.secondaryButton && (
            <button
              onClick={props.secondaryButton.onClick}
              class="border border-gray-600 text-gray-300 px-8 py-3 rounded-lg hover:bg-gray-800 font-semibold text-lg transition-colors"
            >
              {props.secondaryButton.text}
            </button>
          )}
        </div>
      </div>
    </section>
  );
};

export default CTASection;

