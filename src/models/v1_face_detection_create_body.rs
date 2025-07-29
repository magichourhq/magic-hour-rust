/// V1FaceDetectionCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionCreateBody {
    /// Provide the assets for face detection
    pub assets: crate::models::V1FaceDetectionCreateBodyAssets,
    /// Confidence threshold for filtering detected faces.
    /// * Higher values (e.g., 0.9) include only faces detected with high certainty, reducing false positives.
    /// * Lower values (e.g., 0.3) include more faces, but may increase the chance of incorrect detections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
}
