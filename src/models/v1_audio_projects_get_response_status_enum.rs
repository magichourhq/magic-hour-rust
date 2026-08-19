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
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AudioProjectsGetResponseStatusEnum {
    #[default]
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "rendering")]
    Rendering,
}
impl std::fmt::Display for V1AudioProjectsGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AudioProjectsGetResponseStatusEnum::Canceled => "canceled",
            V1AudioProjectsGetResponseStatusEnum::Complete => "complete",
            V1AudioProjectsGetResponseStatusEnum::Draft => "draft",
            V1AudioProjectsGetResponseStatusEnum::Error => "error",
            V1AudioProjectsGetResponseStatusEnum::Queued => "queued",
            V1AudioProjectsGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
