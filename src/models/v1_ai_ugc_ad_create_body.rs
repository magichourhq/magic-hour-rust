/// V1AiUgcAdCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiUgcAdCreateBody {
    pub assets: crate::models::V1AiUgcAdCreateBodyAssets,
    /// Video duration in seconds. Supported values by model:
    /// * kling-3.0: 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 seconds.
    /// * ltx-2.5: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60 seconds.
    /// * veo3.1: 4, 6, 8, 15, 22, 29, 36, 43, 50, 57, 64, 71, 78, 85, 92, 99, 106, 113, 120, 127, 134, 141 seconds.
    /// Free plans are limited to 5 seconds unless a credit pack has been purchased.
    pub end_seconds: f64,
    /// Defaults to kling-3.0 on paid plans and ltx-2.5 on free plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiUgcAdCreateBodyModelEnum>,
    /// Defaults to 720p on paid plans and 480p on free plans. Unsupported model/resolution combinations and resolutions unavailable for your plan are rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<crate::models::V1AiUgcAdCreateBodyResolutionEnum>,
    /// The words the actor should speak. Character limits apply after trimming:
    /// * kling-3.0: 2500 characters.
    /// * ltx-2.5: 1804 characters.
    /// * veo3.1: 2500 characters.
    /// LTX's dialogue wrapper counts toward its model prompt limit.
    pub script: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
