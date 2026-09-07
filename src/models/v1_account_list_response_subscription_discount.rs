/// Discount applied to the subscription. `null` if no discount is applied.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AccountListResponseSubscriptionDiscount {
    /// Fixed amount taken off `price.amount` each billing interval, in the smallest unit of the currency. `null` if the discount is a percentage.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub amount_off: Option<i64>,
    /// Percentage taken off `price.amount` each billing interval. `null` if the discount is a fixed amount.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub percent_off: Option<f64>,
}
