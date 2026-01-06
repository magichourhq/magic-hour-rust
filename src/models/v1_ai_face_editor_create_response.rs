/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiFaceEditorCreateResponse {
    /// The amount of credits deducted from your account to generate the image. We charge credits right when the request is made.
    ///
    /// If an error occurred while generating the image(s), credits will be refunded and this field will be updated to include the refund.
    pub credits_charged: i64,
    /// Deprecated: Previously represented the number of frames (original name of our credit system) used for image generation. Use 'credits_charged' instead.
    pub frame_cost: i64,
    /// Unique ID of the image. Use it with the [Get image Project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch status and downloads.
    pub id: String,
}
