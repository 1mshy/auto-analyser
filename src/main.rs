use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let resp = provider.search_ticker("oracle").await.unwrap();
    let historical_data = provider.get_quote_range("AAPL", "1d", "max").await.unwrap();
    let quotes = historical_data.quotes().unwrap();
    println!("AAPL historical data:");
    for quote in quotes {
        println!("{:?}", quote);
    }
    println!("All tickers found while searching for 'Oracle':");
    for item in resp.quotes
    {
        println!("{}", item.symbol)
    }
}

