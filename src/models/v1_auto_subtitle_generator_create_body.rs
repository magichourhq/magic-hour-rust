/// V1AutoSubtitleGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AutoSubtitleGeneratorCreateBody {
    /// Provide the assets for auto subtitle generator
    pub assets: crate::models::V1AutoSubtitleGeneratorCreateBodyAssets,
    /// The end time of the input video in seconds
    pub end_seconds: f64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The start time of the input video in seconds
    pub start_seconds: f64,
    /// Style of the subtitle. At least one of `.style.template` or `.style.custom_config` must be provided.
    /// * If only `.style.template` is provided, default values for the template will be used.
    /// * If both are provided, the fields in `.style.custom_config` will be used to overwrite the fields in `.style.template`.
    /// * If only `.style.custom_config` is provided, then all fields in `.style.custom_config` will be used.
    ///
    /// To use custom config only, the following `custom_config` params are required:
    /// * `.style.custom_config.font`
    /// * `.style.custom_config.text_color`
    /// * `.style.custom_config.vertical_position`
    /// * `.style.custom_config.horizontal_position`
    ///
    pub style: crate::models::V1AutoSubtitleGeneratorCreateBodyStyle,
}
