/// V1AiFaceEditorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiFaceEditorCreateBody {
    /// Provide the assets for face editor
    pub assets: crate::models::V1AiFaceEditorCreateBodyAssets,
    /// The name of image. This value is mainly used for your own identification of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Face editing parameters
    pub style: crate::models::V1AiFaceEditorCreateBodyStyle,
}
