use std::collections::HashMap;
use log::info;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::Local;

use crate::model::symbol::{Category, Ticker};
use crate::action::ticker::get_ticker;

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

        diffs.sort_by(|a, b| b.diff_rate.cmp(&a.diff_rate));

        if diffs.len() > 0 {
            println!("========= Spot-Swap Arbitrage Opportunities [{}] ==========", Local::now().format("%Y-%m-%d %H:%M:%S"));
            for d in diffs.iter() {
                println!("{}-{} | spot: {} | swap: {} | diff: {} | diff_rate: {:.4}%", 
                    d.base, d.quote, d.spot_px, d.swap_px, d.diff, d.diff_rate.clone() * BigDecimal::from_f64(100.0).unwrap());
            }
            println!("");
        }

        self.spot = spot;
        self.swap = swap;
    }
}
