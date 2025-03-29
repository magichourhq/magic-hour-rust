/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AnimationCreateResponse404 {
    pub message: crate::models::V1AnimationCreateResponse404MessageEnum,
}
