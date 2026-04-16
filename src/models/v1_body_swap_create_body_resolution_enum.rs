/// Output resolution. Determines credits charged for the run.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1BodySwapCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1k")]
    Enum1k,
    #[serde(rename = "2k")]
    Enum2k,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "640px")]
    Enum640px,
}
impl std::fmt::Display for V1BodySwapCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1BodySwapCreateBodyResolutionEnum::Enum1k => "1k",
            V1BodySwapCreateBodyResolutionEnum::Enum2k => "2k",
            V1BodySwapCreateBodyResolutionEnum::Enum4k => "4k",
            V1BodySwapCreateBodyResolutionEnum::Enum640px => "640px",
        };
        write!(f, "{}", str_val)
    }
}
