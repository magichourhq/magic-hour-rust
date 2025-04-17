/// To use our templates, pass in one of the enum values.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiMemeGeneratorCreateBodyStyleTemplateEnum {
    #[default]
    #[serde(rename = "Bike Fall")]
    BikeFall,
    #[serde(rename = "Change My Mind")]
    ChangeMyMind,
    #[serde(rename = "Disappointed Guy")]
    DisappointedGuy,
    #[serde(rename = "Drake Hotline Bling")]
    DrakeHotlineBling,
    #[serde(rename = "Galaxy Brain")]
    GalaxyBrain,
    #[serde(rename = "Gru's Plan")]
    GruSPlan,
    #[serde(rename = "Is This a Pigeon")]
    IsThisAPigeon,
    #[serde(rename = "Panik Kalm Panik")]
    PanikKalmPanik,
    #[serde(rename = "Random")]
    Random,
    #[serde(rename = "Side Eyeing Chloe")]
    SideEyeingChloe,
    #[serde(rename = "Tuxedo Winnie The Pooh")]
    TuxedoWinnieThePooh,
    #[serde(rename = "Two Buttons")]
    TwoButtons,
    #[serde(rename = "Waiting Skeleton")]
    WaitingSkeleton,
}
impl std::fmt::Display for V1AiMemeGeneratorCreateBodyStyleTemplateEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::BikeFall => "Bike Fall",
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::ChangeMyMind => {
                "Change My Mind"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DisappointedGuy => {
                "Disappointed Guy"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::DrakeHotlineBling => {
                "Drake Hotline Bling"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::GalaxyBrain => "Galaxy Brain",
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::GruSPlan => "Gru's Plan",
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::IsThisAPigeon => {
                "Is This a Pigeon"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::PanikKalmPanik => {
                "Panik Kalm Panik"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::Random => "Random",
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::SideEyeingChloe => {
                "Side Eyeing Chloe"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::TuxedoWinnieThePooh => {
                "Tuxedo Winnie The Pooh"
            }
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::TwoButtons => "Two Buttons",
            V1AiMemeGeneratorCreateBodyStyleTemplateEnum::WaitingSkeleton => {
                "Waiting Skeleton"
            }
        };
        write!(f, "{}", str_val)
    }
}
