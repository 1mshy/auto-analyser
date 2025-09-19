import React from 'react';
import { Filter, DollarSign, TrendingUp, Building, Globe, Calendar } from 'lucide-react';

const FilterPanel = ({ filter, onFilterChange, filterStats }) => {
  const handleInputChange = (field, value) => {
    onFilterChange({
      ...filter,
      [field]: value === '' ? null : value
    });
  };

  const handleNumericChange = (field, value) => {
    onFilterChange({
      ...filter,
      [field]: value === '' ? null : parseFloat(value)
    });
  };

  const formatNumber = (num) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num?.toString() || '0';
  };

  return (
    <div className="bg-white rounded-xl p-6 card-shadow">
      <div className="flex items-center space-x-2 mb-6">
        <Filter className="h-5 w-5 text-blue-600" />
        <h3 className="text-lg font-semibold text-gray-900">Analysis Filters</h3>
      </div>

      {/* Filter Stats Summary */}
      <div className="mb-6 p-4 bg-blue-50 rounded-lg">
        <p className="text-sm text-blue-700 font-medium">
          {formatNumber(filterStats?.filtered_tickers)} of {formatNumber(filterStats?.total_tickers)} stocks match your criteria
        </p>
      </div>

      <div className="space-y-6">
        {/* Price Range */}
        <div>
          <div className="flex items-center space-x-2 mb-3">
            <DollarSign className="h-4 w-4 text-gray-600" />
            <label className="text-sm font-medium text-gray-700">Price Range</label>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-gray-500 mb-1">Min Price</label>
              <input
                type="number"
                value={filter.min_price || ''}
                onChange={(e) => handleNumericChange('min_price', e.target.value)}
                placeholder="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-1">Max Price</label>
              <input
                type="number"
                value={filter.max_price || ''}
                onChange={(e) => handleNumericChange('max_price', e.target.value)}
                placeholder="∞"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        {/* Market Cap Range */}
        <div>
          <div className="flex items-center space-x-2 mb-3">
            <Building className="h-4 w-4 text-gray-600" />
            <label className="text-sm font-medium text-gray-700">Market Cap (Millions)</label>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-gray-500 mb-1">Min Cap</label>
              <input
                type="number"
                value={filter.min_market_cap || ''}
                onChange={(e) => handleNumericChange('min_market_cap', e.target.value)}
                placeholder="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-1">Max Cap</label>
              <input
                type="number"
                value={filter.max_market_cap || ''}
                onChange={(e) => handleNumericChange('max_market_cap', e.target.value)}
                placeholder="∞"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        {/* Volume Range */}
        <div>
          <div className="flex items-center space-x-2 mb-3">
            <TrendingUp className="h-4 w-4 text-gray-600" />
            <label className="text-sm font-medium text-gray-700">Volume Range</label>
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-gray-500 mb-1">Min Volume</label>
              <input
                type="number"
                value={filter.min_volume || ''}
                onChange={(e) => handleInputChange('min_volume', e.target.value === '' ? null : parseInt(e.target.value))}
                placeholder="0"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-1">Max Volume</label>
              <input
                type="number"
                value={filter.max_volume || ''}
                onChange={(e) => handleInputChange('max_volume', e.target.value === '' ? null : parseInt(e.target.value))}
                placeholder="∞"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        {/* RSI Thresholds */}
        <div>
          <label className="text-sm font-medium text-gray-700 mb-3 block">RSI Trading Signals</label>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-gray-500 mb-1">Oversold (&lt;)</label>
              <input
                type="number"
                value={filter.oversold_rsi_threshold || ''}
                onChange={(e) => handleNumericChange('oversold_rsi_threshold', e.target.value)}
                placeholder="30"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-1">Overbought (&gt;)</label>
              <input
                type="number"
                value={filter.overbought_rsi_threshold || ''}
                onChange={(e) => handleNumericChange('overbought_rsi_threshold', e.target.value)}
                placeholder="70"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        {/* Percentage Change */}
        <div>
          <label className="text-sm font-medium text-gray-700 mb-3 block">Daily % Change</label>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="block text-xs text-gray-500 mb-1">Min Change</label>
              <input
                type="number"
                value={filter.min_pct_change || ''}
                onChange={(e) => handleNumericChange('min_pct_change', e.target.value)}
                placeholder="-100"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-1">Max Change</label>
              <input
                type="number"
                value={filter.max_pct_change || ''}
                onChange={(e) => handleNumericChange('max_pct_change', e.target.value)}
                placeholder="100"
                className="w-full px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        {/* Sector Distribution */}
        {filterStats?.sectors && Object.keys(filterStats.sectors).length > 0 && (
          <div>
            <div className="flex items-center space-x-2 mb-3">
              <Building className="h-4 w-4 text-gray-600" />
              <label className="text-sm font-medium text-gray-700">Top Sectors</label>
            </div>
            <div className="space-y-2">
              {Object.entries(filterStats.sectors)
                .sort(([,a], [,b]) => b - a)
                .slice(0, 5)
                .map(([sector, count]) => (
                  <div key={sector} className="flex justify-between text-sm">
                    <span className="text-gray-600 truncate">{sector}</span>
                    <span className="text-gray-900 font-medium">{count}</span>
                  </div>
                ))}
            </div>
          </div>
        )}

        {/* Reset Button */}
        <button
          onClick={() => onFilterChange({
            min_market_cap: null,
            max_market_cap: null,
            min_price: null,
            max_price: null,
            min_volume: null,
            max_volume: null,
            min_pct_change: null,
            max_pct_change: null,
            min_rsi: null,
            max_rsi: null,
            sectors: null,
            countries: null,
            industries: null,
            min_ipo_year: null,
            max_ipo_year: null,
            oversold_rsi_threshold: 30,
            overbought_rsi_threshold: 70
          })}
          className="w-full px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-md text-sm font-medium transition-colors"
        >
          Reset Filters
        </button>
      </div>
    </div>
  );
};

export default FilterPanel;
