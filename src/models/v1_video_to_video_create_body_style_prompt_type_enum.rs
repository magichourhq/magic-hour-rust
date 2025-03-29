/// * `default` - Use the default recommended prompt for the art style.
/// * `custom` - Only use the prompt passed in the API. Note: for v1, lora prompt will still be auto added to apply the art style properly.
/// * `append_default` - Add the default recommended prompt to the end of the prompt passed in the API.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyStylePromptTypeEnum {
    #[default]
    #[serde(rename = "append_default")]
    AppendDefault,
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "default")]
    Default,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyStylePromptTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyStylePromptTypeEnum::AppendDefault => {
                "append_default"
            }
            V1VideoToVideoCreateBodyStylePromptTypeEnum::Custom => "custom",
            V1VideoToVideoCreateBodyStylePromptTypeEnum::Default => "default",
        };
        write!(f, "{}", str_val)
    }
}
