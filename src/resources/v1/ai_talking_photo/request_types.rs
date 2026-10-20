/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for creating a talking photo
    pub assets: crate::models::V1AiTalkingPhotoCreateBodyAssets,
    /// The end time of the input audio in seconds. Maximum clip length depends on style.generation_mode: realistic 300s, prompted 45s.
    pub end_seconds: f64,
    /// Deprecated: use `resolution`. Still accepted as a larger-dimension cap, limited by your plan. Ignored when `resolution` is provided.
    ///
    /// After plan capping:
    /// * Below 1280 maps to `480p` (intermediate values round down).
    /// * 1280 or above maps to `720p`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution: Option<i64>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output preset. Overrides `max_resolution`. Defaults to 480p in prompted mode; realistic mode uses the mapped plan maximum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiTalkingPhotoCreateBodyResolutionEnum>,
    /// The start time of the input audio in seconds. Maximum clip length depends on style.generation_mode: realistic 300s, prompted 45s.
    pub start_seconds: f64,
    /// Attributes used to dictate the style of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1AiTalkingPhotoCreateBodyStyle>,
}
