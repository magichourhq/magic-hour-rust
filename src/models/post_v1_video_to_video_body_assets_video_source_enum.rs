/// PostV1VideoToVideoBodyAssetsVideoSourceEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1VideoToVideoBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for PostV1VideoToVideoBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1VideoToVideoBodyAssetsVideoSourceEnum::File => "file",
            PostV1VideoToVideoBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
