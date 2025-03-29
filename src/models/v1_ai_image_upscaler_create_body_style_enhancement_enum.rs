/// V1AiImageUpscalerCreateBodyStyleEnhancementEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageUpscalerCreateBodyStyleEnhancementEnum {
    #[default]
    #[serde(rename = "Balanced")]
    Balanced,
    #[serde(rename = "Creative")]
    Creative,
    #[serde(rename = "Resemblance")]
    Resemblance,
}
impl std::fmt::Display for V1AiImageUpscalerCreateBodyStyleEnhancementEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Balanced => "Balanced",
            V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Creative => "Creative",
            V1AiImageUpscalerCreateBodyStyleEnhancementEnum::Resemblance => "Resemblance",
        };
        write!(f, "{}", str_val)
    }
}
