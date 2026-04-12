/// Maximum resolution (longest edge) for the output image.
///
/// **Options:**
/// - `640px` — up to 640px
/// - `1k` — up to 1024px
/// - `2k` — up to 2048px
/// - `4k` — up to 4096px
/// - `auto` — **Deprecated.** Mapped server-side from your subscription tier to the best matching resolution the model supports
///
/// **Per-model support:**
/// - `qwen-edit` - 640px, 1k, 2k
/// - `nano-banana` - 640px, 1k
/// - `nano-banana-2` - 640px, 1k, 2k, 4k
/// - `seedream-v4` - 640px, 1k, 2k, 4k
/// - `nano-banana-pro` - 1k, 2k, 4k
/// - `seedream-v4.5` - 640px, 1k, 2k, 4k
///
/// Note: Resolution availability depends on the model and your subscription tier.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "1k")]
    Enum1k,
    #[serde(rename = "2k")]
    Enum2k,
    #[serde(rename = "4k")]
    Enum4k,
    #[serde(rename = "640px")]
    Enum640px,
    #[serde(rename = "auto")]
    Auto,
}
impl std::fmt::Display for V1AiImageEditorCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateBodyResolutionEnum::Enum1k => "1k",
            V1AiImageEditorCreateBodyResolutionEnum::Enum2k => "2k",
            V1AiImageEditorCreateBodyResolutionEnum::Enum4k => "4k",
            V1AiImageEditorCreateBodyResolutionEnum::Enum640px => "640px",
            V1AiImageEditorCreateBodyResolutionEnum::Auto => "auto",
        };
        write!(f, "{}", str_val)
    }
}
