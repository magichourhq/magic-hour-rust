/// V1AiImageEditorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageEditorCreateBodyStyle {
    /// The AI model to use for image editing.
    /// * `Nano Banana` - Precise, realistic edits with consistent results
    /// * `Seedream` - Creative, imaginative images with artistic freedom
    /// * `default` - Use the model we recommend, which will change over time. This is recommended unless you need a specific model. This is the default behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::models::V1AiImageEditorCreateBodyStyleModelEnum>,
    /// The prompt used to edit the image.
    pub prompt: String,
}
