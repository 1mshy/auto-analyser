import React from "react";
import { Card, CardBody, CardHeader, Input, Button, Chip } from "@heroui/react";
import { Filter, DollarSign, TrendingUp, Building } from "lucide-react";

import { formatNumber } from "../utils/formatters";
import { Filter as FilterType, FilterStats } from "../services/api";

interface FilterPanelProps {
  filter: FilterType;
  onFilterChange: (filter: FilterType) => void;
  filterStats: FilterStats | null;
}

const FilterPanel: React.FC<FilterPanelProps> = ({
  filter,
  onFilterChange,
  filterStats,
}) => {
  const handleInputChange = (field: keyof FilterType, value: string) => {
    onFilterChange({
      ...filter,
      [field]: value === "" ? null : value,
    });
  };

  const handleNumericChange = (field: keyof FilterType, value: string) => {
    onFilterChange({
      ...filter,
      [field]: value === "" ? null : parseFloat(value),
    });
  };

  const resetFilters = () => {
    onFilterChange({
      max_analysis: null,
      max_market_cap: null,
      max_pct_change: null,
      max_price: null,
      max_rsi: null,
      max_tickers: null,
      max_volume: null,
      min_ipo_year: null,
      min_market_cap: null,
      min_pct_change: null,
      min_price: null,
      min_rsi: null,
      min_volume: null,
      overbought_rsi_threshold: 70,
      oversold_rsi_threshold: 30,
      countries: null,
      industries: null,
      sectors: null,
    });
  };

  return (
    <Card className="w-full">
      <CardHeader className="flex items-center gap-2 px-6 pt-6">
        <Filter className="h-5 w-5 text-primary" />
        <h3 className="text-lg font-semibold">Analysis Filters</h3>
      </CardHeader>
      <CardBody className="px-6 pb-6">
        {/* Filter Stats Summary */}
        {filterStats && (
          <div className="mb-6 p-4 bg-primary-50 rounded-lg">
            <p className="text-sm text-primary-700 font-medium">
              {formatNumber(filterStats.filtered_tickers)} of{" "}
              {formatNumber(filterStats.total_tickers)} stocks match your
              criteria
            </p>
          </div>
        )}

        <div className="space-y-6">
          {/* Price Range */}
          <div>
            <div className="flex items-center gap-2 mb-3">
              <DollarSign className="h-4 w-4 text-gray-600" />
              <label className="text-sm font-medium text-gray-700">
                Price Range
              </label>
            </div>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Min Price"
                placeholder="0"
                size="sm"
                type="number"
                value={filter.min_price?.toString() || ""}
                onChange={(e) => handleNumericChange("min_price", e.target.value)}
              />
              <Input
                label="Max Price"
                placeholder="∞"
                size="sm"
                type="number"
                value={filter.max_price?.toString() || ""}
                onChange={(e) => handleNumericChange("max_price", e.target.value)}
              />
            </div>
          </div>

          {/* Market Cap Range */}
          <div>
            <div className="flex items-center gap-2 mb-3">
              <Building className="h-4 w-4 text-gray-600" />
              <label className="text-sm font-medium text-gray-700">
                Market Cap (Millions)
              </label>
            </div>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Min Cap"
                placeholder="0"
                size="sm"
                type="number"
                value={filter.min_market_cap?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("min_market_cap", e.target.value)
                }
              />
              <Input
                label="Max Cap"
                placeholder="∞"
                size="sm"
                type="number"
                value={filter.max_market_cap?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("max_market_cap", e.target.value)
                }
              />
            </div>
          </div>

          {/* Volume Range */}
          <div>
            <div className="flex items-center gap-2 mb-3">
              <TrendingUp className="h-4 w-4 text-gray-600" />
              <label className="text-sm font-medium text-gray-700">
                Volume Range
              </label>
            </div>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Min Volume"
                placeholder="0"
                size="sm"
                type="number"
                value={filter.min_volume?.toString() || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_volume",
                    e.target.value === "" ? "" : parseInt(e.target.value).toString(),
                  )
                }
              />
              <Input
                label="Max Volume"
                placeholder="∞"
                size="sm"
                type="number"
                value={filter.max_volume?.toString() || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_volume",
                    e.target.value === "" ? "" : parseInt(e.target.value).toString(),
                  )
                }
              />
            </div>
          </div>

          {/* Analysis Limits */}
          <div>
            <label className="text-sm font-medium text-gray-700 mb-3 block">
              Analysis Limits
            </label>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Max Tickers to Fetch"
                placeholder="All"
                size="sm"
                type="number"
                value={filter.max_tickers?.toString() || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_tickers",
                    e.target.value === "" ? "" : parseInt(e.target.value).toString(),
                  )
                }
              />
              <Input
                label="Max to Analyze"
                placeholder="All"
                size="sm"
                type="number"
                value={filter.max_analysis?.toString() || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_analysis",
                    e.target.value === "" ? "" : parseInt(e.target.value).toString(),
                  )
                }
              />
            </div>
            <p className="text-xs text-gray-500 mt-2">
              Leave empty to fetch/analyze all available stocks. Warning:
              analyzing all stocks may take a very long time.
            </p>
          </div>

          {/* RSI Thresholds */}
          <div>
            <label className="text-sm font-medium text-gray-700 mb-3 block">
              RSI Trading Signals
            </label>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Oversold (<)"
                placeholder="30"
                size="sm"
                type="number"
                value={filter.oversold_rsi_threshold?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("oversold_rsi_threshold", e.target.value)
                }
              />
              <Input
                label="Overbought (>)"
                placeholder="70"
                size="sm"
                type="number"
                value={filter.overbought_rsi_threshold?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("overbought_rsi_threshold", e.target.value)
                }
              />
            </div>
          </div>

          {/* Percentage Change */}
          <div>
            <label className="text-sm font-medium text-gray-700 mb-3 block">
              Daily % Change
            </label>
            <div className="grid grid-cols-2 gap-3">
              <Input
                label="Min Change"
                placeholder="-100"
                size="sm"
                type="number"
                value={filter.min_pct_change?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("min_pct_change", e.target.value)
                }
              />
              <Input
                label="Max Change"
                placeholder="100"
                size="sm"
                type="number"
                value={filter.max_pct_change?.toString() || ""}
                onChange={(e) =>
                  handleNumericChange("max_pct_change", e.target.value)
                }
              />
            </div>
          </div>

          {/* Sector Distribution */}
          {filterStats?.sectors &&
            Object.keys(filterStats.sectors).length > 0 && (
              <div>
                <div className="flex items-center gap-2 mb-3">
                  <Building className="h-4 w-4 text-gray-600" />
                  <label className="text-sm font-medium text-gray-700">
                    Top Sectors
                  </label>
                </div>
                <div className="space-y-2">
                  {Object.entries(filterStats.sectors)
                    .sort(([, a], [, b]) => b - a)
                    .slice(0, 5)
                    .map(([sector, count]) => (
                      <div key={sector} className="flex justify-between items-center">
                        <span className="text-sm text-gray-600 truncate">
                          {sector}
                        </span>
                        <Chip size="sm" variant="flat">
                          {count}
                        </Chip>
                      </div>
                    ))}
                </div>
              </div>
            )}

          {/* Reset Button */}
          <Button
            className="w-full"
            color="default"
            variant="flat"
            onClick={resetFilters}
          >
            Reset Filters
          </Button>
        </div>
      </CardBody>
    </Card>
  );
};

export default FilterPanel;
