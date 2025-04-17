import { Component, For } from 'solid-js';

export interface DepartmentFilterProps {
  departments: string[];
  selectedDepartment: string;
  onDepartmentChange: (department: string) => void;
  label?: string;
}

const DepartmentFilter: Component<DepartmentFilterProps> = (props) => {
  const label = props.label || 'Filter by Department';

  return (
    <div class="bg-white rounded-lg shadow-sm p-4 mb-6">
      <label for="department-filter" class="block text-sm font-medium text-gray-700 mb-2">
        {label}
      </label>
      <select
        id="department-filter"
        value={props.selectedDepartment}
        onChange={(e) => props.onDepartmentChange(e.currentTarget.value)}
        class="w-full md:w-64 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent"
      >
        <For each={props.departments}>
          {(department) => (
            <option value={department}>
              {department === '' ? '-- Select Area of Interest --' : department}
            </option>
          )}
        </For>
      </select>
    </div>
  );
};

export default DepartmentFilter;

