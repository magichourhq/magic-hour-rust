/// V1TextToVideoCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBodyStyle {
    /// The prompt used for the video.
    pub prompt: String,
    /// DEPRECATED: Please use `resolution` field instead. For backward compatibility:
    /// * `quick` maps to 720p resolution
    /// * `studio` maps to 1080p resolution
    ///
    /// This field will be removed in a future version. Use the `resolution` field to directly to specify the resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<crate::models::V1TextToVideoCreateBodyStyleQualityModeEnum>,
}
