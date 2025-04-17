import { Component, JSXElement } from 'solid-js';

export interface FloatingSidebarProps {
  title: string;
  children: JSXElement;
}

const FloatingSidebar: Component<FloatingSidebarProps> = (props) => {
  return (
    <div class="w-full lg:w-80">
      <div class="bg-white rounded-lg shadow-lg p-6 border border-gray-200 max-h-[calc(100vh-4rem)] overflow-y-auto">
        <h2 class="text-xl font-bold text-gray-900 mb-4">
          {props.title}
        </h2>
        <div class="space-y-4">
          {props.children}
        </div>
      </div>
    </div>
  );
};

export default FloatingSidebar;

