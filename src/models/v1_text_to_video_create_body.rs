/// V1TextToVideoCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1TextToVideoCreateBody {
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Determines the orientation of the output video
    pub orientation: crate::models::V1TextToVideoCreateBodyOrientationEnum,
    pub style: crate::models::V1TextToVideoCreateBodyStyle,
}
