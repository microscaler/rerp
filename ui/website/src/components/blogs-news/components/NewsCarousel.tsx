import { Component, createSignal, onMount, For } from 'solid-js';
import NewsItem, { NewsItemData } from './NewsItem';
import CarouselIndicators from './CarouselIndicators';
import CarouselNavigation from './CarouselNavigation';

export interface NewsCarouselProps {
  items: NewsItemData[];
  autoRotate?: boolean;
  interval?: number; // milliseconds
  showIndicators?: boolean;
  showNavigation?: boolean;
}

const NewsCarousel: Component<NewsCarouselProps> = (props) => {
  const [currentIndex, setCurrentIndex] = createSignal(0);
  const autoRotate = props.autoRotate !== false; // default true
  const interval = props.interval || 5000;
  const showIndicators = props.showIndicators !== false; // default true
  const showNavigation = props.showNavigation !== false; // default true

  onMount(() => {
    if (autoRotate) {
      const intervalId = setInterval(() => {
        setCurrentIndex((prev) => (prev + 1) % props.items.length);
      }, interval);

      return () => clearInterval(intervalId);
    }
  });

  const goToItem = (index: number) => {
    setCurrentIndex(index);
  };

  const goToPrevious = () => {
    setCurrentIndex((prev) => (prev - 1 + props.items.length) % props.items.length);
  };

  const goToNext = () => {
    setCurrentIndex((prev) => (prev + 1) % props.items.length);
  };

  return (
    <div class="relative bg-white rounded-xl shadow-2xl overflow-hidden">
      <div class="relative h-96">
        <For each={props.items}>
          {(item, index) => (
            <NewsItem item={item} isActive={index() === currentIndex()} />
          )}
        </For>
      </div>

      {showIndicators && (
        <CarouselIndicators
          count={props.items.length}
          currentIndex={currentIndex()}
          onIndicatorClick={goToItem}
        />
      )}

      {showNavigation && (
        <CarouselNavigation
          onPrevious={goToPrevious}
          onNext={goToNext}
          previousLabel="Previous news"
          nextLabel="Next news"
        />
      )}
    </div>
  );
};

export default NewsCarousel;

