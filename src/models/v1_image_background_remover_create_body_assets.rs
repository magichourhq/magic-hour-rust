/// Provide the assets for background removal
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageBackgroundRemoverCreateBodyAssets {
    /// The image used as the new background for the image_file_path. This image will be resized to match the image in image_file_path. Please make sure the resolution between the images are similar.
    ///
    /// This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image_file_path: Option<String>,
    /// The image to remove the background. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub image_file_path: String,
}
