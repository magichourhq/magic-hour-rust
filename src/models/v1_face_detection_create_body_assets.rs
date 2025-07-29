/// Provide the assets for face detection
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionCreateBodyAssets {
    /// This is the image or video where the face will be detected. This value can be either the `file_path` field from the response of the [upload urls API](https://docs.magichour.ai/api-reference/files/generate-asset-upload-urls), or the url of the file.
    pub target_file_path: String,
}
