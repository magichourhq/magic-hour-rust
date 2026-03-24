/// Maximum resolution for the generated image.
///
/// **Options:**
/// - `auto` - Automatic resolution (all tiers, default)
/// - `2k` - Up to 2048px (requires Pro or Business tier)
/// - `4k` - Up to 4096px (requires Business tier)
///
/// Note: Resolution availability depends on your subscription tier. Defaults to `auto` if not specified.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "2k")]
    Enum2k,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "auto")]
    Auto,
}
impl std::fmt::Display for V1AiImageEditorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateBodyResolutionEnum::Enum2k => "2k",
            V1AiImageEditorCreateBodyResolutionEnum::Enum4k => "4k",
            V1AiImageEditorCreateBodyResolutionEnum::Auto => "auto",
        };
        write!(f, "{}", str_val)
    }
}
