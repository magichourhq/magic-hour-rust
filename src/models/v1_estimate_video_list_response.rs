/// Success
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1EstimateVideoListResponse {
    /// Credits a create request with these fields charges.
    pub estimated_credits: i64,
}
