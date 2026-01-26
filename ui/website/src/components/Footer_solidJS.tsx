import { Component, For } from 'solid-js';

// Social Media Icons
const GitHubIcon = (props: { class?: string }) => (
  <svg fill="currentColor" viewBox="0 0 24 24" class={props.class} aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
      clip-rule="evenodd"
    />
  </svg>
);

const navigation = {
  product: [
    { name: 'About', href: '#about' },
    { name: 'Suites', href: '#suites' },
    { name: 'Architecture', href: '#architecture' },
  ],
  support: [
    { name: 'Documentation', href: 'https://github.com/microscaler/rerp' },
    { name: 'GitHub', href: 'https://github.com/microscaler/rerp' },
    { name: 'Roadmap', href: '#roadmap' },
  ],
  company: [
    { name: 'About Us', href: '#about' },
  ],
  legal: [
    { name: 'License', href: 'https://github.com/microscaler/rerp' },
    { name: 'Privacy', href: 'https://github.com/microscaler/rerp' },
  ],
  social: [
    {
      name: 'GitHub',
      href: 'https://github.com/microscaler/rerp',
      icon: GitHubIcon,
    },
  ],
};

const Footer: Component = () => {
  return (
    <footer class="bg-gray-900">
      <div class="mx-auto max-w-7xl px-6 pt-12 pb-8 sm:pt-16 lg:px-8 lg:pt-20">
        <div class="xl:grid xl:grid-cols-3 xl:gap-8">
          <div class="grid grid-cols-2 gap-8 xl:col-span-2">
            <div class="md:grid md:grid-cols-2 md:gap-8">
              <div>
                <h3 class="text-sm/6 font-semibold text-white">Product</h3>
                <ul role="list" class="mt-6 space-y-4">
                  <For each={navigation.product}>
                    {(item) => (
                      <li>
                        <a href={item.href} class="text-sm/6 text-gray-400 hover:text-white">
                          {item.name}
                        </a>
                      </li>
                    )}
                  </For>
                </ul>
              </div>
              <div class="mt-10 md:mt-0">
                <h3 class="text-sm/6 font-semibold text-white">Support</h3>
                <ul role="list" class="mt-6 space-y-4">
                  <For each={navigation.support}>
                    {(item) => (
                      <li>
                        <a href={item.href} class="text-sm/6 text-gray-400 hover:text-white">
                          {item.name}
                        </a>
                      </li>
                    )}
                  </For>
                </ul>
              </div>
            </div>
            <div class="md:grid md:grid-cols-2 md:gap-8">
              <div>
                <h3 class="text-sm/6 font-semibold text-white">Company</h3>
                <ul role="list" class="mt-6 space-y-4">
                  <For each={navigation.company}>
                    {(item) => (
                      <li>
                        <a href={item.href} class="text-sm/6 text-gray-400 hover:text-white">
                          {item.name}
                        </a>
                      </li>
                    )}
                  </For>
                </ul>
              </div>
              <div class="mt-10 md:mt-0">
                <h3 class="text-sm/6 font-semibold text-white">Legal</h3>
                <ul role="list" class="mt-6 space-y-4">
                  <For each={navigation.legal}>
                    {(item) => (
                      <li>
                        <a href={item.href} class="text-sm/6 text-gray-400 hover:text-white">
                          {item.name}
                        </a>
                      </li>
                    )}
                  </For>
                </ul>
              </div>
            </div>
          </div>
          <div class="mt-10 xl:mt-0">
            <h3 class="text-sm/6 font-semibold text-white">Subscribe to our newsletter</h3>
            <p class="mt-2 text-sm/6 text-gray-400">
              The latest news, articles, and resources, sent to your inbox weekly.
            </p>
            <form class="mt-6 sm:flex sm:max-w-md">
              <label for="email-address" class="sr-only">
                Email address
              </label>
              <input
                id="email-address"
                name="email-address"
                type="email"
                required
                placeholder="Enter your email"
                autocomplete="email"
                class="w-full min-w-0 rounded-md bg-white/5 px-3 py-1.5 text-base text-white outline-1 -outline-offset-1 outline-gray-700 placeholder:text-gray-500 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-500 sm:w-64 sm:text-sm/6 xl:w-full"
              />
              <div class="mt-4 sm:mt-0 sm:ml-4 sm:shrink-0">
                <button
                  type="submit"
                  class="flex w-full items-center justify-center rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                >
                  Subscribe
                </button>
              </div>
            </form>
          </div>
        </div>
        <div class="mt-16 border-t border-white/10 pt-8 sm:mt-20 md:flex md:items-center md:justify-between lg:mt-24">
          <div class="flex gap-x-6 md:order-2">
            <For each={navigation.social}>
              {(item) => (
                <a href={item.href} class="text-gray-400 hover:text-white">
                  <span class="sr-only">{item.name}</span>
                  <item.icon class="size-6" aria-hidden="true" />
                </a>
              )}
            </For>
          </div>
          <p class="mt-8 text-sm/6 text-gray-400 md:order-1 md:mt-0">
            &copy; {new Date().getFullYear()} RERP. Open source under PolyForm Shield License.
          </p>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
