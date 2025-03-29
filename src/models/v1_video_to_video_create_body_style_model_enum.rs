/// * `Dreamshaper` - a good all-around model that works for both animations as well as realism.
/// * `Absolute Reality` - better at realism, but you'll often get similar results with Dreamshaper as well.
/// * `Flat 2D Anime` - best for a flat illustration style that's common in most anime.
/// * `default` - use the default recommended model for the selected art style.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyStyleModelEnum {
    #[default]
    #[serde(rename = "Absolute Reality")]
    AbsoluteReality,
    #[serde(rename = "Dreamshaper")]
    Dreamshaper,
    #[serde(rename = "Flat 2D Anime")]
    Flat2dAnime,
    #[serde(rename = "default")]
    Default,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyStyleModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyStyleModelEnum::AbsoluteReality => "Absolute Reality",
            V1VideoToVideoCreateBodyStyleModelEnum::Dreamshaper => "Dreamshaper",
            V1VideoToVideoCreateBodyStyleModelEnum::Flat2dAnime => "Flat 2D Anime",
            V1VideoToVideoCreateBodyStyleModelEnum::Default => "default",
        };
        write!(f, "{}", str_val)
    }
}
