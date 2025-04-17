import { Component } from 'solid-js';

const Contact: Component = () => {
  return (
    <section id="contact" class="py-20 bg-white">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        <div class="text-center mb-16">
          <h2 class="text-4xl font-bold text-gray-900 mb-4">Get in Touch</h2>
          <p class="text-xl text-gray-600 max-w-3xl mx-auto">
            Join the RERP community. Get help, share ideas, contribute code, or just say hello.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
          <a
            href="https://github.com/microscaler/rerp/discussions"
            target="_blank"
            rel="noopener noreferrer"
            class="bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-shadow border border-gray-200 text-center"
          >
            <div class="w-16 h-16 bg-primary rounded-full flex items-center justify-center mx-auto mb-4">
              <i class="fa-brands fa-github text-white text-2xl"></i>
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">GitHub Discussions</h3>
            <p class="text-gray-600 text-sm">Ask questions, share ideas, and connect with the community</p>
          </a>

          <a
            href="https://github.com/microscaler/rerp/issues"
            target="_blank"
            rel="noopener noreferrer"
            class="bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-shadow border border-gray-200 text-center"
          >
            <div class="w-16 h-16 bg-secondary rounded-full flex items-center justify-center mx-auto mb-4">
              <i class="fa-solid fa-bug text-white text-2xl"></i>
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">Report Issues</h3>
            <p class="text-gray-600 text-sm">Found a bug or have a feature request? Let us know</p>
          </a>

          <a
            href="https://github.com/microscaler/rerp"
            target="_blank"
            rel="noopener noreferrer"
            class="bg-gray-50 rounded-xl p-8 hover:shadow-lg transition-shadow border border-gray-200 text-center"
          >
            <div class="w-16 h-16 bg-accent rounded-full flex items-center justify-center mx-auto mb-4">
              <i class="fa-solid fa-code-branch text-white text-2xl"></i>
            </div>
            <h3 class="text-xl font-semibold text-gray-900 mb-2">Contribute</h3>
            <p class="text-gray-600 text-sm">Help build RERP by contributing code, documentation, or feedback</p>
          </a>
        </div>
      </div>
    </section>
  );
};

export default Contact;
