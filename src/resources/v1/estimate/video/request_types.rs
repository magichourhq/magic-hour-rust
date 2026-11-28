/// ListRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    /// The `end_seconds` of the create request. Used by every tool.
    pub end_seconds: f64,
    /// The create endpoint to price, named by its path. For example `image-to-video` for `POST /v1/image-to-video`.
    pub tool: crate::models::V1EstimateVideoListToolEnum,
    /// The `audio` of the create request. Used by `image-to-video`, `text-to-video`. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<bool>,
    /// Whether any of the create request's `references` is a video. Used by `text-to-video`. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_video_reference: Option<bool>,
    /// The `model` of the create request. Used by `image-to-video`, `text-to-video`, `ai-ugc-ad`, `ai-video-editor`, `character-replace`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The `resolution` of the create request. Used by `image-to-video`, `text-to-video`, `ai-ugc-ad`, `ai-video-editor`, `character-replace`, `audio-to-video`, `ai-video-translator`, `ai-video-upscaler`. `ai-video-translator` defaults to `480p`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// The `start_seconds` of the create request, for tools whose create request has it. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
}
