import { Component, For } from 'solid-js';

// SVG Icons
const CloudArrowUpIcon = () => (
  <svg class="absolute top-1 left-1 size-5 text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M9.25 13.25a.75.75 0 001.5 0V4.636l2.955 3.129a.75.75 0 001.09-1.03l-4.25-4.5a.75.75 0 00-1.09 0l-4.25 4.5a.75.75 0 101.09 1.03L9.25 4.636v8.614z" />
    <path d="M3.5 12.75a.75.75 0 00-1.5 0v2.5A2.75 2.75 0 004.75 18h10.5A2.75 2.75 0 0018 15.25v-2.5a.75.75 0 00-1.5 0v2.5c0 .69-.56 1.25-1.25 1.25H4.75c-.69 0-1.25-.56-1.25-1.25v-2.5z" />
  </svg>
);

const LockClosedIcon = () => (
  <svg class="absolute top-1 left-1 size-5 text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M10 1a4.5 4.5 0 00-4.5 4.5V9H5a2 2 0 00-2 2v6a2 2 0 002 2h10a2 2 0 002-2v-6a2 2 0 00-2-2h-.5V5.5A4.5 4.5 0 0010 1zm3 8V5.5a3 3 0 10-6 0V9h6z"
      clip-rule="evenodd"
    />
  </svg>
);

const ServerIcon = () => (
  <svg class="absolute top-1 left-1 size-5 text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path d="M4.632 3.533a2.5 2.5 0 015.736 0 .75.75 0 01-.734.954H5.396a.75.75 0 01-.764-.954zM3 5.5A2.5 2.5 0 015.5 3h9A2.5 2.5 0 0117 5.5v9a2.5 2.5 0 01-2.5 2.5h-9A2.5 2.5 0 013 14.5v-9zM5.5 4a1 1 0 00-1 1v9a1 1 0 001 1h9a1 1 0 001-1v-9a1 1 0 00-1-1h-9zM8 7a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 7zm0 3a.75.75 0 01.75-.75h2.5a.75.75 0 010 1.5h-2.5A.75.75 0 018 10z" />
  </svg>
);

const features = [
  {
    name: 'Simple Implementation',
    description: 'No expert consultants required. Guided setup wizards and comprehensive documentation get you running in weeks, not months.',
    icon: CloudArrowUpIcon,
  },
  {
    name: 'No Vendor Lock-In',
    description: 'True open-source with no functional limitations. Self-host or use managed services—your choice. No paywalls or hidden restrictions.',
    icon: LockClosedIcon,
  },
  {
    name: 'Enterprise-Ready',
    description: 'Built to handle enterprise scale confidently. Start small, grow as needed. No need to worry about outgrowing the system.',
    icon: ServerIcon,
  },
];

const WhyRERPWithImage: Component = () => {
  return (
    <section id="why-rerp" class="overflow-hidden bg-gray-900 py-24 sm:py-32">
      <div class="mx-auto max-w-7xl px-6 lg:px-8">
        <div class="mx-auto grid max-w-2xl grid-cols-1 gap-x-8 gap-y-16 sm:gap-y-20 lg:mx-0 lg:max-w-none lg:grid-cols-2">
          <div class="lg:pt-4 lg:pr-8">
            <div class="lg:max-w-lg">
              <h2 class="text-base/7 font-semibold text-indigo-400">Why RERP?</h2>
              <p class="mt-2 text-4xl font-semibold tracking-tight text-pretty text-white sm:text-5xl">
                Solving the Problems with Current ERPs
              </p>
              <p class="mt-6 text-lg/8 text-gray-300">
                Current ERPs are complex to implement, difficult to use, and create vendor lock-in. RERP solves these problems with simple setup, intuitive design, and true open-source freedom.
              </p>
              <div class="mt-8 max-w-xl text-base/7 text-gray-400 lg:max-w-none">
                <p>
                  Traditional ERP systems require expert consultants and months of setup time. They're built for developers, not business users, making them difficult to use without IT support. Integration with modern tools is often an afterthought, creating silos and inefficiencies.
                </p>
                <p class="mt-6">
                  Even "open-source" solutions create vendor lock-in through hosting models, support contracts, and feature limitations. As your business grows, you face scalability concerns and may need to switch systems entirely—a costly and disruptive process.
                </p>
                <p class="mt-6">
                  RERP changes all of this. Built from the ground up with business users in mind, RERP offers guided setup that gets you running in weeks, not months. The intuitive interface means your team can use it from day one, without extensive training or IT support.
                </p>
              </div>
              <dl class="mt-10 max-w-xl space-y-8 text-base/7 text-gray-400 lg:max-w-none">
                <For each={features}>
                  {(feature) => (
                    <div class="relative pl-9">
                      <dt class="inline font-semibold text-white">
                        <feature.icon />
                        {feature.name}
                      </dt>{' '}
                      <dd class="inline">{feature.description}</dd>
                    </div>
                  )}
                </For>
              </dl>
              <div class="mt-10 max-w-xl text-base/7 text-gray-400 lg:max-w-none">
                <p>
                  With comprehensive documentation, training materials, and support options, RERP ensures you have the resources you need to succeed. Whether you're a startup or an enterprise, RERP grows with you—no need to worry about outgrowing the system or facing migration challenges down the road.
                </p>
              </div>
            </div>
          </div>
          <img
            alt="Business team collaboration"
            src="https://images.unsplash.com/photo-1522071820081-009f0129c71c?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-x=.5&w=2560&h=3413&q=80"
            width={2560}
            height={3413}
            class="w-3xl max-w-none rounded-xl shadow-xl ring-1 ring-white/10 sm:w-228 md:-ml-4 lg:-ml-0"
          />
        </div>
      </div>
    </section>
  );
};

export default WhyRERPWithImage;
