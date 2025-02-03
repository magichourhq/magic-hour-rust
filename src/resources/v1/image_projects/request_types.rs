/// DeleteRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    /// The id of the image project
    pub id: String,
}
/// GetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    /// The id of the image project
    pub id: String,
}
