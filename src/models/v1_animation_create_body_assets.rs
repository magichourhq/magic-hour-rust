/// Provide the assets for animation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AnimationCreateBodyAssets {
    /// The path of the input audio. This field is required if `audio_source` is `file`. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file_path: Option<String>,
    /// Optionally add an audio source if you'd like to incorporate audio into your video
    pub audio_source: crate::models::V1AnimationCreateBodyAssetsAudioSourceEnum,
    /// An initial image to use a the first frame of the video. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_path: Option<String>,
    /// Using a youtube video as the input source. This field is required if `audio_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
