/// ListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    /// The create endpoint to price, named by its path. For example `ai-image-editor` for `POST /v1/ai-image-editor`.
    pub tool: crate::models::V1EstimateImageListToolEnum,
    /// The `image_count` of the create request. Used by `ai-image-generator`, `ai-image-editor`. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_count: Option<i64>,
    /// The `style.mode` of the create request. Used by `ai-image-upscaler`. One of `preserve`, `balanced`, `creative`. Defaults to `balanced`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The `model` of the create request. Used by `ai-image-generator`, `ai-image-editor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The `resolution` of the create request. Used by `ai-image-generator`, `ai-image-editor`, `body-swap`. One of `640px`, `1k`, `2k`, `4k`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The `scale_factor` of the create request. Used by `ai-image-upscaler`. One of `2`, `4`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<f64>,
}
