/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageUpscalerBodyStyle {
    pub enhancement: crate::models::PostV1AiImageUpscalerBodyStyleEnhancementEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}
