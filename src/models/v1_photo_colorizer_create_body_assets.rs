/// Provide the assets for photo colorization
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1PhotoColorizerCreateBodyAssets {
    /// The image used to generate the colorized image. This value is either
    /// - a direct URL to the file. The URL path must end in a supported file extension.
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub image_file_path: String,
}
