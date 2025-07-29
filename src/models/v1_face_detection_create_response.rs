/// V1FaceDetectionCreateResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionCreateResponse {
    /// The credits charged for the task.
    pub credits_charged: i64,
    /// The id of the task
    pub id: String,
}
