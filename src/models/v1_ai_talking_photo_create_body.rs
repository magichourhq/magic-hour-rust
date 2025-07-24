/// Provide the assets for creating a talking photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiTalkingPhotoCreateBody {
    /// Provide the assets for creating a talking photo
    pub assets: crate::models::V1AiTalkingPhotoCreateBodyAssets,
    /// The end time of the input audio in seconds. The maximum duration allowed is 60 seconds.
    pub end_seconds: f64,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input audio in seconds. The maximum duration allowed is 60 seconds.
    pub start_seconds: f64,
    /// Attributes used to dictate the style of the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<crate::models::V1AiTalkingPhotoCreateBodyStyle>,
}
