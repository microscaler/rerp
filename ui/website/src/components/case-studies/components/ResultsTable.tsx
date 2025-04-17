import { Component, For } from 'solid-js';
import { formatPercentageForBrochure } from '../../../utils/format-percentage';
import { abbreviateUnits } from '../../../utils/abbreviate-units';

export interface ResultMetric {
  metric: string;
  before: string;
  after: string;
  improvement: string;
}

export interface ResultsTableProps {
  results: ResultMetric[];
}

const ResultsTable: Component<ResultsTableProps> = (props) => {
  return (
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <For each={props.results}>
        {(result) => {
          const formattedImprovement = formatPercentageForBrochure(result.improvement);
          const abbreviatedAfter = abbreviateUnits(result.after);
          return (
            <div class="bg-white rounded-lg p-5 border border-gray-200 min-w-0 overflow-hidden flex flex-col shadow-sm hover:shadow-md transition-shadow">
              <div class="text-sm font-semibold text-gray-700 mb-3 break-words line-clamp-2 leading-snug">
                {result.metric}
              </div>
              <div class="flex-1 flex flex-col justify-center space-y-2">
                <div class="flex flex-col space-y-1">
                  <div class="flex items-baseline gap-2 flex-wrap">
                    <span class="text-lg font-bold text-gray-900 break-words leading-tight">{abbreviatedAfter}</span>
                    <span class="text-sm text-green-600 font-bold whitespace-nowrap flex-shrink-0 px-2 py-0.5 bg-green-50 rounded">
                      {formattedImprovement}
                    </span>
                  </div>
                  <div class="text-xs text-gray-500 line-through break-words" title={result.before}>
                    Before: {result.before}
                  </div>
                </div>
              </div>
            </div>
          );
        }}
      </For>
    </div>
  );
};

export default ResultsTable;

