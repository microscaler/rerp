import { Component, JSX } from 'solid-js';

export interface SectionHeaderProps {
  title: string;
  description?: string | JSX.Element;
  subtitle?: string;
  variant?: 'default' | 'hero';
  titleSize?: 'default' | 'large';
}

const SectionHeader: Component<SectionHeaderProps> = (props) => {
  const variant = props.variant || 'default';
  const titleSize = props.titleSize || 'default';
  
  const titleClass = variant === 'hero'
    ? `text-4xl md:text-5xl font-bold text-white mb-4`
    : titleSize === 'large'
    ? `text-4xl md:text-5xl font-bold text-gray-900 mb-4`
    : `text-4xl font-bold text-gray-900 mb-4`;
  
  const descriptionClass = variant === 'hero'
    ? `text-xl text-gray-300`
    : `text-xl text-gray-600`;
  
  const subtitleClass = variant === 'hero'
    ? `text-sm text-gray-400 mt-2`
    : `text-sm text-gray-400 mt-2`;
  
  const containerClass = variant === 'hero'
    ? `text-center mb-8`
    : `text-center mb-16`;

  return (
    <div class={containerClass}>
      {variant === 'hero' ? (
        <h1 class={titleClass}>{props.title}</h1>
      ) : (
        <h2 class={titleClass}>{props.title}</h2>
      )}
      {props.description && (
        <p class={`${descriptionClass} max-w-3xl mx-auto`}>
          {props.description}
        </p>
      )}
      {props.subtitle && (
        <p class={subtitleClass}>{props.subtitle}</p>
      )}
    </div>
  );
};

export default SectionHeader;

