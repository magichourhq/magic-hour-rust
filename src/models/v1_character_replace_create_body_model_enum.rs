/// Model to use. Defaults to `wan-animate`.
///
/// * **`wan-animate`**: 480p, 720p. Supports `points` subject selection.
/// * **`kling-3.0`**: 720p, 1080p. Clips of 3–10 seconds in `replace` mode or 3–30 seconds in `animate` mode. Picks the main person automatically, so `points` are rejected.
/// * **`minimax-h3`**: 480p, 720p, 1080p. Clips of 1–60 seconds in `replace` mode or 1–60 seconds in `animate` mode. Subject `points` are `replace`-only; `animate` with `selection_mode` `point` returns 422, and with `auto` or no `selection_mode` the points are ignored.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1CharacterReplaceCreateBodyModelEnum {
    #[default]
    #[serde(rename = "kling-3.0")]
    Kling30,
    #[serde(rename = "minimax-h3")]
    MinimaxH3,
    #[serde(rename = "wan-animate")]
    WanAnimate,
}
impl std::fmt::Display for V1CharacterReplaceCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1CharacterReplaceCreateBodyModelEnum::Kling30 => "kling-3.0",
            V1CharacterReplaceCreateBodyModelEnum::MinimaxH3 => "minimax-h3",
            V1CharacterReplaceCreateBodyModelEnum::WanAnimate => "wan-animate",
        };
        write!(f, "{}", str_val)
    }
}
