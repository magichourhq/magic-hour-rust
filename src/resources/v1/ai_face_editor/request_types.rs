/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Provide the assets for face editor
    pub assets: crate::models::V1AiFaceEditorCreateBodyAssets,
    /// Give your image a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Face editing parameters
    pub style: crate::models::V1AiFaceEditorCreateBodyStyle,
}
