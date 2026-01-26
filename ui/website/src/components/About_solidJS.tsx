import { Component, For } from 'solid-js';

const CheckIcon = () => (
  <svg class="mt-1 size-5 flex-none text-indigo-400" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
      clip-rule="evenodd"
    />
  </svg>
);

const About: Component = () => {
  return (
    <section id="about" class="bg-gray-800/25 px-6 py-24 sm:py-32 lg:px-8">
      <div class="mx-auto max-w-7xl">
        <div class="mx-auto max-w-2xl lg:mx-0 lg:max-w-none">
          <p class="text-base/7 font-semibold text-indigo-400">What is RERP?</p>
          <h2 class="mt-2 text-4xl font-semibold tracking-tight text-pretty text-white sm:text-5xl">
            Enterprise Resource Planning, Reimagined
          </h2>
          <p class="mt-6 text-xl/8 text-gray-300">
            RERP is a comprehensive Enterprise Resource Planning system that manages all aspects of your business operations—from finance and accounting to sales, inventory, manufacturing, and human resources.
          </p>
          <p class="mt-8 text-base/7 text-gray-400">
            Unlike traditional ERPs that require expert consultants and months of setup, RERP offers <strong class="font-semibold text-white">simple, guided implementation</strong> with a business-friendly interface. Start with what you need, add more as you grow, and integrate seamlessly with your existing tools.
          </p>
        </div>
        <div class="mt-10 grid max-w-xl grid-cols-1 gap-8 lg:max-w-none lg:grid-cols-2">
          <div class="rounded-3xl bg-gray-800/50 p-8 ring-1 ring-white/15 xl:p-10">
            <h3 class="text-2xl font-semibold tracking-tight text-white">What RERP Will Provide</h3>
            <p class="mt-6 text-lg/8 text-gray-300">
              RERP delivers a <strong class="font-semibold text-white">complete enterprise management platform</strong> that unifies all business operations—from finance and sales to manufacturing and HR.
            </p>
            <div class="mt-8 space-y-6 text-base/7 text-gray-400">
              <div>
                <p class="font-semibold text-white">Unified Business Operations</p>
                <p class="mt-2">Manage all aspects of your business from a single platform—no more disconnected systems or data silos.</p>
              </div>
              <div>
                <p class="font-semibold text-white">Complete Functionality</p>
                <p class="mt-2">Coverage across finance, sales, inventory, manufacturing, HR, and more—everything you need to run your business.</p>
              </div>
              <div>
                <p class="font-semibold text-white">Operational Efficiency</p>
                <p class="mt-2">Streamline workflows, reduce manual processes, and gain real-time visibility into your entire operation.</p>
              </div>
            </div>
          </div>
          <div class="rounded-3xl bg-gray-800/50 p-8 ring-1 ring-white/15 xl:p-10">
            <h3 class="text-2xl font-semibold tracking-tight text-white">The RERP Difference</h3>
            <p class="mt-6 text-base/7 text-gray-400">
              RERP isn't just another ERP—it's built to solve the real problems businesses face with current systems:
            </p>
            <ul role="list" class="mt-8 space-y-4 text-base/7 text-gray-400">
              <For
                each={[
                  { title: 'Simple Setup:', description: 'Guided implementation without expert consultants. Get up and running in weeks, not months.' },
                  { title: 'Easy to Use:', description: 'Business-friendly interface designed for your team, not just developers.' },
                  { title: 'Works with Your Tools:', description: 'Seamless integration with your existing software and workflows.' },
                  { title: 'Grows with You:', description: 'Start small, scale as needed. Enterprise-ready from day one.' },
                ]}
              >
                {(item) => (
                  <li class="flex gap-x-3">
                    <CheckIcon />
                    <span>
                      <strong class="font-semibold text-white">{item.title}</strong> {item.description}
                    </span>
                  </li>
                )}
              </For>
            </ul>
          </div>
        </div>
      </div>
    </section>
  );
};

export default About;
