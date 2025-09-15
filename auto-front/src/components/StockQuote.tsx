import React, { useState, useEffect } from 'react';
import { Card, CardBody, CardHeader } from '@heroui/card';
import { Button } from '@heroui/button';
import { Input } from '@heroui/input';

import { apiService, Quote, TechnicalIndicators } from '@/services/api';

export const StockQuote: React.FC = () => {
  const [symbol, setSymbol] = useState('AAPL');
  const [quote, setQuote] = useState<Quote | null>(null);
  const [indicators, setIndicators] = useState<TechnicalIndicators | null>(
    null
  );
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const fetchStockData = async (stockSymbol: string) => {
    setLoading(true);
    setError('');

    try {
      const [quoteData, indicatorData] = await Promise.all([
        apiService.getQuote(stockSymbol),
        apiService.getTechnicalIndicators(stockSymbol),
      ]);

      setQuote(quoteData);
      setIndicators(indicatorData);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to fetch data');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchStockData(symbol);
  }, []);

  const handleSearch = () => {
    if (symbol.trim()) {
      fetchStockData(symbol.toUpperCase());
    }
  };

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(value);
  };

  const formatPercent = (value: number) => {
    return `${value > 0 ? '+' : ''}${value.toFixed(2)}%`;
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex gap-4 items-center w-full">
          <Input
            placeholder="Enter symbol (e.g., AAPL)"
            value={symbol}
            className="flex-1"
            onChange={(e) => setSymbol(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          />
          <Button color="primary" isLoading={loading} onPress={handleSearch}>
            Search
          </Button>
        </div>
      </CardHeader>
      <CardBody>
        {error && <div className="text-red-500 text-center mb-4">{error}</div>}

        {quote && (
          <div className="space-y-6">
            {/* Price Information */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <h3 className="text-2xl font-bold">{quote.symbol}</h3>
                <div className="text-3xl font-bold">
                  {formatCurrency(quote.price)}
                </div>
                <div
                  className={`text-lg ${quote.change >= 0 ? 'text-green-600' : 'text-red-600'}`}
                >
                  {formatCurrency(quote.change)} (
                  {formatPercent(quote.change_percent)})
                </div>
              </div>
              <div className="space-y-2">
                <div className="text-gray-600">Volume</div>
                <div className="text-xl font-semibold">
                  {quote.volume.toLocaleString()}
                </div>
                <div className="text-gray-600 text-sm">
                  Last updated: {new Date(quote.timestamp).toLocaleTimeString()}
                </div>
              </div>
            </div>

            {/* Technical Indicators */}
            {indicators && (
              <div className="border-t pt-4">
                <h4 className="text-lg font-semibold mb-4">
                  Technical Indicators
                </h4>
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  {indicators.sma_20 && (
                    <div className="text-center p-3 bg-gray-50 rounded">
                      <div className="text-sm text-gray-600">SMA 20</div>
                      <div className="font-semibold">
                        {formatCurrency(indicators.sma_20)}
                      </div>
                    </div>
                  )}
                  {indicators.sma_50 && (
                    <div className="text-center p-3 bg-gray-50 rounded">
                      <div className="text-sm text-gray-600">SMA 50</div>
                      <div className="font-semibold">
                        {formatCurrency(indicators.sma_50)}
                      </div>
                    </div>
                  )}
                  {indicators.rsi_14 && (
                    <div className="text-center p-3 bg-gray-50 rounded">
                      <div className="text-sm text-gray-600">RSI 14</div>
                      <div
                        className={`font-semibold ${
                          indicators.rsi_14 > 70
                            ? 'text-red-600'
                            : indicators.rsi_14 < 30
                              ? 'text-green-600'
                              : 'text-gray-900'
                        }`}
                      >
                        {indicators.rsi_14.toFixed(2)}
                      </div>
                    </div>
                  )}
                  {indicators.macd && (
                    <div className="text-center p-3 bg-gray-50 rounded">
                      <div className="text-sm text-gray-600">MACD</div>
                      <div className="font-semibold">
                        {indicators.macd.toFixed(4)}
                      </div>
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>
        )}
      </CardBody>
    </Card>
  );
};
