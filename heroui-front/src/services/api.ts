import axios from "axios";

const API_BASE_URL = "http://127.0.0.1:3001/api";

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
});

export interface Filter {
  min_market_cap?: number | null;
  max_market_cap?: number | null;
  min_price?: number | null;
  max_price?: number | null;
  min_volume?: number | null;
  max_volume?: number | null;
  min_pct_change?: number | null;
  max_pct_change?: number | null;
  min_rsi?: number | null;
  max_rsi?: number | null;
  sectors?: string[] | null;
  countries?: string[] | null;
  industries?: string[] | null;
  min_ipo_year?: number | null;
  max_ipo_year?: number | null;
  oversold_rsi_threshold?: number | null;
  overbought_rsi_threshold?: number | null;
  max_tickers?: number | null;
  max_analysis?: number | null;
}

export interface FilterStats {
  total_tickers: number;
  filtered_tickers: number;
  sectors?: Record<string, number>;
  countries?: Record<string, number>;
  industries?: Record<string, number>;
}

export interface StockResult {
  symbol: string;
  name: string;
  current_price?: number;
  volume?: number;
  pct_change?: number;
  rsi?: number;
  is_opportunity: boolean;
  market_cap?: number;
  sector?: string;
  industry?: string;
  country?: string;
  ipo_year?: number;
  error_message?: string;
}

export interface AnalysisStatus {
  session_id: string;
  status: "running" | "completed" | "error";
  progress: number;
  analyzed_count: number;
  total_count: number;
  opportunities_found: number;
  results: StockResult[];
  error_message?: string;
}

export interface AnalysisRequest {
  filter: Filter;
  max_tickers?: number | null;
  max_analysis?: number | null;
}

export const healthCheck = async () => {
  const response = await api.get("/health");

  return response.data;
};

export const getTickers = async (limit: number | null = null) => {
  const params = limit ? { limit } : {};
  const response = await api.get("/tickers", { params });

  return response.data;
};

export const getFilterStats = async (filter: Filter): Promise<FilterStats> => {
  const response = await api.post("/filter-stats", filter);

  return response.data;
};

export const startAnalysis = async (analysisRequest: AnalysisRequest) => {
  const response = await api.post("/analysis", analysisRequest);

  return response.data;
};

export const getAnalysisStatus = async (
  sessionId: string,
): Promise<AnalysisStatus> => {
  const response = await api.get(`/analysis/${sessionId}`);

  return response.data;
};

export const getAnalysisResults = async (sessionId: string) => {
  const response = await api.get(`/analysis/${sessionId}/results`);

  return response.data;
};
