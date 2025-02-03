/// PostV1AiClothesChangerBodyAssetsGarmentTypeEnum
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum PostV1AiClothesChangerBodyAssetsGarmentTypeEnum {
    #[default]
    #[serde(rename = "dresses")]
    Dresses,
    #[serde(rename = "lower_body")]
    LowerBody,
    #[serde(rename = "upper_body")]
    UpperBody,
}
impl std::fmt::Display for PostV1AiClothesChangerBodyAssetsGarmentTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostV1AiClothesChangerBodyAssetsGarmentTypeEnum::Dresses => "dresses",
            PostV1AiClothesChangerBodyAssetsGarmentTypeEnum::LowerBody => "lower_body",
            PostV1AiClothesChangerBodyAssetsGarmentTypeEnum::UpperBody => "upper_body",
        };
        write!(f, "{}", str_val)
    }
}
