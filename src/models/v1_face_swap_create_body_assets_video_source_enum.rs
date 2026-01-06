/// Choose your video source.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceSwapCreateBodyAssetsVideoSourceEnum {
    #[default]
    #[serde(rename = "file")]
    File,
    #[serde(rename = "youtube")]
    Youtube,
}
impl std::fmt::Display for V1FaceSwapCreateBodyAssetsVideoSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceSwapCreateBodyAssetsVideoSourceEnum::File => "file",
            V1FaceSwapCreateBodyAssetsVideoSourceEnum::Youtube => "youtube",
        };
        write!(f, "{}", str_val)
    }
}
