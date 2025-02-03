/// PostV1TextToVideoBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1TextToVideoBody {
    /// The total duration of the output video in seconds.
    pub end_seconds: f64,
    /// The name of video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Determines the orientation of the output video
    pub orientation: crate::models::PostV1TextToVideoBodyOrientationEnum,
    pub style: crate::models::PostV1TextToVideoBodyStyle,
}
