import React, { useState, useEffect } from 'react';
import { Activity, TrendingUp, TrendingDown, DollarSign, BarChart3, Settings, Play, Square } from 'lucide-react';
import FilterPanel from './components/FilterPanel';
import AnalysisResults from './components/AnalysisResults';
import DashboardStats from './components/DashboardStats';
import * as api from './services/api';

function App() {
  const [analysisStatus, setAnalysisStatus] = useState(null);
  const [sessionId, setSessionId] = useState(null);
  const [isRunning, setIsRunning] = useState(false);
  const [filter, setFilter] = useState({
    min_market_cap: null,
    max_market_cap: null,
    min_price: 1,
    max_price: 1000,
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
  });
  const [filterStats, setFilterStats] = useState(null);
  const [showFilterPanel, setShowFilterPanel] = useState(false);

  // Fetch filter stats when filter changes
  useEffect(() => {
    const fetchStats = async () => {
      try {
        const stats = await api.getFilterStats(filter);
        setFilterStats(stats);
      } catch (error) {
        console.error('Failed to fetch filter stats:', error);
      }
    };
    
    fetchStats();
  }, [filter]);

  // Poll for analysis updates
  useEffect(() => {
    let interval;
    if (sessionId && isRunning) {
      interval = setInterval(async () => {
        try {
          const status = await api.getAnalysisStatus(sessionId);
          setAnalysisStatus(status);
          
          if (status.status === 'completed' || status.status === 'error') {
            setIsRunning(false);
            clearInterval(interval);
          }
        } catch (error) {
          console.error('Failed to fetch analysis status:', error);
          setIsRunning(false);
        }
      }, 2000);
    }

    return () => {
      if (interval) clearInterval(interval);
    };
  }, [sessionId, isRunning]);

  const startAnalysis = async () => {
    try {
      setIsRunning(true);
      const response = await api.startAnalysis({
        filter,
        max_tickers: 500,
        max_analysis: 100
      });
      setSessionId(response.session_id);
      setAnalysisStatus({
        session_id: response.session_id,
        status: 'running',
        progress: 0,
        analyzed_count: 0,
        total_count: 0,
        opportunities_found: 0,
        results: []
      });
    } catch (error) {
      console.error('Failed to start analysis:', error);
      setIsRunning(false);
    }
  };

  const stopAnalysis = () => {
    setIsRunning(false);
    setSessionId(null);
    setAnalysisStatus(null);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="gradient-bg text-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <Activity className="h-8 w-8" />
              <div>
                <h1 className="text-2xl font-bold">Auto Stock Analyser</h1>
                <p className="text-blue-100">Real-time Market Analysis Dashboard</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-4">
              <button
                onClick={() => setShowFilterPanel(!showFilterPanel)}
                className="flex items-center space-x-2 bg-white bg-opacity-20 hover:bg-opacity-30 px-4 py-2 rounded-lg transition-colors"
              >
                <Settings className="h-5 w-5" />
                <span>Filters</span>
              </button>
              
              {!isRunning ? (
                <button
                  onClick={startAnalysis}
                  className="flex items-center space-x-2 bg-green-500 hover:bg-green-600 px-6 py-2 rounded-lg font-medium transition-colors"
                >
                  <Play className="h-5 w-5" />
                  <span>Start Analysis</span>
                </button>
              ) : (
                <button
                  onClick={stopAnalysis}
                  className="flex items-center space-x-2 bg-red-500 hover:bg-red-600 px-6 py-2 rounded-lg font-medium transition-colors"
                >
                  <Square className="h-5 w-5" />
                  <span>Stop Analysis</span>
                </button>
              )}
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-8">
          {/* Filter Panel */}
          {showFilterPanel && (
            <div className="lg:col-span-1">
              <FilterPanel
                filter={filter}
                onFilterChange={setFilter}
                filterStats={filterStats}
              />
            </div>
          )}
          
          {/* Main Content */}
          <div className={showFilterPanel ? "lg:col-span-3" : "lg:col-span-4"}>
            {/* Dashboard Stats */}
            <DashboardStats
              analysisStatus={analysisStatus}
              filterStats={filterStats}
              isRunning={isRunning}
            />
            
            {/* Analysis Results */}
            <div className="mt-8">
              <AnalysisResults
                analysisStatus={analysisStatus}
                isRunning={isRunning}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
