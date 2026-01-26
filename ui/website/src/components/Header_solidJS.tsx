import { Component, createSignal, For } from 'solid-js';
import * as Dialog from '@kobalte/core/dialog';

// SVG Icons (replacing @heroicons/react)
const Bars3Icon = () => (
  <svg class="size-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
  </svg>
);

const XMarkIcon = () => (
  <svg class="size-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
  </svg>
);

const GitHubIcon = () => (
  <svg fill="currentColor" viewBox="0 0 24 24" class="size-5" aria-hidden="true">
    <path
      fill-rule="evenodd"
      d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
      clip-rule="evenodd"
    />
  </svg>
);

const navigation = [
  { name: 'About', href: '#about' },
  { name: 'Why RERP', href: '#why-rerp' },
  { name: 'Suites', href: '#suites' },
  { name: 'Architecture', href: '#architecture' },
];

const Header: Component = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = createSignal(false);

  return (
    <header class="fixed inset-x-0 top-0 z-50 bg-gray-900">
      <nav aria-label="Global" class="mx-auto flex max-w-7xl items-center justify-between gap-x-6 p-6 lg:px-8">
        <div class="flex lg:flex-1">
          <a href="/" class="-m-1.5 p-1.5">
            <span class="sr-only">RERP</span>
            <span class="text-xl font-bold text-white">RERP</span>
          </a>
        </div>
        <div class="hidden lg:flex lg:gap-x-12">
          <For each={navigation}>
            {(item) => (
              <a href={item.href} class="text-sm/6 font-semibold text-white">
                {item.name}
              </a>
            )}
          </For>
        </div>
        <div class="flex flex-1 items-center justify-end">
          <a
            href="https://github.com/microscaler/rerp"
            class="flex items-center gap-x-2 rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
          >
            <GitHubIcon />
            <span>GitHub</span>
          </a>
        </div>
        <div class="flex lg:hidden">
          <button
            type="button"
            onClick={() => setMobileMenuOpen(true)}
            class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-400"
          >
            <span class="sr-only">Open main menu</span>
            <Bars3Icon />
          </button>
        </div>
      </nav>
      <Dialog.Root open={mobileMenuOpen()} onOpenChange={setMobileMenuOpen}>
        <Dialog.Portal>
          <div class="fixed inset-0 z-50" />
          <Dialog.Content class="fixed inset-y-0 right-0 z-50 w-full overflow-y-auto bg-gray-900 p-6 sm:max-w-sm sm:ring-1 sm:ring-gray-100/10">
            <div class="flex items-center gap-x-6">
              <a href="/" class="-m-1.5 p-1.5">
                <span class="sr-only">RERP</span>
                <span class="text-xl font-bold text-white">RERP</span>
              </a>
              <a
                href="https://github.com/microscaler/rerp"
                class="ml-auto flex items-center gap-x-2 rounded-md bg-indigo-500 px-3 py-2 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
              >
                <GitHubIcon />
                <span>GitHub</span>
              </a>
              <Dialog.CloseButton
                type="button"
                onClick={() => setMobileMenuOpen(false)}
                class="-m-2.5 rounded-md p-2.5 text-gray-400"
              >
                <span class="sr-only">Close menu</span>
                <XMarkIcon />
              </Dialog.CloseButton>
            </div>
            <div class="mt-6 flow-root">
              <div class="-my-6 divide-y divide-white/10">
                <div class="space-y-2 py-6">
                  <For each={navigation}>
                    {(item) => (
                      <a
                        href={item.href}
                        class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-white hover:bg-white/5"
                        onClick={() => setMobileMenuOpen(false)}
                      >
                        {item.name}
                      </a>
                    )}
                  </For>
                </div>
                <div class="py-6">
                  <a
                    href="https://github.com/microscaler/rerp"
                    class="-mx-3 flex items-center gap-x-2 rounded-lg px-3 py-2.5 text-base/7 font-semibold text-white hover:bg-white/5"
                  >
                    <GitHubIcon />
                    <span>GitHub</span>
                  </a>
                </div>
              </div>
            </div>
          </Dialog.Content>
        </Dialog.Portal>
      </Dialog.Root>
    </header>
  );
};

export default Header;
