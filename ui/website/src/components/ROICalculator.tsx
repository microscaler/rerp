import { Component, createSignal, createMemo } from 'solid-js';

const ROICalculator: Component = () => {
  const [hoursPerWeek, setHoursPerWeek] = createSignal(10);
  const [opportunitiesPerMonth, setOpportunitiesPerMonth] = createSignal(5);
  const [hourlyRate, setHourlyRate] = createSignal(50);

  // Use createMemo to make results reactive - automatically recalculates when signals change
  const results = createMemo(() => {
    const hoursSaved = hoursPerWeek() * 0.75; // 75% time savings
    const timeValue = hoursSaved * 4.33 * hourlyRate(); // Monthly value
    const opportunitiesGained = opportunitiesPerMonth() * 2; // 2x more opportunities
    const avgOpportunityValue = 500; // Average opportunity value
    const opportunityValue = opportunitiesGained * avgOpportunityValue;
    const totalMonthlyValue = timeValue + opportunityValue;
    const monthlyCost = 99; // Professional plan
    const roi = ((totalMonthlyValue - monthlyCost) / monthlyCost) * 100;
    const paybackDays = (monthlyCost / (totalMonthlyValue / 30)).toFixed(1);

    return {
      hoursSaved: hoursSaved.toFixed(1),
      timeValue: timeValue.toFixed(0),
      opportunitiesGained: opportunitiesGained.toFixed(0),
      opportunityValue: opportunityValue.toFixed(0),
      totalMonthlyValue: totalMonthlyValue.toFixed(0),
      roi: roi.toFixed(0),
      paybackDays: paybackDays,
    };
  });

  return (
    <div class="bg-white rounded-2xl shadow-lg p-8 border border-gray-200">
      <div class="text-center mb-8">
        <h3 class="text-2xl font-bold text-gray-900 mb-2">Calculate Your ROI</h3>
        <p class="text-gray-600">See how much PriceWhisperer could save you</p>
      </div>

      <div class="space-y-6 mb-8">
        {/* Hours per week */}
        <div>
          <label class="block text-sm font-semibold text-gray-900 mb-2">
            How many hours do you spend researching trades per week?
          </label>
          <div class="flex items-center space-x-4">
            <input
              type="range"
              min="1"
              max="40"
              value={hoursPerWeek()}
              onInput={(e) => setHoursPerWeek(parseInt(e.currentTarget.value))}
              class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-primary"
            />
            <span class="text-lg font-semibold text-primary w-16 text-right">{hoursPerWeek()} hrs</span>
          </div>
        </div>

        {/* Opportunities per month */}
        <div>
          <label class="block text-sm font-semibold text-gray-900 mb-2">
            How many trading opportunities do you typically find per month?
          </label>
          <div class="flex items-center space-x-4">
            <input
              type="range"
              min="1"
              max="20"
              value={opportunitiesPerMonth()}
              onInput={(e) => setOpportunitiesPerMonth(parseInt(e.currentTarget.value))}
              class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-primary"
            />
            <span class="text-lg font-semibold text-primary w-16 text-right">{opportunitiesPerMonth()}</span>
          </div>
        </div>

        {/* Hourly rate */}
        <div>
          <label class="block text-sm font-semibold text-gray-900 mb-2">
            What's your time worth per hour? (for ROI calculation)
          </label>
          <div class="flex items-center space-x-4">
            <input
              type="range"
              min="25"
              max="200"
              step="25"
              value={hourlyRate()}
              onInput={(e) => setHourlyRate(parseInt(e.currentTarget.value))}
              class="flex-1 h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-primary"
            />
            <span class="text-lg font-semibold text-primary w-20 text-right">${hourlyRate()}/hr</span>
          </div>
        </div>
      </div>

      {/* Results */}
      <div class="bg-gradient-to-br from-primary/10 to-blue-50 rounded-xl p-6 mb-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
          <div class="bg-white rounded-lg p-4">
            <div class="text-sm text-gray-600 mb-1">Time Saved</div>
            <div class="text-2xl font-bold text-primary">{results().hoursSaved} hrs/week</div>
            <div class="text-xs text-gray-500 mt-1">≈ ${results().timeValue}/month value</div>
          </div>
          <div class="bg-white rounded-lg p-4">
            <div class="text-sm text-gray-600 mb-1">More Opportunities</div>
            <div class="text-2xl font-bold text-secondary">+{results().opportunitiesGained}</div>
            <div class="text-xs text-gray-500 mt-1">≈ ${results().opportunityValue}/month value</div>
          </div>
        </div>
        <div class="border-t border-primary/20 pt-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-gray-700 font-semibold">Estimated Monthly Value</span>
            <span class="text-3xl font-bold text-primary">${results().totalMonthlyValue}</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-gray-600 text-sm">Professional Plan Cost</span>
            <span class="text-xl font-semibold text-gray-900">$99/month</span>
          </div>
        </div>
      </div>

      {/* ROI Summary */}
      <div class="bg-green-50 border border-green-200 rounded-lg p-4 mb-6">
        <div class="flex items-center justify-between">
          <div>
            <div class="text-sm text-green-800 font-semibold mb-1">Estimated ROI</div>
            <div class="text-2xl font-bold text-green-700">{results().roi}%</div>
            <div class="text-xs text-green-600 mt-1">Payback in {results().paybackDays} days</div>
          </div>
          <div class="text-4xl">💰</div>
        </div>
      </div>

      <p class="text-xs text-center text-gray-500 mt-4">
        * Estimates based on average user results. Actual results may vary.
      </p>
    </div>
  );
};

export default ROICalculator;

