/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageEditorCreateResponse404 {
    pub message: crate::models::V1AiImageEditorCreateResponse404MessageEnum,
}
