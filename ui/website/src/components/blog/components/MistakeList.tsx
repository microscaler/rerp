import { Component } from 'solid-js';

export interface MistakeItem {
  title: string;
  description: string;
}

export interface MistakeListProps {
  items: MistakeItem[];
}

const MistakeList: Component<MistakeListProps> = (props) => {
  return (
    <div class="bg-red-50 rounded-lg p-6 mb-6">
      <ul class="space-y-3 text-gray-700">
        {props.items.map((item) => (
          <li class="flex items-start">
            <i class="fa-solid fa-times-circle text-red-600 mr-3 mt-1"></i>
            <span>
              <strong>{item.title}:</strong> {item.description}
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default MistakeList;

