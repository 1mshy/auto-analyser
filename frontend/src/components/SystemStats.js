import React from 'react';
import { Database, Zap, HardDrive, RefreshCw } from 'lucide-react';
import { useCacheStats, useDatabaseStats, useClearCache } from '../hooks/useQueries';

const SystemStats = () => {
  const { data: cacheStats, isLoading: cacheLoading } = useCacheStats();
  const { data: databaseStats, isLoading: dbLoading } = useDatabaseStats();
  const clearCacheMutation = useClearCache();

  const formatNumber = (num) => {
    if (typeof num !== 'number') return 'N/A';
    return new Intl.NumberFormat().format(num);
  };

  const formatDate = (dateString) => {
    if (!dateString) return 'N/A';
    try {
      return new Date(dateString).toLocaleString();
    } catch {
      return 'N/A';
    }
  };

  const handleClearCache = () => {
    clearCacheMutation.mutate();
  };

  return (
    <div className="bg-white rounded-xl p-6 card-shadow">
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center space-x-2">
          <HardDrive className="h-5 w-5 text-blue-600" />
          <h3 className="text-lg font-semibold text-gray-900">System Statistics</h3>
        </div>
        
        <button
          onClick={handleClearCache}
          disabled={clearCacheMutation.isPending}
          className="flex items-center space-x-2 px-3 py-2 bg-red-100 hover:bg-red-200 text-red-700 rounded-lg text-sm transition-colors disabled:opacity-50"
        >
          <RefreshCw className={`h-4 w-4 ${clearCacheMutation.isPending ? 'animate-spin' : ''}`} />
          <span>Clear Cache</span>
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Cache Statistics */}
        <div className="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-lg p-4">
          <div className="flex items-center space-x-2 mb-4">
            <Zap className="h-5 w-5 text-blue-600" />
            <h4 className="font-medium text-gray-900">Cache Performance</h4>
          </div>
          
          {cacheLoading ? (
            <div className="animate-pulse space-y-2">
              <div className="h-4 bg-blue-200 rounded"></div>
              <div className="h-4 bg-blue-200 rounded w-3/4"></div>
              <div className="h-4 bg-blue-200 rounded w-1/2"></div>
            </div>
          ) : cacheStats ? (
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Stock Data Entries</span>
                <span className="font-medium">{formatNumber(cacheStats.stock_data_entries)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Indicators Entries</span>
                <span className="font-medium">{formatNumber(cacheStats.indicators_entries)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Tickers Entries</span>
                <span className="font-medium">{formatNumber(cacheStats.tickers_entries)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Rate Limiter Entries</span>
                <span className="font-medium">{formatNumber(cacheStats.rate_limiter_entries)}</span>
              </div>
            </div>
          ) : (
            <p className="text-gray-500 text-sm">No cache data available</p>
          )}
        </div>

        {/* Database Statistics */}
        <div className="bg-gradient-to-br from-green-50 to-emerald-100 rounded-lg p-4">
          <div className="flex items-center space-x-2 mb-4">
            <Database className="h-5 w-5 text-green-600" />
            <h4 className="font-medium text-gray-900">Database Analytics</h4>
          </div>
          
          {dbLoading ? (
            <div className="animate-pulse space-y-2">
              <div className="h-4 bg-green-200 rounded"></div>
              <div className="h-4 bg-green-200 rounded w-3/4"></div>
              <div className="h-4 bg-green-200 rounded w-1/2"></div>
            </div>
          ) : databaseStats && !databaseStats.error ? (
            <div className="space-y-3">
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Total Results</span>
                <span className="font-medium">{formatNumber(databaseStats.total_results)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Unique Tickers</span>
                <span className="font-medium">{formatNumber(databaseStats.unique_tickers)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Analysis Sessions</span>
                <span className="font-medium">{formatNumber(databaseStats.total_sessions)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-sm text-gray-600">Opportunities Found</span>
                <span className="font-medium text-green-600">{formatNumber(databaseStats.opportunities)}</span>
              </div>
              {databaseStats.avg_rsi && (
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Average RSI</span>
                  <span className="font-medium">{databaseStats.avg_rsi.toFixed(1)}</span>
                </div>
              )}
              {databaseStats.oldest_result && (
                <div className="flex justify-between">
                  <span className="text-sm text-gray-600">Oldest Record</span>
                  <span className="font-medium text-xs">{formatDate(databaseStats.oldest_result)}</span>
                </div>
              )}
            </div>
          ) : (
            <div className="text-center py-4">
              <Database className="h-8 w-8 text-gray-400 mx-auto mb-2" />
              <p className="text-gray-500 text-sm">
                {databaseStats?.error || 'Database not available'}
              </p>
            </div>
          )}
        </div>
      </div>

      {clearCacheMutation.isSuccess && (
        <div className="mt-4 p-3 bg-green-100 border border-green-400 text-green-700 rounded-lg text-sm">
          Cache cleared successfully! Data will be refetched on next request.
        </div>
      )}

      {clearCacheMutation.error && (
        <div className="mt-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded-lg text-sm">
          Failed to clear cache: {clearCacheMutation.error.message}
        </div>
      )}
    </div>
  );
};

export default SystemStats;