use reqwest::Method;

use crate::error::Result;
use crate::models::account::{Account, BuyingPowerResponse, Commission, HoldingsOverview, SellableQuantityResponse};
use crate::models::common::Currency;
use crate::TossInvestClient;

impl TossInvestClient {
    /// 계좌 목록 조회
    ///
    /// 반환된 `Account::account_seq`를 [`TossInvestClient::with_account_seq`]에 전달하세요.
    pub async fn get_accounts(&self) -> Result<Vec<Account>> {
        let req = self.auth_req(Method::GET, "/api/v1/accounts");
        self.send(req).await
    }

    /// 보유 주식 조회
    ///
    /// `symbol`: 특정 종목만 조회. None이면 전체 조회.
    /// `X-Tossinvest-Account` 헤더가 필요하므로 `with_account_seq()`를 먼저 호출하세요.
    pub async fn get_holdings(&self, symbol: Option<&str>) -> Result<HoldingsOverview> {
        let mut params: Vec<(&str, String)> = vec![];
        if let Some(s) = symbol {
            params.push(("symbol", s.to_string()));
        }
        let req = self.account_req(Method::GET, "/api/v1/holdings")?.query(&params);
        self.send(req).await
    }

    /// 매수 가능 금액 조회
    ///
    /// `currency`: 조회할 통화 (KRW 또는 USD)
    pub async fn get_buying_power(&self, currency: Currency) -> Result<BuyingPowerResponse> {
        let cur = match currency {
            Currency::KRW => "KRW",
            Currency::USD => "USD",
        };
        let req = self
            .account_req(Method::GET, "/api/v1/buying-power")?
            .query(&[("currency", cur)]);
        self.send(req).await
    }

    /// 판매 가능 수량 조회
    pub async fn get_sellable_quantity(&self, symbol: &str) -> Result<SellableQuantityResponse> {
        let req = self
            .account_req(Method::GET, "/api/v1/sellable-quantity")?
            .query(&[("symbol", symbol)]);
        self.send(req).await
    }

    /// 매매 수수료 조회
    pub async fn get_commissions(&self) -> Result<Vec<Commission>> {
        let req = self.account_req(Method::GET, "/api/v1/commissions")?;
        self.send(req).await
    }
}
