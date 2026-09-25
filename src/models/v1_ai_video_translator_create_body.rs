/// V1AiVideoTranslatorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVideoTranslatorCreateBody {
    /// Source video for the translation job.
    pub assets: crate::models::V1AiVideoTranslatorCreateBodyAssets,
    /// End time of your clip (seconds). Must be greater than start_seconds. The clip must be 1-30 seconds long.
    pub end_seconds: f64,
    /// Give your video a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Output video resolution. Defaults to 480p. 720p and 1080p require a paid plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiVideoTranslatorCreateBodyResolutionEnum>,
    /// Start time of your clip (seconds). Must be ≥ 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_seconds: Option<f64>,
    /// Language to translate the video's speech into.
    pub target_language: crate::models::V1AiVideoTranslatorCreateBodyTargetLanguageEnum,
}
