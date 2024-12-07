pub mod scrapers;

use scrapers::models;
use scrapers::TableType;
use serde_json;

fn main() {
    let _guard = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        debug: false,
        ..Default::default()
    });
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}

async fn run() {
    if let Some(result) = scrapers::get_info(TableType::ChinaExportBovine).await {
        result.iter().for_each(|info| {
            if info.get("state").unwrap().to_string() == "UF" {
                return;
            }

            let china_export_bovine = models::ChinaExportBovine::new(
                info.get("state").unwrap().to_string(),
                info.get("gross_price").unwrap().to_string(),
                info.get("net_price").unwrap().to_string(),
            );

            let json = serde_json::to_string_pretty(&china_export_bovine).unwrap();
            println!("{}", json);
        });
    };
    if let Some(result) = scrapers::get_info(TableType::CattlePrice).await {
        result.iter().for_each(|info| {
            let cattle_price = models::CattlePrice::new(
                info.get("region").unwrap().to_string(),
                info.get("price_today").unwrap().to_string(),
                info.get("price_yesterday").unwrap().to_string(),
                info.get("price_change").unwrap().to_string(),
            );

            let json = serde_json::to_string_pretty(&cattle_price).unwrap();
            println!("{}", json)
        });
    };
    if let Some(result) = scrapers::get_info(TableType::FatOxPrices).await {
        result.iter().for_each(|info| {
            let cattle_market_price = models::CattleMarketData::new(
                models::CattleType::FatOx,
                info.get("state").unwrap().to_string(),
                info.get("cash_price").unwrap().to_string(),
                info.get("price_30_days").unwrap().to_string(),
                info.get("funrural_discount_cash").unwrap().to_string(),
                info.get("funrural_discount_30_days").unwrap().to_string(),
                info.get("senar_contribution_cash").unwrap().to_string(),
                info.get("senar_contribution_30_days").unwrap().to_string(),
            );

            let json = serde_json::to_string_pretty(&cattle_market_price).unwrap();
            println!("{}", json)
        });
    };
    if let Some(result) = scrapers::get_info(TableType::FatCowPrices).await {
        result.iter().for_each(|info| {
            let cattle_market_price = models::CattleMarketData::new(
                models::CattleType::FatCow,
                info.get("state").unwrap().to_string(),
                info.get("cash_price").unwrap().to_string(),
                info.get("price_30_days").unwrap().to_string(),
                info.get("funrural_discount_cash").unwrap().to_string(),
                info.get("funrural_discount_30_days").unwrap().to_string(),
                info.get("senar_contribution_cash").unwrap().to_string(),
                info.get("senar_contribution_30_days").unwrap().to_string(),
            );

            let json = serde_json::to_string_pretty(&cattle_market_price).unwrap();
            println!("{}", json)
        });
    };
    if let Some(result) = scrapers::get_info(TableType::FatHeiferPrices).await {
        result.iter().for_each(|info| {
            let cattle_market_price = models::CattleMarketData::new(
                models::CattleType::FatHeifer,
                info.get("state").unwrap().to_string(),
                info.get("cash_price").unwrap().to_string(),
                info.get("price_30_days").unwrap().to_string(),
                info.get("funrural_discount_cash").unwrap().to_string(),
                info.get("funrural_discount_30_days").unwrap().to_string(),
                info.get("senar_contribution_cash").unwrap().to_string(),
                info.get("senar_contribution_30_days").unwrap().to_string(),
            );

            let json = serde_json::to_string_pretty(&cattle_market_price).unwrap();
            println!("{}", json)
        });
    };
}
