# Auto Stock Analyser - React Frontend

A modern React dashboard for real-time stock market analysis, providing an intuitive interface to monitor and analyze stock opportunities.

## Features

### 🎯 Real-time Analysis Dashboard
- **Live Progress Tracking**: Watch your analysis progress in real-time
- **Opportunity Detection**: Highlight potential buy/sell opportunities
- **Advanced Filtering**: Customize analysis criteria on-the-fly
- **Interactive Charts**: Visualize RSI distribution and market trends

### 📊 Comprehensive Stock Metrics
- **Technical Indicators**: RSI, SMA 20/50, MACD analysis
- **Market Data**: Real-time prices, volume, market cap
- **Trading Signals**: Oversold/overbought alerts
- **Performance Tracking**: Daily percentage changes

### 🎨 Modern UI/UX
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Real-time Updates**: Live data streaming during analysis
- **Intuitive Filters**: Easy-to-use filter panel
- **Beautiful Charts**: Interactive data visualizations

## Quick Start

### Prerequisites
- Node.js 16+ 
- npm or yarn
- Rust backend server running on port 3001

### Installation

1. **Navigate to frontend directory**:
```bash
cd frontend
```

2. **Install dependencies**:
```bash
npm install
```

3. **Start the development server**:
```bash
npm start
```

The React app will be available at `http://localhost:3000`

### Backend Integration

The frontend connects to the Rust backend API on `http://127.0.0.1:3001`. Make sure to start the backend server first:

```bash
# In the main project directory
cargo run --bin server
```

## Usage Guide

### 1. Configure Filters
- Click the "Filters" button to open the filter panel
- Set price ranges, market cap, volume, and RSI thresholds
- Watch the "filtered tickers" count update in real-time

### 2. Start Analysis
- Click "Start Analysis" to begin real-time stock analysis
- Monitor progress in the dashboard stats
- Watch opportunities appear in the results table

### 3. View Results
- Filter results by opportunity type (All, Opportunities, Oversold, Overbought)
- Sort by RSI, price, volume, or percentage change
- View detailed technical indicators for each stock

### 4. Monitor Signals
- Green badges indicate buy opportunities (oversold)
- Red badges indicate sell warnings (overbought)
- Blue badges highlight general opportunities

## Available Scripts

- `npm start` - Start development server
- `npm build` - Build for production
- `npm test` - Run tests
- `npm eject` - Eject from Create React App

## Project Structure

```
frontend/
├── public/                 # Static files
├── src/
│   ├── components/        # React components
│   │   ├── DashboardStats.js    # Main dashboard statistics
│   │   ├── FilterPanel.js       # Analysis filters sidebar
│   │   └── AnalysisResults.js   # Results table and charts
│   ├── services/         # API services
│   │   └── api.js        # Backend API integration
│   ├── App.js           # Main application component
│   ├── index.js         # Application entry point
│   └── index.css        # Global styles
├── package.json         # Dependencies and scripts
└── tailwind.config.js   # Tailwind CSS configuration
```

## API Integration

The frontend communicates with the Rust backend through these endpoints:

- `GET /api/health` - Health check
- `GET /api/tickers` - Fetch available tickers
- `POST /api/filter-stats` - Get filter statistics
- `POST /api/analysis` - Start analysis session
- `GET /api/analysis/:id` - Get analysis status
- `GET /api/analysis/:id/results` - Get analysis results

## Customization

### Styling
The project uses Tailwind CSS for styling. Customize colors and themes in:
- `tailwind.config.js` - Tailwind configuration
- `src/index.css` - Custom CSS classes

### Components
All components are modular and can be customized:
- `DashboardStats` - Overview metrics and progress
- `FilterPanel` - Analysis configuration
- `AnalysisResults` - Results display and charts

### API Configuration
Update the API base URL in `src/services/api.js` if your backend runs on a different port.

## Deployment

### Development
```bash
npm start
```

### Production Build
```bash
npm run build
```

The build folder will contain the optimized production files ready for deployment.

## Technologies Used

- **React 18** - UI framework
- **Tailwind CSS** - Utility-first CSS framework
- **Recharts** - Chart library for data visualization
- **Axios** - HTTP client for API requests
- **Lucide React** - Modern icon library
- **Headless UI** - Unstyled accessible UI components

## Performance

- **Real-time Updates**: Polls backend every 2 seconds during analysis
- **Optimized Rendering**: Efficient state management and re-rendering
- **Responsive Charts**: Smooth data visualization updates
- **Background Processing**: Non-blocking analysis execution

## Troubleshooting

### Backend Connection Issues
- Ensure the Rust server is running on port 3001
- Check CORS configuration in the backend
- Verify firewall settings

### Performance Issues
- Monitor browser console for errors
- Check network tab for API response times
- Ensure adequate system resources

### UI Issues
- Clear browser cache and reload
- Check for JavaScript console errors
- Verify all dependencies are installed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License.
