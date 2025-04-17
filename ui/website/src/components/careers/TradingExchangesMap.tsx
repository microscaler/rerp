import { Component } from 'solid-js';

interface Exchange {
  name: string;
  x: number; // Percentage from left (0-100)
  y: number; // Percentage from top (0-100)
  region: string;
}

const TradingExchangesMap: Component = () => {
  const exchanges: Exchange[] = [
    { name: 'NYSE', x: 25, y: 35, region: 'North America' },
    { name: 'NASDAQ', x: 25, y: 35, region: 'North America' },
    { name: 'CME', x: 25, y: 35, region: 'North America' },
    { name: 'LSE', x: 48, y: 28, region: 'Europe' },
    { name: 'Euronext', x: 50, y: 30, region: 'Europe' },
    { name: 'XETRA', x: 52, y: 28, region: 'Europe' },
    { name: 'TSE', x: 85, y: 32, region: 'Asia Pacific' },
    { name: 'HKEX', x: 80, y: 42, region: 'Asia Pacific' },
    { name: 'SSE', x: 78, y: 38, region: 'Asia Pacific' },
    { name: 'ASX', x: 88, y: 68, region: 'Asia Pacific' },
    { name: 'B3', x: 30, y: 58, region: 'Latin America' },
    { name: 'TSX', x: 22, y: 28, region: 'North America' },
  ];

  return (
    <div class="relative w-full h-[500px] bg-gradient-to-br from-blue-50 to-indigo-100 rounded-lg overflow-hidden border border-gray-200">
      {/* Simplified World Map Background */}
      <svg viewBox="0 0 1000 500" class="absolute inset-0 w-full h-full">
        {/* Simplified continents */}
        <path
          d="M 100 150 Q 150 100 200 120 Q 250 140 300 130 Q 350 120 400 140 Q 450 160 500 150 Q 550 140 600 160 Q 650 180 700 170 Q 750 160 800 180 Q 850 200 900 190 L 900 250 Q 850 260 800 250 Q 750 240 700 250 Q 650 260 600 250 Q 550 240 500 250 Q 450 260 400 250 Q 350 240 300 250 Q 250 260 200 250 Q 150 240 100 250 Z"
          fill="#cbd5e1"
          opacity="0.3"
        />
        <path
          d="M 200 200 Q 250 180 300 200 Q 350 220 400 210 Q 450 200 500 220 Q 550 240 600 230 Q 650 220 700 240 Q 750 260 800 250 L 800 300 Q 750 310 700 300 Q 650 290 600 300 Q 550 310 500 300 Q 450 290 400 300 Q 350 310 300 300 Q 250 290 200 300 Z"
          fill="#cbd5e1"
          opacity="0.3"
        />
      </svg>

      {/* Exchange Markers */}
      {exchanges.map((exchange, index) => (
        <div
          class="absolute transform -translate-x-1/2 -translate-y-1/2 group"
          style={{
            left: `${exchange.x}%`,
            top: `${exchange.y}%`,
          }}
        >
          <div class="relative">
            {/* Outer pulsing ring */}
            <div 
              class="absolute inset-0 animate-ping"
              style={{
                'animation-delay': `${index * 0.3}s`,
                'animation-duration': '2s',
              }}
            >
              <div class="w-6 h-6 bg-primary rounded-full opacity-40 -translate-x-1 -translate-y-1"></div>
            </div>
            {/* Middle pulsing ring */}
            <div 
              class="absolute inset-0 animate-ping"
              style={{
                'animation-delay': `${index * 0.3 + 0.5}s`,
                'animation-duration': '2s',
              }}
            >
              <div class="w-5 h-5 bg-primary rounded-full opacity-50 -translate-x-0.5 -translate-y-0.5"></div>
            </div>
            {/* Main marker - blinking */}
            <div 
              class="relative w-4 h-4 bg-primary rounded-full border-2 border-white shadow-lg animate-pulse"
              style={{
                'animation-duration': '1.5s',
                'animation-delay': `${index * 0.2}s`,
              }}
            >
              <div class="absolute inset-0 bg-primary rounded-full"></div>
            </div>
            {/* Tooltip */}
            <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-gray-900 text-white text-xs rounded whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
              {exchange.name}
              <div class="absolute top-full left-1/2 transform -translate-x-1/2 border-4 border-transparent border-t-gray-900"></div>
            </div>
          </div>
        </div>
      ))}

      {/* Legend */}
      <div class="absolute bottom-4 left-4 bg-white/90 backdrop-blur-sm rounded-lg p-3 shadow-lg">
        <div class="text-sm font-semibold text-gray-900 mb-2">Major Trading Exchanges</div>
        <div class="flex items-center space-x-2 text-xs text-gray-600">
          <div class="w-3 h-3 bg-primary rounded-full animate-pulse"></div>
          <span>Active Exchange</span>
        </div>
      </div>
    </div>
  );
};

export default TradingExchangesMap;

