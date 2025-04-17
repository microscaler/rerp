import { Component } from 'solid-js';

export interface BlogPostCardProps {
  slug: string;
  title: string;
  excerpt: string;
  category: string;
  date: string;
  readTime: string;
  icon: string;
}

const BlogPostCard: Component<BlogPostCardProps> = (props) => {
  const categorySlug = props.category.toLowerCase().replace(/\s+/g, '-');

  return (
    <article class="bg-gray-50 rounded-xl overflow-hidden hover:shadow-lg transition-shadow">
      <div class="p-8">
        <div class="flex items-center mb-4">
          <div class="w-10 h-10 bg-primary rounded-lg flex items-center justify-center mr-3">
            <i class={`fa-solid ${props.icon} text-white`}></i>
          </div>
          <a
            href={`#category-${categorySlug}`}
            onClick={(e) => {
              e.preventDefault();
              window.location.hash = `#category-${categorySlug}`;
              window.scrollTo({ top: 0, behavior: 'smooth' });
            }}
            class="text-sm font-medium text-gray-600 hover:text-primary transition-colors"
          >
            {props.category}
          </a>
        </div>
        <h3 class="text-xl font-semibold text-gray-900 mb-3">{props.title}</h3>
        <p class="text-gray-600 mb-4 leading-relaxed">{props.excerpt}</p>
        <div class="flex items-center justify-between text-sm text-gray-500">
          <span>{props.date}</span>
          <span>{props.readTime}</span>
        </div>
        <a
          href={`#blog-${props.slug}`}
          onClick={(e) => {
            e.preventDefault();
            window.location.hash = `#blog-${props.slug}`;
            window.scrollTo({ top: 0, behavior: 'smooth' });
          }}
          class="mt-4 inline-flex items-center text-primary-600 hover:text-primary-700 font-medium text-sm"
        >
          Read more
          <i class="fa-solid fa-arrow-right ml-2"></i>
        </a>
      </div>
    </article>
  );
};

export default BlogPostCard;

