import React, { useState } from 'react';
import { TrendingUp, TrendingDown, AlertTriangle, DollarSign, Activity, BarChart3, Clock, Wifi, WifiOff } from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';

const AnalysisResults = ({ results, continuousStatus, isConnected }) => {
  const [sortBy, setSortBy] = useState('rsi');
  const [filterType, setFilterType] = useState('all'); // all, opportunities, oversold, overbought

  if (!results || results.length === 0) {
    return (
      <div className="bg-white rounded-xl p-8 card-shadow text-center">
        <div className="flex items-center justify-center mb-4">
          {isConnected ? (
            <Wifi className="h-12 w-12 text-blue-400" />
          ) : (
            <WifiOff className="h-12 w-12 text-red-400" />
          )}
        </div>
        <h3 className="text-lg font-medium text-gray-900 mb-2">
          {isConnected ? 'Waiting for Analysis Data' : 'Connection Lost'}
        </h3>
        <p className="text-gray-600">
          {isConnected 
            ? 'The server is continuously analyzing stocks. Results will appear here.'
            : 'Reconnecting to the analysis server...'
          }
        </p>
        {continuousStatus && (
          <div className="mt-4 text-sm text-gray-500">
            {continuousStatus.is_running 
              ? `Currently analyzing: ${continuousStatus.analyzed_count}/${continuousStatus.total_count}`
              : `Last analysis completed: ${continuousStatus.analyzed_count} stocks analyzed`
            }
          </div>
        )}
      </div>
    );
  }

  const filteredResults = results.filter(stock => {
    switch (filterType) {
      case 'opportunities':
        return stock.is_opportunity;
      case 'oversold':
        return stock.rsi && stock.rsi <= 30;
      case 'overbought':
        return stock.rsi && stock.rsi >= 70;
      default:
        return true;
    }
  });

  const sortedResults = [...filteredResults].sort((a, b) => {
    switch (sortBy) {
      case 'rsi':
        return (a.rsi || 0) - (b.rsi || 0);
      case 'price':
        return (a.current_price || 0) - (b.current_price || 0);
      case 'volume':
        return (b.volume || 0) - (a.volume || 0);
      case 'change':
        return (b.pct_change || 0) - (a.pct_change || 0);
      default:
        return 0;
    }
  });

  const getRSIColor = (rsi) => {
    if (!rsi) return 'text-gray-400';
    if (rsi <= 30) return 'text-green-600';
    if (rsi >= 70) return 'text-red-600';
    return 'text-yellow-600';
  };

  const getRSIBgColor = (rsi) => {
    if (!rsi) return 'bg-gray-100';
    if (rsi <= 30) return 'bg-green-100';
    if (rsi >= 70) return 'bg-red-100';
    return 'bg-yellow-100';
  };

  const formatCurrency = (value) => {
    if (!value) return 'N/A';
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(value);
  };

  const formatVolume = (volume) => {
    if (!volume) return 'N/A';
    if (volume >= 1000000) return `${(volume / 1000000).toFixed(1)}M`;
    if (volume >= 1000) return `${(volume / 1000).toFixed(1)}K`;
    return volume.toString();
  };

  // Prepare chart data
  const rsiDistribution = results.reduce((acc, stock) => {
    if (stock.rsi) {
      const bucket = Math.floor(stock.rsi / 10) * 10;
      acc[bucket] = (acc[bucket] || 0) + 1;
    }
    return acc;
  }, {});

  const chartData = rsiDistribution ? 
    Object.entries(rsiDistribution).map(([range, count]) => ({
      range: `${range}-${parseInt(range) + 9}`,
      count,
      name: `RSI ${range}-${parseInt(range) + 9}`
    })).sort((a, b) => parseInt(a.range) - parseInt(b.range)) : [];

  return (
    <div className="space-y-6">
      {/* Header with Controls */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between space-y-4 sm:space-y-0">
          <div>
            <h2 className="text-xl font-bold text-gray-900">Analysis Results</h2>
            <p className="text-gray-600">
              {continuousStatus?.is_running ? 'Continuous analysis running...' : 'Analysis data from server'}
            </p>
          </div>
          
          <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-4">
            <select
              value={filterType}
              onChange={(e) => setFilterType(e.target.value)}
              className="px-4 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="all">All Stocks</option>
              <option value="opportunities">Opportunities Only</option>
              <option value="oversold">Oversold (RSI ≤ 30)</option>
              <option value="overbought">Overbought (RSI ≥ 70)</option>
            </select>
            
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value)}
              className="px-4 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="rsi">Sort by RSI</option>
              <option value="price">Sort by Price</option>
              <option value="volume">Sort by Volume</option>
              <option value="change">Sort by % Change</option>
            </select>
          </div>
        </div>
      </div>

      {/* RSI Distribution Chart */}
      {chartData.length > 0 && (
        <div className="bg-white rounded-xl p-6 card-shadow">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">RSI Distribution</h3>
          <ResponsiveContainer width="100%" height={200}>
            <BarChart data={chartData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="range" />
              <YAxis />
              <Tooltip />
              <Bar dataKey="count" fill="#3b82f6" />
            </BarChart>
          </ResponsiveContainer>
        </div>
      )}

      {/* Results Table */}
      <div className="bg-white rounded-xl card-shadow overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900">
            Stock Analysis ({sortedResults.length} stocks)
          </h3>
        </div>
        
        {sortedResults.length === 0 ? (
          <div className="p-8 text-center">
            <BarChart3 className="h-12 w-12 text-gray-400 mx-auto mb-4" />
            <p className="text-gray-600">
              {continuousStatus?.is_running ? 'Analyzing stocks...' : 'No results match your current filters'}
            </p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Stock
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Price
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    RSI
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    SMA 20/50
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Volume
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Change %
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Signals
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {sortedResults.map((stock, index) => (
                  <tr key={`${stock.ticker}-${index}`} className={stock.is_opportunity ? 'bg-green-50' : ''}>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div>
                        <div className="text-sm font-medium text-gray-900">{stock.ticker}</div>
                        <div className="text-sm text-gray-500 truncate max-w-xs">{stock.name}</div>
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="text-sm text-gray-900">{formatCurrency(stock.current_price)}</div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {stock.rsi ? (
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getRSIBgColor(stock.rsi)} ${getRSIColor(stock.rsi)}`}>
                          {stock.rsi.toFixed(1)}
                        </span>
                      ) : (
                        <span className="text-gray-400">N/A</span>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      <div>
                        {stock.sma_20 ? formatCurrency(stock.sma_20) : 'N/A'} / {stock.sma_50 ? formatCurrency(stock.sma_50) : 'N/A'}
                      </div>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {formatVolume(stock.volume)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {stock.pct_change ? (
                        <span className={`text-sm font-medium ${stock.pct_change >= 0 ? 'text-green-600' : 'text-red-600'}`}>
                          {stock.pct_change > 0 ? '+' : ''}{stock.pct_change.toFixed(2)}%
                        </span>
                      ) : (
                        <span className="text-gray-400">N/A</span>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <div className="flex flex-wrap gap-1">
                        {stock.signals.map((signal, i) => (
                          <span
                            key={i}
                            className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                              signal.includes('Buy') ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                            }`}
                          >
                            {signal.includes('Buy') ? <TrendingUp className="h-3 w-3 mr-1" /> : <TrendingDown className="h-3 w-3 mr-1" />}
                            {signal.split(' - ')[1] || signal}
                          </span>
                        ))}
                        {stock.is_opportunity && stock.signals.length === 0 && (
                          <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                            <AlertTriangle className="h-3 w-3 mr-1" />
                            Opportunity
                          </span>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {/* Loading state for running analysis */}
      {continuousStatus?.is_running && (
        <div className="bg-white rounded-xl p-6 card-shadow">
          <div className="flex items-center justify-center space-x-4">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            <div>
              <p className="text-sm font-medium text-gray-900">Continuous Analysis Running</p>
              <p className="text-sm text-gray-600">
                {continuousStatus.analyzed_count > 0 && `Processing ${continuousStatus.analyzed_count}/${continuousStatus.total_count} stocks (Cycle ${continuousStatus.current_cycle})`}
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AnalysisResults;
