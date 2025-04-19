/// V1TextToVideoCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBodyStyle {
    /// The prompt used for the video.
    pub prompt: String,
    /// * `quick` - Fastest option for rapid results. Takes ~3 minutes per 5s of video.
    /// *  `studio` - Polished visuals with longer runtime. Takes ~8.5 minutes per 5s of video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<crate::models::V1TextToVideoCreateBodyStyleQualityModeEnum>,
}
