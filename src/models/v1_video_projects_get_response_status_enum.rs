/// The status of the video.
///
/// - `draft` - the project was created but has not been submitted for rendering
/// - `queued` - the job is waiting for an available server
/// - `rendering` - the job is being processed; the `video.started` webhook event fires when rendering begins
/// - `complete` - the job finished successfully; fires `video.completed`
/// - `error` - the job failed during processing; fires `video.errored`
/// - `canceled` - the job was manually canceled (for example from the Magic Hour web app)
///
/// **Note:** `rendering`, `complete`, and `error` have matching webhook events; `canceled` does not - a canceled job emits no webhook event, so poll this endpoint to detect cancellation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoProjectsGetResponseStatusEnum {
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
impl std::fmt::Display for V1VideoProjectsGetResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoProjectsGetResponseStatusEnum::Canceled => "canceled",
            V1VideoProjectsGetResponseStatusEnum::Complete => "complete",
            V1VideoProjectsGetResponseStatusEnum::Draft => "draft",
            V1VideoProjectsGetResponseStatusEnum::Error => "error",
            V1VideoProjectsGetResponseStatusEnum::Queued => "queued",
            V1VideoProjectsGetResponseStatusEnum::Rendering => "rendering",
        };
        write!(f, "{}", str_val)
    }
}
