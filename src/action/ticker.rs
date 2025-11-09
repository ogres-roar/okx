use crate::global;
use crate::model::symbol::Category;
use crate::model::symbol::Ticker;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct PriceResponse {
    pub code: String,
    pub msg: String,
    pub data: Vec<InstTicker>,
}

#[derive(Serialize, Deserialize)]
struct InstTicker {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    pub last: String,
    #[serde(rename = "lastSz")]
    pub last_sz: String,
    #[serde(rename = "bidPx")]
    pub bid_px: String,
    #[serde(rename = "bidSz")]
    pub bid_sz: String,
    #[serde(rename = "askPx")]
    pub ask_px: String,
    #[serde(rename = "askSz")]
    pub ask_sz: String,
    #[serde(rename = "open24h")]
    pub open_24h: String,
    #[serde(rename = "high24h")]
    pub high_24h: String,
    #[serde(rename = "low24h")]
    pub low_24h: String,
    #[serde(rename = "vol24h")]
    pub vol_24h: String,
    #[serde(rename = "volCcy24h")]
    pub vol_ccy_24h: String,
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    pub ts: String,
}
impl InstTicker {
    fn to_ticker(&self) -> Option<Ticker> {
        let parse_decimal = |s: &str| -> Option<BigDecimal> {
            BigDecimal::from_str(s).ok()
        };
        
        let parse_u64 = |s: &str| -> Option<u64> {
            s.parse::<u64>().ok()
        };

        // Extract base and quote from inst_id (e.g., "BTC-USD-SWAP" -> base: "BTC", quote: "USD")
        let parts: Vec<&str> = self.inst_id.split('-').collect();
        let (base, quote) = if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            return None;
        };

        Some(Ticker {
            inst_id: self.inst_id.clone(),
            base,
            quote,
            last: parse_decimal(&self.last)?,
            last_sz: parse_decimal(&self.last_sz)?,
            bid_px: parse_decimal(&self.bid_px)?,
            bid_sz: parse_decimal(&self.bid_sz)?,
            ask_px: parse_decimal(&self.ask_px)?,
            ask_sz: parse_decimal(&self.ask_sz)?,
            open_24h: parse_decimal(&self.open_24h)?,
            high_24h: parse_decimal(&self.high_24h)?,
            low_24h: parse_decimal(&self.low_24h)?,
            vol_24h: parse_decimal(&self.vol_24h)?,
            vol_ccy_24h: parse_decimal(&self.vol_ccy_24h)?,
            ts: parse_u64(&self.ts)?,
        })
    }
}

pub async fn get_ticker(category: Category) -> Option<Vec<Ticker>> {
    let url = format!("{}/api/v5/market/tickers?instType={}", 
        global::REST_API_HOST,
        category.as_str()
    );
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.ok()?;
    let response_text = response.text().await.ok()?;
    
    let price_response: PriceResponse = serde_json::from_str(&response_text).ok()?;
    
    // Check if the response is successful (code "0")
    if price_response.code != "0" {
        return None;
    }
    
    let mut tickers = Vec::new();
    for inst_ticker in price_response.data {
        if let Some(ticker) = inst_ticker.to_ticker() {
            tickers.push(ticker);
        }
    }
    
    Some(tickers)
}