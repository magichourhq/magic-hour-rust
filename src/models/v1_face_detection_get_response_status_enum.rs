/// The status of the detection.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceDetectionGetResponseStatusEnum {
    #[default]
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "rendering")]
    Rendering,
}
impl std::fmt::Display for V1FaceDetectionGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceDetectionGetResponseStatusEnum::Complete => "complete",
            V1FaceDetectionGetResponseStatusEnum::Error => "error",
            V1FaceDetectionGetResponseStatusEnum::Queued => "queued",
            V1FaceDetectionGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
