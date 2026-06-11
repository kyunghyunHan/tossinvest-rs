use reqwest::Method;

use crate::error::Result;
use crate::models::common::Currency;
use crate::models::market_info::{ExchangeRateResponse, KrMarketCalendarResponse, UsMarketCalendarResponse};
use crate::TossInvestClient;

impl TossInvestClient {
    /// 환율 조회
    ///
    /// - `base_currency`: 기준 통화
    /// - `quote_currency`: 표시 통화
    /// - `date_time`: 특정 시점 환율 조회 (ISO 8601). None이면 현재 환율.
    pub async fn get_exchange_rate(
        &self,
        base_currency: Currency,
        quote_currency: Currency,
        date_time: Option<&str>,
    ) -> Result<ExchangeRateResponse> {
        let base = match base_currency {
            Currency::KRW => "KRW",
            Currency::USD => "USD",
        };
        let quote = match quote_currency {
            Currency::KRW => "KRW",
            Currency::USD => "USD",
        };
        let mut params: Vec<(&str, String)> = vec![
            ("baseCurrency", base.to_string()),
            ("quoteCurrency", quote.to_string()),
        ];
        if let Some(dt) = date_time {
            params.push(("dateTime", dt.to_string()));
        }
        let req = self
            .auth_req(Method::GET, "/api/v1/exchange-rate")
            .query(&params);
        self.send(req).await
    }

    /// 국내 장 운영 정보 조회
    ///
    /// `date`: 조회 기준일 (YYYY-MM-DD). None이면 오늘.
    pub async fn get_kr_market_calendar(&self, date: Option<&str>) -> Result<KrMarketCalendarResponse> {
        let mut params: Vec<(&str, String)> = vec![];
        if let Some(d) = date {
            params.push(("date", d.to_string()));
        }
        let req = self
            .auth_req(Method::GET, "/api/v1/market-calendar/KR")
            .query(&params);
        self.send(req).await
    }

    /// 해외 장 운영 정보 조회
    ///
    /// `date`: 조회 기준일 (YYYY-MM-DD, 미국 현지 기준). None이면 오늘.
    pub async fn get_us_market_calendar(&self, date: Option<&str>) -> Result<UsMarketCalendarResponse> {
        let mut params: Vec<(&str, String)> = vec![];
        if let Some(d) = date {
            params.push(("date", d.to_string()));
        }
        let req = self
            .auth_req(Method::GET, "/api/v1/market-calendar/US")
            .query(&params);
        self.send(req).await
    }
}
