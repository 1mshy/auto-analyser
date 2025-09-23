# Auto Stock Analyser - HeroUI Frontend

This is the migrated frontend for the Auto Stock Analyser application, built with HeroUI components and Tailwind CSS.

## Features

- **Real-time Analysis Dashboard**: Monitor stock analysis progress with live updates
- **Advanced Filtering**: Comprehensive filter options for market cap, price, volume, RSI, and more
- **Interactive Results**: Sortable and filterable analysis results with visual indicators
- **Responsive Design**: Mobile-friendly interface using HeroUI components
- **Chart Visualization**: RSI distribution charts using Recharts

## Technology Stack

- **React 18** with TypeScript
- **HeroUI** - Modern React UI library
- **Tailwind CSS** - Utility-first CSS framework
- **Vite** - Fast build tool and dev server
- **Recharts** - Charts and data visualization
- **Lucide React** - Beautiful icons
- **Axios** - HTTP client for API communication

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Components

### Core Components

- `DashboardStats.tsx` - Statistics cards showing analysis progress, opportunities found, etc.
- `FilterPanel.tsx` - Comprehensive filtering interface with various stock criteria  
- `AnalysisResults.tsx` - Interactive table showing analysis results with charts

### Services

- `api.ts` - API service layer with TypeScript interfaces for backend communication
- `formatters.ts` - Utility functions for number, currency, and data formatting

## Configuration

The app expects the backend API to be running on port 3001. You can modify the API base URL in `src/services/api.ts`.

```bash
public-hoist-pattern[]=*@heroui/*
```

After modifying the `.npmrc` file, you need to run `pnpm install` again to ensure that the dependencies are installed correctly.

## License

Licensed under the [MIT license](https://github.com/heroui-inc/vite-template/blob/main/LICENSE).
