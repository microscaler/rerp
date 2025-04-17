import { Component } from 'solid-js';

export interface KeyTakeawaysBoxProps {
  items: string[];
}

const KeyTakeawaysBox: Component<KeyTakeawaysBoxProps> = (props) => {
  return (
    <div class="bg-gray-50 rounded-lg p-6 mt-8">
      <h3 class="text-xl font-semibold text-gray-900 mb-4">Key Takeaways</h3>
      <ul class="list-disc pl-6 text-gray-700 space-y-2">
        {props.items.map((item) => (
          <li>{item}</li>
        ))}
      </ul>
    </div>
  );
};

export default KeyTakeawaysBox;

