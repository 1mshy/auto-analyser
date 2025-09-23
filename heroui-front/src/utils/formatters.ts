export const formatNumber = (num: number | null | undefined): string => {
  if (num === null || num === undefined) return "0";
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;

  return num.toString();
};

export const formatCurrency = (value: number | null | undefined): string => {
  if (!value) return "N/A";

  return new Intl.NumberFormat("en-US", {
    currency: "USD",
    maximumFractionDigits: 2,
    minimumFractionDigits: 2,
    style: "currency",
  }).format(value);
};

export const formatVolume = (volume: number | null | undefined): string => {
  if (!volume) return "N/A";
  if (volume >= 1000000) return `${(volume / 1000000).toFixed(1)}M`;
  if (volume >= 1000) return `${(volume / 1000).toFixed(1)}K`;

  return volume.toString();
};

export const getRSIColor = (rsi: number | null | undefined): string => {
  if (!rsi) return "text-gray-400";
  if (rsi <= 30) return "text-success";
  if (rsi >= 70) return "text-danger";

  return "text-warning";
};

export const getRSIStatus = (
  rsi: number | null | undefined,
): "success" | "danger" | "warning" | "default" => {
  if (!rsi) return "default";
  if (rsi <= 30) return "success";
  if (rsi >= 70) return "danger";

  return "warning";
};
