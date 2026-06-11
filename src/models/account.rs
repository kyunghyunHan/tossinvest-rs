use serde::Deserialize;
use serde_json::Value;

use super::common::{Currency, MarketCountry};

/// 계좌 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_no: String,
    /// 계좌 식별 키. 주문 등 API 호출 시 사용.
    pub account_seq: i64,
    pub account_type: String,
}

/// 보유 주식 전체 요약
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingsOverview {
    /// 투자원금 (통화별 합산)
    pub total_purchase_amount: Value,
    pub market_value: Value,
    pub profit_loss: Value,
    pub daily_profit_loss: Value,
    pub items: Vec<HoldingsItem>,
}

/// 보유 주식 항목 하나
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingsItem {
    pub symbol: String,
    pub name: String,
    pub market_country: MarketCountry,
    pub currency: Currency,
    pub quantity: String,
    pub last_price: String,
    pub average_purchase_price: String,
    pub market_value: HoldingsMarketValue,
    pub profit_loss: ProfitLoss,
    pub daily_profit_loss: DailyProfitLoss,
    pub cost: Cost,
}

/// 시장 평가금액
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldingsMarketValue {
    pub purchase_amount: String,
    pub amount: String,
    pub amount_after_cost: String,
}

/// 손익
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfitLoss {
    pub amount: String,
    pub amount_after_cost: String,
    /// 손익률 (소수비율, 예: 0.1077 = 10.77%)
    pub rate: String,
    pub rate_after_cost: String,
}

/// 일간 손익
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyProfitLoss {
    pub amount: String,
    /// 일간 손익률 (소수비율)
    pub rate: String,
}

/// 수수료/세금
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub commission: String,
    pub tax: Option<String>,
}

/// 매수 가능 금액 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyingPowerResponse {
    pub currency: Currency,
    /// 현금 기반 매수 가능 금액 (미수 미발생 기준)
    pub cash_buying_power: String,
}

/// 판매 가능 수량 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SellableQuantityResponse {
    /// KR: 정수 (주 단위), US: 소수점 포함 가능
    pub sellable_quantity: String,
}

/// 수수료 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commission {
    pub market_country: MarketCountry,
    /// 수수료율 (%, 예: 0.015 = 0.015%)
    pub commission_rate: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}
