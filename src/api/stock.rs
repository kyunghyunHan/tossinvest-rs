use reqwest::Method;

use crate::error::Result;
use crate::models::stock::{StockInfo, StockWarning};
use crate::TossInvestClient;

impl TossInvestClient {
    /// 종목 기본 정보 조회 (최대 200종목)
    pub async fn get_stocks(&self, symbols: &[&str]) -> Result<Vec<StockInfo>> {
        let joined = symbols.join(",");
        let req = self
            .auth_req(Method::GET, "/api/v1/stocks")
            .query(&[("symbols", &joined)]);
        self.send(req).await
    }

    /// 종목 매수 유의사항 조회
    pub async fn get_stock_warnings(&self, symbol: &str) -> Result<Vec<StockWarning>> {
        let path = format!("/api/v1/stocks/{}/warnings", symbol);
        let req = self.auth_req(Method::GET, &path);
        self.send(req).await
    }
}
