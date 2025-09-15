import React, { useState, useEffect } from 'react';
import { Card, CardBody, CardHeader } from '@heroui/card';
import { Button } from '@heroui/button';
import { Input } from '@heroui/input';
import { Select, SelectItem } from '@heroui/select';

import { apiService, Alert, CreateAlertRequest } from '@/services/api';

export const AlertsManager: React.FC = () => {
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [loading, setLoading] = useState(false);
  const [showCreateForm, setShowCreateForm] = useState(false);

  // Form state
  const [newAlert, setNewAlert] = useState<CreateAlertRequest>({
    symbol: '',
    alert_type: 'price_above',
    threshold: 0,
  });

  const loadAlerts = async () => {
    setLoading(true);
    try {
      const alertsData = await apiService.getAlerts();
      setAlerts(alertsData);
    } catch (error) {
      console.error('Failed to load alerts:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadAlerts();
  }, []);

  const createAlert = async () => {
    if (!newAlert.symbol.trim() || newAlert.threshold <= 0) return;

    try {
      await apiService.createAlert({
        ...newAlert,
        symbol: newAlert.symbol.toUpperCase(),
      });
      setNewAlert({ symbol: '', alert_type: 'price_above', threshold: 0 });
      setShowCreateForm(false);
      await loadAlerts();
    } catch (error) {
      console.error('Failed to create alert:', error);
    }
  };

  const deleteAlert = async (id: number) => {
    try {
      await apiService.deleteAlert(id);
      await loadAlerts();
    } catch (error) {
      console.error('Failed to delete alert:', error);
    }
  };

  const toggleAlert = async (alert: Alert) => {
    try {
      await apiService.updateAlert(alert.id, {
        is_active: !alert.is_active,
      });
      await loadAlerts();
    } catch (error) {
      console.error('Failed to update alert:', error);
    }
  };

  const alertTypeLabels = {
    price_above: 'Price Above',
    price_below: 'Price Below',
    rsi_overbought: 'RSI Overbought (>70)',
    rsi_oversold: 'RSI Oversold (<30)',
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <div className="flex justify-between items-center w-full">
          <h3 className="text-xl font-bold">Price Alerts</h3>
          <Button
            color="primary"
            onPress={() => setShowCreateForm(!showCreateForm)}
          >
            {showCreateForm ? 'Cancel' : 'Create Alert'}
          </Button>
        </div>
      </CardHeader>
      <CardBody>
        {showCreateForm && (
          <div className="mb-6 p-4 border rounded-lg bg-gray-50">
            <h4 className="font-semibold mb-4">Create New Alert</h4>
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
              <Input
                label="Symbol"
                placeholder="e.g., AAPL"
                value={newAlert.symbol}
                onChange={(e) =>
                  setNewAlert({ ...newAlert, symbol: e.target.value })
                }
              />
              <Select
                label="Alert Type"
                selectedKeys={[newAlert.alert_type]}
                onSelectionChange={(keys) => {
                  const selectedKey = Array.from(keys)[0] as string;
                  setNewAlert({
                    ...newAlert,
                    alert_type: selectedKey as CreateAlertRequest['alert_type'],
                  });
                }}
              >
                {Object.entries(alertTypeLabels).map(([key, label]) => (
                  <SelectItem key={key} value={key}>
                    {label}
                  </SelectItem>
                ))}
              </Select>
              <Input
                type="number"
                label="Threshold"
                placeholder="0.00"
                value={newAlert.threshold.toString()}
                onChange={(e) =>
                  setNewAlert({
                    ...newAlert,
                    threshold: parseFloat(e.target.value) || 0,
                  })
                }
              />
              <Button color="primary" className="mt-6" onPress={createAlert}>
                Create
              </Button>
            </div>
          </div>
        )}

        {loading ? (
          <div className="text-center py-8">Loading alerts...</div>
        ) : alerts.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            No alerts configured. Create one to get notified about price
            changes!
          </div>
        ) : (
          <div className="space-y-3">
            {alerts.map((alert) => (
              <div
                key={alert.id}
                className="flex items-center justify-between p-4 border rounded-lg"
              >
                <div className="flex-1">
                  <div className="flex items-center gap-4">
                    <div className="font-semibold text-lg">{alert.symbol}</div>
                    <div className="text-sm text-gray-600">
                      {alertTypeLabels[alert.alert_type]}
                    </div>
                    <div className="font-semibold">
                      {alert.alert_type.includes('price')
                        ? `$${alert.threshold}`
                        : alert.threshold}
                    </div>
                  </div>
                  <div className="text-xs text-gray-500 mt-1">
                    Created: {new Date(alert.created_at).toLocaleDateString()}
                  </div>
                </div>
                <div className="flex items-center gap-3">
                  <Switch
                    isSelected={alert.is_active}
                    onValueChange={() => toggleAlert(alert)}
                  />
                  <Button
                    color="danger"
                    variant="light"
                    size="sm"
                    onPress={() => deleteAlert(alert.id)}
                  >
                    Delete
                  </Button>
                </div>
              </div>
            ))}
          </div>
        )}
      </CardBody>
    </Card>
  );
};
