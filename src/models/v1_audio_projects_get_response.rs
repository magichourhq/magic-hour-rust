/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioProjectsGetResponse {
    pub created_at: String,
    /// The amount of credits deducted from your account to generate the audio. We charge credits right when the request is made.
    ///
    /// If an error occurred while generating the audio, credits will be refunded and this field will be updated to include the refund.
    pub credits_charged: i64,
    pub downloads: Vec<crate::models::V1AudioProjectsGetResponseDownloadsItem>,
    /// Indicates whether the resource is deleted
    pub enabled: bool,
    /// In the case of an error, this object will contain the error encountered during video render
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub error: Option<crate::models::V1AudioProjectsGetResponseError>,
    /// Unique ID of the audio. This value can be used in the [get audio project API](https://docs.magichour.ai/api-reference/audio-projects/get-audio-details) to fetch additional details such as status
    pub id: String,
    /// The name of the audio.
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    /// The status of the audio.
    pub status: crate::models::V1AudioProjectsGetResponseStatusEnum,
    /// The type of the audio project. Possible values are VOICE_GENERATOR, VOICE_CHANGER
    #[serde(rename = "type")]
    pub type_: String,
}
