/// * `default` - Use the default recommended prompt for the art style.
/// * `custom` - Only use the prompt passed in the API. Note: for v1, lora prompt will still be auto added to apply the art style properly.
/// * `append_default` - Add the default recommended prompt to the end of the prompt passed in the API.
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
