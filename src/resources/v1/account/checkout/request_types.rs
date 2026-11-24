/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Credits a blocked request is missing, e.g. `credit_shortfall` from a 402 response. The checkout starts with enough packs to cover it. Defaults to one pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_shortfall: Option<f64>,
    /// Number of credit packs the checkout starts with. Takes priority over `credit_shortfall`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_count: Option<i64>,
    /// What to buy. `credit_pack` is a one-time credit purchase.
    #[serde(rename = "type")]
    pub type_: crate::models::V1AccountCheckoutCreateBodyTypeEnum,
}
