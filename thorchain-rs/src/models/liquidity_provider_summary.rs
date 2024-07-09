/*
 * Thornode API
 *
 * Thornode REST API.
 *
 * The version of the OpenAPI document: 1.119.0
 * Contact: devs@thorchain.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LiquidityProviderSummary {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "rune_address", skip_serializing_if = "Option::is_none")]
    pub rune_address: Option<String>,
    #[serde(rename = "asset_address", skip_serializing_if = "Option::is_none")]
    pub asset_address: Option<String>,
    #[serde(rename = "last_add_height", skip_serializing_if = "Option::is_none")]
    pub last_add_height: Option<i64>,
    #[serde(rename = "last_withdraw_height", skip_serializing_if = "Option::is_none")]
    pub last_withdraw_height: Option<i64>,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "pending_rune")]
    pub pending_rune: String,
    #[serde(rename = "pending_asset")]
    pub pending_asset: String,
    #[serde(rename = "pending_tx_id", skip_serializing_if = "Option::is_none")]
    pub pending_tx_id: Option<String>,
    #[serde(rename = "rune_deposit_value")]
    pub rune_deposit_value: String,
    #[serde(rename = "asset_deposit_value")]
    pub asset_deposit_value: String,
}

impl LiquidityProviderSummary {
    pub fn new(asset: String, units: String, pending_rune: String, pending_asset: String, rune_deposit_value: String, asset_deposit_value: String) -> LiquidityProviderSummary {
        LiquidityProviderSummary {
            asset,
            rune_address: None,
            asset_address: None,
            last_add_height: None,
            last_withdraw_height: None,
            units,
            pending_rune,
            pending_asset,
            pending_tx_id: None,
            rune_deposit_value,
            asset_deposit_value,
        }
    }
}


