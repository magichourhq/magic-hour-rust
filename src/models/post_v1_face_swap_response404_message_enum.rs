/// PostV1FaceSwapResponse404MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1FaceSwapResponse404MessageEnum {
    #[default]
    #[serde(rename = "Not Found")]
    NotFound,
}
impl std::fmt::Display for PostV1FaceSwapResponse404MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1FaceSwapResponse404MessageEnum::NotFound => "Not Found",
        };
        write!(f, "{}", str_val)
    }
}
