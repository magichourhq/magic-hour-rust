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
    /// The prompt used for the video. Maximum length depends on the model that runs, in characters:
    ///
    /// * **`gemini-omni-1.2`**: 2500 characters.
    /// * **`kling-2.6`**: 2500 characters.
    /// * **`kling-3.0`**: 2500 characters.
    /// * **`kling-4.0`**: 10000 characters.
    /// * **`ltx-2.5`**: 2000 characters.
    /// * **`minimax-h3`**: 20000 characters.
    /// * **`seedance-1.5`**: 2000 characters.
    /// * **`seedance-2.0`**: 2000 characters.
    /// * **`seedance-2.0-fast`**: 2000 characters.
    /// * **`seedance-2.0-mini`**: 2000 characters.
    /// * **`seedance-2.5`**: 2000 characters.
    /// * **`veo3.1`**: 2500 characters.
    /// * **`veo3.1-lite`**: 2500 characters.
    /// * **`wan-2.2`**: 10000 characters.
    /// * **`wan-3.0`**: 10000 characters.
    ///
    /// For `default`, the limit of the automatically selected model applies. Selection depends on subscription tier, resolution and duration. Specify an explicit model to use a known prompt limit. Prompts exceeding the effective model's limit are rejected, not truncated.
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
