/// PostV1ImageToVideoResponse404MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1ImageToVideoResponse404MessageEnum {
    #[default]
    #[serde(rename = "Not Found")]
    NotFound,
}
impl std::fmt::Display for PostV1ImageToVideoResponse404MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1ImageToVideoResponse404MessageEnum::NotFound => "Not Found",
        };
        write!(f, "{}", str_val)
    }
}
