/// V1AiImageEditorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiImageEditorCreateBody {
    /// Provide the assets for image edit
    pub assets: crate::models::V1AiImageEditorCreateBodyAssets,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiImageEditorCreateBodyStyle,
}
