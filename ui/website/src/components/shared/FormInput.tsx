import { Component } from 'solid-js';

export interface FormInputProps {
  id: string;
  label: string;
  type?: 'text' | 'email' | 'tel' | 'url' | 'password';
  value: string;
  onInput: (value: string) => void;
  required?: boolean;
  placeholder?: string;
  class?: string;
  error?: string;
}

const FormInput: Component<FormInputProps> = (props) => {
  return (
    <div class={props.class}>
      <label for={props.id} class="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span class="text-red-500 ml-1">*</span>}
      </label>
      <input
        type={props.type || 'text'}
        id={props.id}
        required={props.required}
        placeholder={props.placeholder}
        value={props.value}
        onInput={(e) => props.onInput(e.currentTarget.value)}
        class={`w-full px-3 py-2 border ${
          props.error ? 'border-red-300' : 'border-gray-300'
        } rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent`}
      />
      {props.error && (
        <p class="mt-1 text-sm text-red-600">{props.error}</p>
      )}
    </div>
  );
};

export default FormInput;

