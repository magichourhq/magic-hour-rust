/// How to locate the subject in the source video. `auto` detects a person automatically. `point` uses your `points` to mark the subject. Defaults to `auto`.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1CharacterReplaceCreateBodyStyleSelectionModeEnum {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "point")]
    Point,
}
impl std::fmt::Display for V1CharacterReplaceCreateBodyStyleSelectionModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Auto => "auto",
            V1CharacterReplaceCreateBodyStyleSelectionModeEnum::Point => "point",
        };
        write!(f, "{}", str_val)
    }
}
