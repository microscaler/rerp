import { Component } from 'solid-js';

export interface DoDontGridProps {
  doItems: string[];
  dontItems: string[];
}

const DoDontGrid: Component<DoDontGridProps> = (props) => {
  return (
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
      <div class="bg-green-50 rounded-lg p-6">
        <h3 class="font-semibold text-gray-900 mb-2">✅ Do</h3>
        <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
          {props.doItems.map((item) => (
            <li>{item}</li>
          ))}
        </ul>
      </div>
      <div class="bg-red-50 rounded-lg p-6">
        <h3 class="font-semibold text-gray-900 mb-2">❌ Don't</h3>
        <ul class="list-disc pl-6 text-gray-700 text-sm space-y-1">
          {props.dontItems.map((item) => (
            <li>{item}</li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default DoDontGrid;

