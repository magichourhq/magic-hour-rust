/// Generated by Sideko (sideko.dev)
/// PostV1AiImageGeneratorResponse404MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AiImageGeneratorResponse404MessageEnum {
    #[default]
    #[serde(rename = "Not Found")]
    NotFound,
}
impl std::fmt::Display for PostV1AiImageGeneratorResponse404MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AiImageGeneratorResponse404MessageEnum::NotFound => "Not Found",
        };
        write!(f, "{}", str_val)
    }
}
