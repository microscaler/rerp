import { Component, For } from 'solid-js';

// SVG Icons
const CloudArrowUpIcon = () => (
  <svg class="mt-1 size-5 flex-none text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M9.25 13.25a.75.75 0 001.5 0V4.636l2.955 3.129a.75.75 0 001.09-1.03l-4.25-4.5a.75.75 0 00-1.09 0l-4.25 4.5a.75.75 0 101.09 1.03L9.25 4.636v8.614z" />
    <path d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z" />
  </svg>
);

const LockClosedIcon = () => (
  <svg class="mt-1 size-5 flex-none text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z"
      clip-rule="evenodd"
    />
  </svg>
);

const ServerIcon = () => (
  <svg class="mt-1 size-5 flex-none text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M4.632 3.533a2.5 2.5 0 015.736 0 .75.75 0 01-.734.954H5.396a.75.75 0 01-.764-.954zM3 5.5A2.5 2.5 0 015.5 3h9A2.5 2.5 0 0117 5.5v9a2.5 2.5 0 01-2.5 2.5h-9A2.5 2.5 0 013 14.5v-9zM5.5 4a1 1 0 00-1 1v9a1 1 0 001 1h9a1 1 0 001-1v-9a1 1 0 00-1-1h-9zM8 7a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 7zm0 3a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 10z" />
  </svg>
);

const features = [
  {
    name: 'Simple Setup',
    description: 'Guided implementation without expert consultants. Get up and running in weeks, not months.',
    icon: CloudArrowUpIcon,
  },
  {
    name: 'Easy to Use',
    description: 'Business-friendly interface designed for your team, not just developers.',
    icon: LockClosedIcon,
  },
  {
    name: 'Grows with You',
    description: 'Start small, scale as needed. Enterprise-ready from day one.',
    icon: ServerIcon,
  },
];

const AboutWithImage: Component = () => {
  return (
    <section id="about" class="relative bg-gray-900">
      <div class="mx-auto max-w-7xl lg:flex lg:justify-between lg:px-8 xl:justify-end">
        <div class="lg:flex lg:w-1/2 lg:shrink lg:grow-0 xl:absolute xl:inset-y-0 xl:right-1/2 xl:w-1/2">
          <div class="relative h-80 lg:-ml-8 lg:h-auto lg:w-full lg:grow xl:ml-0">
            <img
              alt="Business operations"
              src="https://images.unsplash.com/photo-1559136555-9303baea8ebd?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.4&w=2560&h=3413&&q=80"
              class="absolute inset-0 size-full bg-gray-800 object-cover"
            />
          </div>
        </div>
        <div class="px-6 lg:contents">
          <div class="mx-auto max-w-2xl pt-16 pb-24 sm:pt-20 sm:pb-32 lg:mr-0 lg:ml-8 lg:w-full lg:max-w-lg lg:flex-none lg:pt-32 xl:w-1/2">
            <p class="text-base/7 font-semibold text-indigo-400">What is RERP?</p>
            <h2 class="mt-2 text-4xl font-semibold tracking-tight text-pretty text-white sm:text-5xl">
              Enterprise Resource Planning, Reimagined
            </h2>
            <p class="mt-6 text-xl/8 text-gray-300">
              RERP is a comprehensive Enterprise Resource Planning system that manages all aspects of your business operations—from finance and accounting to sales, inventory, manufacturing, and human resources.
            </p>
            <div class="mt-10 max-w-xl text-base/7 text-gray-400 lg:max-w-none">
              <p>
                Unlike traditional ERPs that require expert consultants and months of setup, RERP offers simple, guided implementation with a business-friendly interface. Start with what you need, add more as you grow, and integrate seamlessly with your existing tools.
              </p>
              <ul role="list" class="mt-8 space-y-8 text-gray-400">
                <For each={features}>
                  {(feature) => (
                    <li class="flex gap-x-3">
                      <feature.icon />
                      <span>
                        <strong class="font-semibold text-white">{feature.name}.</strong> {feature.description}
                      </span>
                    </li>
                  )}
                </For>
              </ul>
              <p class="mt-8">
                RERP delivers a complete enterprise management platform that unifies all business operations—from finance and sales to manufacturing and HR. Manage all aspects of your business from a single platform—no more disconnected systems or data silos.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default AboutWithImage;
