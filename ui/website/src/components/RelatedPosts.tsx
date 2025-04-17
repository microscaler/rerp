import { Component } from 'solid-js';
import FloatingSidebar from './FloatingSidebar';
import { getRelatedPages } from '../utils/internal-linking';

interface RelatedPostsProps {
  currentSlug: string;
  title?: string;
}

const RelatedPosts: Component<RelatedPostsProps> = (props) => {
  const relatedPages = getRelatedPages(props.currentSlug);

  if (relatedPages.length === 0) {
    return null;
  }

  const handleClick = (url: string, external?: boolean) => {
    if (external) {
      return; // Let browser handle external links
    }
    
    // Track related post click
    if (typeof window !== 'undefined' && (window as any).gtag) {
      (window as any).gtag('event', 'related_post_click', {
        event_category: 'engagement',
        event_label: props.currentSlug,
        related_slug: url.replace('#', '')
      });
    }
  };

  return (
    <FloatingSidebar title={props.title || 'Related Articles'}>
      {relatedPages.map((page) => (
        <a
          href={page.url}
          onClick={(e) => {
            if (!page.external) {
              e.preventDefault();
              window.location.hash = page.url;
              window.scrollTo({ top: 0, behavior: 'smooth' });
            }
            handleClick(page.url, page.external);
          }}
          target={page.external ? '_blank' : undefined}
          rel={page.external ? 'noopener noreferrer' : undefined}
          class="block bg-gray-50 rounded-lg p-4 hover:shadow-md transition-all border border-gray-200 hover:border-primary hover:bg-primary-50"
        >
          <h3 class="text-sm font-semibold text-gray-900 mb-2 hover:text-primary transition-colors line-clamp-2">
            {page.title}
          </h3>
          <p class="text-xs text-gray-600 mb-2 line-clamp-2">{page.description}</p>
          <div class="flex items-center text-primary-600 text-xs font-medium">
            <span>Read more</span>
            <i class={`fa-solid ${page.external ? 'fa-external-link-alt' : 'fa-arrow-right'} ml-2`}></i>
          </div>
        </a>
      ))}
    </FloatingSidebar>
  );
};

export default RelatedPosts;

