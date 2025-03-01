/// The status of the video.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetV1VideoProjectsIdResponseStatusEnum {
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
impl std::fmt::Display for GetV1VideoProjectsIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetV1VideoProjectsIdResponseStatusEnum::Canceled => "canceled",
            GetV1VideoProjectsIdResponseStatusEnum::Complete => "complete",
            GetV1VideoProjectsIdResponseStatusEnum::Draft => "draft",
            GetV1VideoProjectsIdResponseStatusEnum::Error => "error",
            GetV1VideoProjectsIdResponseStatusEnum::Queued => "queued",
            GetV1VideoProjectsIdResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
