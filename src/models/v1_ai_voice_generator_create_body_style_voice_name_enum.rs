/// The voice to use for the speech. Available voices: Elon Musk, Mark Zuckerberg, Joe Rogan, Barack Obama, Morgan Freeman, Kanye West, Donald Trump, Joe Biden, Kim Kardashian, Taylor Swift
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    #[default]
    #[serde(rename = "Barack Obama")]
    BarackObama,
    #[serde(rename = "Donald Trump")]
    DonaldTrump,
    #[serde(rename = "Elon Musk")]
    ElonMusk,
    #[serde(rename = "Joe Biden")]
    JoeBiden,
    #[serde(rename = "Joe Rogan")]
    JoeRogan,
    #[serde(rename = "Kanye West")]
    KanyeWest,
    #[serde(rename = "Kim Kardashian")]
    KimKardashian,
    #[serde(rename = "Mark Zuckerberg")]
    MarkZuckerberg,
    #[serde(rename = "Morgan Freeman")]
    MorganFreeman,
    #[serde(rename = "Taylor Swift")]
    TaylorSwift,
}
impl std::fmt::Display for V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::BarackObama => "Barack Obama",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::DonaldTrump => "Donald Trump",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::ElonMusk => "Elon Musk",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeBiden => "Joe Biden",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::JoeRogan => "Joe Rogan",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KanyeWest => "Kanye West",
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::KimKardashian => {
                "Kim Kardashian"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MarkZuckerberg => {
                "Mark Zuckerberg"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::MorganFreeman => {
                "Morgan Freeman"
            }
            V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum::TaylorSwift => "Taylor Swift",
        };
        write!(f, "{}", str_val)
    }
}
