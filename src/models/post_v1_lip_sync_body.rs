/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1LipSyncBody {
    pub assets: crate::models::PostV1LipSyncBodyAssets,
    pub end_seconds: f64,
    pub height: f64,
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    pub start_seconds: f64,
    pub width: f64,
}
