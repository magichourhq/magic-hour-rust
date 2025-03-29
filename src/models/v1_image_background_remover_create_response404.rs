/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageBackgroundRemoverCreateResponse404 {
    pub message: crate::models::V1ImageBackgroundRemoverCreateResponse404MessageEnum,
}
