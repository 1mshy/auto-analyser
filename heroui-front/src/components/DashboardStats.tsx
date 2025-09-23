import React from "react";
import { Card, CardBody, Progress, Chip } from "@heroui/react";
import { Activity, TrendingUp, BarChart3, Clock } from "lucide-react";

import { formatNumber } from "../utils/formatters";
import { AnalysisStatus, FilterStats } from "../services/api";

interface DashboardStatsProps {
  analysisStatus: AnalysisStatus | null;
  filterStats: FilterStats | null;
  isRunning: boolean;
}

const DashboardStats: React.FC<DashboardStatsProps> = ({
  analysisStatus,
  filterStats,
  isRunning,
}) => {
  const getOpportunityRate = (): string => {
    if (!analysisStatus || analysisStatus.analyzed_count === 0) return "0";

    return (
      (analysisStatus.opportunities_found / analysisStatus.analyzed_count) *
      100
    ).toFixed(1);
  };

  const getStatusColor = (
    status?: string,
  ): "success" | "primary" | "danger" | "default" => {
    switch (status) {
      case "completed":
        return "success";
      case "running":
        return "primary";
      case "error":
        return "danger";
      default:
        return "default";
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      {/* Analysis Progress */}
      <Card className="shadow-md">
        <CardBody className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-default-600">
                Analysis Progress
              </p>
              <p className="text-2xl font-bold text-foreground">
                {analysisStatus
                  ? `${analysisStatus.analyzed_count}/${analysisStatus.total_count}`
                  : "0/0"}
              </p>
            </div>
            <div
              className={`p-3 rounded-full ${
                isRunning ? "bg-primary-100" : "bg-default-100"
              }`}
            >
              <Activity
                className={`h-6 w-6 ${
                  isRunning ? "text-primary" : "text-default-600"
                }`}
              />
            </div>
          </div>
          {analysisStatus && (
            <div className="mt-4">
              <div className="flex justify-between text-sm text-default-600 mb-2">
                <span>Progress</span>
                <span>{(analysisStatus.progress * 100).toFixed(1)}%</span>
              </div>
              <Progress
                color="primary"
                size="sm"
                value={analysisStatus.progress * 100}
              />
            </div>
          )}
        </CardBody>
      </Card>

      {/* Opportunities Found */}
      <Card className="shadow-md">
        <CardBody className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-default-600">
                Opportunities Found
              </p>
              <p className="text-2xl font-bold text-success">
                {analysisStatus?.opportunities_found || 0}
              </p>
            </div>
            <div className="p-3 bg-success-100 rounded-full">
              <TrendingUp className="h-6 w-6 text-success" />
            </div>
          </div>
          <div className="mt-4">
            <span className="text-sm text-default-600">
              Success Rate:{" "}
              <span className="font-medium text-success">
                {getOpportunityRate()}%
              </span>
            </span>
          </div>
        </CardBody>
      </Card>

      {/* Filtered Tickers */}
      <Card className="shadow-md">
        <CardBody className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-default-600">
                Filtered Tickers
              </p>
              <p className="text-2xl font-bold text-secondary">
                {formatNumber(filterStats?.filtered_tickers)}
              </p>
            </div>
            <div className="p-3 bg-secondary-100 rounded-full">
              <BarChart3 className="h-6 w-6 text-secondary" />
            </div>
          </div>
          <div className="mt-4">
            <span className="text-sm text-default-600">
              From {formatNumber(filterStats?.total_tickers)} total
            </span>
          </div>
        </CardBody>
      </Card>

      {/* Status */}
      <Card className="shadow-md">
        <CardBody className="p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-default-600">Status</p>
              <div className="flex items-center gap-2 mt-1">
                <Chip
                  color={getStatusColor(analysisStatus?.status)}
                  size="sm"
                  variant="flat"
                >
                  {analysisStatus?.status?.toUpperCase() || "READY"}
                </Chip>
              </div>
            </div>
            <div
              className={`p-3 rounded-full ${
                analysisStatus?.status === "completed"
                  ? "bg-success-100"
                  : analysisStatus?.status === "running"
                    ? "bg-primary-100"
                    : analysisStatus?.status === "error"
                      ? "bg-danger-100"
                      : "bg-default-100"
              }`}
            >
              <Clock
                className={`h-6 w-6 ${
                  analysisStatus?.status === "completed"
                    ? "text-success"
                    : analysisStatus?.status === "running"
                      ? "text-primary"
                      : analysisStatus?.status === "error"
                        ? "text-danger"
                        : "text-default-600"
                }`}
              />
            </div>
          </div>
          {analysisStatus?.error_message && (
            <div className="mt-2">
              <span className="text-sm text-danger">
                {analysisStatus.error_message}
              </span>
            </div>
          )}
        </CardBody>
      </Card>
    </div>
  );
};

export default DashboardStats;
