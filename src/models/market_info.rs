use serde::Deserialize;
use serde_json::Value;

use super::common::Currency;

/// 환율 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRateResponse {
    pub base_currency: Currency,
    pub quote_currency: Currency,
    /// 매수 환율 (1 baseCurrency = ? quoteCurrency)
    pub rate: String,
    /// 매매기준율 (은행간 mid rate)
    pub mid_rate: String,
    /// 매매기준율 대비 basis points
    pub basis_point: String,
    pub rate_change_type: String,
    pub valid_from: String,
    pub valid_until: String,
}

/// 국내 장 운영 정보 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KrMarketCalendarResponse {
    pub today: KrMarketDay,
    pub previous_business_day: KrMarketDay,
    pub next_business_day: KrMarketDay,
}

/// 국내 영업일 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KrMarketDay {
    /// 영업일 (KST 기준, YYYY-MM-DD)
    pub date: String,
    /// 통합 모드(KRX+NXT) 거래 시간. 휴장이면 None.
    pub integrated: Option<Value>,
}

/// 해외 장 운영 정보 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsMarketCalendarResponse {
    pub today: UsMarketDay,
    pub previous_business_day: UsMarketDay,
    pub next_business_day: UsMarketDay,
}

/// 해외 영업일 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsMarketDay {
    /// 영업일 (미국 현지 기준, YYYY-MM-DD)
    pub date: String,
    /// 데이마켓 세션. 휴장이면 None.
    pub day_market: Option<Value>,
    /// 프리마켓 세션. 휴장이면 None.
    pub pre_market: Option<Value>,
    /// 정규장 세션. 휴장이면 None.
    pub regular_market: Option<Value>,
    /// 애프터마켓 세션. 휴장이면 None.
    pub after_market: Option<Value>,
}
