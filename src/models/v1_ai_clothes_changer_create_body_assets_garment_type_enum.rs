/// Type of garment to swap. If not provided, swaps the entire outfit.
/// * `upper_body` - for shirts/jackets
/// * `lower_body` - for pants/skirts
/// * `dresses` - for entire outfit (deprecated, use `entire_outfit` instead)
/// * `entire_outfit` - for entire outfit
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum {
    #[default]
    #[serde(rename = "dresses")]
    Dresses,
    #[serde(rename = "entire_outfit")]
    EntireOutfit,
    #[serde(rename = "lower_body")]
    LowerBody,
    #[serde(rename = "upper_body")]
    UpperBody,
}
impl std::fmt::Display for V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::Dresses => "dresses",
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::EntireOutfit => {
                "entire_outfit"
            }
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::LowerBody => "lower_body",
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::UpperBody => "upper_body",
        };
        write!(f, "{}", str_val)
    }
}
