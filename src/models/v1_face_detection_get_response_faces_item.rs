/// V1FaceDetectionGetResponseFacesItem
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1FaceDetectionGetResponseFacesItem {
    /// The path to the face image. This should be used in face swap photo/video API calls as `.assets.face_mappings.original_face`
    pub path: String,
    /// The url to the face image. This is used to render the image in your applications.
    pub url: String,
}
