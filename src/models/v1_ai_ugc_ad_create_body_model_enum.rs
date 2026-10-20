/// Defaults to kling-3.0 on paid plans and ltx-2.5 on free plans.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiUgcAdCreateBodyModelEnum {
    #[default]
    #[serde(rename = "kling-3.0")]
    Kling30,
    #[serde(rename = "ltx-2.5")]
    Ltx25,
    #[serde(rename = "veo3.1")]
    Veo31,
}
impl std::fmt::Display for V1AiUgcAdCreateBodyModelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiUgcAdCreateBodyModelEnum::Kling30 => "kling-3.0",
            V1AiUgcAdCreateBodyModelEnum::Ltx25 => "ltx-2.5",
            V1AiUgcAdCreateBodyModelEnum::Veo31 => "veo3.1",
        };
        write!(f, "{}", str_val)
    }
}
