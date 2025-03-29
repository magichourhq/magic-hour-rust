/// V1AiQrCodeGeneratorCreateBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiQrCodeGeneratorCreateBody {
    /// The content of the QR code.
    pub content: String,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::V1AiQrCodeGeneratorCreateBodyStyle,
}
