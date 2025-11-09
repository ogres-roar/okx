use serde::{Deserialize, Serialize};

use crate::{global, model::symbol::{Category, State, Symbol}};

/// /api/v5/public/instruments 接口的返回结果
/// instType	String	产品类型
/// instId	String	产品id， 如 BTC-USDT
/// uly	String	标的指数，如 BTC-USD，仅适用于杠杆/交割/永续/期权
/// instFamily	String	交易品种，如 BTC-USD，仅适用于杠杆/交割/永续/期权
/// category	String	币种类别（已废弃）
/// baseCcy	String	交易货币币种，如 BTC-USDT 中的 BTC ，仅适用于币币/币币杠杆
/// quoteCcy	String	计价货币币种，如 BTC-USDT 中的USDT ，仅适用于币币/币币杠杆
/// settleCcy	String	盈亏结算和保证金币种，如 BTC 仅适用于交割/永续/期权
/// ctVal	String	合约面值，仅适用于交割/永续/期权
/// ctMult	String	合约乘数，仅适用于交割/永续/期权
/// ctValCcy	String	合约面值计价币种，仅适用于交割/永续/期权
/// optType	String	期权类型，C或P 仅适用于期权
/// stk	String	行权价格，仅适用于期权
/// listTime	String	上线时间
/// Unix时间戳的毫秒数格式，如 1597026383085
/// auctionEndTime	String	集合竞价结束时间，Unix时间戳的毫秒数格式，如 1597026383085
/// 仅适用于通过集合竞价方式上线的币币，其余情况返回""（已废弃，请使用contTdSwTime）
/// contTdSwTime	String	连续交易开始时间，从集合竞价、提前挂单切换到连续交易的时间，Unix时间戳格式，单位为毫秒。e.g. 1597026383085。
/// 仅适用于通过集合竞价或提前挂单上线的SPOT/MARGIN，在其他情况下返回""。
/// preMktSwTime	String	盘前永续合约转为普通永续合约的时间，Unix时间戳的毫秒数格式，如 1597026383085
/// 仅适用于盘前SWAP
/// openType	String	开盘类型
/// fix_price: 定价开盘
/// pre_quote: 提前挂单
/// call_auction: 集合竞价
/// 只适用于SPOT/MARGIN，其他业务线返回""
/// expTime	String	产品下线时间
/// 适用于币币/杠杆/交割/永续/期权，对于 交割/期权，为交割/行权日期；亦可以为产品下线时间，有变动就会推送。
/// lever	String	该instId支持的最大杠杆倍数，不适用于币币、期权
/// tickSz	String	下单价格精度，如 0.0001
/// 对于期权来说，是梯度中的最小下单价格精度，如果想要获取期权价格梯度，请使用"获取期权价格梯度"接口
/// lotSz	String	下单数量精度
/// 合约的数量单位是张，现货的数量单位是交易货币
/// minSz	String	最小下单数量
/// 合约的数量单位是张，现货的数量单位是交易货币
/// ctType	String	合约类型
/// linear：正向合约
/// inverse：反向合约
/// 仅适用于交割/永续
/// alias	String	合约日期别名
/// this_week：本周
/// next_week：次周
/// this_month：本月
/// next_month：次月
/// quarter：季度
/// next_quarter：次季度
/// third_quarter：第三季度
/// 仅适用于交割
/// 不建议使用，用户应通过 expTime 字段获取合约的交割日期
/// state	String	产品状态
/// live：交易中
/// suspend：暂停中
/// preopen：预上线，交割和期权合约轮转生成到开始交易；部分交易产品上线前
/// test：测试中（测试产品，不可交易）
/// ruleType	String	交易规则类型
/// normal：普通交易
/// pre_market：盘前交易
/// maxLmtSz	String	限价单的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是交易货币
/// maxMktSz	String	市价单的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是USDT
/// maxLmtAmt	String	限价单的单笔最大美元价值
/// maxMktAmt	String	市价单的单笔最大美元价值
/// 仅适用于币币/币币杠杆
/// maxTwapSz	String	时间加权单的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是交易货币。
/// 单笔最小委托数量为 minSz*2
/// maxIcebergSz	String	冰山委托的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是交易货币
/// maxTriggerSz	String	计划委托委托的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是交易货币
/// maxStopSz	String	止盈止损市价委托的单笔最大委托数量
/// 合约的数量单位是张，现货的数量单位是USDT
/// futureSettlement	Boolean	交割合约是否支持每日结算
/// 适用于全仓交割
/// tradeQuoteCcyList	Array of strings	可用于交易的计价币种列表，如 ["USD", "USDC”].
/// instIdCode	Integer	产品唯一标识代码。
/// 对于简单二进制编码，您必须使用 instIdCode 而不是 instId。
/// 对于同一instId，实盘和模拟盘的值可能会不一样。
#[derive(Debug, Serialize, Deserialize)]
struct InstSymbol {
    #[serde(rename = "instId")]
    pub inst_id: String,
    #[serde(rename = "instFamily")]
    pub inst_family: String,
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "baseCcy")]
    pub base_ccy: Option<String>,
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: Option<String>,
    #[serde(rename = "settleCcy")]
    pub settle_ccy: Option<String>,
    #[serde(rename = "ctVal")]
    pub ct_val: Option<String>,
    #[serde(rename = "ctMult")]
    pub ct_mult: Option<String>,
    #[serde(rename = "ctValCcy")]
    pub ct_val_ccy: Option<String>,
    #[serde(rename = "listTime")]
    pub list_time: String,
    #[serde(rename = "expTime")]
    pub exp_time: Option<String>,
    pub state: String,
    #[serde(rename = "tickSz")]
    pub tick_sz: String,
    #[serde(rename = "lotSz")]
    pub lot_sz: String,
    #[serde(rename = "minSz")]
    pub min_sz: String,
    pub lever: Option<String>,
    #[serde(rename = "maxLmtSz")]
    pub max_lmt_sz: Option<String>,
    #[serde(rename = "maxMktSz")]
    pub max_mkt_sz: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub code: String,
    pub msg: String,
    data: Vec<InstSymbol>,
}

/// 获取交易对列表
/// /api/v5/public/instruments
pub async fn get_symbols(category: Category) -> Option<Vec<Symbol>> {
    let url = format!("{}/api/v5/public/instruments?instType={}", 
        global::REST_API_HOST,
        category.as_str()
    );

    // 创建 HTTP 客户端
    let client = reqwest::Client::new();
    
    // 发送 GET 请求
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<ApiResponse>().await {
                Ok(api_response) => {
                    if api_response.code == "0" {
                        if category == Category::Spot {
                            Some(api_response.data.into_iter().map(|inst| {
                                Symbol {
                                    inst_id: inst.inst_id,
                                    base: inst.base_ccy.unwrap_or_default().to_uppercase(),
                                    quote: inst.quote_ccy.unwrap_or_default().to_uppercase(),
                                    category: category.clone(),
                                    list_time: inst.list_time.parse().unwrap_or(0),
                                    exp_time: inst.exp_time.unwrap_or("0".to_string()).parse().unwrap_or(0),
                                    state: State::from_str(&inst.state).unwrap_or(State::Suspend),
                                }
                            }).collect())
                        } else if Category::Swap == category || Category::Future == category {
                            Some(api_response.data.into_iter().map(|inst| {
                                let inst_family = inst.inst_family.clone();
                                let inst_family: Vec<&str> = inst_family.split('-').collect();

                                Symbol {
                                    inst_id: inst.inst_id,
                                    base: inst_family[0].to_uppercase().to_string(),
                                    quote: inst_family[1].to_uppercase().to_string(),
                                    category: category.clone(),
                                    list_time: inst.list_time.parse().unwrap_or(0),
                                    exp_time: inst.exp_time.unwrap_or("0".to_string()).parse().unwrap_or(0),
                                    state: State::from_str(&inst.state).unwrap_or(State::Suspend),
                                }
                            }).collect())
                        } else {
                            // 其他类别暂不支持
                            None

                        }
                    } else {
                        eprintln!("API Error: code={}, msg={}", api_response.code, api_response.msg);
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse JSON response: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("HTTP request failed: {}", e);
            None
        }
    }
}