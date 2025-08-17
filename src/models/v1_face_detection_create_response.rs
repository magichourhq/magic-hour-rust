/// V1FaceDetectionCreateResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionCreateResponse {
    /// The credits charged for the task.
    pub credits_charged: i64,
    /// The id of the task. Use this value in the [get face detection details API](/api-reference/files/get-face-detection-details) to get the details of the face detection task.
    pub id: String,
}
