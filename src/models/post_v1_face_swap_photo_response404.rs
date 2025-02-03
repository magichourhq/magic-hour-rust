/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1FaceSwapPhotoResponse404 {
    pub message: crate::models::PostV1FaceSwapPhotoResponse404MessageEnum,
}
