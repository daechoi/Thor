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
pub struct Saver {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "asset_address")]
    pub asset_address: String,
    #[serde(rename = "last_add_height", skip_serializing_if = "Option::is_none")]
    pub last_add_height: Option<i64>,
    #[serde(rename = "last_withdraw_height", skip_serializing_if = "Option::is_none")]
    pub last_withdraw_height: Option<i64>,
    #[serde(rename = "units")]
    pub units: String,
    #[serde(rename = "asset_deposit_value")]
    pub asset_deposit_value: String,
    #[serde(rename = "asset_redeem_value")]
    pub asset_redeem_value: String,
    #[serde(rename = "growth_pct")]
    pub growth_pct: String,
}

impl Saver {
    pub fn new(asset: String, asset_address: String, units: String, asset_deposit_value: String, asset_redeem_value: String, growth_pct: String) -> Saver {
        Saver {
            asset,
            asset_address,
            last_add_height: None,
            last_withdraw_height: None,
            units,
            asset_deposit_value,
            asset_redeem_value,
            growth_pct,
        }
    }
}


