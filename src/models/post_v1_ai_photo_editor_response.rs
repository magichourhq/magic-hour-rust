/// Generated by Sideko (sideko.dev)
/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiPhotoEditorResponse {
    pub frame_cost: f64,
    /// Unique ID of the image. This value can be used in the [get image project API](/api/tag/image-projects/get/v1/image-projects/{id}) to fetch additional details such as status
    pub id: String,
}
