/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for creating a talking photo
    pub assets: crate::models::V1AiTalkingPhotoCreateBodyAssets,
    /// The end time of the input audio in seconds. Maximum clip length depends on style.generation_mode: realistic 180s, prompted 45s.
    pub end_seconds: f64,
    /// Constrains the larger dimension (height or width) of the output video. Allows you to set a lower resolution than your plan's maximum if desired. The value is capped by your plan's max resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_resolution: Option<i64>,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input audio in seconds. Maximum clip length depends on style.generation_mode: realistic 180s, prompted 45s.
    pub start_seconds: f64,
    /// Attributes used to dictate the style of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1AiTalkingPhotoCreateBodyStyle>,
}
