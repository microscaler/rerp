import { Component, createSignal, onMount, createEffect, For } from 'solid-js';
import { updateSEO } from '../utils/seo';
import { getCategorySEO } from '../data/seo-data';

// Blog posts data (should match BlogsAndNewsPage)
const blogPosts = [
  {
    slug: 'iron-condor-strategy',
    title: 'Mastering the Iron Condor Options Strategy',
    excerpt: 'Complete guide to Iron Condor options strategy. Learn how to identify, set up, and manage Iron Condor trades for consistent income generation.',
    category: 'Options Trading',
    date: 'November 15, 2024',
    readTime: '8 min read',
    icon: 'fa-chart-bar',
  },
  {
    slug: 'pattern-recognition-guide',
    title: 'Pattern Recognition Guide: Trading Chart Patterns',
    excerpt: 'Discover how PriceWhisperer uses AI to identify profitable chart patterns. Learn about breakouts, reversals, and consolidation setups.',
    category: 'Technical Analysis',
    date: 'November 12, 2024',
    readTime: '10 min read',
    icon: 'fa-search',
  },
  {
    slug: 'risk-management-basics',
    title: 'Risk Management Fundamentals for Traders',
    excerpt: 'Protect your capital with proper risk management. Learn position sizing, stop-loss placement, and portfolio risk limits.',
    category: 'Risk Management',
    date: 'November 10, 2024',
    readTime: '12 min read',
    icon: 'fa-shield-alt',
  },
  {
    slug: 'options-greeks-explained',
    title: 'Understanding Options Greeks: Delta, Gamma, Theta, Vega',
    excerpt: 'Master the Greeks to become a better options trader. Learn how Delta, Gamma, Theta, and Vega affect your positions.',
    category: 'Options Trading',
    date: 'November 8, 2024',
    readTime: '15 min read',
    icon: 'fa-calculator',
  },
  {
    slug: 'paper-trading-guide',
    title: 'How to Use Paper Trading to Build Confidence',
    excerpt: 'Learn how to test strategies risk-free with paper trading. Build confidence before risking real capital.',
    category: 'Trading Basics',
    date: 'November 5, 2024',
    readTime: '6 min read',
    icon: 'fa-flask',
  },
  {
    slug: 'multi-exchange-trading',
    title: 'Trading Across Global Exchanges: A Complete Guide',
    excerpt: 'Learn how to find opportunities across NYSE, NASDAQ, LSE, and other global exchanges using PriceWhisperer.',
    category: 'Market Analysis',
    date: 'November 3, 2024',
    readTime: '9 min read',
    icon: 'fa-globe',
  },
];

const categoryDescriptions: Record<string, string> = {
  'Options Trading': 'Learn options trading strategies, Greeks, and advanced techniques to improve your options trading performance.',
  'Technical Analysis': 'Master chart patterns, technical indicators, and pattern recognition to identify profitable trading opportunities.',
  'Risk Management': 'Protect your capital with proven risk management strategies, position sizing, and portfolio protection techniques.',
  'Trading Basics': 'Essential trading concepts, tools, and practices for traders of all experience levels.',
  'Market Analysis': 'Understand global markets, multi-exchange trading, and how to find opportunities across different exchanges.',
};

const BlogCategoryPage: Component = () => {
  const [category, setCategory] = createSignal<string>('');

  onMount(() => {
    // Extract category from hash (e.g., #category-options-trading -> Options Trading)
    const hash = window.location.hash;
    if (hash.startsWith('#category-')) {
      const categorySlug = hash.replace('#category-', '').replace(/-/g, ' ');
      // Convert to title case
      const categoryName = categorySlug
        .split(' ')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
      setCategory(categoryName);
    }

    // Listen for hash changes
    const handleHashChange = () => {
      const newHash = window.location.hash;
      if (newHash.startsWith('#category-')) {
        const categorySlug = newHash.replace('#category-', '').replace(/-/g, ' ');
        const categoryName = categorySlug
          .split(' ')
          .map(word => word.charAt(0).toUpperCase() + word.slice(1))
          .join(' ');
        setCategory(categoryName);
      }
    };
    window.addEventListener('hashchange', handleHashChange);
    
    return () => {
      window.removeEventListener('hashchange', handleHashChange);
    };
  });

  // Update SEO when category changes
  createEffect(() => {
    const currentCategory = category();
    if (currentCategory) {
      const seoData = getCategorySEO(currentCategory);
      if (seoData) {
        updateSEO(seoData);
      }
    }
  });

  const categoryPosts = () => {
    const currentCategory = category();
    if (!currentCategory) return [];
    return blogPosts.filter(post => post.category === currentCategory);
  };

  const getCategorySlug = (categoryName: string) => {
    return categoryName.toLowerCase().replace(/\s+/g, '-');
  };

  if (!category()) {
    return (
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        <div class="bg-white rounded-lg shadow-lg p-8 text-center">
          <h1 class="text-3xl font-bold text-gray-900 mb-4">Category Not Found</h1>
          <p class="text-gray-600 mb-6">The category page you're looking for doesn't exist.</p>
          <a
            href="#blogs"
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = '#blogs';
              window.scrollTo({ top: 0, behavior: 'instant' });
            }}
            class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
          >
            <i class="fa-solid fa-arrow-left mr-2"></i>
            Back to Blog
          </a>
        </div>
      </div>
    );
  }

  return (
    <div class="min-h-screen bg-gray-50">
      <main>
        <div class="max-w-7xl mx-auto px-6 lg:px-8 py-12">
          {/* Back Link */}
          <div class="mb-6">
            <a
              href="#blogs"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#blogs';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="inline-flex items-center space-x-2 text-gray-600 hover:text-primary font-medium transition-colors"
            >
              <i class="fa-solid fa-arrow-left"></i>
              <span>Back to Blogs</span>
            </a>
          </div>

          {/* Category Header */}
          <div class="bg-white rounded-lg shadow-lg p-8 mb-8">
            <h1 class="text-4xl font-bold text-gray-900 mb-4">{category()} Articles</h1>
            <p class="text-lg text-gray-600">
              {categoryDescriptions[category()] || `Explore our ${category()} articles and guides.`}
            </p>
            <div class="mt-4 flex items-center space-x-4 text-sm text-gray-500">
              <span>
                <i class="fa-solid fa-file-alt mr-2"></i>
                {categoryPosts().length} {categoryPosts().length === 1 ? 'Article' : 'Articles'}
              </span>
            </div>
          </div>

          {/* Blog Posts Grid */}
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <For each={categoryPosts()}>
              {(post) => (
                <article class="bg-white rounded-xl overflow-hidden hover:shadow-lg transition-shadow border border-gray-200">
                  <div class="p-8">
                    <div class="flex items-center mb-4">
                      <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center mr-3">
                        <i class={`fa-solid ${post.icon} text-white`}></i>
                      </div>
                      <span class="text-sm font-medium text-gray-600">{post.category}</span>
                    </div>
                    <h2 class="text-xl font-semibold text-gray-900 mb-3 hover:text-primary transition-colors">
                      <a
                        href={`#blog-${post.slug}`}
                        onClick={(e) => {
                          e.preventDefault();
                          window.location.hash = `#blog-${post.slug}`;
                          window.scrollTo({ top: 0, behavior: 'smooth' });
                        }}
                      >
                        {post.title}
                      </a>
                    </h2>
                    <p class="text-gray-600 mb-4 leading-relaxed">{post.excerpt}</p>
                    <div class="flex items-center justify-between text-sm text-gray-500 mb-4">
                      <span>{post.date}</span>
                      <span>{post.readTime}</span>
                    </div>
                    <a
                      href={`#blog-${post.slug}`}
                      onClick={(e) => {
                        e.preventDefault();
                        window.location.hash = `#blog-${post.slug}`;
                        window.scrollTo({ top: 0, behavior: 'smooth' });
                      }}
                      class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium text-sm"
                    >
                      Read article
                      <i class="fa-solid fa-arrow-right ml-2"></i>
                    </a>
                  </div>
                </article>
              )}
            </For>
          </div>

          {/* All Categories Link */}
          <div class="mt-12 text-center">
            <a
              href="#blogs"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '#blogs';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="inline-flex items-center text-primary-600 hover:text-primary-700 font-medium"
            >
              <i class="fa-solid fa-list mr-2"></i>
              View All Categories
            </a>
          </div>
        </div>
      </main>
    </div>
  );
};

export default BlogCategoryPage;

