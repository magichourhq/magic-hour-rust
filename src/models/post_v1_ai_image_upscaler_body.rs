/// PostV1AiImageUpscalerBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageUpscalerBody {
    /// Provide the assets for upscaling
    pub assets: crate::models::PostV1AiImageUpscalerBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub scale_factor: f64,
    pub style: crate::models::PostV1AiImageUpscalerBodyStyle,
}
