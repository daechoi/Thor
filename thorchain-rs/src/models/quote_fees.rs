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
pub struct QuoteFees {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "affiliate", skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<String>,
    #[serde(rename = "outbound")]
    pub outbound: String,
    /// liquidity fees paid to pools
    #[serde(rename = "liquidity", skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<String>,
    /// total basis points in fees relative to amount out
    #[serde(rename = "total_bps", skip_serializing_if = "Option::is_none")]
    pub total_bps: Option<String>,
}

impl QuoteFees {
    pub fn new(asset: String, outbound: String) -> QuoteFees {
        QuoteFees {
            asset,
            affiliate: None,
            outbound,
            liquidity: None,
            total_bps: None,
        }
    }
}


