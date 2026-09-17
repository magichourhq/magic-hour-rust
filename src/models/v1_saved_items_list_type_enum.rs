/// Only return saved items of this type.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1SavedItemsListTypeEnum {
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
impl std::fmt::Display for V1SavedItemsListTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1SavedItemsListTypeEnum::BrandKit => "brand_kit",
            V1SavedItemsListTypeEnum::Character => "character",
            V1SavedItemsListTypeEnum::Moodboard => "moodboard",
            V1SavedItemsListTypeEnum::Reference => "reference",
            V1SavedItemsListTypeEnum::Voice => "voice",
        };
        write!(f, "{}", str_val)
    }
}
