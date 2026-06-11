use reqwest::Method;

use crate::error::Result;
use crate::models::market_data::{CandleInterval, CandlePageResponse, OrderbookResponse, PriceLimitResponse, PriceResponse, Trade};
use crate::TossInvestClient;

impl TossInvestClient {
    /// 호가 조회
    ///
    /// `symbol`: KRX 6자리 숫자(`005930`) 또는 US 티커(`AAPL`)
    pub async fn get_orderbook(&self, symbol: &str) -> Result<OrderbookResponse> {
        let req = self
            .auth_req(Method::GET, "/api/v1/orderbook")
            .query(&[("symbol", symbol)]);
        self.send(req).await
    }

    /// 현재가 일괄 조회 (최대 200종목)
    ///
    /// `symbols`: 콤마 구분 없이 슬라이스로 전달 (`&["005930", "AAPL"]`)
    pub async fn get_prices(&self, symbols: &[&str]) -> Result<Vec<PriceResponse>> {
        let joined = symbols.join(",");
        let req = self
            .auth_req(Method::GET, "/api/v1/prices")
            .query(&[("symbols", &joined)]);
        self.send(req).await
    }

    /// 최근 체결 내역 조회
    ///
    /// `count`: 조회 건수 (기본값 50, 최대 50)
    pub async fn get_trades(&self, symbol: &str, count: Option<u32>) -> Result<Vec<Trade>> {
        let mut params: Vec<(&str, String)> = vec![("symbol", symbol.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let req = self
            .auth_req(Method::GET, "/api/v1/trades")
            .query(&params);
        self.send(req).await
    }

    /// 상/하한가 조회
    pub async fn get_price_limits(&self, symbol: &str) -> Result<PriceLimitResponse> {
        let req = self
            .auth_req(Method::GET, "/api/v1/price-limits")
            .query(&[("symbol", symbol)]);
        self.send(req).await
    }

    /// 캔들 차트 조회
    ///
    /// - `interval`: [`CandleInterval::OneMinute`] 또는 [`CandleInterval::OneDay`]
    /// - `count`: 조회 봉 수 (최대 200)
    /// - `before`: 페이지네이션 상한 (ISO 8601). `CandlePageResponse::next_before` 값을 사용.
    /// - `adjusted`: 수정주가 적용 여부
    pub async fn get_candles(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        before: Option<&str>,
        adjusted: Option<bool>,
    ) -> Result<CandlePageResponse> {
        let mut params: Vec<(&str, String)> = vec![
            ("symbol", symbol.to_string()),
            ("interval", interval.as_str().to_string()),
        ];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        if let Some(b) = before {
            params.push(("before", b.to_string()));
        }
        if let Some(a) = adjusted {
            params.push(("adjusted", a.to_string()));
        }
        let req = self
            .auth_req(Method::GET, "/api/v1/candles")
            .query(&params);
        self.send(req).await
    }
}
