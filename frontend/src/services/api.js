import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:3001/api';
const WS_BASE_URL = 'ws://127.0.0.1:3001/ws';

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
});

export const healthCheck = async () => {
  const response = await api.get('/health');
  return response.data;
};

export const getTickers = async (limit = null) => {
  const params = limit ? { limit } : {};
  const response = await api.get('/tickers', { params });
  return response.data;
};

export const getFilterStats = async (filter) => {
  const response = await api.post('/filter-stats', filter);
  return response.data;
};

export const startAnalysis = async (analysisRequest) => {
  const response = await api.post('/analysis', analysisRequest);
  return response.data;
};

export const getAnalysisStatus = async (sessionId) => {
  const response = await api.get(`/analysis/${sessionId}`);
  return response.data;
};

export const getAnalysisResults = async (sessionId) => {
  const response = await api.get(`/analysis/${sessionId}/results`);
  return response.data;
};

// New continuous analysis endpoints
export const getContinuousStatus = async () => {
  const response = await api.get('/continuous-status');
  return response.data;
};

export const getFilteredResults = async (filter) => {
  const response = await api.post('/filtered-results', filter);
  return response.data;
};

// WebSocket connection for real-time updates
export const connectWebSocket = (onMessage, onError = null) => {
  console.log('🔌 Connecting to WebSocket:', WS_BASE_URL);
  const ws = new WebSocket(WS_BASE_URL);
  
  ws.onopen = () => {
    console.log('✅ WebSocket connected');
  };
  
  ws.onmessage = (event) => {
    try {
      const data = JSON.parse(event.data);
      onMessage(data);
    } catch (error) {
      console.error('Failed to parse WebSocket message:', error);
    }
  };
  
  ws.onerror = (error) => {
    console.error('❌ WebSocket error:', error);
    if (onError) onError(error);
  };
  
  ws.onclose = (event) => {
    console.log('🔌 WebSocket connection closed', {
      code: event.code,
      reason: event.reason,
      wasClean: event.wasClean
    });
  };
  
  return ws;
};

// Additional API functions for new endpoints
export const getCacheStats = async () => {
  const response = await api.get('/cache-stats');
  return response.data;
};

export const getDatabaseStats = async () => {
  const response = await api.get('/database-stats');
  return response.data;
};

export const clearCache = async () => {
  const response = await api.post('/clear-cache');
  return response.data;
};
