use deserr::Deserr;
use milli::LocalizedAttributesRule;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Generate a Locale enum and its From and Into implementations for milli::tokenizer::Language.
///
/// this enum implements `Deserr` in order to be used in the API.
macro_rules! make_locale {

    ($($language:tt), +) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Deserr, Serialize, Deserialize, Ord, PartialOrd)]
        #[deserr(rename_all = camelCase)]
        #[serde(rename_all = "camelCase")]
        pub enum Locale {
            $($language),+,
        }

        impl From<milli::tokenizer::Language> for Locale {
            fn from(other: milli::tokenizer::Language) -> Locale {
                match other {
                    $(milli::tokenizer::Language::$language => Locale::$language), +
                }
            }
        }

        impl From<Locale> for milli::tokenizer::Language {
            fn from(other: Locale) -> milli::tokenizer::Language {
                match other {
                    $(Locale::$language => milli::tokenizer::Language::$language), +,
                }
            }
        }

        #[derive(Debug)]
        pub struct LocaleFormatError {
            pub invalid_locale: String,
        }

        impl std::fmt::Display for LocaleFormatError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let valid_locales = [$(Locale::$language),+].iter().map(|l| format!("`{}`", json!(l).as_str().unwrap())).collect::<Vec<_>>().join(", ");
                write!(f, "Unsupported locale `{}`, expected one of {}", self.invalid_locale, valid_locales)
            }
        }
    };
}

make_locale! {
    Epo,
    Eng,
    Rus,
    Cmn,
    Spa,
    Por,
    Ita,
    Ben,
    Fra,
    Deu,
    Ukr,
    Kat,
    Ara,
    Hin,
    Jpn,
    Heb,
    Yid,
    Pol,
    Amh,
    Jav,
    Kor,
    Nob,
    Dan,
    Swe,
    Fin,
    Tur,
    Nld,
    Hun,
    Ces,
    Ell,
    Bul,
    Bel,
    Mar,
    Kan,
    Ron,
    Slv,
    Hrv,
    Srp,
    Mkd,
    Lit,
    Lav,
    Est,
    Tam,
    Vie,
    Urd,
    Tha,
    Guj,
    Uzb,
    Pan,
    Aze,
    Ind,
    Tel,
    Pes,
    Mal,
    Ori,
    Mya,
    Nep,
    Sin,
    Khm,
    Tuk,
    Aka,
    Zul,
    Sna,
    Afr,
    Lat,
    Slk,
    Cat,
    Tgl,
    Hye
}

impl std::error::Error for LocaleFormatError {}

impl std::str::FromStr for Locale {
    type Err = LocaleFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        milli::tokenizer::Language::from_code(s)
            .map(Self::from)
            .ok_or(LocaleFormatError { invalid_locale: s.to_string() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserr, Serialize, Deserialize)]
#[deserr(rename_all = camelCase)]
#[serde(rename_all = "camelCase")]
pub struct LocalizedAttributesRuleView {
    pub attribute_patterns: Vec<String>,
    pub locales: Vec<Locale>,
}

impl From<LocalizedAttributesRule> for LocalizedAttributesRuleView {
    fn from(rule: LocalizedAttributesRule) -> Self {
        Self {
            attribute_patterns: rule.attribute_patterns,
            locales: rule.locales.into_iter().map(|l| l.into()).collect(),
        }
    }
}

impl From<LocalizedAttributesRuleView> for LocalizedAttributesRule {
    fn from(view: LocalizedAttributesRuleView) -> Self {
        Self {
            attribute_patterns: view.attribute_patterns,
            locales: view.locales.into_iter().map(|l| l.into()).collect(),
        }
    }
}
