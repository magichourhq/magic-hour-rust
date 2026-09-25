/// Language to translate the video's speech into.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVideoTranslatorCreateBodyTargetLanguageEnum {
    #[default]
    #[serde(rename = "Afrikaans")]
    Afrikaans,
    #[serde(rename = "Arabic")]
    Arabic,
    #[serde(rename = "Bengali")]
    Bengali,
    #[serde(rename = "Bulgarian")]
    Bulgarian,
    #[serde(rename = "Catalan")]
    Catalan,
    #[serde(rename = "Chinese (Simplified)")]
    ChineseSimplified,
    #[serde(rename = "Chinese (Traditional)")]
    ChineseTraditional,
    #[serde(rename = "Croatian")]
    Croatian,
    #[serde(rename = "Czech")]
    Czech,
    #[serde(rename = "Danish")]
    Danish,
    #[serde(rename = "Dutch")]
    Dutch,
    #[serde(rename = "English")]
    English,
    #[serde(rename = "Estonian")]
    Estonian,
    #[serde(rename = "Finnish")]
    Finnish,
    #[serde(rename = "French")]
    French,
    #[serde(rename = "German")]
    German,
    #[serde(rename = "Greek")]
    Greek,
    #[serde(rename = "Gujarati")]
    Gujarati,
    #[serde(rename = "Hebrew")]
    Hebrew,
    #[serde(rename = "Hindi")]
    Hindi,
    #[serde(rename = "Hungarian")]
    Hungarian,
    #[serde(rename = "Indonesian")]
    Indonesian,
    #[serde(rename = "Italian")]
    Italian,
    #[serde(rename = "Japanese")]
    Japanese,
    #[serde(rename = "Kannada")]
    Kannada,
    #[serde(rename = "Kazakh")]
    Kazakh,
    #[serde(rename = "Korean")]
    Korean,
    #[serde(rename = "Latvian")]
    Latvian,
    #[serde(rename = "Lithuanian")]
    Lithuanian,
    #[serde(rename = "Malay")]
    Malay,
    #[serde(rename = "Malayalam")]
    Malayalam,
    #[serde(rename = "Marathi")]
    Marathi,
    #[serde(rename = "Norwegian")]
    Norwegian,
    #[serde(rename = "Persian")]
    Persian,
    #[serde(rename = "Polish")]
    Polish,
    #[serde(rename = "Portuguese")]
    Portuguese,
    #[serde(rename = "Punjabi")]
    Punjabi,
    #[serde(rename = "Romanian")]
    Romanian,
    #[serde(rename = "Russian")]
    Russian,
    #[serde(rename = "Serbian")]
    Serbian,
    #[serde(rename = "Slovak")]
    Slovak,
    #[serde(rename = "Slovenian")]
    Slovenian,
    #[serde(rename = "Spanish")]
    Spanish,
    #[serde(rename = "Swahili")]
    Swahili,
    #[serde(rename = "Swedish")]
    Swedish,
    #[serde(rename = "Tamil")]
    Tamil,
    #[serde(rename = "Telugu")]
    Telugu,
    #[serde(rename = "Thai")]
    Thai,
    #[serde(rename = "Turkish")]
    Turkish,
    #[serde(rename = "Ukrainian")]
    Ukrainian,
    #[serde(rename = "Urdu")]
    Urdu,
    #[serde(rename = "Vietnamese")]
    Vietnamese,
    #[serde(rename = "Welsh")]
    Welsh,
}
impl std::fmt::Display for V1AiVideoTranslatorCreateBodyTargetLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Afrikaans => "Afrikaans",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Arabic => "Arabic",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Bengali => "Bengali",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Bulgarian => "Bulgarian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Catalan => "Catalan",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::ChineseSimplified => {
                "Chinese (Simplified)"
            }
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::ChineseTraditional => {
                "Chinese (Traditional)"
            }
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Croatian => "Croatian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Czech => "Czech",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Danish => "Danish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Dutch => "Dutch",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::English => "English",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Estonian => "Estonian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Finnish => "Finnish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::French => "French",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::German => "German",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Greek => "Greek",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Gujarati => "Gujarati",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Hebrew => "Hebrew",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Hindi => "Hindi",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Hungarian => "Hungarian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Indonesian => "Indonesian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Italian => "Italian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Japanese => "Japanese",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Kannada => "Kannada",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Kazakh => "Kazakh",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Korean => "Korean",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Latvian => "Latvian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Lithuanian => "Lithuanian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Malay => "Malay",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Malayalam => "Malayalam",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Marathi => "Marathi",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Norwegian => "Norwegian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Persian => "Persian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Polish => "Polish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Portuguese => "Portuguese",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Punjabi => "Punjabi",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Romanian => "Romanian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Russian => "Russian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Serbian => "Serbian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Slovak => "Slovak",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Slovenian => "Slovenian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Spanish => "Spanish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Swahili => "Swahili",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Swedish => "Swedish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Tamil => "Tamil",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Telugu => "Telugu",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Thai => "Thai",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Turkish => "Turkish",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Ukrainian => "Ukrainian",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Urdu => "Urdu",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Vietnamese => "Vietnamese",
            V1AiVideoTranslatorCreateBodyTargetLanguageEnum::Welsh => "Welsh",
        };
        write!(f, "{}", str_val)
    }
}
