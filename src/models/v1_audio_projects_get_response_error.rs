/// In the case of an error, this object will contain the error encountered during video render
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AudioProjectsGetResponseError {
    /// An error code to indicate why a failure happened.
    pub code: String,
    /// Details on the reason why a failure happened.
    pub message: String,
}
