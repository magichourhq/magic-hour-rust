/// The status of the image.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1ImageProjectsGetResponseStatusEnum {
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
impl std::fmt::Display for V1ImageProjectsGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1ImageProjectsGetResponseStatusEnum::Canceled => "canceled",
            V1ImageProjectsGetResponseStatusEnum::Complete => "complete",
            V1ImageProjectsGetResponseStatusEnum::Draft => "draft",
            V1ImageProjectsGetResponseStatusEnum::Error => "error",
            V1ImageProjectsGetResponseStatusEnum::Queued => "queued",
            V1ImageProjectsGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
