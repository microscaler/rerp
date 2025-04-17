import { Component, For } from 'solid-js';

export interface RegionFilterProps {
  regions: string[];
  selectedRegion: string;
  onRegionChange: (region: string) => void;
  label?: string;
}

const RegionFilter: Component<RegionFilterProps> = (props) => {
  const label = props.label || 'Filter by Region';

  return (
    <div class="bg-white rounded-lg shadow-sm p-4 mb-6">
      <label for="region-filter" class="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <select
        id="region-filter"
        value={props.selectedRegion}
        onChange={(e) => props.onRegionChange(e.currentTarget.value)}
        class="w-full md:w-64 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
      >
        <For each={props.regions}>
          {(region) => (
            <option value={region}>
              {region === '' ? '-- Select Region --' : region}
            </option>
          )}
        </For>
      </select>
    </div>
  );
};

export default RegionFilter;

