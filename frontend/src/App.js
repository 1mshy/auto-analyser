import React, { useState, useEffect } from 'react';
import { Activity, TrendingUp, TrendingDown, DollarSign, BarChart3, Settings, Wifi, WifiOff } from 'lucide-react';
import FilterPanel from './components/FilterPanel';
import AnalysisResults from './components/AnalysisResults';
import DashboardStats from './components/DashboardStats';
import * as api from './services/api';

function App() {
  const [continuousStatus, setContinuousStatus] = useState(null);
  const [filteredResults, setFilteredResults] = useState([]);
  const [isConnected, setIsConnected] = useState(false);
  const [filter, setFilter] = useState({
    min_market_cap: 100000000, // $100M minimum market cap (broader range)
    max_market_cap: 100000000000, // $100B maximum market cap
    min_price: 1,
    max_price: 500,
    min_volume: null,
    max_volume: null,
    min_pct_change: null,
    max_pct_change: null,
    min_rsi: null,
    max_rsi: 40, // Look for stocks with RSI below 40 (low RSI)
    sectors: null,
    countries: null,
    industries: null,
    min_ipo_year: null,
    max_ipo_year: null,
    oversold_rsi_threshold: 30, // Broader threshold for opportunities  
    overbought_rsi_threshold: 40, // Upper limit for low RSI
    max_tickers: null,
    max_analysis: null
  });
  const [filterStats, setFilterStats] = useState(null);
  const [showFilterPanel, setShowFilterPanel] = useState(false);

  // Establish WebSocket connection for real-time updates
  useEffect(() => {
    const ws = api.connectWebSocket(
      (data) => {
        setContinuousStatus(data);
        setIsConnected(true);
      },
      (error) => {
        console.error('WebSocket error:', error);
        setIsConnected(false);
      }
    );
    
    ws.onopen = () => {
      setIsConnected(true);
    };
    
    ws.onclose = () => {
      setIsConnected(false);
    };

    return () => {
      if (ws) {
        ws.close();
      }
    };
  }, []);

  // Fetch initial continuous status
  useEffect(() => {
    const fetchContinuousStatus = async () => {
      try {
        const status = await api.getContinuousStatus();
        setContinuousStatus(status);
      } catch (error) {
        console.error('Failed to fetch continuous status:', error);
      }
    };
    
    fetchContinuousStatus();
  }, []);

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

  // Fetch filtered results when filter changes
  useEffect(() => {
    const fetchFilteredResults = async () => {
      try {
        const results = await api.getFilteredResults(filter);
        setFilteredResults(results);
      } catch (error) {
        console.error('Failed to fetch filtered results:', error);
      }
    };
    
    fetchFilteredResults();
  }, [filter]);

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
                <p className="text-blue-100">24/7 Continuous Market Analysis</p>
              </div>
            </div>
            
            <div className="flex items-center space-x-4">
              {/* Connection Status */}
              <div className="flex items-center space-x-2">
                {isConnected ? (
                  <>
                    <Wifi className="h-5 w-5 text-green-300" />
                    <span className="text-sm text-green-300">Live</span>
                  </>
                ) : (
                  <>
                    <WifiOff className="h-5 w-5 text-red-300" />
                    <span className="text-sm text-red-300">Disconnected</span>
                  </>
                )}
              </div>
              
              <button
                onClick={() => setShowFilterPanel(!showFilterPanel)}
                className="flex items-center space-x-2 bg-white bg-opacity-20 hover:bg-opacity-30 px-4 py-2 rounded-lg transition-colors"
              >
                <Settings className="h-5 w-5" />
                <span>Filters</span>
              </button>
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
              continuousStatus={continuousStatus}
              filterStats={filterStats}
              filteredResultsCount={filteredResults.length}
              isConnected={isConnected}
            />
            
            {/* Analysis Results */}
            <div className="mt-8">
              <AnalysisResults
                results={filteredResults}
                continuousStatus={continuousStatus}
                isConnected={isConnected}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
