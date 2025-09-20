/// The download url and expiration date of the audio project
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioProjectsGetResponseDownloadsItem {
    pub expires_at: String,
    pub url: String,
}
