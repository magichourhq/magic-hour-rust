/// V1TextToVideoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBody {
    /// Determines the aspect ratio of the output video.
    /// * **Seedance**: Supports `9:16`, `16:9`, `1:1`.
    /// * **Kling 2.5 Audio**: Supports `9:16`, `16:9`, `1:1`.
    /// * **Sora 2**: Supports `9:16`, `16:9`.
    /// * **Veo 3.1 Audio**: Supports `9:16`, `16:9`.
    /// * **Veo 3.1**: Supports `9:16`, `16:9`.
    /// * **Kling 1.6**: Supports `9:16`, `16:9`, `1:1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<crate::models::V1TextToVideoCreateBodyAspectRatioEnum>,
    /// The total duration of the output video in seconds.
    ///
    /// Supported durations depend on the chosen model:
    /// * **Default**: 5-60 seconds (either 5 or 10 for 480p).
    /// * **Seedance**: 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12
    /// * **Kling 2.5 Audio**: 5, 10
    /// * **Sora 2**: 4, 8, 12, 24, 36, 48, 60
    /// * **Veo 3.1 Audio**: 4, 6, 8, 16, 24, 32, 40, 48, 56
    /// * **Veo 3.1**: 4, 6, 8, 16, 24, 32, 40, 48, 56
    /// * **Kling 1.6**: 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60
    pub end_seconds: f64,
    /// The AI model to use for video generation.
    /// * `default`: Our recommended model for general use (Kling 2.5 Audio). Note: For backward compatibility, if you use default and end_seconds > 10, we'll fall back to Kling 1.6.
    /// * `seedance`: Great for fast iteration and start/end frame
    /// * `kling-2.5-audio`: Great for motion, action, and camera control
    /// * `sora-2`: Great for story-telling, dialogue & creativity
    /// * `veo3.1-audio`: Great for dialogue + SFX generated natively
    /// * `veo3.1`: Great for realism, polish, & prompt adherence
    /// * `kling-1.6`: Great for dependable clips with smooth motion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1TextToVideoCreateBodyModelEnum>,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Deprecated. Use `aspect_ratio` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::models::V1TextToVideoCreateBodyOrientationEnum>,
    /// Controls the output video resolution. Defaults to `720p` if not specified.
    ///
    /// * **Default**: Supports `480p`, `720p`, and `1080p`.
    /// * **Seedance**: Supports `480p`, `720p`, `1080p`.
    /// * **Kling 2.5 Audio**: Supports `720p`, `1080p`.
    /// * **Sora 2**: Supports `720p`.
    /// * **Veo 3.1 Audio**: Supports `720p`, `1080p`.
    /// * **Veo 3.1**: Supports `720p`, `1080p`.
    /// * **Kling 1.6**: Supports `720p`, `1080p`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1TextToVideoCreateBodyResolutionEnum>,
    pub style: crate::models::V1TextToVideoCreateBodyStyle,
}
