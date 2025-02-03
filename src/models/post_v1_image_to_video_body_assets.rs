/// Provide the assets for image-to-video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1ImageToVideoBodyAssets {
    /// The path of the image file. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub image_file_path: String,
}
