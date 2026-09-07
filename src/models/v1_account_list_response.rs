/// V1AccountListResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AccountListResponse {
    /// Credits currently available to spend. Includes subscription credits and any purchased credit packs.
    pub credits: i64,
    /// Email address of the account.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub email: Option<String>,
    /// Unique ID of the account that owns the API key.
    pub id: String,
    /// Details of the account's subscription plan. `null` if the account has no subscription, e.g. a free account, an account that only purchased credit packs, or an account on usage-based API pricing.
    ///
    /// Reflects the plan currently configured on the subscription. If a plan change is scheduled, `tier` stays on the current plan until the next payment succeeds, so `tier` and `name` can briefly disagree.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub subscription: Option<crate::models::V1AccountListResponseSubscription>,
    /// Subscription tier in effect for the account. `free` if there is no active subscription, including while a subscription is `past_due`.
    pub tier: crate::models::V1AccountListResponseTierEnum,
}
