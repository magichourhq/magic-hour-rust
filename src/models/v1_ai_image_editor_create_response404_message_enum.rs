/// V1AiImageEditorCreateResponse404MessageEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiImageEditorCreateResponse404MessageEnum {
    #[default]
    #[serde(rename = "Not Found")]
    NotFound,
}
impl std::fmt::Display for V1AiImageEditorCreateResponse404MessageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiImageEditorCreateResponse404MessageEnum::NotFound => "Not Found",
        };
        write!(f, "{}", str_val)
    }
}
