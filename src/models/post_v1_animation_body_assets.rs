/// Provide the assets for animation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AnimationBodyAssets {
    /// The path of the input audio. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls). This field is required if `audio_source` is `file`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file_path: Option<String>,
    /// Optionally add an audio source if you'd like to incorporate audio into your video
    pub audio_source: crate::models::PostV1AnimationBodyAssetsAudioSourceEnum,
    /// An initial image to use a the first frame of the video. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_path: Option<String>,
    /// Using a youtube video as the input source. This field is required if `audio_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
