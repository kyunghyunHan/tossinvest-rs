/// 계좌 / 주문 예제
///
/// 실행 방법:
/// ```
/// TOSSINVEST_CLIENT_ID=xxx TOSSINVEST_CLIENT_SECRET=yyy cargo run --example account_order
/// ```
///
/// ※ 실제 주문 코드는 주석 처리되어 있습니다. 테스트 시 주의하세요.
use tossinvest_rs::{
    models::{Currency, OrderStatusFilter},
    TossInvestClient,
};

#[tokio::main]
async fn main() -> tossinvest_rs::Result<()> {
    dotenvy::dotenv().ok();

    let client_id = std::env::var("TOSSINVEST_CLIENT_ID").expect("TOSSINVEST_CLIENT_ID not set");
    let client_secret =
        std::env::var("TOSSINVEST_CLIENT_SECRET").expect("TOSSINVEST_CLIENT_SECRET not set");

    let token = TossInvestClient::issue_token(&client_id, &client_secret).await?;
    let client = TossInvestClient::new(&token.access_token);

    // ── 계좌 목록 ────────────────────────────────────────────
    println!("[계좌 목록]");
    let accounts = client.get_accounts().await?;
    for acc in &accounts {
        println!(
            "  계좌번호: {} | seq: {} | 유형: {}",
            acc.account_no, acc.account_seq, acc.account_type
        );
    }

    if accounts.is_empty() {
        println!("  계좌가 없습니다.");
        return Ok(());
    }

    // 첫 번째 계좌 사용
    let client = client.with_account_seq(accounts[0].account_seq);
    println!("\n계좌 seq {} 로 계속합니다.", accounts[0].account_seq);

    // ── 보유 주식 ────────────────────────────────────────────
    println!("\n[보유 주식]");
    let holdings = client.get_holdings(None).await?;
    if holdings.items.is_empty() {
        println!("  보유 종목 없음");
    } else {
        for item in &holdings.items {
            println!(
                "  {} ({}) | 수량: {} | 현재가: {} | 손익: {} ({}%)",
                item.symbol,
                item.name,
                item.quantity,
                item.last_price,
                item.profit_loss.amount,
                {
                    // 소수 비율 → 퍼센트로 변환하여 출력
                    let rate: f64 = item.profit_loss.rate.parse().unwrap_or(0.0);
                    format!("{:.2}", rate * 100.0)
                }
            );
        }
    }

    // ── 매수 가능 금액 ───────────────────────────────────────
    println!("\n[매수 가능 금액]");
    let buying_power_krw = client.get_buying_power(Currency::KRW).await?;
    println!("  KRW: {}원", buying_power_krw.cash_buying_power);

    let buying_power_usd = client.get_buying_power(Currency::USD).await?;
    println!("  USD: ${}",  buying_power_usd.cash_buying_power);

    // ── 수수료 조회 ──────────────────────────────────────────
    println!("\n[수수료]");
    let commissions = client.get_commissions().await?;
    for c in &commissions {
        println!(
            "  {:?} | 수수료율: {}% | {} ~ {}",
            c.market_country,
            c.commission_rate,
            c.start_date.as_deref().unwrap_or("-"),
            c.end_date.as_deref().unwrap_or("무기한")
        );
    }

    // ── 진행 중 주문 조회 ─────────────────────────────────────
    println!("\n[진행 중 주문]");
    let orders = client
        .get_orders(OrderStatusFilter::Open, None, None, None, None, None)
        .await?;
    if orders.orders.is_empty() {
        println!("  진행 중인 주문 없음");
    } else {
        for o in &orders.orders {
            println!(
                "  {} | {} {:?} {:?} | 수량:{} 가격:{} | 상태:{:?}",
                o.order_id,
                o.symbol,
                o.side,
                o.order_type,
                o.quantity,
                o.price.as_deref().unwrap_or("시장가"),
                o.status
            );
        }
    }

    // ── 주문 생성 (주석 처리 — 실제 주문 발생 주의) ─────────────
    //
    // 삼성전자 지정가 매수 1주 @ 70,000원
    // let req = OrderCreateRequest::limit("005930", OrderSide::Buy, "1", "70000")
    //     .with_client_order_id("my-order-001");
    // let resp = client.create_order(&req).await?;
    // println!("\n[주문 생성] orderId: {}", resp.order_id);
    //
    // 주문 취소
    // let cancel = client.cancel_order(&resp.order_id).await?;
    // println!("[주문 취소] 새 orderId: {}", cancel.order_id);

    // ── 판매 가능 수량 (보유 종목이 있을 경우) ──────────────────
    if let Some(first) = holdings.items.first() {
        println!("\n[판매 가능 수량] {}", first.symbol);
        let sellable = client.get_sellable_quantity(&first.symbol).await?;
        println!("  판매 가능: {}주", sellable.sellable_quantity);
    }

    Ok(())
}
