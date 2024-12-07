pub mod models;

use easy_scraper::Pattern;
use reqwest::Client;
use sentry::{add_breadcrumb, Breadcrumb, Level};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;

type TableResult = Result<Vec<BTreeMap<String, String>>, TableError>;
type InfoResult = Option<Vec<BTreeMap<String, String>>>;

#[derive(Debug)]
struct TableError;

impl Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to get table")
    }
}

impl std::error::Error for TableError {}

pub enum TableType {
    ChinaExportBovine,
    CattlePrice,
    FatOxPrices,
    FatCowPrices,
    FatHeiferPrices,
}

impl TableType {
    fn get_pattern(&self) -> Pattern {
        match self {
            TableType::ChinaExportBovine => Pattern::new(
                r#"
                <table cellpadding="0" cellspacing="0" width="660px">
                    <tr class="conteudo">
                        <td>{{state}}</td>
                        <td>{{gross_price}}</td>
                        <td>{{net_price}}</td>
                    </tr>
                </table>
                "#,
            )
            .expect("Failed to create `BoiGordoBoiChinaAPrazo` pattern"),
            TableType::FatOxPrices => Pattern::new(
                r#"
                    <table subseq border="0"
                    cellpadding="0"
                    cellspacing="0"
                    width="660"
                    style="margin-top: 10px">
                        <tr class="conteudo">
                            <td>{{state}}</td>
                            <td>{{cash_price}}</td>
                            <td>{{price_30_days}}</td>
                            <td>
                                <img src="{{market_indicator}}">
                            </td>
                            <td>{{differential}}</td>
                            <td>{{funrural_discount_cash}}</td>
                            <td>{{funrural_discount_30_days}}</td>
                            <td>{{senar_contribution_cash}}</td>
                            <td></td>
                            <td>{{senar_contribution_30_days}}</td>
                            <td></td>
                        </tr>
                    </table>
                        "#,
            )
            .expect("Failed to create `BoiGordoMercadoFisico` pattern"),
            TableType::CattlePrice => Pattern::new(
                r#"
                    <table subseq>
                        <tr class="conteudo">
                            <td>{{region}}</td>
                            <td>{{price_today}}</td>
                            <td>{{price_yesterday}}</td>
                            <td>
                                {{price_change}}
                                <img src="{{market_indicator}}">
                            </td>
                        </tr>
                    </table>
                "#,
            )
            .expect("Failed to create `CattlePrice` pattern"),
            TableType::FatCowPrices => Pattern::new(
                r#"
                    <table subseq>
                        <tr>
                            <td>{{state}}</td>
                            <td>{{cash_price}}</td>
                            <td>{{price_30_days}}</td>
                            <td>
                                <img src="{{pointer}}">
                            </td>
                            <td>{{funrural_discount_cash}}</td>
                            <td>{{funrural_discount_30_days}}</td>
                            <td>{{senar_contribution_cash}}</td>
                            <td></td>
                            <td>{{senar_contribution_30_days}}</td>
                            <td></td>
                        </tr>
                    </table>
                "#,
            )
            .expect("Failed to create `FatCowPrices` pattern"),
            TableType::FatHeiferPrices => Pattern::new(
                r#"
                    <table subseq>
                        <tr>
                            <td>{{state}}</td>
                            <td>{{cash_price}}</td>
                            <td>{{price_30_days}}</td>
                            <td>
                                <img src="{{pointer}}">
                            </td>
                            <td>{{funrural_discount_cash}}</td>
                            <td>{{funrural_discount_30_days}}</td>
                            <td>{{senar_contribution_cash}}</td>
                            <td></td>
                            <td>{{senar_contribution_30_days}}</td>
                            <td></td>
                        </tr>
                    </table>
                "#,
            )
            .expect("Failed to create `FatHeiferPrices` pattern"),
        }
    }
}

async fn get_table(pattern: TableType, body: &String) -> TableResult {
    let info_pattern = pattern.get_pattern();
    let infos = info_pattern.matches(&body);
    if infos.is_empty() {
        return Err(TableError);
    }
    add_breadcrumb(Breadcrumb {
        category: Some("info".into()),
        message: Some("Got table".into()),
        level: Level::Info,
        ..Default::default()
    });
    Ok(infos)
}

async fn get_page(url: &str) -> Option<String> {
    let client = Client::new();

    let response = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => {
            sentry::capture_error(&e);
            return None;
        }
    };

    let response_bytes = match response.bytes().await {
        Ok(r) => r,
        Err(e) => {
            sentry::capture_error(&e);
            return None;
        }
    };

    let decoded_content = response_bytes
        .iter()
        .map(|&b| b as char)
        .collect::<String>();

    Some(decoded_content)
}

pub async fn get_info(pattern: TableType) -> InfoResult {
    let url = match pattern {
        TableType::ChinaExportBovine | TableType::FatOxPrices => {
            "https://www.scotconsultoria.com.br/cotacoes/boi-gordo/?ref=smn"
        }
        TableType::CattlePrice => {
            "https://www.scotconsultoria.com.br/cotacoes/indicadores/?ref=smn"
        }
        TableType::FatCowPrices => {
            "https://www.scotconsultoria.com.br/cotacoes/vaca-gorda/?ref=smn"
        }
        TableType::FatHeiferPrices => {
            "https://www.scotconsultoria.com.br/cotacoes/novilha/?ref=smn"
        }
    };

    let response_text = match get_page(url).await {
        Some(r) => r,
        None => return None,
    };

    let infos = match get_table(pattern, &response_text).await {
        Ok(r) => r,
        Err(e) => {
            sentry::capture_error(&e);
            return None;
        }
    };
    Some(infos)
}
