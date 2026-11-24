/// What to buy. `credit_pack` is a one-time credit purchase.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AccountCheckoutCreateBodyTypeEnum {
    #[default]
    #[serde(rename = "credit_pack")]
    CreditPack,
}
impl std::fmt::Display for V1AccountCheckoutCreateBodyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AccountCheckoutCreateBodyTypeEnum::CreditPack => "credit_pack",
        };
        write!(f, "{}", str_val)
    }
}
