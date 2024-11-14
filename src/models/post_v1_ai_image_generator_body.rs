/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiImageGeneratorBody {
    pub image_count: f64,
    #[serde(deserialize_with = "crate::core::patch::deserialize_required_nullable")]
    pub name: Option<String>,
    pub orientation: crate::models::PostV1AiImageGeneratorBodyOrientationEnum,
    pub style: crate::models::PostV1AiImageGeneratorBodyStyle,
}