/// V1AccountListResponseSubscriptionPrice
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AccountListResponseSubscriptionPrice {
    /// Price charged per billing interval, in the smallest unit of the currency (e.g. 4900 is $49.00 for `usd`). Discounts are not applied.
    pub amount: i64,
    /// Three-letter ISO 4217 currency code, lowercase.
    pub currency: String,
}
