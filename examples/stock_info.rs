/// 종목 정보 및 시장 정보 예제
///
/// 실행 방법:
/// ```
/// TOSSINVEST_CLIENT_ID=xxx TOSSINVEST_CLIENT_SECRET=yyy cargo run --example stock_info
/// ```
use tossinvest_rs::{
    models::Currency,
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

    // ── 종목 기본 정보 ───────────────────────────────────────
    println!("[종목 정보] 삼성전자(005930) / Apple(AAPL)");
    let stocks = client.get_stocks(&["005930", "AAPL"]).await?;
    for s in &stocks {
        println!(
            "  {} | {} ({}) | 시장: {} | 유형: {} | 발행주식수: {}",
            s.symbol, s.name, s.english_name, s.market, s.security_type, s.shares_outstanding
        );
        if let Some(lf) = &s.leverage_factor {
            println!("    레버리지: {}", lf);
        }
    }

    // ── 매수 유의사항 ────────────────────────────────────────
    println!("\n[유의사항] 삼성전자(005930)");
    let warnings = client.get_stock_warnings("005930").await?;
    if warnings.is_empty() {
        println!("  유의사항 없음");
    } else {
        for w in &warnings {
            println!(
                "  [{}] 거래소:{} | {} ~ {}",
                w.warning_type,
                w.exchange.as_deref().unwrap_or("-"),
                w.start_date.as_deref().unwrap_or("미정"),
                w.end_date.as_deref().unwrap_or("진행중")
            );
        }
    }

    // ── 환율 ─────────────────────────────────────────────────
    println!("\n[환율] USD → KRW");
    let rate = client
        .get_exchange_rate(Currency::USD, Currency::KRW, None)
        .await?;
    println!(
        "  매수환율: {} | 매매기준율: {} | 유효기간: {} ~ {}",
        rate.rate, rate.mid_rate, rate.valid_from, rate.valid_until
    );

    // ── 국내 장 운영 정보 ─────────────────────────────────────
    println!("\n[국내 장 운영] 오늘");
    let kr_cal = client.get_kr_market_calendar(None).await?;
    println!("  오늘: {}", kr_cal.today.date);
    println!(
        "  이전 영업일: {} | 다음 영업일: {}",
        kr_cal.previous_business_day.date, kr_cal.next_business_day.date
    );
    if let Some(integrated) = &kr_cal.today.integrated {
        println!("  통합 거래 시간: {}", integrated);
    } else {
        println!("  오늘은 휴장입니다.");
    }

    // ── 해외 장 운영 정보 ─────────────────────────────────────
    println!("\n[해외 장 운영] 오늘");
    let us_cal = client.get_us_market_calendar(None).await?;
    println!("  오늘(미국 현지): {}", us_cal.today.date);
    if let Some(reg) = &us_cal.today.regular_market {
        println!("  정규장: {}", reg);
    } else {
        println!("  정규장 휴장");
    }

    Ok(())
}
