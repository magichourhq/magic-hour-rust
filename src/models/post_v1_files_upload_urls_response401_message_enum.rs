/// Generated by Sideko (sideko.dev)
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1FilesUploadUrlsResponse401MessageEnum {
    #[default]
    #[serde(rename = "Unauthorized")]
    Unauthorized,
}
impl std::fmt::Display for PostV1FilesUploadUrlsResponse401MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1FilesUploadUrlsResponse401MessageEnum::Unauthorized => "Unauthorized",
        };
        write!(f, "{}", str_val)
    }
}
