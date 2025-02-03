/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiPhotoEditorResponse404 {
    pub message: crate::models::PostV1AiPhotoEditorResponse404MessageEnum,
}
