import { Component, onMount } from 'solid-js';
import { technologiesRaw } from '../../data/technologies-data';

// WordCloud will be loaded dynamically from CDN
declare global {
  interface Window {
    WordCloud: any;
  }
}

interface Technology {
  name: string;
  weight: number; // Fibonacci weight (rounded up from M×D score)
  category: string;
  description?: string;
  score: number; // M×D score (Marketability × Difficulty)
}

// Helper function to round M×D score up to nearest Fibonacci number
function scoreToFibonacci(score: number): number {
  // Fibonacci sequence: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89
  const fib = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
  
  // Find the smallest Fibonacci number >= score
  for (const f of fib) {
    if (f >= score) return f;
  }
  
  // If score exceeds all, return max (21 for display purposes)
  return 21;
}

const TechnologiesWordCloud: Component = () => {
  // Technologies data imported from shared data file
  // Calculate Fibonacci weights by rounding up M×D scores
  // Filter to only include weight ≥ 5
  const technologies: Technology[] = technologiesRaw
    .map(tech => ({
      ...tech,
      weight: scoreToFibonacci(tech.score)
    }))
    .filter(tech => tech.weight >= 5)
    .slice(0, 42); // Ensure exactly 42 items

  onMount(() => {
    // Load wordcloud from CDN
    if (window.WordCloud) {
      renderWordCloud();
      return;
    }

    const script = document.createElement('script');
    script.src = 'https://cdn.jsdelivr.net/npm/wordcloud@1.2.3/src/wordcloud2.min.js';
    script.onload = () => {
      if (window.WordCloud) {
        renderWordCloud();
      }
    };
    script.onerror = () => {
      console.error('Failed to load wordcloud library');
    };
    document.head.appendChild(script);

    function renderWordCloud() {
      const canvas = document.getElementById('technologies-wordcloud') as HTMLCanvasElement;
      if (!canvas || !window.WordCloud) return;

      // Set canvas size - responsive, larger to fit all 42 technologies
      const container = canvas.parentElement;
      if (container) {
        const rect = container.getBoundingClientRect();
        const width = Math.max(rect.width || 1400, 1200); // Larger width for 42 items
        const height = 700; // Increased height to fit all 42 words
        canvas.width = width;
        canvas.height = height;
        // Set display size for high DPI screens
        canvas.style.width = `${width}px`;
        canvas.style.height = `${height}px`;
      }

      // Map technologies to wordcloud format: [word, weight]
      // Use Fibonacci weights directly from scoring document: 5, 8, 13, 21
      const words = technologies.map(tech => [tech.name, tech.weight] as [string, number]);
      
      // Debug: Log to ensure all 42 are included
      console.log(`WordCloud: Rendering ${words.length} technologies`);
      console.log('Technologies with weights:', words.map(w => `${w[0]} (${w[1]})`).join(', '));
      
      const colors: Record<string, string> = {
        'Language': '#ef4444',      // Red
        'ML': '#8b5cf6',           // Purple
        'Analytics': '#06b6d4',     // Cyan
        'Data Lake': '#10b981',     // Green
        'Streaming': '#f59e0b',    // Amber
        'Storage': '#6366f1',      // Indigo
        'Infrastructure': '#ec4899', // Pink
        'Observability': '#14b8a6', // Teal
        'GitOps': '#f97316',       // Orange
        'Frontend': '#3b82f6',     // Blue
        'API': '#06b6d4',          // Cyan
        'Real-time': '#8b5cf6',    // Purple
        'NLP': '#ec4899',          // Pink
        'ML Framework': '#8b5cf6', // Purple
        'Monitoring': '#10b981',    // Green
        'Visualization': '#f59e0b', // Amber
        'Logging': '#6366f1',       // Indigo
        'Tracing': '#14b8a6',       // Teal
        'Database': '#3b82f6',     // Blue
        'Cache': '#ef4444',         // Red
        'Build Tool': '#f97316',    // Orange
        'Containerization': '#06b6d4', // Cyan
        'API Spec': '#8b5cf6',     // Purple
        'Broker': '#10b981',       // Green
        'Messaging': '#ec4899',    // Pink
        'CI/CD': '#f97316',        // Orange
        'Data Provider': '#06b6d4', // Cyan
        'Version Control': '#6366f1', // Indigo
        'Documentation': '#14b8a6', // Teal
        'Orchestration': '#8b5cf6', // Purple
        'Web Server': '#f59e0b',   // Amber
        'Dev Tool': '#f97316',     // Orange
        'Architecture': '#06b6d4', // Cyan
      };

      try {
        window.WordCloud(canvas, {
          list: words,
          gridSize: 3, // Very small grid to fit all 42 words
          weightFactor: (size: number) => {
            // size parameter is the Fibonacci weight (5, 8, 13, or 21)
            // Map Fibonacci weights to font sizes maintaining their ratio
            // Fibonacci ratio: 5:8:13:21 = 1:1.6:2.6:4.2
            const baseFontSize = 16; // Base size for weight 5
            // Direct mapping: weight 5 = base, weight 8 = 1.6x, weight 13 = 2.6x, weight 21 = 4.2x
            const fontSize = (size / 5) * baseFontSize;
            // Scale for canvas width, but keep reasonable sizes
            return fontSize * (canvas.width / 1400) * 0.8;
          },
          fontFamily: 'Inter, system-ui, -apple-system, sans-serif',
          color: (word: string) => {
            const tech = technologies.find(t => t.name === word);
            return tech ? (colors[tech.category] || '#3b82f6') : '#3b82f6';
          },
          rotateRatio: 0.5, // More rotation to fit better
          rotationSteps: 2,
          shuffle: false,
          shape: 'circle',
          ellipticity: 0.8, // More elliptical to fit more words
          backgroundColor: 'transparent',
          minSize: 5, // Very small min size to ensure all words fit
          drawOutOfBound: false,
          wait: 0, // Render immediately
        }, (items: any[], bounds: any) => {
          // Callback after rendering - log how many items were placed
          console.log(`WordCloud rendered: ${items.length} items placed out of ${words.length} total`);
          const placedNames = items.map((item: any) => item[0] || item.text || 'unknown').join(', ');
          console.log('Placed technologies:', placedNames);
          if (items.length < words.length) {
            const missing = words.filter(w => !items.some((item: any) => (item[0] || item.text) === w[0]));
            console.warn(`Warning: Missing ${words.length - items.length} technologies:`, missing.map(w => w[0]));
          }
        });

        // Add hover tooltips
        canvas.style.cursor = 'pointer';
        canvas.title = 'Hover over technologies to learn more';
      } catch (error) {
        console.error('Error rendering word cloud:', error);
      }
    }
  });

  return (
    <div class="relative w-full bg-gradient-to-br from-gray-50 to-blue-50 rounded-lg overflow-hidden border border-gray-200 p-8">
      <div class="flex flex-col items-center justify-center min-h-[700px]">
        <canvas
          id="technologies-wordcloud"
          class="max-w-full h-auto"
          style={{ width: '100%', height: '700px' }}
        ></canvas>
      </div>
    </div>
  );
};

export default TechnologiesWordCloud;
