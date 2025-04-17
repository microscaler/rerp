import { Component, JSX, Show } from 'solid-js';

export interface AlertProps {
  type: 'success' | 'error' | 'warning' | 'info';
  message: string | JSX.Element;
  description?: string | JSX.Element;
  onClose?: () => void;
}

const Alert: Component<AlertProps> = (props) => {
  const typeStyles = {
    success: 'bg-green-50 border-green-200 text-green-800',
    error: 'bg-red-50 border-red-200 text-red-800',
    warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
    info: 'bg-blue-50 border-blue-200 text-blue-800',
  };

  const icons = {
    success: 'fa-check-circle',
    error: 'fa-exclamation-circle',
    warning: 'fa-exclamation-triangle',
    info: 'fa-info-circle',
  };

  return (
    <div class={`border px-4 py-3 rounded-lg ${typeStyles[props.type]}`}>
      <div class="flex items-start">
        <i class={`fa-solid ${icons[props.type]} mr-2 mt-0.5`}></i>
        <div class="flex-1">
          <div class="font-semibold">
            {typeof props.message === 'string' ? <p>{props.message}</p> : props.message}
          </div>
          <Show when={props.description}>
            <div class="text-sm mt-1">
              {typeof props.description === 'string' ? <p>{props.description}</p> : props.description}
            </div>
          </Show>
        </div>
        <Show when={props.onClose}>
          <button
            onClick={props.onClose}
            class="ml-2 text-current opacity-70 hover:opacity-100"
            aria-label="Close"
          >
            <i class="fa-solid fa-times"></i>
          </button>
        </Show>
      </div>
    </div>
  );
};

export default Alert;

