# tossinvest-rs

토스증권 Open API의 비공식 Rust 클라이언트 라이브러리입니다.

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

### 사전 준비

[토스증권 Open API](https://developers.tossinvest.com) 에서 `client_id`와 `client_secret`을 발급받으세요.

### .env 파일로 실행 (권장)

프로젝트 루트에 `.env` 파일을 만들면 자동으로 로드됩니다:

```
TOSSINVEST_CLIENT_ID=your_client_id
TOSSINVEST_CLIENT_SECRET=your_client_secret
```

```bash
# 시세 데이터 예제 — 호가, 현재가, 체결내역, 상하한가, 캔들
cargo run --example market_data

# 종목 정보 예제 — 종목 기본정보, 유의사항, 환율, 장 운영 정보
cargo run --example stock_info

# 계좌/주문 예제 — 계좌 목록, 보유주식, 매수가능금액, 주문 내역
cargo run --example account_order
```

### 환경 변수로 직접 실행

```bash
TOSSINVEST_CLIENT_ID=your_client_id \
TOSSINVEST_CLIENT_SECRET=your_client_secret \
cargo run --example market_data
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

## 라이선스

MIT
