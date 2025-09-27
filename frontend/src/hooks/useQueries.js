import React from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as api from '../services/api';

// Query keys
export const QUERY_KEYS = {
  continuousStatus: 'continuousStatus',
  filteredResults: 'filteredResults',
  filterStats: 'filterStats',
  cacheStats: 'cacheStats',
  databaseStats: 'databaseStats',
};

// Continuous status query
export function useContinuousStatus() {
  return useQuery({
    queryKey: [QUERY_KEYS.continuousStatus],
    queryFn: api.getContinuousStatus,
    refetchInterval: 5000, // Refetch every 5 seconds
    staleTime: 1000 * 3, // Consider stale after 3 seconds
  });
}

// Filtered results query
export function useFilteredResults(filter) {
  return useQuery({
    queryKey: [QUERY_KEYS.filteredResults, filter],
    queryFn: () => api.getFilteredResults(filter),
    refetchInterval: 10000, // Refetch every 10 seconds
    staleTime: 1000 * 5, // Consider stale after 5 seconds
    enabled: !!filter, // Only run if filter is provided
  });
}

// Filter stats query
export function useFilterStats(filter) {
  return useQuery({
    queryKey: [QUERY_KEYS.filterStats, filter],
    queryFn: () => api.getFilterStats(filter),
    staleTime: 1000 * 60 * 5, // Consider stale after 5 minutes
    enabled: !!filter, // Only run if filter is provided
  });
}

// Cache stats query
export function useCacheStats() {
  return useQuery({
    queryKey: [QUERY_KEYS.cacheStats],
    queryFn: api.getCacheStats,
    refetchInterval: 30000, // Refetch every 30 seconds
  });
}

// Database stats query  
export function useDatabaseStats() {
  return useQuery({
    queryKey: [QUERY_KEYS.databaseStats],
    queryFn: api.getDatabaseStats,
    refetchInterval: 60000, // Refetch every minute
  });
}

// Clear cache mutation
export function useClearCache() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: api.clearCache,
    onSuccess: () => {
      // Invalidate all queries after clearing cache
      queryClient.invalidateQueries();
    },
  });
}

// Start analysis mutation
export function useStartAnalysis() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: api.startAnalysis,
    onSuccess: () => {
      // Invalidate relevant queries after starting analysis
      queryClient.invalidateQueries([QUERY_KEYS.continuousStatus]);
      queryClient.invalidateQueries([QUERY_KEYS.filteredResults]);
    },
  });
}

// WebSocket hook for real-time updates
export function useWebSocketUpdates(onMessage, onError) {
  const queryClient = useQueryClient();
  const wsRef = React.useRef(null);
  const onMessageRef = React.useRef(onMessage);
  const onErrorRef = React.useRef(onError);
  const mountedRef = React.useRef(true);
  const reconnectTimeoutRef = React.useRef(null);
  const isConnectingRef = React.useRef(false);
  
  // Update refs when callbacks change
  React.useEffect(() => {
    onMessageRef.current = onMessage;
    onErrorRef.current = onError;
  }, [onMessage, onError]);

  const connectWebSocket = React.useCallback(() => {
    // Don't connect if component is unmounted or already connecting
    if (!mountedRef.current || isConnectingRef.current) return;
    
    // Close existing connection if any
    if (wsRef.current) {
      if (wsRef.current.readyState === WebSocket.CONNECTING || wsRef.current.readyState === WebSocket.OPEN) {
        wsRef.current.close(1000, 'Reconnecting');
      }
      wsRef.current = null;
    }
    
    // Clear any pending reconnection
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }
    
    isConnectingRef.current = true;
    console.log('🔌 Establishing WebSocket connection...');
    
    try {
      const ws = api.connectWebSocket(
        (data) => {
          // Update the continuous status query cache
          queryClient.setQueryData([QUERY_KEYS.continuousStatus], data);
          
          // Invalidate filtered results to trigger refetch
          queryClient.invalidateQueries([QUERY_KEYS.filteredResults]);
          
          if (onMessageRef.current) onMessageRef.current(data);
        },
        (error) => {
          console.warn('WebSocket error:', error);
          if (onErrorRef.current) onErrorRef.current(error);
        }
      );
      
      ws.onopen = () => {
        console.log('✅ WebSocket connected');
        isConnectingRef.current = false;
      };
      
      ws.onclose = (event) => {
        isConnectingRef.current = false;
        console.log(`🔌 WebSocket closed: ${event.code} ${event.reason || ''}`);
        
        // Only auto-reconnect for unexpected closures and if component is still mounted
        if (mountedRef.current && event.code !== 1000 && event.code !== 1001) {
          console.log('🔄 Scheduling reconnection in 5 seconds...');
          reconnectTimeoutRef.current = setTimeout(() => {
            if (mountedRef.current) {
              connectWebSocket();
            }
          }, 5000);
        }
      };
      
      ws.onerror = () => {
        isConnectingRef.current = false;
      };
      
      wsRef.current = ws;
    } catch (error) {
      isConnectingRef.current = false;
      console.error('Failed to create WebSocket:', error);
    }
  }, [queryClient]);

  React.useEffect(() => {
    mountedRef.current = true;
    
    // In development mode, delay initial connection to avoid rapid reconnects during hot reload
    const isDevelopment = process.env.NODE_ENV === 'development';
    const delay = isDevelopment ? 1000 : 0;
    
    const timeoutId = setTimeout(() => {
      if (mountedRef.current) {
        connectWebSocket();
      }
    }, delay);

    return () => {
      mountedRef.current = false;
      isConnectingRef.current = false;
      
      // Clear connection timeout
      clearTimeout(timeoutId);
      
      // Clear reconnection timeout
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
        reconnectTimeoutRef.current = null;
      }
      
      console.log('🔌 Component unmounting, closing WebSocket...');
      if (wsRef.current) {
        wsRef.current.close(1000, 'Component unmounted'); // Clean close
        wsRef.current = null;
      }
    };
  }, [connectWebSocket]);
  
  return wsRef.current;
}