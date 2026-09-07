/// Subscription tier in effect for the account. `free` if there is no active subscription, including while a subscription is `past_due`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AccountListResponseTierEnum {
    #[default]
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "pro")]
    Pro,
}
impl std::fmt::Display for V1AccountListResponseTierEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AccountListResponseTierEnum::Business => "business",
            V1AccountListResponseTierEnum::Creator => "creator",
            V1AccountListResponseTierEnum::Free => "free",
            V1AccountListResponseTierEnum::Pro => "pro",
        };
        write!(f, "{}", str_val)
    }
}
