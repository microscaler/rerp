import { Component, onMount, onCleanup, createEffect } from 'solid-js';
import flatpickr from 'flatpickr';
import 'flatpickr/dist/flatpickr.css';

export interface DatePickerProps {
  id: string;
  label?: string;
  placeholder?: string;
  value: string; // ISO date string (YYYY-MM-DD)
  onInput: (value: string) => void;
  required?: boolean;
  class?: string;
  error?: string;
  max?: string; // ISO date string
  min?: string; // ISO date string
  mode?: 'single' | 'multiple' | 'range' | 'time';
}

const DatePicker: Component<DatePickerProps> = (props) => {
  let inputRef: HTMLInputElement | undefined;
  let flatpickrInstance: flatpickr.Instance | null = null;

  // Calculate min/max dates
  const currentYear = new Date().getFullYear();
  const minYear = currentYear - 38;
  const maxYear = currentYear;

  const minDate = props.min ? new Date(props.min) : new Date(minYear, 0, 1);
  const maxDate = props.max ? new Date(props.max) : new Date(maxYear, 11, 31);

  onMount(() => {
    if (!inputRef) return;

    flatpickrInstance = flatpickr(inputRef, {
      mode: props.mode || 'single',
      static: true,
      monthSelectorType: 'static',
      dateFormat: 'Y-m-d',
      defaultDate: props.value || undefined,
      minDate: minDate,
      maxDate: maxDate,
      onChange: (selectedDates, dateStr) => {
        if (dateStr) {
          props.onInput(dateStr);
        }
      },
    });
  });

  // Update value when prop changes
  createEffect(() => {
    if (flatpickrInstance && props.value) {
      flatpickrInstance.setDate(props.value, false);
    }
  });

  // Update min/max when props change
  createEffect(() => {
    if (flatpickrInstance) {
      if (props.min) {
        flatpickrInstance.set('minDate', new Date(props.min));
      }
      if (props.max) {
        flatpickrInstance.set('maxDate', new Date(props.max));
      }
    }
  });

  onCleanup(() => {
    if (flatpickrInstance) {
      flatpickrInstance.destroy();
    }
  });

  return (
    <div class={props.class}>
      {props.label && (
        <label for={props.id} class="block text-sm font-medium text-gray-700 mb-1">
          {props.label}
          {props.required && <span class="text-red-500 ml-1">*</span>}
        </label>
      )}

      <div class="relative">
        <input
          ref={inputRef}
          id={props.id}
          type="text"
          placeholder={props.placeholder || 'Select date'}
          required={props.required}
          class={`h-11 w-full rounded-lg border appearance-none pl-4 pr-12 py-2.5 text-sm shadow-sm placeholder:text-gray-400 focus:outline-none focus:ring-2 ${
            props.error
              ? 'border-red-300 focus:border-red-500 focus:ring-red-500/20'
              : 'border-gray-300 focus:border-primary focus:ring-primary/20'
          } bg-transparent text-gray-800`}
        />

        <span class="absolute text-gray-500 -translate-y-1/2 pointer-events-none right-3 top-1/2">
          <svg
            class="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
            />
          </svg>
        </span>
      </div>
      {props.error && (
        <p class="mt-1 text-sm text-red-600">{props.error}</p>
      )}
    </div>
  );
};

export default DatePicker;

