/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1ImageProjectsGetResponse {
    pub created_at: String,
    /// The amount of credits deducted from your account to generate the image. We charge credits right when the request is made.
    ///
    /// If an error occurred while generating the image(s), credits will be refunded and this field will be updated to include the refund.
    pub credits_charged: i64,
    pub downloads: Vec<crate::models::V1ImageProjectsGetResponseDownloadsItem>,
    /// Whether this resource is active. If false, it is deleted.
    pub enabled: bool,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1ImageProjectsGetResponseError>,
    /// Unique ID of the image. Use it with the [Get image Project API](https://docs.magichour.ai/api-reference/image-projects/get-image-details) to fetch status and downloads.
    pub id: String,
    /// Number of images generated
    pub image_count: i64,
    /// The name of the image.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The status of the image.
    ///
    /// - `draft` - the project was created but has not been submitted for rendering
    /// - `queued` - the job is waiting for an available server
    /// - `rendering` - the job is being processed; the `image.started` webhook event fires when rendering begins
    /// - `complete` - the job finished successfully; fires `image.completed`
    /// - `error` - the job failed during processing; fires `image.errored`
    /// - `canceled` - the job was manually canceled (for example from the Magic Hour web app)
    ///
    /// **Note:** `rendering`, `complete`, and `error` have matching webhook events; `canceled` does not - a canceled job emits no webhook event, so poll this endpoint to detect cancellation.
    pub status: crate::models::V1ImageProjectsGetResponseStatusEnum,
    /// Deprecated: Previously represented the number of frames (original name of our credit system) used for image generation. Use 'credits_charged' instead.
    pub total_frame_cost: i64,
    /// The type of the image project. Possible values are FACE_EDITOR, AI_IMAGE_EDITOR, AI_SELFIE, AI_HEADSHOT, AI_INFLUENCER, AI_IMAGE, AI_MEME, CLOTHES_CHANGER, BACKGROUND_REMOVER, FACE_SWAP, IMAGE_UPSCALER, IMAGE_ENHANCER, AI_GIF, QR_CODE, PHOTO_EDITOR, PHOTO_COLORIZER, IMAGE_COLOR_GRADER, HEAD_SWAP, BODY_SWAP, STORYBOARD, IMAGE_EXPANDER
    #[serde(rename = "type")]
    pub type_: String,
}
