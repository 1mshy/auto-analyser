import axios from 'axios';

const API_BASE_URL = 'http://127.0.0.1:3001/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 30000,
});

export const healthCheck = async () => {
  const response = await api.get('/health');
  return response.data;
};

export const getTickers = async (limit = 100) => {
  const response = await api.get('/tickers', { params: { limit } });
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
