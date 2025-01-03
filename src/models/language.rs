// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

/// Represents a language with various options.
///
/// The `Language` enum defines a list of supported languages, each with
/// an associated variant. The default language is Japanese.
///
/// # Variants
///
/// * `Japanese` - The Japanese language.
/// * `English` - The English language.
/// * `Korean` - The Korean language.
/// * `Italian` - The Italian language.
/// * `Spanish` - The Spanish language.
/// * `Portuguese` - The Portuguese language.
/// * `French` - The French language.
/// * `German` - The German language.
/// * `Hebrew` - The Hebrew language.
/// * `Hungarian` - The Hungarian language.
/// * `Chinese` - The Chinese language.
/// * `Arabic` - The Arabic language.
/// * `Filipino` - The Filipino language.
/// * `Catalan` - The Catalan language.
/// * `Finnish` - The Finnish language.
/// * `Turkish` - The Turkish language.
/// * `Dutch` - The Dutch language.
/// * `Swedish` - The Swedish language.
/// * `Thai` - The Thai language.
/// * `Tagalog` - The Tagalog language.
/// * `Malaysian` - The Malaysian language.
/// * `Indonesian` - The Indonesian language.
/// * `Vietnamese` - The Vietnamese language.
/// * `Nepali` - The Nepali language.
/// * `Hindi` - The Hindi language.
/// * `Urdu` - The Urdu language.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Language {
    /// The Japanese language.
    #[default]
    Japanese,
    /// The English language.
    English,
    /// The Korean language.
    Korean,
    /// The Italian language.
    Italian,
    /// The Spanish language.
    Spanish,
    /// The Portuguese language.
    Portuguese,
    /// The French language.
    French,
    /// The German language.
    German,
    /// The Hebrew language.
    Hebrew,
    /// The Hungarian language.
    Hungarian,
    /// The Chinese language.
    Chinese,
    /// The Arabic language.
    Arabic,
    /// The Filipino language.
    Filipino,
    /// The Catalan language.
    Catalan,
    /// The Finnish language.
    Finnish,
    /// The Turkish language.
    Turkish,
    /// The Dutch language.
    Dutch,
    /// The Swedish language.
    Swedish,
    /// The Thai language.
    Thai,
    /// The Tagalog language.
    Tagalog,
    /// The Malaysian language.
    Malaysian,
    /// The Indonesian language.
    Indonesian,
    /// The Vietnamese language.
    Vietnamese,
    /// The Nepali language.
    Nepali,
    /// The Hindi language.
    Hindi,
    /// The Urdu language.
    Urdu,
}

impl Language {
    /// Returns the ISO 639-1 code of the language.
    pub fn code(&self) -> &str {
        match self {
            Language::Japanese => "ja",
            Language::English => "en",
            Language::Korean => "ko",
            Language::Italian => "it",
            Language::Spanish => "es",
            Language::Portuguese => "pt",
            Language::French => "fr",
            Language::German => "de",
            Language::Hebrew => "he",
            Language::Hungarian => "hu",
            Language::Chinese => "zh",
            Language::Arabic => "ar",
            Language::Filipino => "fil",
            Language::Catalan => "ca",
            Language::Finnish => "fi",
            Language::Turkish => "tr",
            Language::Dutch => "nl",
            Language::Swedish => "sv",
            Language::Thai => "th",
            Language::Tagalog => "tl",
            Language::Malaysian => "ms",
            Language::Indonesian => "id",
            Language::Vietnamese => "vi",
            Language::Nepali => "ne",
            Language::Hindi => "hi",
            Language::Urdu => "ur",
        }
    }

    /// Returns the ISO 639-1 code of the language.
    ///
    /// Alias of `code`.
    pub fn iso(&self) -> &str {
        self.code()
    }

    /// Returns the name of the language in the native language.
    pub fn native(&self) -> &str {
        match self {
            Language::Japanese => "日本語",
            Language::English => "English",
            Language::Korean => "한국어",
            Language::Italian => "Italiano",
            Language::Spanish => "Español",
            Language::Portuguese => "Português",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Hebrew => "עברית",
            Language::Hungarian => "Magyar",
            Language::Chinese => "中文",
            Language::Arabic => "العربية",
            Language::Filipino => "Filipino",
            Language::Catalan => "Català",
            Language::Finnish => "Suomi",
            Language::Turkish => "Türkçe",
            Language::Dutch => "Nederlands",
            Language::Swedish => "Svenska",
            Language::Thai => "ไทย",
            Language::Tagalog => "Tagalog",
            Language::Malaysian => "Bahasa Melayu",
            Language::Indonesian => "Bahasa Indonesia",
            Language::Vietnamese => "Tiếng Việt",
            Language::Nepali => "नेपाली",
            Language::Hindi => "हिंदी",
            Language::Urdu => "اردو",
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value.trim().to_uppercase().as_str() {
            "JA" | "JP" | "JAPANESE" => Language::Japanese,
            "EN" | "UK" | "ENGLISH" => Language::English,
            "KO" | "KOREAN" => Language::Korean,
            "IT" | "ITALIAN" => Language::Italian,
            "ES" | "SPANISH" => Language::Spanish,
            "PT" | "PORTUGUESE" => Language::Portuguese,
            "FR" | "FRENCH" => Language::French,
            "DE" | "GERMAN" => Language::German,
            "HE" | "HEBREW" => Language::Hebrew,
            "HU" | "HUNGARIAN" => Language::Hungarian,
            "ZH" | "CHINESE" => Language::Chinese,
            "AR" | "ARABIC" => Language::Arabic,
            "FIL" | "PHILIPPINE" => Language::Filipino,
            "CA" | "CATALAN" => Language::Catalan,
            "FI" | "FINNISH" => Language::Finnish,
            "TR" | "TURKISH" => Language::Turkish,
            "NL" | "DUTCH" => Language::Dutch,
            "SV" | "SWEDISH" => Language::Swedish,
            "TH" | "THAI" => Language::Thai,
            "TL" | "TAGALOG" => Language::Tagalog,
            "MS" | "MALAYSIAN" => Language::Malaysian,
            "ID" | "INDONESIAN" => Language::Indonesian,
            "VI" | "VIETNAMESE" => Language::Vietnamese,
            "NE" | "NEPALI" => Language::Nepali,
            "HI" | "HINDI" => Language::Hindi,
            "UR" | "URDU" => Language::Urdu,
            _ => Language::default(),
        }
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        Language::from(value.as_str())
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Japanese => write!(f, "Japanese"),
            Language::English => write!(f, "English"),
            Language::Korean => write!(f, "Korean"),
            Language::Italian => write!(f, "Italian"),
            Language::Spanish => write!(f, "Spanish"),
            Language::Portuguese => write!(f, "Portuguese"),
            Language::French => write!(f, "French"),
            Language::German => write!(f, "German"),
            Language::Hebrew => write!(f, "Hebrew"),
            Language::Hungarian => write!(f, "Hungarian"),
            Language::Chinese => write!(f, "Chinese"),
            Language::Arabic => write!(f, "Arabic"),
            Language::Filipino => write!(f, "Filipino"),
            Language::Catalan => write!(f, "Catalan"),
            Language::Finnish => write!(f, "Finnish"),
            Language::Turkish => write!(f, "Turkish"),
            Language::Dutch => write!(f, "Dutch"),
            Language::Swedish => write!(f, "Swedish"),
            Language::Thai => write!(f, "Thai"),
            Language::Tagalog => write!(f, "Tagalog"),
            Language::Malaysian => write!(f, "Malaysian"),
            Language::Indonesian => write!(f, "Indonesian"),
            Language::Vietnamese => write!(f, "Vietnamese"),
            Language::Nepali => write!(f, "Nepali"),
            Language::Hindi => write!(f, "Hindi"),
            Language::Urdu => write!(f, "Urdu"),
        }
    }
}
