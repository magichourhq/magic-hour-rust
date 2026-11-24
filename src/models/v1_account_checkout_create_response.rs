/// V1AccountCheckoutCreateResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AccountCheckoutCreateResponse {
    /// Open in a browser to complete the purchase.
    pub checkout_url: String,
}
