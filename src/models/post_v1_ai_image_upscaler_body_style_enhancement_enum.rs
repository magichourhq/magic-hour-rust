/// PostV1AiImageUpscalerBodyStyleEnhancementEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AiImageUpscalerBodyStyleEnhancementEnum {
    #[default]
    #[serde(rename = "Balanced")]
    Balanced,
    #[serde(rename = "Creative")]
    Creative,
    #[serde(rename = "Resemblance")]
    Resemblance,
}
impl std::fmt::Display for PostV1AiImageUpscalerBodyStyleEnhancementEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AiImageUpscalerBodyStyleEnhancementEnum::Balanced => "Balanced",
            PostV1AiImageUpscalerBodyStyleEnhancementEnum::Creative => "Creative",
            PostV1AiImageUpscalerBodyStyleEnhancementEnum::Resemblance => "Resemblance",
        };
        write!(f, "{}", str_val)
    }
}
