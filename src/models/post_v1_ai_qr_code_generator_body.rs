/// PostV1AiQrCodeGeneratorBody
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostV1AiQrCodeGeneratorBody {
    /// The content of the QR code.
    pub content: String,
    /// The name of image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub style: crate::models::PostV1AiQrCodeGeneratorBodyStyle,
}
