import { Component } from 'solid-js';

export interface SuccessListProps {
  items: string[];
}

const SuccessList: Component<SuccessListProps> = (props) => {
  return (
    <div class="bg-green-50 rounded-lg p-6 mb-6">
      <ul class="space-y-3 text-gray-700">
        {props.items.map((item) => (
          <li class="flex items-start">
            <i class="fa-solid fa-check-circle text-green-600 mr-3 mt-1"></i>
            <span>{item}</span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default SuccessList;

