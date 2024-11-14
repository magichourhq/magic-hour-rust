/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1VideoToVideoBodyStylePromptTypeEnum {
    #[default]
    #[serde(rename = "append_default")]
    AppendDefault,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "default")]
    Default,
}
impl std::fmt::Display for PostV1VideoToVideoBodyStylePromptTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1VideoToVideoBodyStylePromptTypeEnum::AppendDefault => "append_default",
            PostV1VideoToVideoBodyStylePromptTypeEnum::Custom => "custom",
            PostV1VideoToVideoBodyStylePromptTypeEnum::Default => "default",
        };
        write!(f, "{}", str_val)
    }
}