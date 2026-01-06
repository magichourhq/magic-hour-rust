/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceClonerCreateResponse {
    /// The amount of credits deducted from your account to generate the audio. We charge credits right when the request is made.
    ///
    /// If an error occurred while generating the audio, credits will be refunded and this field will be updated to include the refund.
    pub credits_charged: i64,
    /// Unique ID of the audio. Use it with the [Get audio Project API](https://docs.magichour.ai/api-reference/audio-projects/get-audio-details) to fetch status and downloads.
    pub id: String,
}
