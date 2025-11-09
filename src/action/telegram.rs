use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Message {
    pub chat_id: i64,
    pub text: String,
}

// {
//     "ok": true,
//     "result": {
//         "message_id": 4,
//         "from": {
//             "id": 8217261603,
//             "is_bot": true,
//             "first_name": "tide",
//             "username": "crypto_tides_bot"
//         },
//         "chat": {
//             "id": -5030115587,
//             "title": "鱼头泡饼",
//             "type": "group",
//             "all_members_are_administrators": true,
//             "accepted_gift_types": {
//                 "unlimited_gifts": false,
//                 "limited_gifts": false,
//                 "unique_gifts": false,
//                 "premium_subscription": false
//             }
//         },
//         "date": 1762672037,
//         "text": "hello"
//     }
// }
#[derive(Deserialize)]
struct TelegramResponse {
    pub ok: bool,
    #[allow(dead_code)]
    pub result: Option<MessageResult>,
}

#[derive(Deserialize)]
struct MessageResult {
    #[allow(dead_code)]
    pub message_id: i64,
    #[allow(dead_code)]
    pub from: BotInfo,
    #[allow(dead_code)]
    pub chat: ChatInfo,
    #[allow(dead_code)]
    pub date: i64,
    #[allow(dead_code)]
    pub text: String,
}

#[derive(Deserialize)]
struct BotInfo {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub is_bot: bool,
    #[allow(dead_code)]
    pub first_name: String,
    #[allow(dead_code)]
    pub username: String,
}

#[derive(Deserialize)]
struct ChatInfo {
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub title: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub chat_type: String,
    #[allow(dead_code)]
    pub all_members_are_administrators: bool,
}

pub async fn broadcast(text: &String) -> bool {
    let msg = Message {
        chat_id: -5030115587,
        text: text.clone(),
    };
    let msg = json!(msg);
    let client = reqwest::Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", "8217261603:AAEgKcLQ7inKaaL8EeSIUnTmGszU6B0rc7w");
    
    let resp = client.post(&url)
        .json(&msg)
        .send()
        .await;
    
    match resp {
        Ok(response) => {
            match response.json::<TelegramResponse>().await {
                Ok(telegram_response) => {
                    // 校验返回结果中的ok字段是否为true
                    telegram_response.ok
                },
                Err(_) => false, // JSON解析失败
            }
        },
        Err(_) => false, // HTTP请求失败
    }
}