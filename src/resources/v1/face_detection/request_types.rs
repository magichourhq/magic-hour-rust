/// GetRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    /// The id of the task. This value is returned by the [face detection API](/api-reference/files/face-detection#response-id).
    pub id: String,
}
/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for face detection
    pub assets: crate::models::V1FaceDetectionCreateBodyAssets,
    /// Confidence threshold for filtering detected faces.
    /// * Higher values (e.g., 0.9) include only faces detected with high certainty, reducing false positives.
    /// * Lower values (e.g., 0.3) include more faces, but may increase the chance of incorrect detections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
}
