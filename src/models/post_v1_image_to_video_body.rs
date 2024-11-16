/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1ImageToVideoBody {
    pub assets: crate::models::PostV1ImageToVideoBodyAssets,
    pub end_seconds: f64,
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::PostV1ImageToVideoBodyStyle,
    pub width: f64,
}
