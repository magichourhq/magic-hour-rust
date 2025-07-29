/// Provide the assets for face swap. For video, The `video_source` field determines whether `video_file_path` or `youtube_url` field is used
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapCreateBodyAssets {
    /// This is the array of face mappings used for multiple face swap. The value is required if `face_swap_mode` is `individual-faces`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_mappings: Option<
        Vec<crate::models::V1FaceSwapCreateBodyAssetsFaceMappingsItem>,
    >,
    /// The mode of face swap.
    /// * `all-faces` - Swap all faces in the target image or video. `source_file_path` is required.
    /// * `individual-faces` - Swap individual faces in the target image or video. `source_faces` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_swap_mode: Option<
        crate::models::V1FaceSwapCreateBodyAssetsFaceSwapModeEnum,
    >,
    /// The path of the input image with the face to be swapped.  The value is required if `face_swap_mode` is `all-faces`.
    ///
    /// This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_file_path: Option<String>,
    /// The path of the input video. This field is required if `video_source` is `file`. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file_path: Option<String>,
    pub video_source: crate::models::V1FaceSwapCreateBodyAssetsVideoSourceEnum,
    /// Using a youtube video as the input source. This field is required if `video_source` is `youtube`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_url: Option<String>,
}
