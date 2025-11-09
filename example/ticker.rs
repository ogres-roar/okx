// OKX Ticker API 示例
// 此示例展示如何使用 ticker.rs 中的 get_ticker 函数获取不同类别的市场行情数据

use okx::action::ticker::get_ticker;
use okx::model::symbol::Category;
use tokio;
use bigdecimal::BigDecimal;

#[tokio::main]
async fn main() {
    println!("OKX Ticker API Example");
    println!("======================");
    
    // 示例1: 获取现货市场行情
    println!("\n📊 获取现货市场行情...");
    match get_ticker(Category::Spot).await {
        Some(tickers) => {
            println!("✅ 成功获取到 {} 个现货交易对的行情数据", tickers.len());
            display_top_tickers(&tickers, 5, "现货");
        }
        None => println!("❌ 获取现货行情失败"),
    }
    
    // 示例2: 获取永续合约行情
    println!("\n📊 获取永续合约行情...");
    match get_ticker(Category::Swap).await {
        Some(tickers) => {
            println!("✅ 成功获取到 {} 个永续合约的行情数据", tickers.len());
            display_top_tickers(&tickers, 5, "永续合约");
        }
        None => println!("❌ 获取永续合约行情失败"),
    }
    
    // 示例3: 获取期货行情
    println!("\n📊 获取期货行情...");
    match get_ticker(Category::Future).await {
        Some(tickers) => {
            println!("✅ 成功获取到 {} 个期货合约的行情数据", tickers.len());
            display_top_tickers(&tickers, 3, "期货");
        }
        None => println!("❌ 获取期货行情失败"),
    }
    
    // 示例4: 分析市场数据
    println!("\n📈 市场数据分析...");
    analyze_market_data().await;
    
    // 示例5: 监控特定交易对
    println!("\n🔍 监控特定交易对...");
    monitor_specific_pairs().await;
}

/// 显示前N个交易对的行情信息
fn display_top_tickers(tickers: &[okx::model::symbol::Ticker], count: usize, market_type: &str) {
    println!("\n--- {} 市场前 {} 个交易对 ---", market_type, count);
    
    for (i, ticker) in tickers.iter().take(count).enumerate() {
        let change_24h = calculate_24h_change(&ticker.last, &ticker.open_24h);
        let change_emoji = if change_24h >= BigDecimal::from(0) { "🟢" } else { "🔴" };
        
        println!("{}. {} {}", 
            i + 1, 
            change_emoji, 
            ticker.inst_id
        );
        println!("   💰 最新价格: {}", format_price(&ticker.last));
        println!("   📈 24h变化: {}%", format_percentage(&change_24h));
        println!("   📊 24h成交量: {}", format_volume(&ticker.vol_24h));
        println!("   💹 买一/卖一: {} / {}", 
            format_price(&ticker.bid_px), 
            format_price(&ticker.ask_px)
        );
        println!();
    }
}

/// 计算24小时价格变化百分比
fn calculate_24h_change(current: &BigDecimal, open_24h: &BigDecimal) -> BigDecimal {
    if *open_24h == BigDecimal::from(0) {
        return BigDecimal::from(0);
    }
    
    ((current - open_24h) / open_24h) * BigDecimal::from(100)
}

/// 格式化价格显示
fn format_price(price: &BigDecimal) -> String {
    format!("${:.4}", price)
}

/// 格式化百分比显示
fn format_percentage(percentage: &BigDecimal) -> String {
    format!("{:+.2}", percentage)
}

/// 格式化成交量显示
fn format_volume(volume: &BigDecimal) -> String {
    let vol_f64 = volume.to_string().parse::<f64>().unwrap_or(0.0);
    if vol_f64 >= 1_000_000.0 {
        format!("{:.2}M", vol_f64 / 1_000_000.0)
    } else if vol_f64 >= 1_000.0 {
        format!("{:.2}K", vol_f64 / 1_000.0)
    } else {
        format!("{:.2}", vol_f64)
    }
}

/// 分析市场数据
async fn analyze_market_data() {
    if let Some(spot_tickers) = get_ticker(Category::Spot).await {
        let mut stats = MarketStats::new();
        
        for ticker in &spot_tickers {
            let change_24h = calculate_24h_change(&ticker.last, &ticker.open_24h);
            stats.add_ticker_data(&ticker.base, change_24h);
        }
        
        stats.display_summary();
    }
}

/// 监控特定交易对
async fn monitor_specific_pairs() {
    let target_pairs = vec!["BTC-USDT", "ETH-USDT", "SOL-USDT"];
    
    if let Some(tickers) = get_ticker(Category::Spot).await {
        println!("监控的交易对行情:");
        
        for target in &target_pairs {
            if let Some(ticker) = tickers.iter().find(|t| t.inst_id == *target) {
                let change_24h = calculate_24h_change(&ticker.last, &ticker.open_24h);
                let change_emoji = if change_24h >= BigDecimal::from(0) { "🟢" } else { "🔴" };
                
                println!("  {} {} - {} ({}%)", 
                    change_emoji,
                    ticker.inst_id,
                    format_price(&ticker.last),
                    format_percentage(&change_24h)
                );
                
                // 简单的价格告警逻辑
                check_price_alerts(ticker);
            }
        }
    }
}

/// 简单的价格告警检查
fn check_price_alerts(ticker: &okx::model::symbol::Ticker) {
    let change_24h = calculate_24h_change(&ticker.last, &ticker.open_24h);
    let change_abs = change_24h.abs();
    
    if change_abs > BigDecimal::from(10) {
        println!("  🚨 价格剧烈波动告警: {} 24h变化 {}%", 
            ticker.inst_id, 
            format_percentage(&change_24h)
        );
    }
    
    if change_abs > BigDecimal::from(5) {
        println!("  ⚠️  价格波动提醒: {} 24h变化 {}%", 
            ticker.inst_id, 
            format_percentage(&change_24h)
        );
    }
}

/// 市场统计结构
struct MarketStats {
    total_pairs: usize,
    positive_count: usize,
    negative_count: usize,
    top_gainers: Vec<(String, BigDecimal)>,
    top_losers: Vec<(String, BigDecimal)>,
}

impl MarketStats {
    fn new() -> Self {
        Self {
            total_pairs: 0,
            positive_count: 0,
            negative_count: 0,
            top_gainers: Vec::new(),
            top_losers: Vec::new(),
        }
    }
    
    fn add_ticker_data(&mut self, symbol: &str, change_24h: BigDecimal) {
        self.total_pairs += 1;
        
        if change_24h >= BigDecimal::from(0) {
            self.positive_count += 1;
        } else {
            self.negative_count += 1;
        }
        
        // 更新涨幅榜
        self.top_gainers.push((symbol.to_string(), change_24h.clone()));
        self.top_gainers.sort_by(|a, b| b.1.cmp(&a.1));
        self.top_gainers.truncate(5);
        
        // 更新跌幅榜
        self.top_losers.push((symbol.to_string(), change_24h));
        self.top_losers.sort_by(|a, b| a.1.cmp(&b.1));
        self.top_losers.truncate(5);
    }
    
    fn display_summary(&self) {
        println!("📊 市场概览:");
        println!("  总交易对数: {}", self.total_pairs);
        println!("  上涨: {} ({}%)", 
            self.positive_count, 
            (self.positive_count as f64 / self.total_pairs as f64 * 100.0) as i32
        );
        println!("  下跌: {} ({}%)", 
            self.negative_count,
            (self.negative_count as f64 / self.total_pairs as f64 * 100.0) as i32
        );
        
        println!("\n🚀 涨幅榜前5:");
        for (i, (symbol, change)) in self.top_gainers.iter().enumerate() {
            println!("  {}. {} {}%", i + 1, symbol, format_percentage(change));
        }
        
        println!("\n📉 跌幅榜前5:");
        for (i, (symbol, change)) in self.top_losers.iter().enumerate() {
            println!("  {}. {} {}%", i + 1, symbol, format_percentage(change));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    #[test]
    fn test_calculate_24h_change() {
        let current = BigDecimal::from_str("105.0").unwrap();
        let open_24h = BigDecimal::from_str("100.0").unwrap();
        let change = calculate_24h_change(&current, &open_24h);
        
        assert_eq!(change, BigDecimal::from_str("5.0").unwrap());
    }
    
    #[test]
    fn test_format_price() {
        let price = BigDecimal::from_str("45123.456789").unwrap();
        let formatted = format_price(&price);
        
        assert_eq!(formatted, "$45123.4568");
    }
    
    #[test]
    fn test_format_percentage() {
        let positive = BigDecimal::from_str("5.23").unwrap();
        let negative = BigDecimal::from_str("-3.45").unwrap();
        
        assert_eq!(format_percentage(&positive), "+5.23");
        assert_eq!(format_percentage(&negative), "-3.45");
    }
    
    #[test]
    fn test_format_volume() {
        assert_eq!(format_volume(&BigDecimal::from_str("1234567.89").unwrap()), "1.23M");
        assert_eq!(format_volume(&BigDecimal::from_str("12345.67").unwrap()), "12.35K");
        assert_eq!(format_volume(&BigDecimal::from_str("123.45").unwrap()), "123.45");
    }
    
    #[tokio::test]
    async fn test_ticker_categories() {
        // 测试不同类别的字符串表示
        assert_eq!(Category::Spot.as_str(), "SPOT");
        assert_eq!(Category::Swap.as_str(), "SWAP");
        assert_eq!(Category::Future.as_str(), "FUTURES");
        assert_eq!(Category::Margin.as_str(), "MARGIN");
        assert_eq!(Category::Option.as_str(), "OPTION");
    }
    
    #[test]
    fn test_market_stats() {
        let mut stats = MarketStats::new();
        
        stats.add_ticker_data("BTC", BigDecimal::from_str("5.5").unwrap());
        stats.add_ticker_data("ETH", BigDecimal::from_str("-2.3").unwrap());
        
        assert_eq!(stats.total_pairs, 2);
        assert_eq!(stats.positive_count, 1);
        assert_eq!(stats.negative_count, 1);
    }
}