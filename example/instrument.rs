//! OKX Instruments API Example
//! 
//! 这个示例演示了如何使用 action/instrument.rs 中的 get_symbols 函数
//! 来获取不同类型的交易产品信息：SPOT, MARGIN, SWAP, FUTURES
//! 
//! 对于 OPTION 类别，由于API需要额外的参数，这里暂时跳过

use okx::action::instrument::get_symbols;
use okx::model::symbol::Category;

#[tokio::main]
async fn main() {
    println!("=== OKX Instruments API Example ===\n");
    
    // 定义要测试的所有类别
    let categories = vec![
        Category::Spot,
        Category::Margin, 
        Category::Swap,
        Category::Future,
        // 注意：Option 类别需要额外的参数，这里暂时跳过
        // Category::Option,
    ];
    
    // 对每个类别执行 GET 请求并打印结果
    for category in categories {
        println!("🔍 正在获取 {} 交易对信息...", category.as_str());
        
        match get_symbols(category.clone()).await {
            Some(symbols) => {
                assert!(!symbols.is_empty(), "Expected non-empty symbols list");
                println!("✅ 成功获取 {} 个 {} 交易对:", symbols.len(), category.as_str());
                
                // 打印前5个交易对作为示例
                let display_count = std::cmp::min(5, symbols.len());
                for (i, symbol) in symbols.iter().take(display_count).enumerate() {
                    match category {
                        Category::Spot | Category::Margin => {
                            println!("  {}. 交易对: {}, 基础货币: {}, 计价货币: {}, 状态: {:?}", 
                                i + 1,
                                symbol.inst_id,
                                symbol.base,
                                symbol.quote,
                                symbol.state
                            );
                        }
                        Category::Swap | Category::Future => {
                            // 对于 SWAP 和 FUTURES，显示合约ID
                            println!("  {}. 合约ID: {}, 基础货币: {}, 计价货币: {}, 状态: {:?}", 
                                i + 1,
                                symbol.inst_id,
                                symbol.base,
                                symbol.quote,
                                symbol.state
                            );
                        }
                        _ => {
                            println!("  {}. 产品ID: {}, 状态: {:?}", i + 1, symbol.inst_id, symbol.state);
                        }
                    }
                }
                
                if symbols.len() > 5 {
                    println!("  ... 还有 {} 个交易对", symbols.len() - 5);
                }
                
                println!();
            }
            None => {
                println!("❌ 获取 {} 交易对失败", category.as_str());
                println!();
            }
        }
    }
    
    // 特别处理 OPTION 类别 - 需要指定 uly 参数
    println!("🔍 正在获取 OPTION 交易对信息 (需要特殊处理)...");
    println!("ℹ️  注意：OPTION 类别需要额外的 uly 或 instFamily 参数");
    println!("   可以通过修改 get_symbols 函数来支持这些参数");
    println!();
    
    println!("=== 示例执行完成 ===");
}
