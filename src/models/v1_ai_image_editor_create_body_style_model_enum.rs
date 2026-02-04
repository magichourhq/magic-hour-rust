/// Deprecated: Please use `model` instead. The AI model to use for image editing.
/// * `Nano Banana` - Precise, realistic edits with consistent results
/// * `Seedream` - Creative, imaginative images with artistic freedom
/// * `default` - Use the model we recommend, which will change over time. This is recommended unless you need a specific model. This is the default behavior.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateBodyStyleModelEnum {
    #[default]
    #[serde(rename = "Nano Banana")]
    NanoBanana,
    #[serde(rename = "Seedream")]
    Seedream,
    #[serde(rename = "default")]
    Default,
}
impl std::fmt::Display for V1AiImageEditorCreateBodyStyleModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateBodyStyleModelEnum::NanoBanana => "Nano Banana",
            V1AiImageEditorCreateBodyStyleModelEnum::Seedream => "Seedream",
            V1AiImageEditorCreateBodyStyleModelEnum::Default => "default",
        };
        write!(f, "{}", str_val)
    }
}
