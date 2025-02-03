/// Provide the assets for background removal
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1ImageBackgroundRemoverBodyAssets {
    /// The image used to generate the image. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub image_file_path: String,
}
