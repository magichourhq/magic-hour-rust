/// V1AutoSubtitleGeneratorCreateResponse401MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AutoSubtitleGeneratorCreateResponse401MessageEnum {
    #[default]
    #[serde(rename = "Unauthorized")]
    Unauthorized,
}
impl std::fmt::Display for V1AutoSubtitleGeneratorCreateResponse401MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AutoSubtitleGeneratorCreateResponse401MessageEnum::Unauthorized => {
                "Unauthorized"
            }
        };
        write!(f, "{}", str_val)
    }
}
