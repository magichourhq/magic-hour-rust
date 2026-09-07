/// Status of the subscription.
/// - `active`: payments are up to date.
/// - `past_due`: the latest payment failed. `tier` is `free` until payment succeeds. The subscription is canceled if payment keeps failing.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AccountListResponseSubscriptionStatusEnum {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "past_due")]
    PastDue,
}
impl std::fmt::Display for V1AccountListResponseSubscriptionStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AccountListResponseSubscriptionStatusEnum::Active => "active",
            V1AccountListResponseSubscriptionStatusEnum::PastDue => "past_due",
        };
        write!(f, "{}", str_val)
    }
}
