import { Component } from 'solid-js';
import Community from './Community';

const CommunityPage: Component = () => {
  return (
    <div class="min-h-screen bg-gray-50">
      <main>
        <div class="pt-8 pb-4">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <a
              href="#"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '';
              }}
              class="inline-flex items-center space-x-2 text-gray-600 hover:text-primary font-medium transition-colors mb-6"
            >
              <i class="fa-solid fa-arrow-left"></i>
              <span>Back to Home</span>
            </a>
            <h1 class="text-4xl font-bold text-gray-900 mb-4 text-center">Join the RERP Community</h1>
          </div>
        </div>
        <Community standalone={true} />
      </main>
    </div>
  );
};

export default CommunityPage;
