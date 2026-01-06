/// CreateRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    /// Give your gif a custom name for easy identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The output file format for the generated animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<crate::models::V1AiGifGeneratorCreateBodyOutputFormatEnum>,
    pub style: crate::models::V1AiGifGeneratorCreateBodyStyle,
}
