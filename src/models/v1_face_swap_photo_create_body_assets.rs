/// Provide the assets for face swap photo
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapPhotoCreateBodyAssets {
    /// This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_mappings: Option<
        Vec<crate::models::V1FaceSwapPhotoCreateBodyAssetsFaceMappingsItem>,
    >,
    /// Choose how to swap faces:
    /// **all-faces** (recommended) — swap all detected faces using one source image (`source_file_path` required)
    /// +- **individual-faces** — specify exact mappings using `face_mappings`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_swap_mode: Option<
        crate::models::V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum,
    >,
    /// This is the image from which the face is extracted. The value is required if `face_swap_mode` is `all-faces`.
    ///
    /// This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file_path: Option<String>,
    /// This is the image where the face from the source image will be placed. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    pub target_file_path: String,
}
