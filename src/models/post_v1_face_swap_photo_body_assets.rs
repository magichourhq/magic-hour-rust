/// Generated by Sideko (sideko.dev)
/// Provide the assets for face swap photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FaceSwapPhotoBodyAssets {
    /// This is the image from which the face is extracted. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub source_file_path: String,
    /// This is the image where the face from the source image will be placed. This is the `file_path` field from the response of the [upload urls API](/docs/api/tag/files/post/v1/files/upload-urls)
    pub target_file_path: String,
}
