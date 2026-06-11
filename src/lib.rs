//! # tossinvest_rs
//!
//! 토스증권 Open API의 비공식 Rust 클라이언트 라이브러리입니다.
//!
//! ## 빠른 시작
//!
//! ```no_run
//! use tossinvest_rs::{TossInvestClient, models::CandleInterval};
//!
//! #[tokio::main]
//! async fn main() -> tossinvest_rs::Result<()> {
//!     // 1. 액세스 토큰 발급
//!     let token = TossInvestClient::issue_token("CLIENT_ID", "CLIENT_SECRET").await?;
//!
//!     // 2. 클라이언트 생성
//!     let client = TossInvestClient::new(&token.access_token);
//!
//!     // 3. 현재가 조회 (삼성전자, Apple)
//!     let prices = client.get_prices(&["005930", "AAPL"]).await?;
//!     for p in &prices {
//!         println!("{}: {} {:?}", p.symbol, p.last_price, p.currency);
//!     }
//!
//!     // 4. 계좌가 필요한 API는 with_account_seq() 사용
//!     let accounts = client.get_accounts().await?;
//!     let client = client.with_account_seq(accounts[0].account_seq);
//!     let holdings = client.get_holdings(None).await?;
//!     println!("보유 종목 수: {}", holdings.items.len());
//!
//!     Ok(())
//! }
//! ```

mod api;
mod client;
mod error;
pub mod models;

pub use api::TokenResponse;
pub use client::TossInvestClient;
pub use error::{Error, Result};
