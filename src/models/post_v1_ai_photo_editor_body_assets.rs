/// Generated by Sideko (sideko.dev)
/// Provide the assets for photo editor
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiPhotoEditorBodyAssets {
    /// The image used to generate the output. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub image_file_path: String,
}