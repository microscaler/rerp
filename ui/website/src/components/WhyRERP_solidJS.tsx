import { Component } from 'solid-js';

const WhyRERP: Component = () => {
  return (
    <section id="why-rerp" class="bg-gray-900 px-6 py-24 sm:py-32 lg:px-8">
      <div class="mx-auto max-w-7xl">
        <div class="mx-auto max-w-2xl lg:text-center">
          <h2 class="text-base font-semibold leading-7 text-indigo-400">Why RERP?</h2>
          <p class="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Solving the Problems with Current ERPs
          </p>
          <p class="mt-6 text-lg leading-8 text-gray-300">
            Current ERPs are complex to implement, difficult to use, and create vendor lock-in. RERP solves these problems with simple setup, intuitive design, and true open-source freedom.
          </p>
        </div>
        <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-none">
          <dl class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-16 lg:max-w-none lg:grid-cols-3">
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">⚙️</span>
                </div>
                Simple Implementation
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  No expert consultants required. Guided setup wizards and comprehensive documentation get you running in weeks, not months.
                </p>
              </dd>
            </div>
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">👥</span>
                </div>
                Business-Friendly Interface
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  Intuitive, modern design built for business users. No IT department needed—your team can use it from day one.
                </p>
              </dd>
            </div>
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">🔗</span>
                </div>
                Seamless Integration
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  Connect easily with your existing tools—e-commerce platforms, payment gateways, marketing tools, and more. No complex integration projects.
                </p>
              </dd>
            </div>
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">🔓</span>
                </div>
                No Vendor Lock-In
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  True open-source with no functional limitations. Self-host or use managed services—your choice. No paywalls or hidden restrictions.
                </p>
              </dd>
            </div>
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">📈</span>
                </div>
                Enterprise-Ready
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  Built to handle enterprise scale confidently. Start small, grow as needed. No need to worry about outgrowing the system.
                </p>
              </dd>
            </div>
            <div class="flex flex-col">
              <dt class="text-base/7 font-semibold text-white">
                <div class="mb-6 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">📚</span>
                </div>
                Comprehensive Support
              </dt>
              <dd class="mt-1 flex flex-auto flex-col text-base/7 text-gray-400">
                <p class="flex-auto">
                  Extensive documentation, training materials, and support options. Community and commercial support available when you need it.
                </p>
              </dd>
            </div>
          </dl>
        </div>
      </div>
    </section>
  );
};

export default WhyRERP;
