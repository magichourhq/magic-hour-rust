/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for upscaling
    pub assets: crate::models::V1AiImageUpscalerCreateBodyAssets,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// How much to scale the image. Must be either 2 or 4.
    ///
    /// Note: 4x upscale is only available on Creator, Pro, or Business tier.
    pub scale_factor: f64,
    pub style: crate::models::V1AiImageUpscalerCreateBodyStyle,
}
