/// Defines the style of the output video
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AnimationCreateBodyStyle {
    /// The art style used to create the output video
    pub art_style: crate::models::V1AnimationCreateBodyStyleArtStyleEnum,
    /// Describe custom art style. This field is required if `art_style` is `Custom`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art_style_custom: Option<String>,
    /// The camera effect used to create the output video
    pub camera_effect: crate::models::V1AnimationCreateBodyStyleCameraEffectEnum,
    /// The prompt used for the video. Prompt is required if `prompt_type` is `custom`. Otherwise this value is ignored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    ///
    /// * `custom` - Use your own prompt for the video.
    /// * `use_lyrics` - Use the lyrics of the audio to create the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`.
    /// * `ai_choose` - Let AI write the prompt. If this option is selected, then `assets.audio_source` must be `file` or `youtube`.
    pub prompt_type: crate::models::V1AnimationCreateBodyStylePromptTypeEnum,
    /// Change determines how quickly the video's content changes across frames.
    /// * Higher = more rapid transitions.
    /// * Lower = more stable visual experience.
    pub transition_speed: i64,
}
