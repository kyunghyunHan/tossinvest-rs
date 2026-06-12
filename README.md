# tossinvest-rs

토스증권 Open API의 비공식 Rust 클라이언트 라이브러리입니다.

## 현재 상태

- 공식 OpenAPI `1.1.1` 문서의 REST 엔드포인트 21개를 모두 지원합니다.
- 토스증권 Open API는 현재 REST API만 제공합니다.
- 웹 소켓은 공식 문서상 추후 지원 예정입니다. 이 라이브러리도 현재 WebSocket API는 제공하지 않습니다.
- 시세 API는 실시간성 데이터를 REST로 조회합니다. 실시간 스트리밍이 필요하면 공식 WebSocket 문서가 공개된 뒤 별도 모듈로 추가하는 것이 맞습니다.

## 설치

`Cargo.toml`에 추가하세요:

```toml
[dependencies]
tossinvest_rs = { git = "https://github.com/kyunghyunHan/tossinvest_rs" }
```

## 빠른 시작

```rust
use tossinvest_rs::{TossInvestClient, models::CandleInterval};

#[tokio::main]
async fn main() -> tossinvest_rs::Result<()> {
    // 1. 액세스 토큰 발급
    let token = TossInvestClient::issue_token("CLIENT_ID", "CLIENT_SECRET").await?;

    // 2. 클라이언트 생성
    let client = TossInvestClient::new(&token.access_token);

    // 3. 현재가 조회 (삼성전자, Apple)
    let prices = client.get_prices(&["005930", "AAPL"]).await?;
    for p in &prices {
        println!("{}: {} {:?}", p.symbol, p.last_price, p.currency);
    }

    // 4. 계좌가 필요한 API는 with_account_seq() 사용
    let accounts = client.get_accounts().await?;
    let client = client.with_account_seq(accounts[0].account_seq);
    let holdings = client.get_holdings(None).await?;
    println!("보유 종목 수: {}", holdings.items.len());

    Ok(())
}
```

## Example 실행

[토스증권 Open API](https://developers.tossinvest.com) 에서 발급받은 키를 프로젝트 루트 `.env`에 설정하세요:

```
TOSSINVEST_CLIENT_ID=your_client_id
TOSSINVEST_CLIENT_SECRET=your_client_secret
```

```bash
cargo run --example market_data   # 호가, 현재가, 체결내역, 상하한가, 캔들
cargo run --example stock_info    # 종목 기본정보, 유의사항, 환율, 장 운영 정보
cargo run --example account_order # 계좌 목록, 보유주식, 매수가능금액, 주문 내역
```

## 지원 API

| 카테고리 | 메서드 | 설명 |
|---|---|---|
| **인증** | `issue_token` | OAuth2 액세스 토큰 발급 |
| **시세** | `get_orderbook` | 호가 조회 |
| | `get_prices` | 현재가 일괄 조회 (최대 200종목) |
| | `get_trades` | 최근 체결 내역 조회 |
| | `get_price_limits` | 상/하한가 조회 |
| | `get_candles` | 캔들 차트 조회 (1분봉/일봉) |
| **종목** | `get_stocks` | 종목 기본 정보 조회 |
| | `get_stock_warnings` | 매수 유의사항 조회 |
| **시장** | `get_exchange_rate` | 환율 조회 |
| | `get_kr_market_calendar` | 국내 장 운영 정보 조회 |
| | `get_us_market_calendar` | 해외 장 운영 정보 조회 |
| **계좌** | `get_accounts` | 계좌 목록 조회 |
| | `get_holdings` | 보유 주식 조회 |
| | `get_buying_power` | 매수 가능 금액 조회 |
| | `get_sellable_quantity` | 판매 가능 수량 조회 |
| | `get_commissions` | 매매 수수료 조회 |
| **주문** | `create_order` | 주문 생성 |
| | `get_orders` | 주문 목록 조회 |
| | `get_order` | 주문 상세 조회 |
| | `modify_order` | 주문 정정 |
| | `cancel_order` | 주문 취소 |

## 공식 엔드포인트 대응

| 그룹 | HTTP | 경로 | 메서드 |
|---|---:|---|---|
| Auth | POST | `/oauth2/token` | `issue_token` |
| Market Data | GET | `/api/v1/orderbook` | `get_orderbook` |
| Market Data | GET | `/api/v1/prices` | `get_prices` |
| Market Data | GET | `/api/v1/trades` | `get_trades` |
| Market Data | GET | `/api/v1/price-limits` | `get_price_limits` |
| Market Data | GET | `/api/v1/candles` | `get_candles` |
| Stock Info | GET | `/api/v1/stocks` | `get_stocks` |
| Stock Info | GET | `/api/v1/stocks/{symbol}/warnings` | `get_stock_warnings` |
| Market Info | GET | `/api/v1/exchange-rate` | `get_exchange_rate` |
| Market Info | GET | `/api/v1/market-calendar/KR` | `get_kr_market_calendar` |
| Market Info | GET | `/api/v1/market-calendar/US` | `get_us_market_calendar` |
| Account | GET | `/api/v1/accounts` | `get_accounts` |
| Asset | GET | `/api/v1/holdings` | `get_holdings` |
| Order History | GET | `/api/v1/orders` | `get_orders` |
| Order | POST | `/api/v1/orders` | `create_order` |
| Order History | GET | `/api/v1/orders/{orderId}` | `get_order` |
| Order | POST | `/api/v1/orders/{orderId}/modify` | `modify_order` |
| Order | POST | `/api/v1/orders/{orderId}/cancel` | `cancel_order` |
| Order Info | GET | `/api/v1/buying-power` | `get_buying_power` |
| Order Info | GET | `/api/v1/sellable-quantity` | `get_sellable_quantity` |
| Order Info | GET | `/api/v1/commissions` | `get_commissions` |

## Rate Limits

공식 문서 기준으로 모든 API는 클라이언트와 API 그룹 단위의 TPS 제한을 갖습니다. 한도는 운영 상황에 따라 바뀔 수 있으므로 응답 헤더의 `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`, `Retry-After`를 확인하세요.

| 그룹 | 기본 한도 | 비고 |
|---|---:|---|
| `AUTH` | 초당 5회 | 토큰 발급 |
| `ACCOUNT` | 초당 1회 | 계좌 목록 |
| `ASSET` | 초당 5회 | 보유 주식 |
| `STOCK` | 초당 5회 | 종목 정보 |
| `MARKET_INFO` | 초당 3회 | 환율, 장 운영 정보 |
| `MARKET_DATA` | 초당 10회 | 호가, 현재가, 체결, 상하한가 |
| `MARKET_DATA_CHART` | 초당 5회 | 캔들 |
| `ORDER` | 초당 6회 | 09:00-09:10 KST에는 초당 3회 |
| `ORDER_HISTORY` | 초당 5회 | 주문 목록/상세 |
| `ORDER_INFO` | 초당 6회 | 09:00-09:10 KST에는 초당 3회 |

429 응답을 받으면 `Retry-After`만큼 대기한 뒤 재시도하세요.

## 주문 목록 조회

`get_orders`는 공식 문서의 `OPEN`과 `CLOSED` 필터를 모두 받을 수 있습니다.

- `OPEN`: 진행 중 주문을 반환합니다. `limit`, `cursor`는 무시되고 `from`, `to`는 주문 생성일 기준 필터로 적용됩니다.
- `CLOSED`: 종료 주문 조회에 사용합니다. `limit`, `cursor`, `from`, `to`가 적용됩니다.

## 참고 문서

- [토스증권 Open API 문서](https://developers.tossinvest.com/docs)
- [OpenAPI JSON](https://openapi.tossinvest.com/openapi-docs/latest/openapi.json)
- [API Reference](https://openapi.tossinvest.com/openapi-docs/latest/api-reference/README.md)

## 라이선스

MIT
