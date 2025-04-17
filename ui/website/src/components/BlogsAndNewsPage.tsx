import { Component, For } from 'solid-js';
import { BlogPostCard, SectionHeader } from './blogs-news/components';

const BlogsAndNewsPage: Component = () => {
  const blogPosts = [
    {
      slug: 'iron-condor-strategy',
      title: 'Mastering the Iron Condor Options Strategy',
      excerpt: 'Learn how to profit from range-bound markets using Iron Condors. This comprehensive guide covers setup, risk management, and exit strategies.',
      category: 'Options Trading',
      date: 'November 15, 2024',
      readTime: '8 min read',
      icon: 'fa-chart-bar',
    },
    {
      slug: 'pattern-recognition-guide',
      title: 'Chart Pattern Recognition: A Complete Guide',
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


  return (
    <div class="bg-gray-50">
      {/* Blog Section */}
      <section class="py-20 bg-white">
        <div class="max-w-7xl mx-auto px-6 lg:px-8">
          <SectionHeader
            title="Trading Strategies & Insights"
            description="Deep dives into trading strategies, PriceWhisperer features, and market analysis to help you trade smarter."
          />

          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <For each={blogPosts}>
              {(post) => (
                <BlogPostCard
                  slug={post.slug}
                  title={post.title}
                  excerpt={post.excerpt}
                  category={post.category}
                  date={post.date}
                  readTime={post.readTime}
                  icon={post.icon}
                />
              )}
            </For>
          </div>
        </div>
      </section>
    </div>
  );
};

export default BlogsAndNewsPage;
