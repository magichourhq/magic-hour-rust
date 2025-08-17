/// DeleteRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    /// Unique ID of the video project. This value is returned by all of the POST APIs that create a video.
    pub id: String,
}
/// GetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    /// Unique ID of the video project. This value is returned by all of the POST APIs that create a video.
    pub id: String,
}
