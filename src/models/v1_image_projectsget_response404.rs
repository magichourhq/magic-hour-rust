/// Requested resource is not found
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageProjectsgetResponse404 {
    pub message: crate::models::V1ImageProjectsgetResponse404MessageEnum,
}
