/// Provide the assets for headshot photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiHeadshotGeneratorBodyAssets {
    /// The image used to generate the headshot. This image must contain one detectable face. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub image_file_path: String,
}
