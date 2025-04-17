import type { ResultMetric } from '../components/case-studies/components';

export interface CaseStudyData {
  slug: string;
  title: string;
  customer: string;
  role: string;
  location: string;
  challenge: string;
  solution: string;
  results: ResultMetric[];
  quote: string;
  image: string; // emoji
  excerpt: string; // For preview cards
}

export const caseStudiesData: CaseStudyData[] = [
  {
    slug: 'day-trader-increased-win-rate-40',
    title: 'How a Day Trader Increased Win Rate by 40%',
    customer: 'Michael R.',
    role: 'Day Trader',
    location: 'Chicago, IL',
    challenge: 'Michael was spending 4+ hours daily scanning charts manually, missing profitable opportunities, and struggling with inconsistent results. His win rate hovered around 45%.',
    solution: 'PriceWhisperer\'s AI-powered pattern detection and real-time alerts helped Michael identify high-probability setups in seconds instead of hours. The automated risk management ensured consistent position sizing.',
    results: [
      { metric: 'Win Rate', before: '45%', after: '63%', improvement: '+40%' },
      { metric: 'Time Spent Researching', before: '4+ hours/day', after: '30 minutes/day', improvement: '-87%' },
      { metric: 'Opportunities Found', before: '2-3 per week', after: '8-10 per week', improvement: '+233%' },
    ],
    quote: 'PriceWhisperer transformed my trading. I went from spending hours scanning charts to getting instant alerts on the best setups. My win rate jumped from 45% to 63% in just 3 months.',
    image: '📈',
    excerpt: 'See how Michael increased his win rate from 45% to 63% while reducing research time by 87% using PriceWhisperer\'s AI-powered alerts.',
  },
  {
    slug: 'options-trader-finds-50k-opportunities',
    title: 'Options Trader Finds $50K in Opportunities in 30 Days',
    customer: 'Sarah M.',
    role: 'Options Trader',
    location: 'New York, NY',
    challenge: 'Sarah was manually analyzing options chains, calculating Greeks, and searching for Iron Condor setups. She was only finding 1-2 profitable setups per month.',
    solution: 'PriceWhisperer\'s Options Strategy Finder automatically identified Iron Condor opportunities, analyzed Greeks, and suggested optimal strike prices. The system monitored 50K+ tickers for options flow anomalies.',
    results: [
      { metric: 'Profitable Setups Found', before: '1-2/month', after: '12-15/month', improvement: '+600%' },
      { metric: 'Potential Profit Identified', before: '$5K/month', after: '$50K/month', improvement: '+900%' },
      { metric: 'Time to Find Setup', before: '6-8 hours', after: '< 5 minutes', improvement: '-99%' },
    ],
    quote: 'PriceWhisperer found me 3 profitable Iron Condor setups in my first week. The risk management features alone are worth the subscription. I\'ve increased my win rate by 40% since using it.',
    image: '💰',
    excerpt: 'Discover how Sarah found $50K in profitable options opportunities in just 30 days using PriceWhisperer\'s automated strategy finder.',
  },
  {
    slug: 'swing-trader-saves-15-hours-week',
    title: 'Swing Trader Saves 15 Hours Per Week',
    customer: 'Jennifer L.',
    role: 'Swing Trader',
    location: 'San Francisco, CA',
    challenge: 'Jennifer was manually monitoring 20-30 stocks, reading news, and analyzing charts. She was missing breakout patterns and spending 20+ hours per week on research.',
    solution: 'PriceWhisperer\'s multi-exchange coverage and pattern recognition automatically monitored her watchlist plus thousands more. Sentiment analysis and news aggregation kept her informed without manual research.',
    results: [
      { metric: 'Time Spent Researching', before: '20+ hours/week', after: '5 hours/week', improvement: '-75%' },
      { metric: 'Breakout Patterns Caught', before: '2-3/month', after: '8-10/month', improvement: '+233%' },
      { metric: 'Portfolio Growth', before: '+12% (Q1)', after: '+35% (Q2)', improvement: '+192%' },
    ],
    quote: 'I caught 5 major breakouts before they happened thanks to PriceWhisperer\'s alerts. My portfolio is up 35% this quarter, and I\'m saving 15 hours per week.',
    image: '⏰',
    excerpt: 'Learn how Jennifer reduced her research time by 75% while catching 233% more breakout patterns, leading to 35% portfolio growth.',
  },
  {
    slug: 'professional-trader-monitors-50k-tickers',
    title: 'Professional Trader Monitors 50K+ Tickers Across Global Markets',
    customer: 'David K.',
    role: 'Professional Trader',
    location: 'Austin, TX',
    challenge: 'David needed to monitor opportunities across NYSE, NASDAQ, and international markets but was limited to manually tracking 50-100 stocks. He was missing profitable opportunities on other exchanges and spending hours switching between platforms.',
    solution: 'PriceWhisperer\'s global market coverage unified all exchanges in one dashboard. The API integration allowed David to automate his trading strategies while monitoring 50K+ tickers simultaneously. Real-time alerts kept him informed of opportunities across all markets.',
    results: [
      { metric: 'Tickers Monitored', before: '50-100', after: '50,000+', improvement: '+49,900%' },
      { metric: 'Platforms Required', before: '5+ platforms', after: '1 platform', improvement: '-80%' },
      { metric: 'Opportunities Found', before: '3-5/week', after: '20-30/week', improvement: '+500%' },
    ],
    quote: 'The multi-exchange coverage is a game-changer. I can monitor opportunities across NYSE, NASDAQ, and international markets all in one place. The API integration makes it perfect for my automated strategies.',
    image: '🌍',
    excerpt: 'See how David expanded from monitoring 100 stocks to 50,000+ tickers across global markets using PriceWhisperer\'s unified platform.',
  },
  {
    slug: 'options-specialist-finds-50k-opportunities',
    title: 'Options Specialist Finds $50K in Opportunities Using Strategy Finder',
    customer: 'Emily T.',
    role: 'Options Specialist',
    location: 'Boston, MA',
    challenge: 'Emily was manually searching for Iron Condor, Strangle, and 0-DTE opportunities. She was spending 6-8 hours per day analyzing market conditions and calculating Greeks, only finding 2-3 profitable setups per month.',
    solution: 'PriceWhisperer\'s Options Strategy Finder automatically analyzed market conditions, volatility, and Greeks to suggest the best strategies. The system identified Iron Condors, Strangles, and 0-DTE plays based on real-time market data, saving Emily hours of manual analysis.',
    results: [
      { metric: 'Setups Found', before: '2-3/month', after: '15-20/month', improvement: '+600%' },
      { metric: 'Time Spent Analyzing', before: '6-8 hours/day', after: '< 30 minutes/day', improvement: '-94%' },
      { metric: 'Potential Profit', before: '$8K/month', after: '$50K/month', improvement: '+525%' },
    ],
    quote: 'The options strategy finder is brilliant. It suggests Iron Condors, Strangles, and 0-DTE plays based on market conditions. I\'ve found $50K in opportunities I would have missed.',
    image: '💎',
    excerpt: 'Discover how Emily found $50K in profitable options opportunities while reducing analysis time by 94% using PriceWhisperer\'s automated strategy finder.',
  },
  {
    slug: 'retail-trader-becomes-consistently-profitable',
    title: 'Retail Trader Becomes Consistently Profitable with Education & Risk Management',
    customer: 'Robert P.',
    role: 'Retail Trader',
    location: 'Miami, FL',
    challenge: 'Robert was new to options trading and struggling with inconsistent results. He lacked proper risk management, didn\'t understand position sizing, and was losing money on trades that should have been profitable.',
    solution: 'PriceWhisperer\'s comprehensive Financial Trading Education program taught Robert proper risk management, position sizing, and options strategies. The built-in risk management features automatically calculated position sizes and suggested stop-loss levels, protecting his capital while he learned.',
    results: [
      { metric: 'Win Rate', before: '35%', after: '58%', improvement: '+66%' },
      { metric: 'Monthly P&L', before: '-$2K/month', after: '+$3.5K/month', improvement: '+275%' },
      { metric: 'Risk Per Trade', before: '5-10%', after: '1-2%', improvement: '-75%' },
    ],
    quote: 'As someone new to options trading, PriceWhisperer has been invaluable. The educational content and automated risk management help me trade with confidence. I\'m finally profitable.',
    image: '🎓',
    excerpt: 'Learn how Robert went from losing $2K/month to making $3.5K/month by combining PriceWhisperer\'s education program with automated risk management.',
  },
];

// Map testimonials to case study slugs
export const testimonialToCaseStudy: Record<string, string> = {
  'testimonial-1': 'options-trader-finds-50k-opportunities', // Sarah M.
  'testimonial-2': 'day-trader-increased-win-rate-40', // Michael R.
  'testimonial-3': 'swing-trader-saves-15-hours-week', // Jennifer L.
  'testimonial-4': 'professional-trader-monitors-50k-tickers', // David K.
  'testimonial-5': 'options-specialist-finds-50k-opportunities', // Emily T.
  'testimonial-6': 'retail-trader-becomes-consistently-profitable', // Robert P.
};

export const getCaseStudyBySlug = (slug: string): CaseStudyData | undefined => {
  return caseStudiesData.find(study => study.slug === slug);
};

export const getCaseStudySlugForTestimonial = (testimonialId: string): string | undefined => {
  return testimonialToCaseStudy[testimonialId];
};

