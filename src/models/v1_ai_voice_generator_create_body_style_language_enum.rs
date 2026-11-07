/// Spoken language. Use a canonical lowercase language name, or auto to detect the language from the text. Defaults to auto.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVoiceGeneratorCreateBodyStyleLanguageEnum {
    #[default]
    #[serde(rename = "afrikaans")]
    Afrikaans,
    #[serde(rename = "albanian")]
    Albanian,
    #[serde(rename = "amharic")]
    Amharic,
    #[serde(rename = "arabic")]
    Arabic,
    #[serde(rename = "armenian")]
    Armenian,
    #[serde(rename = "assamese")]
    Assamese,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "azerbaijani")]
    Azerbaijani,
    #[serde(rename = "basque")]
    Basque,
    #[serde(rename = "belarusian")]
    Belarusian,
    #[serde(rename = "bengali")]
    Bengali,
    #[serde(rename = "bulgarian")]
    Bulgarian,
    #[serde(rename = "burmese")]
    Burmese,
    #[serde(rename = "cantonese")]
    Cantonese,
    #[serde(rename = "catalan")]
    Catalan,
    #[serde(rename = "cebuano")]
    Cebuano,
    #[serde(rename = "chinese")]
    Chinese,
    #[serde(rename = "croatian")]
    Croatian,
    #[serde(rename = "czech")]
    Czech,
    #[serde(rename = "danish")]
    Danish,
    #[serde(rename = "dutch")]
    Dutch,
    #[serde(rename = "english")]
    English,
    #[serde(rename = "estonian")]
    Estonian,
    #[serde(rename = "finnish")]
    Finnish,
    #[serde(rename = "french")]
    French,
    #[serde(rename = "galician")]
    Galician,
    #[serde(rename = "georgian")]
    Georgian,
    #[serde(rename = "german")]
    German,
    #[serde(rename = "greek")]
    Greek,
    #[serde(rename = "gujarati")]
    Gujarati,
    #[serde(rename = "haitian creole")]
    HaitianCreole,
    #[serde(rename = "hebrew")]
    Hebrew,
    #[serde(rename = "hindi")]
    Hindi,
    #[serde(rename = "hungarian")]
    Hungarian,
    #[serde(rename = "icelandic")]
    Icelandic,
    #[serde(rename = "indonesian")]
    Indonesian,
    #[serde(rename = "italian")]
    Italian,
    #[serde(rename = "japanese")]
    Japanese,
    #[serde(rename = "javanese")]
    Javanese,
    #[serde(rename = "kannada")]
    Kannada,
    #[serde(rename = "kazakh")]
    Kazakh,
    #[serde(rename = "konkani")]
    Konkani,
    #[serde(rename = "korean")]
    Korean,
    #[serde(rename = "lao")]
    Lao,
    #[serde(rename = "latvian")]
    Latvian,
    #[serde(rename = "lithuanian")]
    Lithuanian,
    #[serde(rename = "luxembourgish")]
    Luxembourgish,
    #[serde(rename = "macedonian")]
    Macedonian,
    #[serde(rename = "maithili")]
    Maithili,
    #[serde(rename = "malagasy")]
    Malagasy,
    #[serde(rename = "malay")]
    Malay,
    #[serde(rename = "malayalam")]
    Malayalam,
    #[serde(rename = "marathi")]
    Marathi,
    #[serde(rename = "mongolian")]
    Mongolian,
    #[serde(rename = "nepali")]
    Nepali,
    #[serde(rename = "norwegian")]
    Norwegian,
    #[serde(rename = "odia")]
    Odia,
    #[serde(rename = "pashto")]
    Pashto,
    #[serde(rename = "persian")]
    Persian,
    #[serde(rename = "polish")]
    Polish,
    #[serde(rename = "portuguese")]
    Portuguese,
    #[serde(rename = "punjabi")]
    Punjabi,
    #[serde(rename = "romanian")]
    Romanian,
    #[serde(rename = "russian")]
    Russian,
    #[serde(rename = "serbian")]
    Serbian,
    #[serde(rename = "sindhi")]
    Sindhi,
    #[serde(rename = "sinhala")]
    Sinhala,
    #[serde(rename = "slovak")]
    Slovak,
    #[serde(rename = "slovenian")]
    Slovenian,
    #[serde(rename = "spanish")]
    Spanish,
    #[serde(rename = "swahili")]
    Swahili,
    #[serde(rename = "swedish")]
    Swedish,
    #[serde(rename = "tagalog")]
    Tagalog,
    #[serde(rename = "tamil")]
    Tamil,
    #[serde(rename = "telugu")]
    Telugu,
    #[serde(rename = "thai")]
    Thai,
    #[serde(rename = "turkish")]
    Turkish,
    #[serde(rename = "ukrainian")]
    Ukrainian,
    #[serde(rename = "urdu")]
    Urdu,
    #[serde(rename = "uzbek")]
    Uzbek,
    #[serde(rename = "vietnamese")]
    Vietnamese,
    #[serde(rename = "welsh")]
    Welsh,
}
impl std::fmt::Display for V1AiVoiceGeneratorCreateBodyStyleLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Afrikaans => "afrikaans",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Albanian => "albanian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Amharic => "amharic",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Arabic => "arabic",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Armenian => "armenian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Assamese => "assamese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Auto => "auto",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Azerbaijani => "azerbaijani",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Basque => "basque",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Belarusian => "belarusian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Bengali => "bengali",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Bulgarian => "bulgarian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Burmese => "burmese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Cantonese => "cantonese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Catalan => "catalan",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Cebuano => "cebuano",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Chinese => "chinese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Croatian => "croatian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Czech => "czech",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Danish => "danish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Dutch => "dutch",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::English => "english",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Estonian => "estonian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Finnish => "finnish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::French => "french",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Galician => "galician",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Georgian => "georgian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::German => "german",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Greek => "greek",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Gujarati => "gujarati",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::HaitianCreole => {
                "haitian creole"
            }
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Hebrew => "hebrew",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Hindi => "hindi",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Hungarian => "hungarian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Icelandic => "icelandic",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Indonesian => "indonesian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Italian => "italian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Japanese => "japanese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Javanese => "javanese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Kannada => "kannada",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Kazakh => "kazakh",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Konkani => "konkani",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Korean => "korean",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Lao => "lao",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Latvian => "latvian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Lithuanian => "lithuanian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Luxembourgish => {
                "luxembourgish"
            }
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Macedonian => "macedonian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Maithili => "maithili",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Malagasy => "malagasy",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Malay => "malay",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Malayalam => "malayalam",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Marathi => "marathi",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Mongolian => "mongolian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Nepali => "nepali",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Norwegian => "norwegian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Odia => "odia",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Pashto => "pashto",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Persian => "persian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Polish => "polish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Portuguese => "portuguese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Punjabi => "punjabi",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Romanian => "romanian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Russian => "russian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Serbian => "serbian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Sindhi => "sindhi",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Sinhala => "sinhala",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Slovak => "slovak",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Slovenian => "slovenian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Spanish => "spanish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Swahili => "swahili",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Swedish => "swedish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Tagalog => "tagalog",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Tamil => "tamil",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Telugu => "telugu",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Thai => "thai",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Turkish => "turkish",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Ukrainian => "ukrainian",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Urdu => "urdu",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Uzbek => "uzbek",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Vietnamese => "vietnamese",
            V1AiVoiceGeneratorCreateBodyStyleLanguageEnum::Welsh => "welsh",
        };
        write!(f, "{}", str_val)
    }
}
