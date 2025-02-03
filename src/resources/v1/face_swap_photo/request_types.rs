/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for face swap photo
    pub assets: crate::models::PostV1FaceSwapPhotoBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
