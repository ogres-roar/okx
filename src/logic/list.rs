use crate::action;
use crate::model::symbol::{Category, Symbol};

use log::{warn, info};

pub async fn list() {
    let mut spot_swap_list = List::new().await;
    info!("listing symbols started");
    loop {
        spot_swap_list.run().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

struct List {
    pub spot: Vec<Symbol>,
    pub swap: Vec<Symbol>,
}

impl List {
    async fn new() -> Self {
        let spot = match action::instrument::get_symbols(Category::Spot).await {
            Some(symbols) => symbols,
            None => vec![],        
        };
        // 初始化阶段使用assert确保获取到数据
        assert!(!spot.is_empty(), "Failed to fetch spot symbols");
        let swap = match action::instrument::get_symbols(Category::Swap).await {
            Some(symbols) => symbols,
            None => vec![],        
        };
        // 初始化阶段使用assert确保获取到数据
        assert!(!swap.is_empty(), "Failed to fetch swap symbols");
        List { spot, swap }
    }


    // 现货 - 永续间套利
    // 可以杠杆交易的对子 179, 可以现货交易的对子 703
    // 所以这里先只看现货和期货 价差
    async fn run(&mut self) {
        // 获取现货交易对列表
        let spot_symbols = match action::instrument::get_symbols(Category::Spot).await{
            Some(symbols) => symbols,
            None => vec![],
        };
        if spot_symbols.is_empty() {
            warn!("read spot symbols failed");
            return;
        }
        let new_spot_symbols = new_symbols(&self.spot, &spot_symbols);
        if new_spot_symbols.len() > 0 {
            info!("new spot symbols: {:?}", new_spot_symbols);
        }
        self.spot = spot_symbols;
        let swap_symbols = match action::instrument::get_symbols(Category::Swap).await{
            Some(symbols) => symbols,
            None => vec![],
        };
        if swap_symbols.is_empty() {
            warn!("read swap symbols failed");
            return;
        }
        let new_swap_symbols = new_symbols(&self.swap, &swap_symbols);
        if new_swap_symbols.len() > 0 {
            info!("new swap symbols: {:?}", new_swap_symbols);
        }
        self.swap = swap_symbols;
    }
}

/// 找出secound中新增的交易对
pub fn new_symbols(first: &Vec<Symbol>, second: &Vec<Symbol>) -> Vec<Symbol> {
    let mut removed: Vec<Symbol> = Vec::new();
    let mut symbol_set = std::collections::HashSet::new();
    for sym in first {
        symbol_set.insert(format!("{}-{}", sym.base, sym.quote));
    }
    for sym in second {
        let key = format!("{}-{}", sym.base, sym.quote);
        if !symbol_set.contains(&key) {
            removed.push(sym.clone());
        }
    }
    return removed;
}

// 找出 first 和 second 中共有的交易对
pub fn common_symbols(first: &Vec<Symbol>, second: &Vec<Symbol>) -> Vec<Symbol> {
    let mut common = Vec::new();
    let mut symbol_set = std::collections::HashSet::new();
    for sym in first {
        symbol_set.insert(format!("{}-{}", sym.base, sym.quote));
    }
    for sym in second {
        let key = format!("{}-{}", sym.base, sym.quote);
        if symbol_set.contains(&key) {
            common.push(sym.clone());
        }
    }
    return common;
}