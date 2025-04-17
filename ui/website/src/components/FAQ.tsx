import { Component, createSignal } from 'solid-js';
import { faqData } from '../data/faq-data';

interface FAQProps {
  standalone?: boolean;
}

const FAQ: Component<FAQProps> = (props) => {
  const [openItems, setOpenItems] = createSignal<Set<string>>(new Set());

  const toggleItem = (id: string) => {
    const newOpen = new Set(openItems());
    if (newOpen.has(id)) {
      newOpen.delete(id);
    } else {
      newOpen.add(id);
    }
    setOpenItems(newOpen);
    
    // Track FAQ engagement
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'faq_interaction', {
        event_category: 'engagement',
        event_label: id
      });
    }
  };

  return (
    <section id="faq" class="py-20 bg-gray-50">
      <div class="max-w-7xl mx-auto px-6 lg:px-8">
        {!props.standalone && (
          <div class="text-center mb-16">
            <h2 class="text-4xl font-bold text-gray-900 mb-4">Frequently Asked Questions</h2>
            <p class="text-xl text-gray-600">
              Everything you need to know about RERP
            </p>
          </div>
        )}
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {faqData.map((faq) => {
            const isOpen = openItems().has(faq.id);
            return (
              <div
                id={faq.id}
                class="bg-white rounded-lg border border-gray-200 overflow-hidden hover:shadow-md transition-shadow"
              >
                <button
                  onClick={() => toggleItem(faq.id)}
                  class="w-full px-5 py-4 text-left flex items-start justify-between hover:bg-gray-50 transition-colors"
                >
                  <span class="font-semibold text-gray-900 pr-4 text-sm leading-tight flex-1">{faq.question}</span>
                  <i
                    class={`fa-solid ${
                      isOpen ? 'fa-chevron-up' : 'fa-chevron-down'
                    } text-primary flex-shrink-0 mt-1`}
                  ></i>
                </button>
                {isOpen && (
                  <div class="px-5 pb-4">
                    <p class="text-gray-600 leading-relaxed text-sm">{faq.answer}</p>
                  </div>
                )}
              </div>
            );
          })}
        </div>
        
        <div class="text-center mt-12">
          <p class="text-gray-600 mb-4">Still have questions?</p>
          <a
            href="https://github.com/microscaler/rerp/discussions"
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center space-x-2 text-primary font-semibold hover:text-blue-700"
          >
            <span>Join GitHub Discussions</span>
            <i class="fa-solid fa-arrow-right"></i>
          </a>
        </div>
      </div>
    </section>
  );
};

export default FAQ;

