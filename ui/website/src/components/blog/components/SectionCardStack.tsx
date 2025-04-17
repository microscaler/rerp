import { Component, JSXElement } from 'solid-js';

export interface SectionCard {
  title: string;
  description: string | JSXElement;
  metadata?: string;
  metadataIcon?: string;
  icon?: string;
  iconColor?: string;
}

export interface SectionCardStackProps {
  items: SectionCard[];
}

const SectionCardStack: Component<SectionCardStackProps> = (props) => {
  return (
    <div class="space-y-4 mb-6">
      {props.items.map((item) => (
        <div class="bg-gray-50 rounded-lg p-6">
          {item.icon ? (
            <>
              <div class="flex items-center mb-3">
                <i class={`${item.icon} ${item.iconColor || 'text-primary'} text-2xl mr-4`}></i>
                <h3 class="text-lg font-semibold text-gray-900">{item.title}</h3>
              </div>
              <div class="text-gray-600 text-sm mb-2">{item.description}</div>
            </>
          ) : (
            <>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">{item.title}</h3>
              <div class="text-gray-600 text-sm mb-2">{item.description}</div>
            </>
          )}
          {item.metadata && (
            <div class="flex items-center text-sm text-gray-500">
              {item.metadataIcon && <i class={`${item.metadataIcon} mr-2`}></i>}
              <span>{item.metadata}</span>
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default SectionCardStack;

