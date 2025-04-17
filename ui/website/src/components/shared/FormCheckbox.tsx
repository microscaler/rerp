import { Component } from 'solid-js';

export interface FormCheckboxProps {
  id: string;
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  class?: string;
}

const FormCheckbox: Component<FormCheckboxProps> = (props) => {
  return (
    <label class={`flex items-center space-x-2 cursor-pointer hover:bg-gray-50 p-2 rounded ${props.class || ''}`}>
      <input
        type="checkbox"
        id={props.id}
        checked={props.checked}
        onChange={(e) => props.onChange(e.currentTarget.checked)}
        class="w-4 h-4 text-primary border-gray-300 rounded focus:ring-primary"
      />
      <span class="text-sm text-gray-700">{props.label}</span>
    </label>
  );
};

export default FormCheckbox;

