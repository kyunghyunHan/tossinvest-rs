use serde::{Deserialize, Serialize};

/// 통화 코드
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    KRW,
    USD,
}

/// 시장 국가 코드
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarketCountry {
    KR,
    US,
}
