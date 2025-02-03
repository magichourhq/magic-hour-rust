/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1ImageBackgroundRemoverResponse404 {
    pub message: crate::models::PostV1ImageBackgroundRemoverResponse404MessageEnum,
}
