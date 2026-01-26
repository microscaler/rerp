import { Component, For } from 'solid-js';

// SVG Icons
const CloudArrowUpIcon = () => (
  <svg fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M9.25 13.25a.75.75 0 001.5 0V4.636l2.955 3.129a.75.75 0 001.09-1.03l-4.25-4.5a.75.75 0 00-1.09 0l-4.25 4.5a.75.75 0 101.09 1.03L9.25 4.636v8.614z" />
    <path d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z" />
  </svg>
);

const LockClosedIcon = () => (
  <svg fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z"
      clip-rule="evenodd"
    />
  </svg>
);

const ServerIcon = () => (
  <svg fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M4.632 3.533a2.5 2.5 0 015.736 0 .75.75 0 01-.734.954H5.396a.75.75 0 01-.764-.954zM3 5.5A2.5 2.5 0 015.5 3h9A2.5 2.5 0 0117 5.5v9a2.5 2.5 0 01-2.5 2.5h-9A2.5 2.5 0 013 14.5v-9zM5.5 4a1 1 0 00-1 1v9a1 1 0 001 1h9a1 1 0 001-1v-9a1 1 0 00-1-1h-9zM8 7a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 7zm0 3a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 10z" />
  </svg>
);

const features = [
  {
    name: 'Independent Deployment & Scaling',
    description: 'Deploy updates to individual services without affecting others. Scale high-traffic services independently of low-traffic ones. Reduce risk and optimize resource usage.',
    icon: CloudArrowUpIcon,
  },
  {
    name: 'Integration from Day One',
    description: 'Every service exposes a well-defined OpenAPI specification. Standard RESTful APIs make integration straightforward. Webhooks and event-driven architecture enable real-time integrations.',
    icon: LockClosedIcon,
  },
  {
    name: 'Stable APIs & Migration',
    description: 'APIs remain stable even as implementations evolve. Easy to migrate between cloud providers. Avoid vendor lock-in at the API level.',
    icon: ServerIcon,
  },
];

const ArchitectureWithImage: Component = () => {
  return (
    <section id="architecture" class="relative isolate overflow-hidden bg-gray-900 px-6 py-24 sm:py-32 lg:overflow-visible lg:px-0">
      <div class="absolute inset-0 -z-10 overflow-hidden">
        <svg
          aria-hidden="true"
          class="absolute top-0 left-[max(50%,25rem)] h-256 w-512 -translate-x-1/2 stroke-gray-800 opacity-20"
          style="mask-image: radial-gradient(64rem 64rem at top, white, transparent);"
        >
          <defs>
            <pattern
              x="50%"
              y={-1}
              id="e813992c-7d03-4cc4-a2bd-151760b470a0"
              width={200}
              height={200}
              patternUnits="userSpaceOnUse"
            >
              <path d="M100 200V.5M.5 .5H200" fill="none" />
            </pattern>
          </defs>
          <svg x="50%" y={-1} class="overflow-visible fill-gray-800/20">
            <path
              d="M-100.5 0h201v201h-201Z M699.5 0h201v201h-201Z M499.5 400h201v201h-201Z M-300.5 600h201v201h-201Z"
              stroke-width={0}
            />
          </svg>
          <rect fill="url(#e813992c-7d03-4cc4-a2bd-151760b470a0)" width="100%" height="100%" stroke-width={0} opacity="0.2" />
        </svg>
      </div>
      <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-4 lg:mx-0 lg:max-w-none lg:grid-cols-2 lg:items-start">
        <div class="lg:col-span-2 lg:col-start-1 lg:row-start-1 lg:mx-auto lg:grid lg:w-full lg:max-w-7xl lg:grid-cols-2 lg:gap-x-8 lg:px-8">
          <div class="lg:pr-4">
            <div class="lg:max-w-lg">
              <p class="text-base/7 font-semibold text-indigo-400">For Developers</p>
              <h2 class="mt-2 text-4xl font-semibold tracking-tight text-pretty text-white sm:text-5xl">
                Technical Architecture
              </h2>
              <p class="mt-6 text-xl/8 text-gray-300">
                RERP is built on a modern, API-first microservices architecture that enables independent deployment and scaling while maintaining seamless integration through well-defined APIs.
              </p>
              <div class="mt-8 max-w-xl text-base/7 text-gray-400 lg:max-w-lg">
                <p>
                  Every service in RERP is defined by an OpenAPI 3.1.0 specification, ensuring that APIs are designed first, not as an afterthought. This API-first approach means that integration capabilities are built into every service from day one, not bolted on later.
                </p>
                <p class="mt-6">
                  The microservices architecture enables independent deployment and scaling. You can deploy updates to individual services without affecting the entire system, reducing risk and downtime. High-traffic services like inventory can be scaled independently of low-traffic services like HR, optimizing resource usage and reducing costs.
                </p>
                <p class="mt-6">
                  RERP's architecture maintains seamless integration while enabling independent deployment and scaling. Every service is designed to work independently while seamlessly integrating with others through well-defined APIs that remain stable even as implementations evolve.
                </p>
                <ul role="list" class="mt-8 space-y-8 text-gray-400">
                  <For each={features}>
                    {(feature) => (
                      <li class="flex gap-x-3">
                        <div class="mt-1 size-5 flex-none text-indigo-400">
                          <feature.icon />
                        </div>
                        <span>
                          <strong class="font-semibold text-white">{feature.name}.</strong> {feature.description}
                        </span>
                      </li>
                    )}
                  </For>
                </ul>
                <p class="mt-8">
                  Third-party developers can build on RERP's APIs, creating a marketplace of integrations and extensions. This ecosystem approach enables innovation beyond the core platform, allowing specialized solutions to integrate seamlessly with RERP's foundation.
                </p>
              </div>
            </div>
          </div>
        </div>
        <div class="-mt-12 -ml-12 p-12 lg:col-start-2 lg:row-span-2 lg:row-start-1 lg:overflow-hidden">
          <img
            alt="Software development and architecture"
            src="https://images.unsplash.com/photo-1460925895917-afdab827c52f?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.5&w=2560&h=3413&q=80"
            class="w-3xl max-w-none rounded-xl bg-gray-800 shadow-xl ring-1 ring-white/10 sm:w-228 object-cover"
          />
        </div>
      </div>
    </section>
  );
};

export default ArchitectureWithImage;
