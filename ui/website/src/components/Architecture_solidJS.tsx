import { Component } from 'solid-js';

const Architecture: Component = () => {
  return (
    <section id="architecture" class="bg-gray-800/25 px-6 py-24 sm:py-32 lg:px-8">
      <div class="mx-auto max-w-7xl">
        <div class="mx-auto max-w-2xl lg:text-center">
          <h2 class="text-base font-semibold leading-7 text-indigo-400">For Developers</h2>
          <p class="mt-2 text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Technical Architecture
          </p>
          <p class="mt-6 text-lg leading-8 text-gray-300">
            RERP is built on a modern, API-first microservices architecture that enables independent development, deployment, and scaling while maintaining type safety and seamless integration.
          </p>
        </div>
        <div class="mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-4xl">
          <dl class="grid max-w-xl grid-cols-1 gap-x-8 gap-y-10 lg:max-w-none lg:grid-cols-2 lg:gap-y-16">
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">1</span>
                </div>
                Independent Development & Deployment
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                Teams can work on different services simultaneously. Deploy updates to individual services without affecting others. Faster iteration cycles and reduced risk.
              </dd>
            </div>
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">2</span>
                </div>
                Independent Scaling
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                Scale high-traffic services (e.g., inventory) independently of low-traffic services (e.g., HR). Optimize resource usage and reduce costs.
              </dd>
            </div>
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">3</span>
                </div>
                Integration from Day One
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                Every service exposes a well-defined OpenAPI specification. Standard RESTful APIs make integration straightforward. Webhooks and event-driven architecture enable real-time integrations.
              </dd>
            </div>
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">4</span>
                </div>
                Developer Experience
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                Auto-generated client libraries from OpenAPI specs. Type-safe integrations reduce bugs. Comprehensive API documentation always up-to-date.
              </dd>
            </div>
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">5</span>
                </div>
                Ecosystem Building
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                Third-party developers can build on RERP's APIs. Marketplace of integrations and extensions. Platform approach enables innovation beyond core team.
              </dd>
            </div>
            <div class="relative pl-16">
              <dt class="text-base/7 font-semibold text-white">
                <div class="absolute top-0 left-0 flex size-10 items-center justify-center rounded-lg bg-indigo-500">
                  <span class="text-white text-xl">6</span>
                </div>
                Future-Proofing
              </dt>
              <dd class="mt-2 text-base/7 text-gray-400">
                APIs remain stable even as implementations evolve. Easy to migrate between cloud providers. Avoid vendor lock-in at the API level.
              </dd>
            </div>
          </dl>
        </div>
        <div class="mx-auto mt-16 max-w-2xl lg:mt-24">
          <div class="relative isolate overflow-hidden bg-gray-800/50 px-6 py-20 after:pointer-events-none after:absolute after:inset-0 after:inset-ring after:inset-ring-white/10 sm:rounded-3xl sm:px-10 sm:py-24 after:sm:rounded-3xl lg:py-24 xl:px-24">
            <h3 class="text-xl font-semibold text-white">What This Enables</h3>
            <ul class="mt-6 space-y-4 text-base/7 text-gray-300">
              <li class="flex gap-x-3">
                <span class="text-indigo-400">✓</span>
                <span><strong class="font-semibold text-white">For Businesses:</strong> Start small, scale smart. Begin with core services, add more as needed. Best-of-breed integration with specialized tools.</span>
              </li>
              <li class="flex gap-x-3">
                <span class="text-indigo-400">✓</span>
                <span><strong class="font-semibold text-white">For Developers:</strong> Rapid development through OpenAPI code generation. Type safety catches errors at compile time. Modern tooling and standard REST APIs.</span>
              </li>
              <li class="flex gap-x-3">
                <span class="text-indigo-400">✓</span>
                <span><strong class="font-semibold text-white">For the Ecosystem:</strong> App marketplace for third-party extensions. Integration hub with pre-built connectors. Industry-specific solutions built on RERP's foundation.</span>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Architecture;
