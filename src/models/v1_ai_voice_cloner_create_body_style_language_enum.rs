/// Spoken language. Use a canonical lowercase language name, or auto to detect the language from the text. Defaults to auto.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub enum V1AiVoiceClonerCreateBodyStyleLanguageEnum {
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
impl std::fmt::Display for V1AiVoiceClonerCreateBodyStyleLanguageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Afrikaans => "afrikaans",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Albanian => "albanian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Amharic => "amharic",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Arabic => "arabic",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Armenian => "armenian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Assamese => "assamese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Auto => "auto",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Azerbaijani => "azerbaijani",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Basque => "basque",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Belarusian => "belarusian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Bengali => "bengali",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Bulgarian => "bulgarian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Burmese => "burmese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Cantonese => "cantonese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Catalan => "catalan",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Cebuano => "cebuano",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Chinese => "chinese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Croatian => "croatian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Czech => "czech",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Danish => "danish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Dutch => "dutch",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::English => "english",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Estonian => "estonian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Finnish => "finnish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::French => "french",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Galician => "galician",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Georgian => "georgian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::German => "german",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Greek => "greek",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Gujarati => "gujarati",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::HaitianCreole => "haitian creole",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Hebrew => "hebrew",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Hindi => "hindi",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Hungarian => "hungarian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Icelandic => "icelandic",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Indonesian => "indonesian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Italian => "italian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Japanese => "japanese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Javanese => "javanese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Kannada => "kannada",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Kazakh => "kazakh",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Konkani => "konkani",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Korean => "korean",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Lao => "lao",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Latvian => "latvian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Lithuanian => "lithuanian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Luxembourgish => "luxembourgish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Macedonian => "macedonian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Maithili => "maithili",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Malagasy => "malagasy",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Malay => "malay",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Malayalam => "malayalam",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Marathi => "marathi",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Mongolian => "mongolian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Nepali => "nepali",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Norwegian => "norwegian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Odia => "odia",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Pashto => "pashto",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Persian => "persian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Polish => "polish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Portuguese => "portuguese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Punjabi => "punjabi",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Romanian => "romanian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Russian => "russian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Serbian => "serbian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Sindhi => "sindhi",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Sinhala => "sinhala",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Slovak => "slovak",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Slovenian => "slovenian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Spanish => "spanish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Swahili => "swahili",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Swedish => "swedish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Tagalog => "tagalog",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Tamil => "tamil",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Telugu => "telugu",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Thai => "thai",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Turkish => "turkish",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Ukrainian => "ukrainian",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Urdu => "urdu",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Uzbek => "uzbek",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Vietnamese => "vietnamese",
            V1AiVoiceClonerCreateBodyStyleLanguageEnum::Welsh => "welsh",
        };
        write!(f, "{}", str_val)
    }
}
