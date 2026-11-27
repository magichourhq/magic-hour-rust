/// State of Auto Reload on the account.
/// - `disabled`: off or never set up.
/// - `active`: on, with no refill processing or blocked.
/// - `auto_reload_pending`: a refill is processing. Check again shortly.
/// - `payment_failed`: the last refill payment failed and paused Auto Reload until the payment method is updated.
/// - `spending_limit_reached`: the monthly limit blocked the last refill.
///
/// After an `insufficient_credits` rejection, resubmit the same request once `credits` reaches that rejection's `credits_required`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AccountListResponseAutoReloadStatusEnum {
    #[default]
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "auto_reload_pending")]
    AutoReloadPending,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "payment_failed")]
    PaymentFailed,
    #[serde(rename = "spending_limit_reached")]
    SpendingLimitReached,
}
impl std::fmt::Display for V1AccountListResponseAutoReloadStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AccountListResponseAutoReloadStatusEnum::Active => "active",
            V1AccountListResponseAutoReloadStatusEnum::AutoReloadPending => {
                "auto_reload_pending"
            }
            V1AccountListResponseAutoReloadStatusEnum::Disabled => "disabled",
            V1AccountListResponseAutoReloadStatusEnum::PaymentFailed => "payment_failed",
            V1AccountListResponseAutoReloadStatusEnum::SpendingLimitReached => {
                "spending_limit_reached"
            }
        };
        write!(f, "{}", str_val)
    }
}
