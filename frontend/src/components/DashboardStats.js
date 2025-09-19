import React from 'react';
import { TrendingUp, TrendingDown, DollarSign, BarChart3, Activity, Clock } from 'lucide-react';

const DashboardStats = ({ analysisStatus, filterStats, isRunning }) => {
  const formatNumber = (num) => {
    if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
    if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
    return num?.toString() || '0';
  };

  const getOpportunityRate = () => {
    if (!analysisStatus || analysisStatus.analyzed_count === 0) return 0;
    return ((analysisStatus.opportunities_found / analysisStatus.analyzed_count) * 100).toFixed(1);
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {/* Analysis Progress */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Analysis Progress</p>
            <p className="text-2xl font-bold text-gray-900">
              {analysisStatus ? `${analysisStatus.analyzed_count}/${analysisStatus.total_count}` : '0/0'}
            </p>
          </div>
          <div className={`p-3 rounded-full ${isRunning ? 'bg-blue-100' : 'bg-gray-100'}`}>
            <Activity className={`h-6 w-6 ${isRunning ? 'text-blue-600' : 'text-gray-600'}`} />
          </div>
        </div>
        {analysisStatus && (
          <div className="mt-4">
            <div className="flex justify-between text-sm text-gray-600 mb-1">
              <span>Progress</span>
              <span>{(analysisStatus.progress * 100).toFixed(1)}%</span>
            </div>
            <div className="w-full bg-gray-200 rounded-full h-2">
              <div
                className="bg-blue-600 h-2 rounded-full transition-all duration-300"
                style={{ width: `${analysisStatus.progress * 100}%` }}
              ></div>
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
              {analysisStatus?.opportunities_found || 0}
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

      {/* Filtered Tickers */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Filtered Tickers</p>
            <p className="text-2xl font-bold text-purple-600">
              {formatNumber(filterStats?.filtered_tickers)}
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
        </div>
      </div>

      {/* Status */}
      <div className="bg-white rounded-xl p-6 card-shadow">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-gray-600">Status</p>
            <p className="text-2xl font-bold text-gray-900 capitalize">
              {analysisStatus?.status || 'Ready'}
            </p>
          </div>
          <div className={`p-3 rounded-full ${
            analysisStatus?.status === 'completed' ? 'bg-green-100' :
            analysisStatus?.status === 'running' ? 'bg-blue-100' :
            analysisStatus?.status === 'error' ? 'bg-red-100' : 'bg-gray-100'
          }`}>
            <Clock className={`h-6 w-6 ${
              analysisStatus?.status === 'completed' ? 'text-green-600' :
              analysisStatus?.status === 'running' ? 'text-blue-600' :
              analysisStatus?.status === 'error' ? 'text-red-600' : 'text-gray-600'
            }`} />
          </div>
        </div>
        {analysisStatus?.error_message && (
          <div className="mt-2">
            <span className="text-sm text-red-600">{analysisStatus.error_message}</span>
          </div>
        )}
      </div>
    </div>
  );
};

export default DashboardStats;
