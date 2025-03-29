/// The status of the video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoProjectsGetResponseStatusEnum {
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
impl std::fmt::Display for V1VideoProjectsGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoProjectsGetResponseStatusEnum::Canceled => "canceled",
            V1VideoProjectsGetResponseStatusEnum::Complete => "complete",
            V1VideoProjectsGetResponseStatusEnum::Draft => "draft",
            V1VideoProjectsGetResponseStatusEnum::Error => "error",
            V1VideoProjectsGetResponseStatusEnum::Queued => "queued",
            V1VideoProjectsGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
