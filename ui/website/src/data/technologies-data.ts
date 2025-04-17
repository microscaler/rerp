/**
 * Technologies Data
 * 
 * Single source of truth for all 42 cutting-edge technologies used at PriceWhisperer
 * This data is shared between the word cloud visualization and job application form
 */

export interface Technology {
  name: string;
  score: number; // M×D score (Marketability × Difficulty)
  category: string;
  description: string;
}

// Technologies with M×D scores - weights calculated by rounding UP to nearest Fibonacci
// All technologies with calculated Fibonacci weight ≥ 5 (42 total - the answer to everything!)
export const technologiesRaw: Array<{name: string, score: number, category: string, description: string}> = [
  // Score 104 → rounds up to 144 (not in sequence) → cap at 21
  { name: 'Rust', score: 104, category: 'Language', description: 'Systems programming with memory safety' },
  { name: 'Iceberg', score: 104, category: 'Data Lake', description: 'Open table format for data lakes' },
  { name: 'Fluvio', score: 104, category: 'Streaming', description: 'Modern event streaming platform' },
  { name: 'Transformer Models', score: 104, category: 'ML', description: 'State-of-the-art ML architecture' },
  { name: 'FinBERT', score: 104, category: 'NLP', description: 'Financial sentiment analysis' },
  { name: 'Kubernetes', score: 104, category: 'Infrastructure', description: 'Container orchestration' },
  { name: 'FluxCD', score: 104, category: 'GitOps', description: 'GitOps continuous delivery' },
  { name: 'GitOpsSets', score: 104, category: 'GitOps', description: 'Advanced GitOps resource generation' },
  
  // Score 65 → rounds up to 89 → cap at 21
  { name: 'DuckDB', score: 65, category: 'Analytics', description: 'Embedded analytical database' },
  { name: 'ONNX Runtime', score: 65, category: 'ML', description: 'Cross-platform ML inference' },
  { name: 'PyTorch', score: 65, category: 'ML Framework', description: 'Deep learning framework' },
  { name: 'OpenTelemetry', score: 65, category: 'Observability', description: 'Unified observability standard' },
  { name: 'BRRTrouter', score: 65, category: 'API', description: 'OpenAPI-first BFF' },
  
  // Score 40 → rounds up to 55 → cap at 21
  { name: 'TensorFlow', score: 40, category: 'ML Framework', description: 'Deep learning framework' },
  { name: 'IBKR', score: 40, category: 'Broker', description: 'Interactive Brokers integration' },
  
  // Score 39 → rounds up to 55 → cap at 21
  { name: 'SolidJS', score: 39, category: 'Frontend', description: 'Reactive UI framework' },
  
  // Score 24 → rounds up to 34 → cap at 21
  { name: 'Python', score: 24, category: 'Language', description: 'ML training, data processing' },
  { name: 'Parquet', score: 24, category: 'Storage', description: 'Columnar storage format' },
  { name: 'PostgreSQL', score: 24, category: 'Database', description: 'Advanced relational database' },
  { name: 'Redis', score: 24, category: 'Cache', description: 'In-memory data store' },
  { name: 'XGBoost', score: 24, category: 'ML Framework', description: 'Gradient-boosted trees' },
  { name: 'Docker', score: 24, category: 'Containerization', description: 'Container platform' },
  { name: 'WebSocket', score: 24, category: 'Real-time', description: 'Real-time bidirectional communication' },
  { name: 'WebSocket API', score: 24, category: 'API', description: 'Real-time API' },
  { name: 'WhatsApp API', score: 24, category: 'Messaging', description: 'WhatsApp Business API' },
  { name: 'Signal API', score: 24, category: 'Messaging', description: 'Signal messaging API' },
  { name: 'GitHub Actions', score: 24, category: 'CI/CD', description: 'CI/CD automation' },
  
  // Score 16 → rounds up to 21
  { name: 'Vite', score: 16, category: 'Build Tool', description: 'Next-generation build tool' },
  { name: 'Tailwind CSS', score: 16, category: 'Frontend', description: 'Utility-first CSS framework' },
  { name: 'Polygon.io', score: 16, category: 'Data Provider', description: 'Market data provider' },
  
  // Score 15 → rounds up to 21
  { name: 'Tradier', score: 15, category: 'Broker', description: 'Tradier broker API' },
  { name: 'Alpaca', score: 15, category: 'Broker', description: 'Alpaca broker API' },
  
  // Score 10 → rounds up to 13
  { name: 'AlphaVantage', score: 10, category: 'Data Provider', description: 'Market data provider' },
  { name: 'Finnhub', score: 10, category: 'Data Provider', description: 'Market data provider' },
  
  // Additional cutting-edge technologies (estimated scores based on marketability × difficulty)
  // LightGBM - similar to XGBoost, modern gradient boosting
  { name: 'LightGBM', score: 24, category: 'ML Framework', description: 'Fast gradient boosting framework' },
  
  // Modern data/ML tools with high marketability
  { name: 'WASM SmartModules', score: 65, category: 'Streaming', description: 'WebAssembly modules for Fluvio streaming' },
  { name: 'DuckLake', score: 104, category: 'Data Lake', description: 'Modern data lake architecture' },
  
  // Advanced ML/AI training
  { name: 'LLM Training', score: 104, category: 'ML', description: 'Large Language Model training and fine-tuning' },
  
  // Modern observability (OpenTelemetry is already there, but these are cutting-edge)
  { name: 'eBPF', score: 65, category: 'Observability', description: 'Extended Berkeley Packet Filter for observability' },
  
  // Advanced ML/AI
  { name: 'LLM Integration', score: 65, category: 'ML', description: 'Large Language Model integration' },
  { name: 'Vector Databases', score: 40, category: 'Database', description: 'Vector similarity search for embeddings' },
  
  // Modern real-time processing
  { name: 'Event Sourcing', score: 40, category: 'Architecture', description: 'Event-driven architecture pattern' },
];

// Helper function to get just the technology names (for forms, checkboxes, etc.)
export const getAllTechnologyNames = (): string[] => {
  return technologiesRaw.map(tech => tech.name);
};

