import { Component } from 'solid-js';

const Testimonials: Component = () => {
  return (
    <section id="testimonials" class="py-20 bg-gray-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">What the Community Says</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto">
            RERP is built by and for the community. As we grow, we'll share stories from organizations using RERP to transform their operations.
          </p>
        </div>
        
        <div class="text-center py-12">
          <p class="text-gray-500 italic">
            Testimonials coming soon. Be among the first to share your RERP success story!
          </p>
        </div>
      </div>
    </section>
  );
};

export default Testimonials;
