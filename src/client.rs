use reqwest::{Client, Method, RequestBuilder};
use serde::Deserialize;

use crate::error::{Error, Result};

pub(crate) const BASE_URL: &str = "https://openapi.tossinvest.com";

#[derive(Deserialize)]
pub(crate) struct ApiResponse<T> {
    pub result: T,
}

#[derive(Deserialize)]
struct ApiErrorPayload {
    code: String,
    message: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ApiErrorPayload,
}

/// Toss Invest Open API 클라이언트.
///
/// [`TossInvestClient::issue_token`]으로 액세스 토큰을 발급한 뒤
/// [`TossInvestClient::new`]에 전달하여 생성합니다.
/// 계좌 관련 API를 사용하려면 [`TossInvestClient::with_account_seq`]로
/// 계좌 식별자를 설정하세요.
pub struct TossInvestClient {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) access_token: String,
    pub(crate) account_seq: Option<i64>,
}

impl TossInvestClient {
    /// 액세스 토큰으로 클라이언트를 생성합니다.
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
            access_token: access_token.into(),
            account_seq: None,
        }
    }

    /// 계좌 식별자(`accountSeq`)를 설정합니다.
    /// `GET /api/v1/accounts` 응답의 `accountSeq` 값을 사용하세요.
    pub fn with_account_seq(mut self, seq: i64) -> Self {
        self.account_seq = Some(seq);
        self
    }

    pub(crate) fn get_account_seq(&self) -> Result<i64> {
        self.account_seq.ok_or(Error::NoAccount)
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub(crate) fn auth_req(&self, method: Method, path: &str) -> RequestBuilder {
        self.http
            .request(method, self.url(path))
            .bearer_auth(&self.access_token)
    }

    pub(crate) fn account_req(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let seq = self.get_account_seq()?;
        Ok(self
            .http
            .request(method, self.url(path))
            .bearer_auth(&self.access_token)
            .header("X-Tossinvest-Account", seq))
    }

    /// 응답 body를 `ApiResponse<T>` 형태로 파싱합니다.
    pub(crate) async fn send<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() {
            let wrapped: ApiResponse<T> = serde_json::from_str(&text)?;
            Ok(wrapped.result)
        } else {
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&text) {
                return Err(Error::Api {
                    code: err.error.code,
                    message: err.error.message,
                });
            }
            Err(Error::Api {
                code: status.to_string(),
                message: text,
            })
        }
    }
}
