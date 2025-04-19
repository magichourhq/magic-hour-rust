/// Attributed used to dictate the style of the output
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageToVideoCreateBodyStyle {
    /// Deprecated: Please use `quality_mode` instead. For backward compatibility, setting `high_quality: true` and `quality_mode: quick` will map to `quality_mode: studio`. Note: `quality_mode: studio` offers the same quality as `high_quality: true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality: Option<bool>,
    /// The prompt used for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// * `quick` - Fastest option for rapid results. Takes ~3 minutes per 5s of video.
    /// *  `studio` - Polished visuals with longer runtime. Takes ~8.5 minutes per 5s of video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<
        crate::models::V1ImageToVideoCreateBodyStyleQualityModeEnum,
    >,
}
