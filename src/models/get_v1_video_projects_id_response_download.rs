/// Deprecated: Please use `.downloads` instead. The download url and expiration date of the video project
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetV1VideoProjectsIdResponseDownload {
    pub expires_at: String,
    pub url: String,
}
