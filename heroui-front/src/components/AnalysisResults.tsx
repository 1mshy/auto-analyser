import React, { useState } from "react";
import {
  Card,
  CardBody,
  CardHeader,
  Table,
  TableHeader,
  TableColumn,
  TableBody,
  TableRow,
  TableCell,
  Chip,
  Select,
  SelectItem,
  Spinner,
} from "@heroui/react";
import {
  Activity,
  TrendingUp,
  TrendingDown,
  AlertTriangle,
} from "lucide-react";
import {
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  BarChart,
  Bar,
} from "recharts";

import {
  formatCurrency,
  formatVolume,
  getRSIStatus,
} from "../utils/formatters";
import { AnalysisStatus, StockResult } from "../services/api";

interface AnalysisResultsProps {
  analysisStatus: AnalysisStatus | null;
  isRunning: boolean;
}

const AnalysisResults: React.FC<AnalysisResultsProps> = ({
  analysisStatus,
  isRunning,
}) => {
  const [sortBy, setSortBy] = useState("rsi");
  const [filterType, setFilterType] = useState("all");

  if (!analysisStatus) {
    return (
      <Card className="w-full shadow-md">
        <CardBody className="p-8 text-center">
          <Activity className="h-12 w-12 text-default-400 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-foreground mb-2">
            Ready to Analyze
          </h3>
          <p className="text-default-600">
            Click &quot;Start Analysis&quot; to begin real-time stock market
            analysis
          </p>
        </CardBody>
      </Card>
    );
  }

  const filteredResults =
    analysisStatus.results?.filter((stock) => {
      switch (filterType) {
        case "opportunities":
          return stock.is_opportunity;
        case "oversold":
          return stock.rsi && stock.rsi <= 30;
        case "overbought":
          return stock.rsi && stock.rsi >= 70;
        default:
          return true;
      }
    }) || [];

  const sortedResults = [...filteredResults].sort((a, b) => {
    switch (sortBy) {
      case "rsi":
        return (a.rsi || 0) - (b.rsi || 0);
      case "price":
        return (a.current_price || 0) - (b.current_price || 0);
      case "volume":
        return (b.volume || 0) - (a.volume || 0);
      case "change":
        return (b.pct_change || 0) - (a.pct_change || 0);
      default:
        return 0;
    }
  });

  // Prepare chart data
  const rsiDistribution = analysisStatus.results?.reduce(
    (acc, stock) => {
      if (stock.rsi) {
        const bucket = Math.floor(stock.rsi / 10) * 10;

        acc[bucket] = (acc[bucket] || 0) + 1;
      }

      return acc;
    },
    {} as Record<number, number>,
  );

  const chartData = rsiDistribution
    ? Object.entries(rsiDistribution)
        .map(([range, count]) => ({
          count,
          name: `RSI ${range}-${parseInt(range) + 9}`,
          range: `${range}-${parseInt(range) + 9}`,
        }))
        .sort((a, b) => parseInt(a.range) - parseInt(b.range))
    : [];

  const filterOptions = [
    { key: "all", label: "All Results" },
    { key: "opportunities", label: "Opportunities Only" },
    { key: "oversold", label: "Oversold (RSI ≤ 30)" },
    { key: "overbought", label: "Overbought (RSI ≥ 70)" },
  ];

  const sortOptions = [
    { key: "rsi", label: "RSI (Low to High)" },
    { key: "price", label: "Price (Low to High)" },
    { key: "volume", label: "Volume (High to Low)" },
    { key: "change", label: "Change (High to Low)" },
  ];

  return (
    <div className="space-y-6">
      {/* Header with Controls */}
      <Card className="shadow-md">
        <CardHeader className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 pb-3">
          <div>
            <h2 className="text-xl font-bold text-foreground">
              Analysis Results
            </h2>
            <p className="text-sm text-default-600">
              {sortedResults.length} of {analysisStatus.total_count} stocks
              {isRunning && (
                <span className="ml-2 inline-flex items-center">
                  <Spinner size="sm" />
                  <span className="ml-1">Analyzing...</span>
                </span>
              )}
            </p>
          </div>

          <div className="flex flex-col sm:flex-row gap-2">
            <Select
              className="w-48"
              placeholder="Filter results"
              selectedKeys={[filterType]}
              size="sm"
              onSelectionChange={(keys) => {
                const selected = Array.from(keys)[0] as string;

                setFilterType(selected);
              }}
            >
              {filterOptions.map((option) => (
                <SelectItem key={option.key}>{option.label}</SelectItem>
              ))}
            </Select>

            <Select
              className="w-48"
              placeholder="Sort by"
              selectedKeys={[sortBy]}
              size="sm"
              onSelectionChange={(keys) => {
                const selected = Array.from(keys)[0] as string;

                setSortBy(selected);
              }}
            >
              {sortOptions.map((option) => (
                <SelectItem key={option.key}>{option.label}</SelectItem>
              ))}
            </Select>
          </div>
        </CardHeader>

        <CardBody className="pt-0">
          {chartData.length > 0 && (
            <div className="mb-6">
              <h3 className="text-lg font-semibold mb-4 text-foreground">
                RSI Distribution
              </h3>
              <div className="h-64 bg-content1 rounded-lg p-4">
                <ResponsiveContainer height="100%" width="100%">
                  <BarChart data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="name" />
                    <YAxis />
                    <Tooltip />
                    <Bar dataKey="count" fill="hsl(var(--heroui-primary))" />
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>
          )}

          {/* Results Table */}
          <Table aria-label="Stock analysis results" className="min-h-[200px]">
            <TableHeader>
              <TableColumn>SYMBOL</TableColumn>
              <TableColumn>NAME</TableColumn>
              <TableColumn>PRICE</TableColumn>
              <TableColumn>CHANGE %</TableColumn>
              <TableColumn>VOLUME</TableColumn>
              <TableColumn>RSI</TableColumn>
              <TableColumn>STATUS</TableColumn>
            </TableHeader>
            <TableBody>
              {sortedResults.map((stock: StockResult, index) => (
                <TableRow key={`${stock.symbol}-${index}`}>
                  <TableCell>
                    <div className="font-mono font-bold text-foreground">
                      {stock.symbol}
                    </div>
                  </TableCell>
                  <TableCell>
                    <div
                      className="max-w-xs truncate text-default-700"
                      title={stock.name}
                    >
                      {stock.name}
                    </div>
                  </TableCell>
                  <TableCell className="text-foreground">
                    {formatCurrency(stock.current_price)}
                  </TableCell>
                  <TableCell>
                    <div
                      className={`flex items-center gap-1 ${
                        (stock.pct_change || 0) >= 0
                          ? "text-success"
                          : "text-danger"
                      }`}
                    >
                      {(stock.pct_change || 0) >= 0 ? (
                        <TrendingUp className="h-4 w-4" />
                      ) : (
                        <TrendingDown className="h-4 w-4" />
                      )}
                      {stock.pct_change?.toFixed(2) || "0.00"}%
                    </div>
                  </TableCell>
                  <TableCell className="text-default-700">
                    {formatVolume(stock.volume)}
                  </TableCell>
                  <TableCell>
                    {stock.rsi ? (
                      <Chip
                        color={getRSIStatus(stock.rsi)}
                        size="sm"
                        variant="flat"
                      >
                        {stock.rsi.toFixed(1)}
                      </Chip>
                    ) : (
                      <span className="text-default-400">N/A</span>
                    )}
                  </TableCell>
                  <TableCell>
                    <div className="flex items-center gap-2">
                      {stock.is_opportunity && (
                        <Chip color="success" size="sm" variant="flat">
                          Opportunity
                        </Chip>
                      )}
                      {stock.error_message && (
                        <Chip
                          color="danger"
                          size="sm"
                          startContent={<AlertTriangle className="h-3 w-3" />}
                          variant="flat"
                        >
                          Error
                        </Chip>
                      )}
                    </div>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>

          {sortedResults.length === 0 && !isRunning && (
            <div className="text-center py-8">
              <p className="text-default-500">
                No results match your current filter criteria.
              </p>
            </div>
          )}
        </CardBody>
      </Card>
    </div>
  );
};

export default AnalysisResults;
