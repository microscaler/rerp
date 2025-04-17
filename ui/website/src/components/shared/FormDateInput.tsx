import { Component, createMemo } from 'solid-js';

export interface FormDateInputProps {
  id: string;
  label: string;
  value: string;
  onInput: (value: string) => void;
  required?: boolean;
  class?: string;
  error?: string;
  max?: string; // For end date, can set max to today
  min?: string; // For start date, can set min
}

const FormDateInput: Component<FormDateInputProps> = (props) => {
  // Calculate min year: current year - 38 years
  const currentYear = new Date().getFullYear();
  const minYear = currentYear - 38;
  const minDate = createMemo(() => {
    if (props.min) return props.min;
    return `${minYear}-01-01`;
  });
  
  const maxDate = createMemo(() => {
    if (props.max) return props.max;
    return `${currentYear}-12-31`;
  });

  return (
    <div class={props.class}>
      <label for={props.id} class="block text-sm font-medium text-gray-700 mb-1">
        {props.label}
        {props.required && <span class="text-red-500 ml-1">*</span>}
      </label>
      <input
        type="date"
        id={props.id}
        required={props.required}
        value={props.value || ''}
        min={minDate()}
        max={maxDate()}
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

export default FormDateInput;

