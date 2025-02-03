/// PostV1FaceSwapBodyAssetsVideoSourceEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1FaceSwapBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for PostV1FaceSwapBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1FaceSwapBodyAssetsVideoSourceEnum::File => "file",
            PostV1FaceSwapBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
