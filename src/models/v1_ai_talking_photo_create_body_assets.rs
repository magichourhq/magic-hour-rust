/// Provide the assets for creating a talking photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiTalkingPhotoCreateBodyAssets {
    /// The audio file to sync with the image. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub audio_file_path: String,
    /// The source image to animate. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub image_file_path: String,
}
