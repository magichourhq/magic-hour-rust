/// Choose how to swap faces:
/// **all-faces** (recommended) — swap all detected faces using one source image (`source_file_path` required)
/// +- **individual-faces** — specify exact mappings using `face_mappings`
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum {
    #[default]
    #[serde(rename = "all-faces")]
    AllFaces,
    #[serde(rename = "individual-faces")]
    IndividualFaces,
}
impl std::fmt::Display for V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum::AllFaces => "all-faces",
            V1FaceSwapPhotoCreateBodyAssetsFaceSwapModeEnum::IndividualFaces => {
                "individual-faces"
            }
        };
        write!(f, "{}", str_val)
    }
}
