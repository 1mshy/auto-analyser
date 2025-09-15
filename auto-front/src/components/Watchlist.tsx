import React, { useState, useEffect } from 'react';
import { Card, CardBody, CardHeader } from '@heroui/card';
import { Button } from '@heroui/button';
import { Input } from '@heroui/input';

import { apiService, WatchlistItem, Quote } from '@/services/api';

export const Watchlist: React.FC = () => {
  const [watchlist, setWatchlist] = useState<WatchlistItem[]>([]);
  const [quotes, setQuotes] = useState<Record<string, Quote>>({});
  const [newSymbol, setNewSymbol] = useState('');
  const [loading, setLoading] = useState(false);
  const [addingSymbol, setAddingSymbol] = useState(false);

  const loadWatchlist = async () => {
    setLoading(true);
    try {
      const watchlistData = await apiService.getWatchlist();
      setWatchlist(watchlistData);

      // Fetch quotes for all symbols
      const quotesData: Record<string, Quote> = {};
      await Promise.all(
        watchlistData.map(async (item) => {
          try {
            const quote = await apiService.getQuote(item.symbol);
            quotesData[item.symbol] = quote;
          } catch (error) {
            console.error(`Failed to fetch quote for ${item.symbol}:`, error);
          }
        })
      );
      setQuotes(quotesData);
    } catch (error) {
      console.error('Failed to load watchlist:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadWatchlist();
  }, []);

  const addSymbol = async () => {
    if (!newSymbol.trim()) return;

    setAddingSymbol(true);
    try {
      await apiService.addToWatchlist(newSymbol.toUpperCase());
      setNewSymbol('');
      await loadWatchlist(); // Reload the watchlist
    } catch (error) {
      console.error('Failed to add symbol:', error);
    } finally {
      setAddingSymbol(false);
    }
  };

  const removeSymbol = async (symbol: string) => {
    try {
      await apiService.removeFromWatchlist(symbol);
      await loadWatchlist(); // Reload the watchlist
    } catch (error) {
      console.error('Failed to remove symbol:', error);
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
        <div className="flex justify-between items-center w-full">
          <h3 className="text-xl font-bold">My Watchlist</h3>
          <div className="flex gap-2">
            <Input
              placeholder="Add symbol"
              value={newSymbol}
              className="w-32"
              onChange={(e) => setNewSymbol(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && addSymbol()}
            />
            <Button
              color="primary"
              size="sm"
              isLoading={addingSymbol}
              onPress={addSymbol}
            >
              Add
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardBody>
        {loading ? (
          <div className="text-center py-8">Loading watchlist...</div>
        ) : watchlist.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            No symbols in your watchlist. Add one to get started!
          </div>
        ) : (
          <div className="space-y-3">
            {watchlist.map((item) => {
              const quote = quotes[item.symbol];
              return (
                <div
                  key={item.id}
                  className="flex items-center justify-between p-4 border rounded-lg hover:bg-gray-50"
                >
                  <div className="flex-1">
                    <div className="font-semibold text-lg">{item.symbol}</div>
                    {quote ? (
                      <div className="space-y-1">
                        <div className="text-xl font-bold">
                          {formatCurrency(quote.price)}
                        </div>
                        <div
                          className={`text-sm ${
                            quote.change >= 0
                              ? 'text-green-600'
                              : 'text-red-600'
                          }`}
                        >
                          {formatCurrency(quote.change)} (
                          {formatPercent(quote.change_percent)})
                        </div>
                      </div>
                    ) : (
                      <div className="text-gray-500">Loading...</div>
                    )}
                  </div>
                  <Button
                    color="danger"
                    variant="light"
                    size="sm"
                    onPress={() => removeSymbol(item.symbol)}
                  >
                    Remove
                  </Button>
                </div>
              );
            })}
          </div>
        )}
      </CardBody>
    </Card>
  );
};
