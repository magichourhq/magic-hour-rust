/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideocreateResponse404 {
    pub message: crate::models::V1TextToVideocreateResponse404MessageEnum,
}
