/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiGifGeneratorCreateResponse {
    /// The frame cost of the image generation
    pub frame_cost: i64,
    /// Unique ID of the image. This value can be used in the [get image project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch additional details such as status
    pub id: String,
}
