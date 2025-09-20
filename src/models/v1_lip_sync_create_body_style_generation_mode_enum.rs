/// A specific version of our lip sync system, optimized for different needs.
/// * `lite` -  Fast and affordable lip sync - best for simple videos. Costs 1 credit per frame of video.
/// * `standard` -  Natural, accurate lip sync - best for most creators. Costs 1 credit per frame of video.
/// * `pro` -  Premium fidelity with enhanced detail - best for professionals. Costs 2 credits per frame of video.
///
/// Note: `standard` and `pro` are only available for users on Creator, Pro, and Business tiers.
///
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1LipSyncCreateBodyStyleGenerationModeEnum {
    #[default]
    #[serde(rename = "lite")]
    Lite,
    #[serde(rename = "pro")]
    Pro,
    #[serde(rename = "standard")]
    Standard,
}
impl std::fmt::Display for V1LipSyncCreateBodyStyleGenerationModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1LipSyncCreateBodyStyleGenerationModeEnum::Lite => "lite",
            V1LipSyncCreateBodyStyleGenerationModeEnum::Pro => "pro",
            V1LipSyncCreateBodyStyleGenerationModeEnum::Standard => "standard",
        };
        write!(f, "{}", str_val)
    }
}
