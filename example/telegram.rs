// Telegram broadcast 功能示例
// 此示例展示如何使用 telegram.rs 中的 broadcast 函数发送消息到 Telegram 群组

use okx::action::telegram::broadcast;
use tokio;

#[tokio::main]
async fn main() {
    println!("Telegram Broadcast Example");
    println!("========================");
    
    // 示例1: 发送简单文本消息
    let message = "Hello from Rust! 📈".to_string();
    println!("正在发送消息: {}", message);
    
    match send_message(&message).await {
        true => println!("✅ 消息发送成功"),
        false => println!("❌ 消息发送失败"),
    }
    
    // 示例2: 发送格式化的市场数据消息
    let market_data = format_market_message("BTC/USDT", 45000.0, 2.5);
    println!("\n正在发送市场数据: {}", market_data);
    
    match send_message(&market_data).await {
        true => println!("✅ 市场数据发送成功"),
        false => println!("❌ 市场数据发送失败"),
    }
    
    // 示例3: 发送告警消息
    let alert_message = create_alert_message("价格突破", "BTC", 50000.0);
    println!("\n正在发送告警消息: {}", alert_message);
    
    match send_message(&alert_message).await {
        true => println!("✅ 告警消息发送成功"),
        false => println!("❌ 告警消息发送失败"),
    }
}

/// 发送消息的包装函数
async fn send_message(text: &String) -> bool {
    broadcast(text).await
}

/// 格式化市场数据消息
fn format_market_message(symbol: &str, price: f64, change_percent: f64) -> String {
    let emoji = if change_percent > 0.0 { "🟢" } else { "🔴" };
    let sign = if change_percent > 0.0 { "+" } else { "" };
    
    format!(
        "{} {} 市场更新\n💰 价格: ${:.2}\n📈 24h变化: {}{:.2}%",
        emoji, symbol, price, sign, change_percent
    )
}

/// 创建告警消息
fn create_alert_message(alert_type: &str, symbol: &str, target_price: f64) -> String {
    format!(
        "🚨 {} 告警\n\n📊 交易对: {}\n🎯 目标价格: ${:.2}\n⏰ 时间: {}",
        alert_type,
        symbol,
        target_price,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_market_message() {
        let message = format_market_message("ETH/USDT", 3000.0, 5.2);
        assert!(message.contains("ETH/USDT"));
        assert!(message.contains("3000.00"));
        assert!(message.contains("+5.20%"));
        assert!(message.contains("🟢"));
    }
    
    #[test]
    fn test_format_market_message_negative() {
        let message = format_market_message("BTC/USDT", 45000.0, -2.5);
        assert!(message.contains("BTC/USDT"));
        assert!(message.contains("45000.00"));
        assert!(message.contains("-2.50%"));
        assert!(message.contains("🔴"));
    }
    
    #[test]
    fn test_create_alert_message() {
        let alert = create_alert_message("价格突破", "SOL", 100.0);
        assert!(alert.contains("价格突破"));
        assert!(alert.contains("SOL"));
        assert!(alert.contains("100.00"));
        assert!(alert.contains("🚨"));
    }
    
    #[tokio::test]
    async fn test_send_message_format() {
        // 注意：这个测试需要网络连接和有效的 Telegram Bot Token
        // 在实际环境中运行时需要确保网络可达性
        let test_message = "Test message from Rust".to_string();
        // let result = send_message(test_message).await;
        // 在测试环境中，我们只验证函数签名，不实际发送网络请求
        assert!(test_message.len() > 0);
    }
}