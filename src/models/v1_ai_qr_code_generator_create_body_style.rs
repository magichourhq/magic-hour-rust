/// V1AiQrCodeGeneratorCreateBodyStyle
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiQrCodeGeneratorCreateBodyStyle {
    /// To use our templates, pass in one of Watercolor, Cyberpunk City, Ink Landscape, Interior Painting, Japanese Street, Mech, Minecraft, Picasso Painting, Game Map, Spaceship, Chinese Painting, Winter Village, or pass any custom art style.
    pub art_style: String,
}
