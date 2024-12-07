use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChinaExportBovine {
    #[serde(rename = "UF")]
    state: String,
    #[serde(rename = "Preço bruto")]
    gross_price: f64,
    #[serde(rename = "Preço livre de impostos")]
    net_price: f64,
}

fn convert_to_f64(value: &str) -> f64 {
    match value
        .replace(".", "")
        .replace(",", ".")
        .trim()
        .parse::<f64>()
    {
        Ok(r) => r,
        Err(e) => {
            sentry::capture_error(&e);
            0.0
        }
    }
}

impl ChinaExportBovine {
    pub fn new(state: String, gross_price: String, net_price: String) -> Self {
        let gross_price = convert_to_f64(&gross_price);
        let net_price = convert_to_f64(&net_price);

        Self {
            state,
            gross_price,
            net_price,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GrossPrices {
    #[serde(rename = "Á Vista")]
    cash: f64,
    #[serde(rename = "Para 30 dias")]
    thirty_days: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscountedPrices {
    #[serde(rename = "Funrural à vista")]
    funrural_cash: f64,
    #[serde(rename = "Funrural para 30 dias")]
    funrural_thirty_days: f64,
    #[serde(rename = "Senar à vista")]
    senar_cash: f64,
    #[serde(rename = "Senar para 30 dias")]
    senar_thirty_days: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CattleType {
    #[serde(rename = "Boi Gordo")]
    FatOx,
    #[serde(rename = "Vaca Gorda")]
    FatCow,
    #[serde(rename = "Novilha Gorda")]
    FatHeifer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CattleMarketData {
    #[serde(rename = "Tipo de Animal")]
    cattle_type: CattleType,
    #[serde(rename = "UF")]
    state_code: String,
    #[serde(rename = "Preços Brutos")]
    gross_prices: GrossPrices,
    #[serde(rename = "Preços com Descontos")]
    discounted_prices: DiscountedPrices,
}

impl CattleMarketData {
    pub fn new(
        cattle_type: CattleType,
        state_code: String,
        cash_price: String,
        price_30_days: String,
        funrural_discount_cash: String,
        funrural_discount_30_days: String,
        senar_contribution_cash: String,
        senar_contribution_30_days: String,
    ) -> Self {
        let gross_prices = GrossPrices {
            cash: convert_to_f64(&cash_price),
            thirty_days: convert_to_f64(&price_30_days),
        };

        let discounted_prices = DiscountedPrices {
            funrural_cash: convert_to_f64(&funrural_discount_cash),
            funrural_thirty_days: convert_to_f64(&funrural_discount_30_days),
            senar_cash: convert_to_f64(&senar_contribution_cash),
            senar_thirty_days: convert_to_f64(&senar_contribution_30_days),
        };

        Self {
            cattle_type,
            state_code,
            gross_prices,
            discounted_prices,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CattlePrice {
    #[serde(rename = "Praça Precuária")]
    region: String,
    #[serde(rename = "Preço Hoje")]
    price_today: f64,
    #[serde(rename = "Preço Ontem")]
    price_yesterday: f64,
    #[serde(rename = "Mudança de Preço")]
    price_change: f64,
}

impl CattlePrice {
    pub fn new(
        region: String,
        price_today: String,
        price_yesterday: String,
        price_change: String,
    ) -> Self {
        let price_today = convert_to_f64(&price_today);
        let price_yesterday = convert_to_f64(&price_yesterday);
        let price_change = convert_to_f64(&price_change);

        Self {
            region,
            price_today,
            price_yesterday,
            price_change,
        }
    }
}
