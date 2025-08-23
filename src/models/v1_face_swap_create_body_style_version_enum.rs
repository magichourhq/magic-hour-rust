/// * `v1` - May preserve skin detail and texture better, but weaker identity preservation.
/// * `v2` - Faster, sharper, better handling of hair and glasses. stronger identity preservation. (Recommended)
/// * `default` - Use the version we recommend, which will change over time. This is recommended unless you need a specific earlier version. This is the default behavior.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceSwapCreateBodyStyleVersionEnum {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "v1")]
    V1,
    #[serde(rename = "v2")]
    V2,
}
impl std::fmt::Display for V1FaceSwapCreateBodyStyleVersionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceSwapCreateBodyStyleVersionEnum::Default => "default",
            V1FaceSwapCreateBodyStyleVersionEnum::V1 => "v1",
            V1FaceSwapCreateBodyStyleVersionEnum::V2 => "v2",
        };
        write!(f, "{}", str_val)
    }
}
