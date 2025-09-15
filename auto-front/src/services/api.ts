// API service for connecting to the Rust backend
const API_BASE_URL = 'http://localhost:3000/api';

export interface User {
  id: number;
  username: string;
  email: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface Quote {
  symbol: string;
  price: number;
  change: number;
  change_percent: number;
  volume: number;
  timestamp: string;
}

export interface HistoricalData {
  date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface TechnicalIndicators {
  symbol: string;
  sma_20?: number;
  sma_50?: number;
  ema_12?: number;
  ema_26?: number;
  rsi_14?: number;
  macd?: number;
  macd_signal?: number;
  bb_upper?: number;
  bb_middle?: number;
  bb_lower?: number;
  timestamp: string;
}

export interface WatchlistItem {
  id: number;
  symbol: string;
  added_at: string;
}

export interface Alert {
  id: number;
  symbol: string;
  alert_type: 'price_above' | 'price_below' | 'rsi_overbought' | 'rsi_oversold';
  threshold: number;
  is_active: boolean;
  created_at: string;
}

export interface CreateAlertRequest {
  symbol: string;
  alert_type: 'price_above' | 'price_below' | 'rsi_overbought' | 'rsi_oversold';
  threshold: number;
}

class ApiService {
  private token: string | null = null;

  constructor() {
    this.token = localStorage.getItem('auth_token');
  }

  setToken(token: string) {
    this.token = token;
    localStorage.setItem('auth_token', token);
  }

  clearToken() {
    this.token = null;
    localStorage.removeItem('auth_token');
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`;
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string>),
    };

    if (this.token) {
      headers.Authorization = `Bearer ${this.token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(error || `HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  // Authentication
  async register(data: RegisterRequest): Promise<AuthResponse> {
    return this.request<AuthResponse>('/auth/register', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async login(data: LoginRequest): Promise<AuthResponse> {
    return this.request<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async getCurrentUser(): Promise<User> {
    return this.request<User>('/auth/me');
  }

  // Market Data
  async getQuote(symbol: string): Promise<Quote> {
    return this.request<Quote>(`/market/quote/${symbol}`);
  }

  async getHistoricalData(symbol: string): Promise<HistoricalData[]> {
    return this.request<HistoricalData[]>(`/market/historical/${symbol}`);
  }

  async getTechnicalIndicators(symbol: string): Promise<TechnicalIndicators> {
    return this.request<TechnicalIndicators>(`/market/indicators/${symbol}`);
  }

  // Watchlist
  async getWatchlist(): Promise<WatchlistItem[]> {
    return this.request<WatchlistItem[]>('/watchlist');
  }

  async addToWatchlist(symbol: string): Promise<void> {
    await this.request('/watchlist', {
      method: 'POST',
      body: JSON.stringify({ symbol }),
    });
  }

  async removeFromWatchlist(symbol: string): Promise<void> {
    await this.request(`/watchlist/${symbol}`, {
      method: 'DELETE',
    });
  }

  // Alerts
  async getAlerts(): Promise<Alert[]> {
    return this.request<Alert[]>('/alerts');
  }

  async createAlert(data: CreateAlertRequest): Promise<Alert> {
    return this.request<Alert>('/alerts', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateAlert(id: number, data: Partial<Alert>): Promise<Alert> {
    return this.request<Alert>(`/alerts/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteAlert(id: number): Promise<void> {
    await this.request(`/alerts/${id}`, {
      method: 'DELETE',
    });
  }
}

export const apiService = new ApiService();
