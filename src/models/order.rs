use serde::{Deserialize, Serialize};

use super::common::Currency;

/// 주문 방향
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

/// 호가 유형
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}

/// 주문 유효 조건 (Time In Force)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeInForce {
    /// 당일 유효 (기본값)
    #[serde(rename = "DAY")]
    Day,
    /// 장 마감 주문 (At the Close). US LIMIT 주문에만 지원.
    #[serde(rename = "CLS")]
    Cls,
}

/// 주문 상태
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "PENDING_CANCEL")]
    PendingCancel,
    #[serde(rename = "PENDING_REPLACE")]
    PendingReplace,
    #[serde(rename = "PARTIAL_FILLED")]
    PartialFilled,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "CANCEL_REJECTED")]
    CancelRejected,
    #[serde(rename = "REPLACE_REJECTED")]
    ReplaceRejected,
    #[serde(rename = "REPLACED")]
    Replaced,
    #[serde(other)]
    Unknown,
}

/// 주문 목록 조회 상태 필터
#[derive(Debug, Clone)]
pub enum OrderStatusFilter {
    /// 진행 중 주문 (PENDING, PARTIAL_FILLED, PENDING_CANCEL, PENDING_REPLACE)
    Open,
    /// 종료된 주문
    Closed,
}

impl OrderStatusFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatusFilter::Open => "OPEN",
            OrderStatusFilter::Closed => "CLOSED",
        }
    }
}

/// 수량 기반 주문 생성 요청
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateRequest {
    /// 클라이언트 지정 주문 식별자 (멱등성 키, 최대 36자)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    /// 미지정 시 DAY
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// 수량 기반 주문 시 사용
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// LIMIT 주문 시 필수
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// 금액 기반 주문 (US MARKET 전용)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_amount: Option<String>,
    /// 1억원 이상 주문 시 true 필요
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_high_value_order: Option<bool>,
}

impl OrderCreateRequest {
    /// 지정가 수량 기반 주문 생성
    pub fn limit(symbol: impl Into<String>, side: OrderSide, quantity: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            client_order_id: None,
            symbol: symbol.into(),
            side,
            order_type: OrderType::Limit,
            time_in_force: None,
            quantity: Some(quantity.into()),
            price: Some(price.into()),
            order_amount: None,
            confirm_high_value_order: None,
        }
    }

    /// 시장가 수량 기반 주문 생성
    pub fn market(symbol: impl Into<String>, side: OrderSide, quantity: impl Into<String>) -> Self {
        Self {
            client_order_id: None,
            symbol: symbol.into(),
            side,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: Some(quantity.into()),
            price: None,
            order_amount: None,
            confirm_high_value_order: None,
        }
    }

    /// 금액 기반 US 시장가 매수 (소수점 주식)
    pub fn market_amount(symbol: impl Into<String>, order_amount: impl Into<String>) -> Self {
        Self {
            client_order_id: None,
            symbol: symbol.into(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: None,
            quantity: None,
            price: None,
            order_amount: Some(order_amount.into()),
            confirm_high_value_order: None,
        }
    }

    pub fn with_client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into());
        self
    }

    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    pub fn with_confirm_high_value(mut self) -> Self {
        self.confirm_high_value_order = Some(true);
        self
    }
}

/// 주문 정정 요청
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderModifyRequest {
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_high_value_order: Option<bool>,
}

/// 주문 생성/정정/취소 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub order_id: String,
    pub client_order_id: Option<String>,
}

/// 주문 정정/취소 응답 (새 orderId 발급)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderOperationResponse {
    pub order_id: String,
}

/// 주문 상세 정보
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: String,
    pub status: OrderStatus,
    pub price: Option<String>,
    pub quantity: String,
    pub order_amount: Option<String>,
    pub currency: Currency,
    pub ordered_at: String,
    pub canceled_at: Option<String>,
    pub execution: OrderExecution,
}

/// 체결 결과
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderExecution {
    pub filled_quantity: String,
    pub average_filled_price: Option<String>,
    pub filled_amount: Option<String>,
    pub commission: Option<String>,
    pub tax: Option<String>,
    pub filled_at: Option<String>,
    pub settlement_date: Option<String>,
}

/// 주문 목록 응답
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedOrderResponse {
    pub orders: Vec<Order>,
    pub next_cursor: Option<String>,
    pub has_next: bool,
}
