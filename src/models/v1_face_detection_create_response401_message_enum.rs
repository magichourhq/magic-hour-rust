/// V1FaceDetectionCreateResponse401MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceDetectionCreateResponse401MessageEnum {
    #[default]
    #[serde(rename = "Unauthorized")]
    Unauthorized,
}
impl std::fmt::Display for V1FaceDetectionCreateResponse401MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceDetectionCreateResponse401MessageEnum::Unauthorized => "Unauthorized",
        };
        write!(f, "{}", str_val)
    }
}
