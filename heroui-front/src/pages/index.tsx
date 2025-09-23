import { useState, useEffect } from "react";
import { Button } from "@heroui/react";
import { Activity, Play, Square, Settings } from "lucide-react";

import DashboardStats from "../components/DashboardStats";
import FilterPanel from "../components/FilterPanel";
import AnalysisResults from "../components/AnalysisResults";
import DefaultLayout from "../layouts/default";
import * as api from "../services/api";
import { Filter, AnalysisStatus, FilterStats } from "../services/api";

export default function IndexPage() {
  const [analysisStatus, setAnalysisStatus] = useState<AnalysisStatus | null>(
    null,
  );
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  const [filter, setFilter] = useState<Filter>({
    max_analysis: null,
    max_market_cap: 100000000000, // $100B maximum market cap
    max_pct_change: null,
    max_price: 500,
    max_rsi: 40, // Look for stocks with RSI below 40 (low RSI)
    max_tickers: null,
    max_volume: null,
    min_ipo_year: null,
    min_market_cap: 100000000, // $100M minimum market cap (broader range)
    min_pct_change: null,
    min_price: 1,
    min_rsi: null,
    min_volume: null,
    overbought_rsi_threshold: 40, // Upper limit for low RSI
    oversold_rsi_threshold: 30, // Broader threshold for opportunities
    countries: null,
    industries: null,
    sectors: null,
  });
  const [filterStats, setFilterStats] = useState<FilterStats | null>(null);
  const [showFilterPanel, setShowFilterPanel] = useState(false);

  // Fetch filter stats when filter changes
  useEffect(() => {
    const fetchStats = async () => {
      try {
        const stats = await api.getFilterStats(filter);

        setFilterStats(stats);
      } catch (error) {
        console.error("Failed to fetch filter stats:", error);
      }
    };

    fetchStats();
  }, [filter]);

  // Poll for analysis updates
  useEffect(() => {
    let interval: NodeJS.Timeout;

    if (sessionId && isRunning) {
      interval = setInterval(async () => {
        try {
          const status = await api.getAnalysisStatus(sessionId);

          setAnalysisStatus(status);

          if (status.status === "completed" || status.status === "error") {
            setIsRunning(false);
            clearInterval(interval);
          }
        } catch (error) {
          console.error("Failed to fetch analysis status:", error);
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
        max_analysis: filter.max_analysis,
        max_tickers: filter.max_tickers,
      });

      setSessionId(response.session_id);
      setAnalysisStatus({
        analyzed_count: 0,
        opportunities_found: 0,
        progress: 0,
        results: [],
        session_id: response.session_id,
        status: "running",
        total_count: 0,
      });
    } catch (error) {
      console.error("Failed to start analysis:", error);
      setIsRunning(false);
    }
  };

  const stopAnalysis = () => {
    setIsRunning(false);
    setSessionId(null);
    setAnalysisStatus(null);
  };

  return (
    <DefaultLayout>
      {/* Header */}
      <div className="gradient-bg text-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <Activity className="h-8 w-8" />
              <div>
                <h1 className="text-2xl font-bold">Auto Stock Analyser</h1>
                <p className="text-blue-100">
                  Real-time Market Analysis Dashboard
                </p>
              </div>
            </div>

            <div className="flex items-center space-x-4">
              <Button
                className="bg-white bg-opacity-20 hover:bg-opacity-30"
                startContent={<Settings className="h-5 w-5" />}
                variant="flat"
                onClick={() => setShowFilterPanel(!showFilterPanel)}
              >
                Filters
              </Button>

              {!isRunning ? (
                <Button
                  className="bg-green-500 hover:bg-green-600 font-medium"
                  startContent={<Play className="h-5 w-5" />}
                  onClick={startAnalysis}
                >
                  Start Analysis
                </Button>
              ) : (
                <Button
                  className="bg-red-500 hover:bg-red-600 font-medium"
                  startContent={<Square className="h-5 w-5" />}
                  onClick={stopAnalysis}
                >
                  Stop Analysis
                </Button>
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
                filterStats={filterStats}
                onFilterChange={setFilter}
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
    </DefaultLayout>
  );
}
