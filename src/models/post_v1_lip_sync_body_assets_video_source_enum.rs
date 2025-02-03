/// PostV1LipSyncBodyAssetsVideoSourceEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1LipSyncBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for PostV1LipSyncBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1LipSyncBodyAssetsVideoSourceEnum::File => "file",
            PostV1LipSyncBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
