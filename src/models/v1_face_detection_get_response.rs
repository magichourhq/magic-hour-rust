/// V1FaceDetectionGetResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionGetResponse {
    /// The credits charged for the task.
    pub credits_charged: i64,
    /// The faces detected in the image or video. The list is populated as faces are detected.
    pub faces: Vec<crate::models::V1FaceDetectionGetResponseFacesItem>,
    /// The id of the task
    pub id: String,
    /// The status of the detection.
    pub status: crate::models::V1FaceDetectionGetResponseStatusEnum,
}
