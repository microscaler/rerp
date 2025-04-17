import { Component } from 'solid-js';

export interface CarouselNavigationProps {
  onPrevious: () => void;
  onNext: () => void;
  previousLabel?: string;
  nextLabel?: string;
}

const CarouselNavigation: Component<CarouselNavigationProps> = (props) => {
  return (
    <>
      <button
        onClick={props.onPrevious}
        class="absolute left-4 top-1/2 transform -translate-y-1/2 bg-white rounded-full p-3 shadow-lg hover:bg-gray-100 transition-colors"
        aria-label={props.previousLabel || 'Previous item'}
      >
        <i class="fa-solid fa-chevron-left text-gray-700"></i>
      </button>
      <button
        onClick={props.onNext}
        class="absolute right-4 top-1/2 transform -translate-y-1/2 bg-white rounded-full p-3 shadow-lg hover:bg-gray-100 transition-colors"
        aria-label={props.nextLabel || 'Next item'}
      >
        <i class="fa-solid fa-chevron-right text-gray-700"></i>
      </button>
    </>
  );
};

export default CarouselNavigation;

