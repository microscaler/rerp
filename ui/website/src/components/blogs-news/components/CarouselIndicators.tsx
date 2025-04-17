import { Component } from 'solid-js';

export interface CarouselIndicatorsProps {
  count: number;
  currentIndex: number;
  onIndicatorClick: (index: number) => void;
}

const CarouselIndicators: Component<CarouselIndicatorsProps> = (props) => {
  return (
    <div class="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2">
      {Array.from({ length: props.count }).map((_, index) => (
        <button
          onClick={() => props.onIndicatorClick(index)}
          class={`h-2 rounded-full transition-all ${
            index === props.currentIndex
              ? 'bg-primary w-8'
              : 'bg-gray-300 w-2 hover:bg-gray-400'
          }`}
          aria-label={`Go to item ${index + 1}`}
        ></button>
      ))}
    </div>
  );
};

export default CarouselIndicators;

