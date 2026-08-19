/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioProjectsGetResponse {
    pub created_at: String,
    /// The amount of credits deducted from your account to generate the audio. We charge credits right when the request is made.
    ///
    /// If an error occurred while generating the audio, credits will be refunded and this field will be updated to include the refund.
    pub credits_charged: i64,
    pub downloads: Vec<crate::models::V1AudioProjectsGetResponseDownloadsItem>,
    /// Whether this resource is active. If false, it is deleted.
    pub enabled: bool,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1AudioProjectsGetResponseError>,
    /// Unique ID of the audio. Use it with the [Get audio Project API](https://docs.magichour.ai/api-reference/audio-projects/get-audio-details) to fetch status and downloads.
    pub id: String,
    /// The name of the audio.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The status of the audio.
    ///
    /// - `draft` - the project was created but has not been submitted for rendering
    /// - `queued` - the job is waiting for an available server
    /// - `rendering` - the job is being processed; the `audio.started` webhook event fires when rendering begins
    /// - `complete` - the job finished successfully; fires `audio.completed`
    /// - `error` - the job failed during processing; fires `audio.errored`
    /// - `canceled` - the job was manually canceled (for example from the Magic Hour web app)
    ///
    /// **Note:** `rendering`, `complete`, and `error` have matching webhook events; `canceled` does not - a canceled job emits no webhook event, so poll this endpoint to detect cancellation.
    pub status: crate::models::V1AudioProjectsGetResponseStatusEnum,
    /// The type of the audio project. Possible values are VOICE_GENERATOR, VOICE_CHANGER, VOICE_CLONER, VIDEO_TO_AUDIO, MUSIC_GENERATOR
    #[serde(rename = "type")]
    pub type_: String,
}
