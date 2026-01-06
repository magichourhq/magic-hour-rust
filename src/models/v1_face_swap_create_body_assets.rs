/// Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapCreateBodyAssets {
    /// This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_mappings: Option<
        Vec<crate::models::V1FaceSwapCreateBodyAssetsFaceMappingsItem>,
    >,
    /// Choose how to swap faces:
    /// **all-faces** (recommended) — swap all detected faces using one source image (`source_file_path` required)
    /// +- **individual-faces** — specify exact mappings using `face_mappings`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_swap_mode: Option<
        crate::models::V1FaceSwapCreateBodyAssetsFaceSwapModeEnum,
    >,
    /// The path of the input image with the face to be swapped.  The value is required if `face_swap_mode` is `all-faces`.
    ///
    /// This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_path: Option<String>,
    /// Your video file. Required if `video_source` is `file`. This value is either
    /// - a direct URL to the video file
    /// - `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls).
    ///
    /// See the [file upload guide](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls#input-file) for details.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_path: Option<String>,
    /// Choose your video source.
    pub video_source: crate::models::V1FaceSwapCreateBodyAssetsVideoSourceEnum,
    /// YouTube URL (required if `video_source` is `youtube`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
