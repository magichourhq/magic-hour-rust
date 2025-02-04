/// The download url and expiration date of the image project
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1VideoProjectsgetResponseDownloadsItem {
    pub expires_at: String,
    pub url: String,
}
