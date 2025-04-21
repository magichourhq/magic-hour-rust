/// Provide the assets for photo colorization
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1PhotoColorizerCreateBodyAssets {
    /// The image used to generate the colorized image. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub image_file_path: String,
}
