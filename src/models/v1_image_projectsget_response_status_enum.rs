/// The status of the image.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageProjectsgetResponseStatusEnum {
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
impl std::fmt::Display for V1ImageProjectsgetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageProjectsgetResponseStatusEnum::Canceled => "canceled",
            V1ImageProjectsgetResponseStatusEnum::Complete => "complete",
            V1ImageProjectsgetResponseStatusEnum::Draft => "draft",
            V1ImageProjectsgetResponseStatusEnum::Error => "error",
            V1ImageProjectsgetResponseStatusEnum::Queued => "queued",
            V1ImageProjectsgetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
