/// Attributes used to dictate the style of the output
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiTalkingPhotoCreateBodyStyle {
    /// Controls overall motion style.
    /// * `pro` -  Higher fidelity, realistic detail, accurate lip sync, and faster generation.
    /// * `standard` -  More expressive motion, but lower visual fidelity.
    ///
    /// * `expressive` - More motion and facial expressiveness; may introduce visual artifacts. (Deprecated: passing this value will be treated as `standard`)
    /// * `stable` -  Reduced motion for cleaner output; may result in minimal animation. (Deprecated: passing this value will be treated as `pro`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_mode: Option<
        crate::models::V1AiTalkingPhotoCreateBodyStyleGenerationModeEnum,
    >,
    /// Note: this value is only applicable when generation_mode is `expressive`. The value can include up to 2 decimal places.
    /// * Lower values yield more stability but can suppress mouth movement.
    /// * Higher values increase motion and expressiveness, with a higher risk of distortion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intensity: Option<f64>,
}
