/// V1AiImageEditorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageEditorCreateBodyStyle {
    /// The prompt used to edit the image.
    pub prompt: String,
}
