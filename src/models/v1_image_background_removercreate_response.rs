/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageBackgroundRemovercreateResponse {
    /// The frame cost of the image generation
    pub frame_cost: i64,
    /// Unique ID of the image. This value can be used in the [get image project API](/api/tag/image-projects/get/v1/image-projects/{id}) to fetch additional details such as status
    pub id: String,
}
