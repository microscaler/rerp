import { Component, JSX } from 'solid-js';

export interface BlogPostHeaderProps {
  category: string;
  title: string;
  date: string;
  readTime: string;
  icon: string;
  iconBgColor?: string;
}

export interface BlogPostLayoutProps {
  header: BlogPostHeaderProps;
  children: JSX.Element;
  showBackLink?: boolean;
}

const BlogPostHeader: Component<BlogPostHeaderProps> = (props) => {
  const iconBgColor = props.iconBgColor || 'bg-primary';
  
  return (
    <div class="mb-8">
      <div class="flex items-center mb-4">
        <div class={`w-12 h-12 ${iconBgColor} rounded-lg flex items-center justify-center mr-4`}>
          <i class={`${props.icon} text-white text-xl`}></i>
        </div>
        <div>
          <div class="text-sm text-gray-500 mb-2">{props.category}</div>
          <h1 class="text-4xl font-extrabold text-gray-900">{props.title}</h1>
          <div class="flex items-center text-gray-500 text-sm mt-2">
            <span>{props.date}</span>
            <span class="mx-2">•</span>
            <span>{props.readTime}</span>
          </div>
        </div>
      </div>
    </div>
  );
};

const BlogPostFooter: Component<{ showBackLink?: boolean }> = (props) => {
  if (!props.showBackLink) return null;
  
  return (
    <div class="mt-12 pt-8 border-t border-gray-200">
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
  );
};

const BlogPostLayout: Component<BlogPostLayoutProps> = (props) => {
  return (
    <article>
      <div class="bg-white rounded-lg shadow-lg p-8 md:p-12">
        <BlogPostHeader {...props.header} />
        
        <div class="prose prose-lg max-w-none">
          {props.children}
        </div>

        <BlogPostFooter showBackLink={props.showBackLink} />
      </div>
    </article>
  );
};

export default BlogPostLayout;

