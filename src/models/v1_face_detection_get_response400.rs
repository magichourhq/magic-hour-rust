/// The request is invalid
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionGetResponse400 {
    pub message: String,
}
