/// Provide the assets for headshot photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiHeadshotGeneratorCreateBodyAssets {
    /// The image used to generate the headshot. This image must contain one detectable face. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub image_file_path: String,
}
