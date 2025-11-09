use std::collections::HashMap;
use log::{info, warn};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Local;

use crate::model::symbol::{Category, Ticker};
use crate::action::{ticker::get_ticker, telegram::broadcast};

pub async fn spot_swap_arbitrage() {
    let mut spot_swap = SpotSwap::new().await;
    info!("spot-swap arbitrage started");
    loop {
        spot_swap.run().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

struct SpotSwap {
    pub spot:Vec<Ticker>,
    pub swap:Vec<Ticker>,
    pub diffs:Vec<Diff>,
}

struct Diff {
    pub base: String,
    pub quote: String,
    pub spot_px: BigDecimal,
    pub swap_px: BigDecimal,
    pub diff: BigDecimal,
    pub diff_rate: BigDecimal,
}

impl SpotSwap {
    pub async fn new() -> Self {
        let spot = match get_ticker(Category::Spot).await{
            Some(tickers) => tickers,
            None => vec![],
        };
        assert!(spot.len() > 0, "fetch spot tickers failed");
        let swap = match get_ticker(Category::Swap).await {
            Some(tickers) => tickers,
            None => vec![],
        };
        assert!(swap.len() > 0, "fetch swap tickers failed");
        SpotSwap {
            spot,
            swap,
            diffs: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        let spot = match get_ticker(Category::Spot).await{
            Some(tickers) => tickers,
            None => vec![],
        };
        if spot.len() == 0 {
            return;
        }
        let swap = match get_ticker(Category::Swap).await {
            Some(tickers) => tickers,
            None => vec![],
        };
        if swap.len() == 0 {
            return;
        }

        let spot_map: HashMap<String, Ticker> = HashMap::from_iter(spot.iter().map(|t| (format!("{}-{}", t.base, t.quote), t.clone())));
        let mut diffs = Vec::new();
        for s in swap.iter() {
            let key = format!("{}-{}", s.base, s.quote);
            if spot_map.contains_key(&key) {
                let spot_ticker = spot_map.get(&key).unwrap();
                let diff = s.last.clone() - spot_ticker.last.clone();
                let diff_rate = &diff / spot_ticker.last.clone();
                let threshold = BigDecimal::from_f64(0.01).unwrap(); // 1% threshold
                if diff_rate.abs() > threshold {
                    let item = Diff {
                        base: s.base.clone(),
                        quote: s.quote.clone(),
                        spot_px: spot_ticker.last.clone(),
                        swap_px: s.last.clone(),
                        diff: diff.clone(),
                        diff_rate: diff_rate.clone(),
                    };
                    diffs.push(item);
                }
            }
        }

        if new_diff(&self.diffs, &diffs) {
            diffs.sort_by(|a, b| b.diff_rate.cmp(&a.diff_rate));
            let mut msg = format!("📊 **现货-合约套利机会** 📊\n⏰ {}\n", Local::now().format("%m-%d %H:%M:%S"));
            
            for (i, d) in diffs.iter().enumerate() {
                let rate_percent = d.diff_rate.clone() * BigDecimal::from_f64(100.0).unwrap();
                let emoji = if d.diff_rate > BigDecimal::from_f64(0.0).unwrap() { "📈" } else { "📉" };
                
                // 格式化价格显示，保留4位小数
                let spot_px_formatted = format!("{:.4}", d.spot_px);
                let swap_px_formatted = format!("{:.4}", d.swap_px);
                let diff_formatted = format!("{:.4}", d.diff);
                
                msg += &format!(
                    "{} **{}{}**\n💰 现货: `{}`\n🔄 合约: `{}`\n📊 差价: `{}` ({:.2}%)\n", 
                    emoji,
                    d.base, 
                    d.quote,
                    spot_px_formatted,
                    swap_px_formatted, 
                    diff_formatted,
                    rate_percent
                );
                
                // 在每个条目之间添加分隔线，除了最后一个
                if i < diffs.len() - 1 {
                    msg += "➖➖➖➖➖➖➖➖➖➖\n";
                }
            }
            
            msg += &format!("\n📝 共发现 {} 个套利机会", diffs.len());

            if !broadcast(&msg).await{
                warn!("sent telegram message failed:\n{}", msg);
            }
            self.diffs = diffs;
        }

        self.spot = spot;
        self.swap = swap;
    }
}

fn new_diff(old:&Vec<Diff>, new:&Vec<Diff>) -> bool {
    if old.len() != new.len() {
        return true;
    }
    let new_set: std::collections::HashSet<String> = new.iter().map(|d| format!("{}-{}", d.base, d.quote)).collect();
    let old_set: std::collections::HashSet<String> = old.iter().map(|d| format!("{}-{}", d.base, d.quote)).collect();
    for item in new_set.iter() {
        if !old_set.contains(item) {
            return true;
        }
    }
    for item in old_set.iter() {
        if !new_set.contains(item) {
            return true;
        }
    }
    return false;
}