/// Choose your video source.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1LipSyncCreateBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for V1LipSyncCreateBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1LipSyncCreateBodyAssetsVideoSourceEnum::File => "file",
            V1LipSyncCreateBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
