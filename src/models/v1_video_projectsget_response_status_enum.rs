/// The status of the video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoProjectsgetResponseStatusEnum {
    #[default]
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "rendering")]
    Rendering,
}
impl std::fmt::Display for V1VideoProjectsgetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoProjectsgetResponseStatusEnum::Canceled => "canceled",
            V1VideoProjectsgetResponseStatusEnum::Complete => "complete",
            V1VideoProjectsgetResponseStatusEnum::Draft => "draft",
            V1VideoProjectsgetResponseStatusEnum::Error => "error",
            V1VideoProjectsgetResponseStatusEnum::Queued => "queued",
            V1VideoProjectsgetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
