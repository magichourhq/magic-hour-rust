/// Custom subtitle configuration.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AutoSubtitleGeneratorCreateBodyStyleCustomConfig {
    /// Font name from Google Fonts. Not all fonts support all languages or character sets.
    /// We recommend verifying language support and appearance directly on https://fonts.google.com before use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Font size in pixels. If not provided, the font size is automatically calculated based on the video resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,
    /// Font style (e.g., normal, italic, bold)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,
    /// Color used to highlight the current spoken text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlighted_text_color: Option<String>,
    /// Horizontal alignment of the text (e.g., left, center, right)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_position: Option<String>,
    /// Stroke (outline) color of the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    /// Width of the text stroke in pixels. If `stroke_color` is provided, but `stroke_width` is not, the `stroke_width` will be calculated automatically based on the font size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
    /// Primary text color in hex format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    /// Vertical alignment of the text (e.g., top, center, bottom)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_position: Option<String>,
}
