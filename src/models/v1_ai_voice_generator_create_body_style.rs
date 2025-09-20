/// The content used to generate speech.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct V1AiVoiceGeneratorCreateBodyStyle {
    /// Text used to generate speech. Starter tier users can use up to 200 characters, while Creator, Pro, or Business users can use up to 1000.
    pub prompt: String,
    /// The voice to use for the speech. Available voices: Elon Musk, Mark Zuckerberg, Joe Rogan, Barack Obama, Morgan Freeman, Kanye West, Donald Trump, Joe Biden, Kim Kardashian, Taylor Swift
    pub voice_name: crate::models::V1AiVoiceGeneratorCreateBodyStyleVoiceNameEnum,
}
