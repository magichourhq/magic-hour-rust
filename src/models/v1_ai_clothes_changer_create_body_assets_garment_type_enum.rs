/// Deprecated: garment_type is no longer needed.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum {
    #[default]
    #[serde(rename = "dresses")]
    Dresses,
    #[serde(rename = "lower_body")]
    LowerBody,
    #[serde(rename = "upper_body")]
    UpperBody,
}
impl std::fmt::Display for V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::Dresses => "dresses",
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::LowerBody => "lower_body",
            V1AiClothesChangerCreateBodyAssetsGarmentTypeEnum::UpperBody => "upper_body",
        };
        write!(f, "{}", str_val)
    }
}
