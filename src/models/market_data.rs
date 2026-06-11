use serde::{Deserialize, Serialize};

use super::common::Currency;

/// 호가 항목 (매도/매수 각각)
#[derive(Debug, Clone, Deserialize)]
pub struct OrderbookEntry {
    /// 호가
    pub price: String,
    /// 잔량
    pub volume: String,
}

/// 호가 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookResponse {
    /// 데이터 시각 (데이터 미제공 시 None)
    pub timestamp: Option<String>,
    pub currency: Currency,
    /// 매도호가 목록 (낮은 가격순)
    pub asks: Vec<OrderbookEntry>,
    /// 매수호가 목록 (높은 가격순)
    pub bids: Vec<OrderbookEntry>,
}

/// 현재가 응답 (종목 하나)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceResponse {
    pub symbol: String,
    /// 체결 미발생 등으로 시각이 없을 경우 None
    pub timestamp: Option<String>,
    pub last_price: String,
    pub currency: Currency,
}

/// 체결 내역 항목
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub price: String,
    pub volume: String,
    pub timestamp: String,
    pub currency: Currency,
}

/// 상/하한가 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitResponse {
    pub timestamp: String,
    /// 미국 주식 등 가격제한이 없는 시장에서는 None
    pub upper_limit_price: Option<String>,
    /// 미국 주식 등 가격제한이 없는 시장에서는 None
    pub lower_limit_price: Option<String>,
    pub currency: Currency,
}

/// 캔들 목록 + 페이지네이션 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandlePageResponse {
    pub candles: Vec<Candle>,
    /// 다음 페이지 조회 시 `before` 파라미터에 사용. 마지막 페이지면 None.
    pub next_before: Option<String>,
}

/// 캔들 하나
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    /// 봉 시작 시각
    pub timestamp: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub currency: Currency,
}

/// 캔들 단위
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CandleInterval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "1d")]
    OneDay,
}

impl CandleInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            CandleInterval::OneMinute => "1m",
            CandleInterval::OneDay => "1d",
        }
    }
}
