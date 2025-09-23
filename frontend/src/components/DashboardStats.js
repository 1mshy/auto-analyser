import React from 'react';
import { TrendingUp, TrendingDown, DollarSign, BarChart3, Activity, Wifi } from 'lucide-react';

const DashboardStats = ({ continuousStatus, filterStats, filteredResultsCount, isConnected, lastResultsUpdate }) => {
  const formatNumber = (num) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num?.toString() || '0';
  };

  const getOpportunityRate = () => {
    if (!continuousStatus || continuousStatus.analyzed_count === 0) return 0;
    return ((continuousStatus.opportunities_found / continuousStatus.analyzed_count) * 100).toFixed(1);
  };

  const formatLastUpdate = () => {
    if (!continuousStatus?.last_update) return 'Never';
    const date = new Date(continuousStatus.last_update);
    return date.toLocaleTimeString();
  };

  const formatResultsUpdate = () => {
    if (!lastResultsUpdate) return 'Never';
    return lastResultsUpdate.toLocaleTimeString();
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {/* Analysis Progress */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Continuous Analysis</p>
            <p className="text-2xl font-bold text-gray-900">
              {continuousStatus ? `${continuousStatus.analyzed_count}/${continuousStatus.total_count}` : '0/0'}
            </p>
          </div>
          <div className={`p-3 rounded-full ${continuousStatus?.is_running ? 'bg-blue-100' : 'bg-gray-100'}`}>
            <Activity className={`h-6 w-6 ${continuousStatus?.is_running ? 'text-blue-600' : 'text-gray-600'}`} />
          </div>
        </div>
        {continuousStatus && (
          <div className="mt-4">
            <div className="flex justify-between text-sm text-gray-600 mb-1">
              <span>Cycle {continuousStatus.current_cycle}</span>
              <span>{(continuousStatus.progress * 100).toFixed(1)}%</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div
                className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                style={{ width: `${continuousStatus.progress * 100}%` }}
              ></div>
            </div>
            <div className="mt-2 text-xs text-gray-500">
              Last update: {formatLastUpdate()}
            </div>
          </div>
        )}
      </div>

      {/* Opportunities Found */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Opportunities Found</p>
            <p className="text-2xl font-bold text-green-600">
              {continuousStatus?.opportunities_found || 0}
            </p>
          </div>
          <div className="p-3 bg-green-100 rounded-full">
            <TrendingUp className="h-6 w-6 text-green-600" />
          </div>
        </div>
        <div className="mt-4">
          <span className="text-sm text-gray-600">
            Success Rate: <span className="font-medium text-green-600">{getOpportunityRate()}%</span>
          </span>
        </div>
      </div>

      {/* Filtered Results */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Filtered Results</p>
            <p className="text-2xl font-bold text-purple-600">
              {formatNumber(filteredResultsCount)}
            </p>
          </div>
          <div className="p-3 bg-purple-100 rounded-full">
            <BarChart3 className="h-6 w-6 text-purple-600" />
          </div>
        </div>
        <div className="mt-4">
          <span className="text-sm text-gray-600">
            From {formatNumber(filterStats?.total_tickers)} total
          </span>
          <div className="text-xs text-gray-500 mt-1">
            Last updated: {formatResultsUpdate()}
          </div>
        </div>
      </div>

      {/* Connection Status */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Connection</p>
            <p className={`text-2xl font-bold ${isConnected ? 'text-green-600' : 'text-red-600'}`}>
              {isConnected ? 'Live' : 'Offline'}
            </p>
          </div>
          <div className={`p-3 rounded-full ${isConnected ? 'bg-green-100' : 'bg-red-100'}`}>
            <Wifi className={`h-6 w-6 ${isConnected ? 'text-green-600' : 'text-red-600'}`} />
          </div>
        </div>
        <div className="mt-4">
          <span className="text-sm text-gray-600">
            Status: <span className={`font-medium ${continuousStatus?.is_running ? 'text-blue-600' : 'text-gray-600'}`}>
              {continuousStatus?.is_running ? 'Scanning' : 'Idle'}
            </span>
          </span>
        </div>
        {continuousStatus?.error_message && (
          <div className="mt-2">
            <span className="text-sm text-red-600">{continuousStatus.error_message}</span>
          </div>
        )}
      </div>
    </div>
  );
};

export default DashboardStats;
