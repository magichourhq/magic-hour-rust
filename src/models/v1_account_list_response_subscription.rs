/// Details of the account's subscription plan. `null` if the account has no subscription, e.g. a free account, an account that only purchased credit packs, or an account on usage-based API pricing.
///
/// Reflects the plan currently configured on the subscription. If a plan change is scheduled, `tier` stays on the current plan until the next payment succeeds, so `tier` and `name` can briefly disagree.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AccountListResponseSubscription {
    /// How often the subscription is billed. `null` if unknown.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub billing_interval: Option<
        crate::models::V1AccountListResponseSubscriptionBillingIntervalEnum,
    >,
    /// Whether the subscription is scheduled to end at `current_period_end` instead of renewing. The subscription stays usable until then.
    pub cancel_at_period_end: bool,
    /// End of the current billing period, in ISO 8601 format. The subscription renews at this time, or ends if `cancel_at_period_end` is `true`.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub current_period_end: Option<String>,
    /// Discount applied to the subscription. `null` if no discount is applied.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub discount: Option<crate::models::V1AccountListResponseSubscriptionDiscount>,
    /// Name of the current subscription plan, e.g. `Creator`, `Pro`, `Pro Plus`, `Business`. `null` if the plan cannot be determined. Use `tier` for a machine-readable value.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    pub price: crate::models::V1AccountListResponseSubscriptionPrice,
    /// Status of the subscription.
    /// - `active`: payments are up to date.
    /// - `past_due`: the latest payment failed. `tier` is `free` until payment succeeds. The subscription is canceled if payment keeps failing.
    pub status: crate::models::V1AccountListResponseSubscriptionStatusEnum,
}
