/// V1VideoToVideoCreateBodyAssetsVideoSourceEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1VideoToVideoCreateBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for V1VideoToVideoCreateBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1VideoToVideoCreateBodyAssetsVideoSourceEnum::File => "file",
            V1VideoToVideoCreateBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
