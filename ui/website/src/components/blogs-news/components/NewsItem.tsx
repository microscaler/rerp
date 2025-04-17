import { Component } from 'solid-js';

export interface NewsItemData {
  ticker: string;
  company: string;
  headline: string;
  summary: string;
  date: string;
  change: string;
  changeType: 'positive' | 'negative';
}

export interface NewsItemProps {
  item: NewsItemData;
  isActive?: boolean;
}

const NewsItem: Component<NewsItemProps> = (props) => {
  return (
    <div
      class={`absolute inset-0 transition-opacity duration-500 ${
        props.isActive ? 'opacity-100' : 'opacity-0'
      }`}
    >
      <div class="h-full flex items-center p-8 md:p-12">
        <div class="w-full">
          <div class="flex items-center mb-4">
            <div class="bg-primary text-white px-4 py-2 rounded-lg font-bold text-xl mr-4">
              {props.item.ticker}
            </div>
            <div>
              <div class="text-gray-900 font-semibold text-lg">{props.item.company}</div>
              <div class="text-gray-500 text-sm">{props.item.date}</div>
            </div>
            <div class="ml-auto">
              <div
                class={`text-2xl font-bold ${
                  props.item.changeType === 'positive' ? 'text-green-600' : 'text-red-600'
                }`}
              >
                {props.item.change}
              </div>
            </div>
          </div>
          <h2 class="text-3xl font-bold text-gray-900 mb-4">{props.item.headline}</h2>
          <p class="text-lg text-gray-600">{props.item.summary}</p>
        </div>
      </div>
    </div>
  );
};

export default NewsItem;

