/// * `Dreamshaper` - a good all-around model that works for both animations as well as realism.
/// * `Absolute Reality` - better at realism, but you'll often get similar results with Dreamshaper as well.
/// * `Flat 2D Anime` - best for a flat illustration style that's common in most anime.
/// * `default` - use the default recommended model for the selected art style.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1VideoToVideoBodyStyleModelEnum {
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
impl std::fmt::Display for PostV1VideoToVideoBodyStyleModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1VideoToVideoBodyStyleModelEnum::AbsoluteReality => "Absolute Reality",
            PostV1VideoToVideoBodyStyleModelEnum::Dreamshaper => "Dreamshaper",
            PostV1VideoToVideoBodyStyleModelEnum::Flat2dAnime => "Flat 2D Anime",
            PostV1VideoToVideoBodyStyleModelEnum::Default => "default",
        };
        write!(f, "{}", str_val)
    }
}
