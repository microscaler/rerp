import { Job } from '../components/careers/JobListing';

export const jobsData: Job[] = [
  // North America
  {
    id: 'senior-backend-engineer-ny',
    shortId: '54ygaf5tg',
    title: 'Senior Backend Engineer',
    department: 'Engineering',
    location: 'New York, USA',
    region: 'North America',
    type: 'Full-time',
    description: 'Build scalable microservices and real-time trading data pipelines. Work with high-frequency market data, implement low-latency systems, and ensure 99.9% uptime for our trading platform.',
    requirements: [
      '5+ years experience with Go, Rust, or C++',
      'Experience with real-time data processing (Kafka, Redis, WebSockets)',
      'Strong understanding of financial markets and trading systems',
      'Experience with microservices architecture and distributed systems',
    ],
    postedDate: '2024-01-15',
  },
  {
    id: 'ai-ml-engineer-sf',
    shortId: '7k2m9p4xq',
    title: 'AI/ML Engineer - Pattern Recognition',
    department: 'Engineering',
    location: 'San Francisco, USA',
    region: 'North America',
    type: 'Full-time',
    description: 'Develop and improve our AI-powered pattern detection algorithms. Work on deep learning models for candlestick pattern recognition, sentiment analysis, and predictive trading signals.',
    requirements: [
      '3+ years experience in ML/AI with Python and TensorFlow/PyTorch',
      'Experience with time-series analysis and financial data',
      'Strong background in computer vision or NLP',
      'Published research or open-source contributions preferred',
    ],
    postedDate: '2024-01-20',
  },
  {
    id: 'frontend-engineer-remote-us',
    shortId: '3n8r5w9zv',
    title: 'Frontend Engineer - Trading Platform',
    department: 'Engineering',
    location: 'Remote (USA)',
    region: 'North America',
    type: 'Full-time',
    description: 'Build beautiful, performant trading dashboards and real-time visualization tools. Work with React/TypeScript to create intuitive interfaces for traders managing complex options strategies.',
    requirements: [
      '4+ years experience with React, TypeScript, and modern frontend frameworks',
      'Experience with real-time data visualization (D3.js, Chart.js, WebGL)',
      'Strong UX/UI design sense',
      'Experience with WebSockets and real-time updates',
    ],
    postedDate: '2024-01-18',
  },
  {
    id: 'devops-engineer-toronto',
    shortId: '9h4j6k2lm',
    title: 'DevOps Engineer - Infrastructure',
    department: 'Engineering',
    location: 'Toronto, Canada',
    region: 'North America',
    type: 'Full-time',
    description: 'Manage our global infrastructure, ensure high availability, and optimize for low-latency trading operations. Work with Kubernetes, AWS/GCP, and implement CI/CD pipelines.',
    requirements: [
      '4+ years experience with Kubernetes, Docker, and cloud platforms',
      'Experience with infrastructure as code (Terraform, Ansible)',
      'Strong understanding of networking and security',
      'Experience with monitoring and observability tools',
    ],
    postedDate: '2024-01-22',
  },

  // Europe
  {
    id: 'senior-software-engineer-london',
    shortId: '2b8c4d7fg',
    title: 'Senior Software Engineer - Trading Systems',
    department: 'Engineering',
    location: 'London, UK',
    region: 'Europe',
    type: 'Full-time',
    description: 'Develop high-performance trading systems for European markets. Work on exchange integrations, order execution engines, and risk management systems.',
    requirements: [
      '5+ years experience with Java, C++, or Go',
      'Experience with European exchanges (LSE, Euronext, XETRA)',
      'Strong understanding of FIX protocol and market data feeds',
      'Experience with low-latency systems and performance optimization',
    ],
    postedDate: '2024-01-16',
  },
  {
    id: 'data-engineer-berlin',
    shortId: '5q1t3v6xy',
    title: 'Data Engineer - Market Data',
    department: 'Engineering',
    location: 'Berlin, Germany',
    region: 'Europe',
    type: 'Full-time',
    description: 'Build and maintain our market data ingestion and processing pipelines. Handle terabytes of daily market data from 25+ global exchanges.',
    requirements: [
      '3+ years experience with data engineering (Spark, Kafka, Airflow)',
      'Experience with time-series databases (InfluxDB, TimescaleDB)',
      'Strong SQL and Python skills',
      'Experience with real-time data processing',
    ],
    postedDate: '2024-01-19',
  },
  {
    id: 'product-manager-amsterdam',
    shortId: '8m5n7p2qr',
    title: 'Product Manager - Trading Features',
    department: 'Product',
    location: 'Amsterdam, Netherlands',
    region: 'Europe',
    type: 'Full-time',
    description: 'Lead product development for our trading platform. Work with traders, engineers, and designers to build features that help users find profitable opportunities.',
    requirements: [
      '4+ years product management experience in fintech or trading',
      'Strong understanding of options trading and derivatives',
      'Experience with agile development and user research',
      'Excellent communication and stakeholder management skills',
    ],
    postedDate: '2024-01-21',
  },
  {
    id: 'security-engineer-zurich',
    shortId: '4s9t1u5vw',
    title: 'Security Engineer - Financial Systems',
    department: 'Security',
    location: 'Zurich, Switzerland',
    region: 'Europe',
    type: 'Full-time',
    description: 'Ensure the security and compliance of our trading platform. Implement security controls, conduct audits, and maintain compliance with financial regulations.',
    requirements: [
      '5+ years experience in cybersecurity, preferably in fintech',
      'Experience with financial regulations (MiFID II, GDPR)',
      'Strong knowledge of encryption, authentication, and security best practices',
      'Relevant certifications (CISSP, CISM, etc.) preferred',
    ],
    postedDate: '2024-01-17',
  },

  // Asia Pacific
  {
    id: 'senior-backend-engineer-tokyo',
    shortId: '6x2y4z8ab',
    title: 'Senior Backend Engineer - Exchange Integration',
    department: 'Engineering',
    location: 'Tokyo, Japan',
    region: 'Asia Pacific',
    type: 'Full-time',
    description: 'Integrate with Asian exchanges (TSE, HKEX, SSE) and build low-latency trading systems. Work on real-time market data processing and order execution.',
    requirements: [
      '5+ years experience with backend development (Go, Java, or C++)',
      'Experience with Asian financial markets and exchanges',
      'Strong understanding of market microstructure',
      'Japanese language skills preferred but not required',
    ],
    postedDate: '2024-01-14',
  },
  {
    id: 'quantitative-developer-singapore',
    shortId: '3c7d9e1fg',
    title: 'Quantitative Developer - Trading Algorithms',
    department: 'Engineering',
    location: 'Singapore',
    region: 'Asia Pacific',
    type: 'Full-time',
    description: 'Develop quantitative trading strategies and algorithms. Work with our research team to implement and backtest trading signals and risk models.',
    requirements: [
      '3+ years experience in quantitative finance or algorithmic trading',
      'Strong Python skills (NumPy, Pandas, QuantLib)',
      'Experience with backtesting frameworks and strategy development',
      'Understanding of options pricing and Greeks',
    ],
    postedDate: '2024-01-23',
  },
  {
    id: 'mobile-engineer-sydney',
    shortId: '9h3i5j7kl',
    title: 'Mobile Engineer - iOS/Android',
    department: 'Engineering',
    location: 'Sydney, Australia',
    region: 'Asia Pacific',
    type: 'Full-time',
    description: 'Build native mobile apps for iOS and Android. Create intuitive trading interfaces that allow users to monitor positions and receive alerts on the go.',
    requirements: [
      '4+ years experience with iOS (Swift) or Android (Kotlin) development',
      'Experience with real-time data and WebSocket connections',
      'Strong UI/UX skills',
      'Experience with financial or trading apps preferred',
    ],
    postedDate: '2024-01-24',
  },
  {
    id: 'customer-success-manager-hongkong',
    shortId: '2m6n8o4pq',
    title: 'Customer Success Manager - APAC',
    department: 'Customer Success',
    location: 'Hong Kong',
    region: 'Asia Pacific',
    type: 'Full-time',
    description: 'Help our APAC customers succeed with PriceWhisperer. Provide training, support, and strategic guidance to traders and institutions.',
    requirements: [
      '3+ years experience in customer success or account management',
      'Strong understanding of trading and financial markets',
      'Excellent communication skills in English and Mandarin',
      'Experience with SaaS platforms and technical products',
    ],
    postedDate: '2024-01-25',
  },

  // Latin America
  {
    id: 'backend-engineer-sao-paulo',
    shortId: '7r3s5t9uv',
    title: 'Backend Engineer - Market Data',
    department: 'Engineering',
    location: 'São Paulo, Brazil',
    region: 'Latin America',
    type: 'Full-time',
    description: 'Build and maintain our Latin American market data infrastructure. Integrate with B3 and other regional exchanges.',
    requirements: [
      '3+ years experience with backend development (Go, Python, or Java)',
      'Experience with market data feeds and real-time processing',
      'Understanding of Brazilian financial markets',
      'Portuguese language skills preferred',
    ],
    postedDate: '2024-01-26',
  },
  {
    id: 'sales-engineer-mexico-city',
    shortId: '5w7x1y3za',
    title: 'Sales Engineer - Latin America',
    department: 'Sales',
    location: 'Mexico City, Mexico',
    region: 'Latin America',
    type: 'Full-time',
    description: 'Support our sales team with technical expertise. Help prospects understand our platform capabilities and demonstrate integrations.',
    requirements: [
      '3+ years experience in sales engineering or technical sales',
      'Strong technical background in software or fintech',
      'Excellent presentation and communication skills',
      'Spanish language skills required',
    ],
    postedDate: '2024-01-27',
  },
];

export const regions = ['All Regions', 'North America', 'Europe', 'Asia Pacific', 'Latin America'];

// Extract unique departments from jobs data
export const departments = Array.from(new Set(jobsData.map(job => job.department))).sort();
export const departmentsWithAll = ['All Departments', ...departments];

export const getJobsByRegion = (region: string): Job[] => {
  if (region === 'All Regions') {
    return jobsData;
  }
  return jobsData.filter((job) => job.region === region);
};

export const getJobsByDepartment = (department: string): Job[] => {
  if (department === 'All Departments') {
    return jobsData;
  }
  return jobsData.filter((job) => job.department === department);
};

export const getJobsByRegionAndDepartment = (region: string, department: string): Job[] => {
  let filtered = jobsData;
  
  if (region !== 'All Regions') {
    filtered = filtered.filter((job) => job.region === region);
  }
  
  if (department !== 'All Departments') {
    filtered = filtered.filter((job) => job.department === department);
  }
  
  return filtered;
};

// Lookup job by shortId
export const getJobByShortId = (shortId: string): Job | undefined => {
  return jobsData.find((job) => job.shortId === shortId);
};

