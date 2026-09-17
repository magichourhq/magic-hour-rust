/// Saved item type.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1SavedItemsListResponseItemsItemTypeEnum {
    #[default]
    #[serde(rename = "brand_kit")]
    BrandKit,
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "moodboard")]
    Moodboard,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "voice")]
    Voice,
}
impl std::fmt::Display for V1SavedItemsListResponseItemsItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1SavedItemsListResponseItemsItemTypeEnum::BrandKit => "brand_kit",
            V1SavedItemsListResponseItemsItemTypeEnum::Character => "character",
            V1SavedItemsListResponseItemsItemTypeEnum::Moodboard => "moodboard",
            V1SavedItemsListResponseItemsItemTypeEnum::Reference => "reference",
            V1SavedItemsListResponseItemsItemTypeEnum::Voice => "voice",
        };
        write!(f, "{}", str_val)
    }
}
