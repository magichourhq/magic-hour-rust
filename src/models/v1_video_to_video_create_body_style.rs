/// V1VideoToVideoCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VideoToVideoCreateBodyStyle {
    pub art_style: crate::models::V1VideoToVideoCreateBodyStyleArtStyleEnum,
    /// * `Dreamshaper` - a good all-around model that works for both animations as well as realism.
    /// * `Absolute Reality` - better at realism, but you'll often get similar results with Dreamshaper as well.
    /// * `Flat 2D Anime` - best for a flat illustration style that's common in most anime.
    /// * `default` - use the default recommended model for the selected art style.
    pub model: crate::models::V1VideoToVideoCreateBodyStyleModelEnum,
    /// The prompt used for the video. Prompt is required if `prompt_type` is `custom` or `append_default`. If `prompt_type` is `default`, then the `prompt` value passed will be ignored.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub prompt: Option<String>,
    /// * `default` - Use the default recommended prompt for the art style.
    /// * `custom` - Only use the prompt passed in the API. Note: for v1, lora prompt will still be auto added to apply the art style properly.
    /// * `append_default` - Add the default recommended prompt to the end of the prompt passed in the API.
    pub prompt_type: crate::models::V1VideoToVideoCreateBodyStylePromptTypeEnum,
    /// * `v1` - more detail, closer prompt adherence, and frame-by-frame previews.
    /// * `v2` - faster, more consistent, and less noisy.
    /// * `default` - use the default version for the selected art style.
    pub version: crate::models::V1VideoToVideoCreateBodyStyleVersionEnum,
}
