use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::Currency;

/// 종목 기본 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockInfo {
    pub symbol: String,
    /// 종목명 (한글)
    pub name: String,
    pub english_name: String,
    /// 국제증권식별번호 (ISO 6166)
    pub isin_code: String,
    /// 상장 시장 (KOSPI, KOSDAQ, NASDAQ 등)
    pub market: String,
    pub security_type: String,
    /// 보통주 여부 (우선주는 false)
    pub is_common_share: bool,
    pub status: String,
    pub currency: Currency,
    /// 상장일 (YYYY-MM-DD). 정보 미제공 시 None.
    pub list_date: Option<String>,
    /// 상장폐지일 (YYYY-MM-DD). 활성 종목은 None.
    pub delist_date: Option<String>,
    pub shares_outstanding: String,
    /// 레버리지 배수. ETF/ETN에만 적용.
    pub leverage_factor: Option<String>,
    /// 국내 시장 상세 정보. 국내 종목에만 제공.
    pub korean_market_detail: Option<Value>,
}

/// 매수 유의사항
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockWarning {
    pub warning_type: String,
    /// 거래소 코드 (KRX, NXT 등)
    pub exchange: Option<String>,
    /// 적용 시작일 (YYYY-MM-DD). 미정 시 None.
    pub start_date: Option<String>,
    /// 적용 종료일 (YYYY-MM-DD). 진행 중이거나 미정 시 None.
    pub end_date: Option<String>,
}
