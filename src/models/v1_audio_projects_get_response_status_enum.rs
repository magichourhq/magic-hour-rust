/// The status of the audio.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AudioProjectsGetResponseStatusEnum {
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
impl std::fmt::Display for V1AudioProjectsGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AudioProjectsGetResponseStatusEnum::Canceled => "canceled",
            V1AudioProjectsGetResponseStatusEnum::Complete => "complete",
            V1AudioProjectsGetResponseStatusEnum::Draft => "draft",
            V1AudioProjectsGetResponseStatusEnum::Error => "error",
            V1AudioProjectsGetResponseStatusEnum::Queued => "queued",
            V1AudioProjectsGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
