/// V1FaceSwapPhotoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceSwapPhotoCreateBody {
    /// Provide the assets for face swap photo
    pub assets: crate::models::V1FaceSwapPhotoCreateBodyAssets,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
