/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::models::V1CharacterReplaceCreateBody>,
}
