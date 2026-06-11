use serde::Deserialize;

use crate::client::BASE_URL;
use crate::error::{Error, Result};
use crate::TossInvestClient;

/// OAuth2 액세스 토큰 응답
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    /// 항상 "Bearer"
    pub token_type: String,
    /// 토큰 만료까지 남은 초
    pub expires_in: i64,
}

#[derive(Deserialize)]
struct OAuth2ErrorResponse {
    error: String,
    error_description: Option<String>,
}

impl TossInvestClient {
    /// OAuth2 Client Credentials Grant 방식으로 액세스 토큰을 발급합니다.
    ///
    /// 발급된 `access_token`을 [`TossInvestClient::new`]에 전달하세요.
    ///
    /// ```no_run
    /// # use tossinvest_rs::TossInvestClient;
    /// # #[tokio::main]
    /// # async fn main() -> tossinvest_rs::Result<()> {
    /// let token = TossInvestClient::issue_token("client_id", "client_secret").await?;
    /// let client = TossInvestClient::new(&token.access_token);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn issue_token(client_id: &str, client_secret: &str) -> Result<TokenResponse> {
        let http = reqwest::Client::new();
        let resp = http
            .post(format!("{}/oauth2/token", BASE_URL))
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", client_id),
                ("client_secret", client_secret),
            ])
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() {
            Ok(serde_json::from_str::<TokenResponse>(&text)?)
        } else {
            if let Ok(err) = serde_json::from_str::<OAuth2ErrorResponse>(&text) {
                return Err(Error::Api {
                    code: err.error,
                    message: err.error_description.unwrap_or_default(),
                });
            }
            Err(Error::Api {
                code: status.to_string(),
                message: text,
            })
        }
    }
}
