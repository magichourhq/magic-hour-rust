/// Why the face detection failed. Present only when `status` is `error`; `null` otherwise.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionGetResponseError {
    /// An error code to indicate why a failure happened.
    pub code: String,
    /// Details on the reason why a failure happened.
    pub message: String,
}
