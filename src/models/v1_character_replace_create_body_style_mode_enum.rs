/// Processing mode. `replace` swaps the detected subject with your reference character. `animate` transfers motion from the video onto your character image.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1CharacterReplaceCreateBodyStyleModeEnum {
    #[default]
    #[serde(rename = "animate")]
    Animate,
    #[serde(rename = "replace")]
    Replace,
}
impl std::fmt::Display for V1CharacterReplaceCreateBodyStyleModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1CharacterReplaceCreateBodyStyleModeEnum::Animate => "animate",
            V1CharacterReplaceCreateBodyStyleModeEnum::Replace => "replace",
        };
        write!(f, "{}", str_val)
    }
}
