import { Component } from 'solid-js';

export interface CustomerInfoProps {
  customer: string;
  role: string;
  location: string;
}

const CustomerInfo: Component<CustomerInfoProps> = (props) => {
  return (
    <div class="space-y-2 mb-6">
      <div class="flex items-center space-x-2">
        <i class="fa-solid fa-user text-primary"></i>
        <span class="text-gray-700 font-medium">{props.customer}</span>
      </div>
      <div class="flex items-center space-x-2">
        <i class="fa-solid fa-briefcase text-primary"></i>
        <span class="text-gray-600">{props.role}</span>
      </div>
      <div class="flex items-center space-x-2">
        <i class="fa-solid fa-map-marker-alt text-primary"></i>
        <span class="text-gray-600">{props.location}</span>
      </div>
    </div>
  );
};

export default CustomerInfo;

