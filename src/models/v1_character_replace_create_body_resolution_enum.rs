/// Output video resolution. Defaults to 480p, the lowest resolution available on your plan.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1CharacterReplaceCreateBodyResolutionEnum {
    #[default]
    #[serde(rename = "480p")]
    Enum480p,
    #[serde(rename = "720p")]
    Enum720p,
}
impl std::fmt::Display for V1CharacterReplaceCreateBodyResolutionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1CharacterReplaceCreateBodyResolutionEnum::Enum480p => "480p",
            V1CharacterReplaceCreateBodyResolutionEnum::Enum720p => "720p",
        };
        write!(f, "{}", str_val)
    }
}
