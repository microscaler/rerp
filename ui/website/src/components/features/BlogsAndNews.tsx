import { Component } from 'solid-js';
import FeatureLayout from './FeatureLayout';
import {
  KeyTakeawayBox,
  FeatureCardGrid,
  SectionCardStack,
  InfoBox,
  FeatureList,
  KeyTakeawaysBox
} from '../blog/components';

const BlogsAndNews: Component = () => {
  return (
    <FeatureLayout
      header={{
        title: 'Blog',
        subtitle: 'Stay informed with market insights and trading education',
        icon: 'fa-solid fa-blog',
        iconBgColor: 'bg-indigo-600'
      }}
      showBackLink={true}
    >
          <KeyTakeawayBox color="indigo" title="What You Get" icon="fa-solid fa-rss">
            Access to our blog featuring market analysis, trading strategies, and educational content 
            to help you become a better trader.
          </KeyTakeawayBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Content Categories</h2>
          <FeatureCardGrid
            columns={2}
            items={[
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-indigo-600',
                title: 'Market Analysis',
                description: 'In-depth analysis of current market conditions, sector trends, and opportunities. Understand what\'s driving markets and where the smart money is moving.'
              },
              {
                icon: 'fa-solid fa-lightbulb',
                iconColor: 'text-primary',
                title: 'Trading Strategies',
                description: 'Learn proven trading strategies, from basic setups to advanced options plays. Real examples with entry, stop, and target levels.'
              },
              {
                icon: 'fa-solid fa-graduation-cap',
                iconColor: 'text-secondary',
                title: 'Educational Content',
                description: 'Deep dives into trading concepts, technical indicators, risk management, and market structure. Expand your knowledge continuously.'
              },
              {
                icon: 'fa-solid fa-chart-line',
                iconColor: 'text-accent',
                title: 'Market Insights',
                description: 'Expert analysis of market trends, economic data releases, and regulatory changes that impact trading opportunities.'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">What You'll Find</h2>
          <SectionCardStack
            items={[
              {
                title: 'Weekly Market Recap',
                description: 'Every week, we analyze the most important market moves, breaking down what happened, why it happened, and what it means for traders.',
                metadata: 'Published every Friday',
                metadataIcon: 'fa-solid fa-calendar-week text-indigo-600'
              },
              {
                title: 'Strategy Deep Dives',
                description: 'Detailed explanations of trading strategies with real examples, backtesting results, and step-by-step implementation guides.',
                metadata: 'Comprehensive strategy guides',
                metadataIcon: 'fa-solid fa-book-open text-primary'
              },
              {
                title: 'Options Education',
                description: 'Learn options trading from basics to advanced strategies. Understand Greeks, volatility, and how to structure profitable options trades.',
                metadata: 'From beginner to expert',
                metadataIcon: 'fa-solid fa-chart-bar text-secondary'
              },
              {
                title: 'Case Studies',
                description: 'Real trading examples showing how PriceWhisperer identified opportunities, executed trades, and managed risk. Learn from actual results.',
                metadata: 'Real trades, real results',
                metadataIcon: 'fa-solid fa-clipboard-check text-accent'
              },
              {
                title: 'Market Commentary',
                description: 'Expert commentary on market events, economic data, Fed policy, and how they impact trading opportunities across different asset classes.',
                metadata: 'Expert market insights',
                metadataIcon: 'fa-solid fa-comments text-purple-600'
              }
            ]}
          />

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Stay Updated</h2>
          <InfoBox>
            <FeatureList
              items={[
                { title: 'Email Newsletter', description: 'Get weekly summaries of the best content delivered to your inbox' },
                { title: 'RSS Feed', description: 'Subscribe to our RSS feed for instant updates on new posts' },
                { title: 'Social Media', description: 'Follow us on Twitter and LinkedIn for real-time updates and market commentary' },
                { title: 'In-App Notifications', description: 'Get notified in PriceWhisperer when new educational content is published' }
              ]}
            />
          </InfoBox>

          <h2 class="text-2xl font-bold text-gray-900 mt-8 mb-4">Featured Topics</h2>
          <FeatureCardGrid
            columns={3}
            items={[
              {
                icon: 'fa-solid fa-chart-candlestick',
                iconColor: 'text-blue-600',
                title: 'Technical Analysis',
                description: ''
              },
              {
                icon: 'fa-solid fa-options',
                iconColor: 'text-green-600',
                title: 'Options Trading',
                description: ''
              },
              {
                icon: 'fa-solid fa-shield-alt',
                iconColor: 'text-purple-600',
                title: 'Risk Management',
                description: ''
              },
              {
                icon: 'fa-solid fa-brain',
                iconColor: 'text-orange-600',
                title: 'Market Psychology',
                description: ''
              },
              {
                icon: 'fa-solid fa-chart-pie',
                iconColor: 'text-red-600',
                title: 'Portfolio Management',
                description: ''
              },
              {
                icon: 'fa-solid fa-code',
                iconColor: 'text-indigo-600',
                title: 'Algorithmic Trading',
                description: ''
              }
            ]}
          />

          <KeyTakeawaysBox
            items={[
              <>Stay informed about market conditions and trading opportunities</>,
              <>Learn new strategies and improve your trading skills continuously</>,
              <>Understand market context behind PriceWhisperer's alerts</>,
              <>Access expert analysis and commentary on market events</>,
              <>Learn from real trading examples and case studies</>,
              <>Get educational content that complements our Financial Trading Education program</>
            ]}
            title="Key Benefits"
          />

    </FeatureLayout>
  );
};

export default BlogsAndNews;
