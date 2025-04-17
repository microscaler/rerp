import { Component, createSignal } from 'solid-js';

interface ServiceStatus {
  name: string;
  description: string;
  status: 'operational' | 'degraded' | 'down';
  uptime: number; // percentage
  icon: string;
}

const StatusPage: Component = () => {
  const [services, setServices] = createSignal<ServiceStatus[]>([
    {
      name: 'API Gateway',
      description: 'BRRTrouter-based API gateway and BFF',
      status: 'operational',
      uptime: 99.99,
      icon: 'fa-network-wired',
    },
    {
      name: 'Signal Service',
      description: 'Real-time signal generation and monitoring',
      status: 'operational',
      uptime: 99.95,
      icon: 'fa-bell',
    },
    {
      name: 'Portfolio Service',
      description: 'Portfolio management and P&L tracking',
      status: 'operational',
      uptime: 99.98,
      icon: 'fa-chart-line',
    },
    {
      name: 'System Service',
      description: 'Health monitoring and system control',
      status: 'operational',
      uptime: 100.0,
      icon: 'fa-server',
    },
    {
      name: 'Alert Service',
      description: 'Notifications and alert delivery',
      status: 'operational',
      uptime: 99.92,
      icon: 'fa-envelope',
    },
    {
      name: 'Market Data',
      description: 'Polygon market data ingestion',
      status: 'operational',
      uptime: 99.89,
      icon: 'fa-chart-bar',
    },
    {
      name: 'Ticker Analysis',
      description: 'Real-time ticker pattern recognition and analysis',
      status: 'operational',
      uptime: 99.94,
      icon: 'fa-search',
    },
    {
      name: 'AI Inference',
      description: 'Machine learning model inference and predictions',
      status: 'operational',
      uptime: 99.91,
      icon: 'fa-brain',
    },
    {
      name: 'Options Predictor',
      description: 'Options strategy analysis and prediction engine',
      status: 'operational',
      uptime: 99.87,
      icon: 'fa-chart-pie',
    },
  ]);

  const getStatusColor = (status: ServiceStatus['status']) => {
    switch (status) {
      case 'operational':
        return 'bg-green-500';
      case 'degraded':
        return 'bg-yellow-500';
      case 'down':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getStatusLabel = (status: ServiceStatus['status']) => {
    switch (status) {
      case 'operational':
        return 'Operational';
      case 'degraded':
        return 'Degraded';
      case 'down':
        return 'Down';
      default:
        return 'Unknown';
    }
  };

  const getOverallStatus = () => {
    const serviceList = services();
    const hasDown = serviceList.some(s => s.status === 'down');
    const hasDegraded = serviceList.some(s => s.status === 'degraded');
    
    if (hasDown) return { status: 'down', label: 'System Issues', color: 'bg-red-500' };
    if (hasDegraded) return { status: 'degraded', label: 'Partial Degradation', color: 'bg-yellow-500' };
    return { status: 'operational', label: 'All Systems Operational', color: 'bg-green-500' };
  };

  const overallStatus = getOverallStatus();
  const averageUptime = () => {
    const serviceList = services();
    const total = serviceList.reduce((sum, s) => sum + s.uptime, 0);
    return (total / serviceList.length).toFixed(2);
  };

  return (
    <div class="min-h-screen bg-gray-50">
      <main>
        <div class="pt-8 pb-4">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <a
              href="#"
              onClick={(e) => {
                e.preventDefault();
                window.location.hash = '';
                window.scrollTo({ top: 0, behavior: 'instant' });
              }}
              class="inline-flex items-center space-x-2 text-gray-600 hover:text-primary font-medium transition-colors mb-6"
            >
              <i class="fa-solid fa-arrow-left"></i>
              <span>Back to Home</span>
            </a>
            
            <div class="text-center mb-8">
              <h1 class="text-4xl font-bold text-gray-900 mb-4">System Status</h1>
              <p class="text-xl text-gray-600 max-w-3xl mx-auto mb-6">
                Real-time status of PriceWhisperer services and infrastructure
              </p>
              
              {/* Overall Status Banner */}
              <div class="bg-white rounded-xl shadow-lg p-6 max-w-2xl mx-auto mb-8">
                <div class="flex items-center justify-center space-x-4 mb-4">
                  <div class={`w-4 h-4 ${overallStatus.color} rounded-full`}></div>
                  <span class="text-lg font-semibold text-gray-900">{overallStatus.label}</span>
                </div>
                <p class="text-gray-600">
                  Average Uptime: <span class="font-semibold text-gray-900">{averageUptime()}%</span>
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Services Grid - Similar to HowItWorks */}
        <section class="py-12 bg-gradient-to-br from-gray-50 to-blue-50">
          <div class="max-w-7xl mx-auto px-6 lg:px-8">
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {services().map((service) => (
                <div class="bg-white rounded-xl p-6 shadow-md hover:shadow-lg transition-shadow">
                  <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center space-x-3">
                      <div class="w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center">
                        <i class={`fa-solid ${service.icon} text-gray-700 text-xl`}></i>
                      </div>
                      <div>
                        <h3 class="text-lg font-semibold text-gray-900">{service.name}</h3>
                      </div>
                    </div>
                    {/* Traffic Light Indicator */}
                    <div class="flex flex-col items-center space-y-1">
                      <div class={`w-4 h-4 ${getStatusColor(service.status)} rounded-full`}></div>
                      <span class="text-xs text-gray-500">{getStatusLabel(service.status)}</span>
                    </div>
                  </div>
                  
                  <p class="text-gray-600 text-sm leading-relaxed mb-4">{service.description}</p>
                  
                  <div class="flex items-center justify-between pt-4 border-t border-gray-100">
                    <span class="text-sm text-gray-500">Uptime</span>
                    <span class="text-sm font-semibold text-gray-900">{service.uptime.toFixed(2)}%</span>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </section>

        {/* Footer Note */}
        <div class="max-w-7xl mx-auto px-6 lg:px-8 py-8">
          <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 text-center">
            <p class="text-sm text-gray-600">
              Status is updated in real-time. For detailed incident reports, please visit our{' '}
              <a href="#" class="text-primary hover:text-blue-700 font-medium">status blog</a>.
            </p>
          </div>
        </div>
      </main>
    </div>
  );
};

export default StatusPage;

