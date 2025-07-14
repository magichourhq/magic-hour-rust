/// Attributed used to dictate the style of the output
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageToVideoCreateBodyStyle {
    /// Deprecated: Please use `resolution` instead. For backward compatibility,
    /// * `false` maps to 720p resolution
    /// * `true` maps to 1080p resolution
    ///
    /// This field will be removed in a future version. Use the `resolution` field to directly specify the resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_quality: Option<bool>,
    /// The prompt used for the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// DEPRECATED: Please use `resolution` field instead. For backward compatibility:
    /// * `quick` maps to 720p resolution
    /// * `studio` maps to 1080p resolution
    ///
    /// This field will be removed in a future version. Use the `resolution` field to directly to specify the resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_mode: Option<
        crate::models::V1ImageToVideoCreateBodyStyleQualityModeEnum,
    >,
}
