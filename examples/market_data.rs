/// 시세 데이터 예제
///
/// 실행 방법:
/// ```
/// TOSSINVEST_CLIENT_ID=xxx TOSSINVEST_CLIENT_SECRET=yyy cargo run --example market_data
/// ```
use tossinvest_rs::{
    models::CandleInterval,
    TossInvestClient,
};

#[tokio::main]
async fn main() -> tossinvest_rs::Result<()> {
    dotenvy::dotenv().ok();

    let client_id = std::env::var("TOSSINVEST_CLIENT_ID").expect("TOSSINVEST_CLIENT_ID not set");
    let client_secret =
        std::env::var("TOSSINVEST_CLIENT_SECRET").expect("TOSSINVEST_CLIENT_SECRET not set");

    // 액세스 토큰 발급
    println!("토큰 발급 중...");
    let token = TossInvestClient::issue_token(&client_id, &client_secret).await?;
    println!("토큰 발급 완료 ({}초 유효)", token.expires_in);

    let client = TossInvestClient::new(&token.access_token);

    // ── 현재가 조회 ──────────────────────────────────────────
    println!("\n[현재가] 삼성전자(005930) / Apple(AAPL)");
    let prices = client.get_prices(&["005930", "AAPL"]).await?;
    for p in &prices {
        println!(
            "  {} | 현재가: {} ({:?}) | {}",
            p.symbol,
            p.last_price,
            p.currency,
            p.timestamp.as_deref().unwrap_or("시각 없음")
        );
    }

    // ── 호가 조회 ────────────────────────────────────────────
    println!("\n[호가] 삼성전자(005930)");
    let orderbook = client.get_orderbook("005930").await?;
    println!("  매도호가 (상위 3):");
    for ask in orderbook.asks.iter().take(3) {
        println!("    {} × {}", ask.price, ask.volume);
    }
    println!("  매수호가 (상위 3):");
    for bid in orderbook.bids.iter().take(3) {
        println!("    {} × {}", bid.price, bid.volume);
    }

    // ── 최근 체결 내역 ───────────────────────────────────────
    println!("\n[체결] 삼성전자(005930) 최근 5건");
    let trades = client.get_trades("005930", Some(5)).await?;
    for t in &trades {
        println!("  {} | {}주 @ {}", t.timestamp, t.volume, t.price);
    }

    // ── 상/하한가 ────────────────────────────────────────────
    println!("\n[상하한가] 삼성전자(005930)");
    let limits = client.get_price_limits("005930").await?;
    println!(
        "  상한가: {} | 하한가: {}",
        limits.upper_limit_price.as_deref().unwrap_or("없음"),
        limits.lower_limit_price.as_deref().unwrap_or("없음")
    );

    // ── 일봉 캔들 ────────────────────────────────────────────
    println!("\n[캔들] 삼성전자(005930) 일봉 5개");
    let candles = client
        .get_candles("005930", CandleInterval::OneDay, Some(5), None, Some(true))
        .await?;
    for c in &candles.candles {
        println!(
            "  {} | O:{} H:{} L:{} C:{} V:{}",
            c.timestamp, c.open_price, c.high_price, c.low_price, c.close_price, c.volume
        );
    }
    if let Some(next) = &candles.next_before {
        println!("  (다음 페이지 before: {})", next);
    }

    Ok(())
}
