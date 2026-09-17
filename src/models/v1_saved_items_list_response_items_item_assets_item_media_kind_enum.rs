/// Media type of the asset.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum {
    #[default]
    #[serde(rename = "AUDIO")]
    Audio,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
}
impl std::fmt::Display for V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum::Audio => "AUDIO",
            V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum::Image => "IMAGE",
            V1SavedItemsListResponseItemsItemAssetsItemMediaKindEnum::Video => "VIDEO",
        };
        write!(f, "{}", str_val)
    }
}
