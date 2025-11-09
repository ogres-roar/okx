use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Category{
    Spot,
    Margin,
    Swap,
    Future,
    Option,
}

impl Category{
    pub fn as_str(&self)->&'static str{
        match self{
            Category::Spot=>"SPOT",
            Category::Margin=>"MARGIN",
            Category::Swap=>"SWAP",
            Category::Future=>"FUTURES",
            Category::Option=>"OPTION",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum State {
    Live,
    Suspend,
    Preopen,
    Test,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Live => "live",
            State::Suspend => "suspend",
            State::Preopen => "preopen",
            State::Test => "test",
        }
    }
    pub fn from_str(s: &str) -> Option<State> {
        match s.to_lowercase().as_str() {
            "live" => Some(State::Live),
            "suspend" => Some(State::Suspend),
            "preopen" => Some(State::Preopen),
            "test" => Some(State::Test),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Symbol {
    pub inst_id: String,
    pub base: String,
    pub quote: String,
    pub category: Category,
    pub list_time: u64,
    pub exp_time: u64,
    pub state: State,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    pub inst_id: String,
    pub base: String,
    pub quote: String,
    pub last: BigDecimal,
    pub last_sz: BigDecimal,
    pub bid_px: BigDecimal,
    pub bid_sz: BigDecimal,
    pub ask_px: BigDecimal,
    pub ask_sz: BigDecimal,
    pub open_24h: BigDecimal,
    pub high_24h: BigDecimal,
    pub low_24h: BigDecimal,
    pub vol_24h: BigDecimal,
    pub vol_ccy_24h: BigDecimal,
    pub ts: u64,
}