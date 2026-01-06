/// Choose how to swap faces:
/// **all-faces** (recommended) — swap all detected faces using one source image (`source_file_path` required)
/// +- **individual-faces** — specify exact mappings using `face_mappings`
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceSwapCreateBodyAssetsFaceSwapModeEnum {
    #[default]
    #[serde(rename = "all-faces")]
    AllFaces,
    #[serde(rename = "individual-faces")]
    IndividualFaces,
}
impl std::fmt::Display for V1FaceSwapCreateBodyAssetsFaceSwapModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::AllFaces => "all-faces",
            V1FaceSwapCreateBodyAssetsFaceSwapModeEnum::IndividualFaces => {
                "individual-faces"
            }
        };
        write!(f, "{}", str_val)
    }
}
