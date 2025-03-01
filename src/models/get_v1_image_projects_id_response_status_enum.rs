/// The status of the image.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum GetV1ImageProjectsIdResponseStatusEnum {
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
impl std::fmt::Display for GetV1ImageProjectsIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetV1ImageProjectsIdResponseStatusEnum::Canceled => "canceled",
            GetV1ImageProjectsIdResponseStatusEnum::Complete => "complete",
            GetV1ImageProjectsIdResponseStatusEnum::Draft => "draft",
            GetV1ImageProjectsIdResponseStatusEnum::Error => "error",
            GetV1ImageProjectsIdResponseStatusEnum::Queued => "queued",
            GetV1ImageProjectsIdResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
