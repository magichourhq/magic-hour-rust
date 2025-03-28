/// PostV1LipSyncResponse401MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1LipSyncResponse401MessageEnum {
    #[default]
    #[serde(rename = "Unauthorized")]
    Unauthorized,
}
impl std::fmt::Display for PostV1LipSyncResponse401MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1LipSyncResponse401MessageEnum::Unauthorized => "Unauthorized",
        };
        write!(f, "{}", str_val)
    }
}
