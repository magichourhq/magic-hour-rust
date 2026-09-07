/// How often the subscription is billed. `null` if unknown.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AccountListResponseSubscriptionBillingIntervalEnum {
    #[default]
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
}
impl std::fmt::Display for V1AccountListResponseSubscriptionBillingIntervalEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AccountListResponseSubscriptionBillingIntervalEnum::Month => "month",
            V1AccountListResponseSubscriptionBillingIntervalEnum::Year => "year",
        };
        write!(f, "{}", str_val)
    }
}
