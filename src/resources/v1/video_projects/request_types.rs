/// DeleteRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    /// The id of the video project
    pub id: String,
}
/// GetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    /// The id of the video
    pub id: String,
}
