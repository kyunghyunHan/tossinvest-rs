use reqwest::Method;

use crate::error::Result;
use crate::models::order::{
    Order, OrderCreateRequest, OrderModifyRequest, OrderOperationResponse,
    OrderResponse, OrderStatusFilter, PaginatedOrderResponse,
};
use crate::TossInvestClient;

impl TossInvestClient {
    /// 주문 생성
    ///
    /// [`OrderCreateRequest::limit`], [`OrderCreateRequest::market`],
    /// [`OrderCreateRequest::market_amount`] 빌더를 사용하세요.
    pub async fn create_order(&self, req: &OrderCreateRequest) -> Result<OrderResponse> {
        let request = self
            .account_req(Method::POST, "/api/v1/orders")?
            .json(req);
        self.send(request).await
    }

    /// 주문 목록 조회
    ///
    /// - `status`: [`OrderStatusFilter::Open`] 또는 [`OrderStatusFilter::Closed`]
    /// - `symbol`: 특정 종목 필터 (None이면 전체)
    /// - `from` / `to`: 조회 기간 (YYYY-MM-DD, KST 기준)
    /// - `cursor`: 페이지네이션 커서
    /// - `limit`: 페이지 크기 (최대 100)
    pub async fn get_orders(
        &self,
        status: OrderStatusFilter,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedOrderResponse> {
        let mut params: Vec<(&str, String)> = vec![("status", status.as_str().to_string())];
        if let Some(s) = symbol {
            params.push(("symbol", s.to_string()));
        }
        if let Some(f) = from {
            params.push(("from", f.to_string()));
        }
        if let Some(t) = to {
            params.push(("to", t.to_string()));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        let request = self
            .account_req(Method::GET, "/api/v1/orders")?
            .query(&params);
        self.send(request).await
    }

    /// 주문 상세 조회
    pub async fn get_order(&self, order_id: &str) -> Result<Order> {
        let path = format!("/api/v1/orders/{}", order_id);
        let request = self.account_req(Method::GET, &path)?;
        self.send(request).await
    }

    /// 주문 정정
    ///
    /// 정정 성공 시 새로운 `orderId`가 발급됩니다.
    pub async fn modify_order(
        &self,
        order_id: &str,
        req: &OrderModifyRequest,
    ) -> Result<OrderOperationResponse> {
        let path = format!("/api/v1/orders/{}/modify", order_id);
        let request = self.account_req(Method::POST, &path)?.json(req);
        self.send(request).await
    }

    /// 주문 취소
    ///
    /// 취소 성공 시 새로운 `orderId`가 발급됩니다.
    pub async fn cancel_order(&self, order_id: &str) -> Result<OrderOperationResponse> {
        let path = format!("/api/v1/orders/{}/cancel", order_id);
        let request = self.account_req(Method::POST, &path)?;
        self.send(request).await
    }
}
