#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::collections::HashMap;
use std::sync::OnceLock;
use crate::poi_util::LanguageType::LanguageType;
#[allow(unused_imports)]
use crate::poi_util::LanguageType::{
    LANGUAGE_AFRIKAANS, LANGUAGE_ALBANIAN, LANGUAGE_ALSATIAN_FRANCE, LANGUAGE_AMHARIC_ETHIOPIA,
    LANGUAGE_ARABIC_ALGERIA, LANGUAGE_ARABIC_BAHRAIN, LANGUAGE_ARABIC_EGYPT, LANGUAGE_ARABIC_IRAQ,
    LANGUAGE_ARABIC_JORDAN, LANGUAGE_ARABIC_KUWAIT, LANGUAGE_ARABIC_LEBANON, LANGUAGE_ARABIC_LIBYA,
    LANGUAGE_ARABIC_MOROCCO, LANGUAGE_ARABIC_OMAN, LANGUAGE_ARABIC_QATAR, LANGUAGE_ARABIC_SAUDI_ARABIA,
    LANGUAGE_ARABIC_SYRIA, LANGUAGE_ARABIC_TUNISIA, LANGUAGE_ARABIC_UAE, LANGUAGE_ARABIC_YEMEN,
    LANGUAGE_ARMENIAN, LANGUAGE_ASSAMESE, LANGUAGE_AZERI_CYRILLIC, LANGUAGE_AZERI_LATIN,
    LANGUAGE_BASHKIR_RUSSIA, LANGUAGE_BASQUE, LANGUAGE_BELARUSIAN, LANGUAGE_BENGALI,
    LANGUAGE_BENGALI_BANGLADESH, LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA,
    LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA, LANGUAGE_BRETON_FRANCE, LANGUAGE_BULGARIAN,
    LANGUAGE_BURMESE, LANGUAGE_CATALAN, LANGUAGE_CHEROKEE_UNITED_STATES, LANGUAGE_CHINESE_HONGKONG,
    LANGUAGE_CHINESE_MACAU, LANGUAGE_CHINESE_SIMPLIFIED, LANGUAGE_CHINESE_SINGAPORE,
    LANGUAGE_CHINESE_TRADITIONAL, LANGUAGE_CORSICAN_FRANCE, LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA,
    LANGUAGE_CZECH, LANGUAGE_DANISH, LANGUAGE_DARI_AFGHANISTAN, LANGUAGE_DHIVEHI, LANGUAGE_DUTCH,
    LANGUAGE_DUTCH_BELGIAN, LANGUAGE_EDO, LANGUAGE_ENGLISH_AUS, LANGUAGE_ENGLISH_BELIZE,
    LANGUAGE_ENGLISH_CAN, LANGUAGE_ENGLISH_CARRIBEAN, LANGUAGE_ENGLISH_EIRE,
    LANGUAGE_ENGLISH_HONG_KONG_SAR, LANGUAGE_ENGLISH_INDIA, LANGUAGE_ENGLISH_INDONESIA,
    LANGUAGE_ENGLISH_JAMAICA, LANGUAGE_ENGLISH_MALAYSIA, LANGUAGE_ENGLISH_NZ,
    LANGUAGE_ENGLISH_PHILIPPINES, LANGUAGE_ENGLISH_SAFRICA, LANGUAGE_ENGLISH_SINGAPORE,
    LANGUAGE_ENGLISH_TRINIDAD, LANGUAGE_ENGLISH_UK, LANGUAGE_ENGLISH_US, LANGUAGE_ENGLISH_ZIMBABWE,
    LANGUAGE_ESTONIAN, LANGUAGE_FAEROESE, LANGUAGE_FARSI, LANGUAGE_FILIPINO, LANGUAGE_FINNISH,
    LANGUAGE_FRENCH, LANGUAGE_FRENCH_BELGIAN, LANGUAGE_FRENCH_CAMEROON, LANGUAGE_FRENCH_CANADIAN,
    LANGUAGE_FRENCH_COTE_D_IVOIRE, LANGUAGE_FRENCH_HAITI, LANGUAGE_FRENCH_LUXEMBOURG,
    LANGUAGE_FRENCH_MALI, LANGUAGE_FRENCH_MONACO, LANGUAGE_FRENCH_MOROCCO, LANGUAGE_FRENCH_NORTH_AFRICA,
    LANGUAGE_FRENCH_REUNION, LANGUAGE_FRENCH_SENEGAL, LANGUAGE_FRENCH_SWISS,
    LANGUAGE_FRENCH_WEST_INDIES, LANGUAGE_FRENCH_ZAIRE, LANGUAGE_FRISIAN_NETHERLANDS,
    LANGUAGE_FULFULDE_NIGERIA, LANGUAGE_GAELIC_IRELAND, LANGUAGE_GAELIC_SCOTLAND, LANGUAGE_GALICIAN,
    LANGUAGE_GEORGIAN, LANGUAGE_GERMAN, LANGUAGE_GERMAN_AUSTRIAN, LANGUAGE_GERMAN_LIECHTENSTEIN,
    LANGUAGE_GERMAN_LUXEMBOURG, LANGUAGE_GERMAN_SWISS, LANGUAGE_GREEK, LANGUAGE_GUARANI_PARAGUAY,
    LANGUAGE_GUJARATI, LANGUAGE_HAUSA_NIGERIA, LANGUAGE_HAWAIIAN_UNITED_STATES, LANGUAGE_HEBREW,
    LANGUAGE_HINDI, LANGUAGE_HUNGARIAN, LANGUAGE_IBIBIO_NIGERIA, LANGUAGE_ICELANDIC,
    LANGUAGE_IGBO_NIGERIA, LANGUAGE_INDONESIAN, LANGUAGE_INUKTITUT_LATIN_CANADA,
    LANGUAGE_INUKTITUT_SYLLABICS_CANADA, LANGUAGE_ITALIAN, LANGUAGE_ITALIAN_SWISS, LANGUAGE_JAPANESE,
    LANGUAGE_KALAALLISUT_GREENLAND, LANGUAGE_KANNADA, LANGUAGE_KANURI_NIGERIA, LANGUAGE_KASHMIRI,
    LANGUAGE_KASHMIRI_INDIA, LANGUAGE_KAZAK, LANGUAGE_KHMER, LANGUAGE_KICHE_GUATEMALA,
    LANGUAGE_KINYARWANDA_RWANDA, LANGUAGE_KIRGHIZ, LANGUAGE_KONKANI, LANGUAGE_KOREAN,
    LANGUAGE_KOREAN_JOHAB, LANGUAGE_LAO, LANGUAGE_LATIN, LANGUAGE_LATVIAN, LANGUAGE_LITHUANIAN,
    LANGUAGE_LITHUANIAN_CLASSIC, LANGUAGE_LUXEMBOURGISH_LUXEMBOURG, LANGUAGE_MACEDONIAN,
    LANGUAGE_MALAYALAM, LANGUAGE_MALAY_BRUNEI_DARUSSALAM, LANGUAGE_MALAY_MALAYSIA, LANGUAGE_MALTESE,
    LANGUAGE_MANIPURI, LANGUAGE_MAORI_NEW_ZEALAND, LANGUAGE_MAPUDUNGUN_CHILE, LANGUAGE_MARATHI,
    LANGUAGE_MOHAWK_CANADA, LANGUAGE_MONGOLIAN_MONGOLIAN, LANGUAGE_NEPALI, LANGUAGE_NEPALI_INDIA,
    LANGUAGE_NORWEGIAN_BOKMAL, LANGUAGE_NORWEGIAN_NYNORSK, LANGUAGE_OCCITAN_FRANCE, LANGUAGE_ORIYA,
    LANGUAGE_OROMO, LANGUAGE_PAPIAMENTU, LANGUAGE_PASHTO, LANGUAGE_POLISH, LANGUAGE_PORTUGUESE,
    LANGUAGE_PORTUGUESE_BRAZILIAN, LANGUAGE_PUNJABI, LANGUAGE_PUNJABI_PAKISTAN,
    LANGUAGE_QUECHUA_BOLIVIA, LANGUAGE_QUECHUA_ECUADOR, LANGUAGE_QUECHUA_PERU, LANGUAGE_RHAETO_ROMAN,
    LANGUAGE_ROMANIAN, LANGUAGE_ROMANIAN_MOLDOVA, LANGUAGE_RUSSIAN, LANGUAGE_RUSSIAN_MOLDOVA,
    LANGUAGE_SAMI_NORTHERN_NORWAY, LANGUAGE_SAMI_LAPPISH, LANGUAGE_SAMI_INARI,
    LANGUAGE_SAMI_LULE_NORWAY, LANGUAGE_SAMI_LULE_SWEDEN, LANGUAGE_SAMI_NORTHERN_FINLAND,
    LANGUAGE_SAMI_NORTHERN_SWEDEN, LANGUAGE_SAMI_SKOLT, LANGUAGE_SAMI_SOUTHERN_NORWAY,
    LANGUAGE_SAMI_SOUTHERN_SWEDEN, LANGUAGE_SANSKRIT, LANGUAGE_SEPEDI, LANGUAGE_NORTHERNSOTHO,
    LANGUAGE_SERBIAN_CYRILLIC, LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA, LANGUAGE_SERBIAN_LATIN,
    LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA, LANGUAGE_SERBIAN_LATIN_NEUTRAL, LANGUAGE_SESOTHO,
    LANGUAGE_SINDHI, LANGUAGE_SINDHI_PAKISTAN, LANGUAGE_SINHALESE_SRI_LANKA, LANGUAGE_SLOVAK,
    LANGUAGE_SLOVENIAN, LANGUAGE_SOMALI, LANGUAGE_UPPER_SORBIAN_GERMANY, LANGUAGE_LOWER_SORBIAN_GERMANY,
    LANGUAGE_SPANISH_DATED, LANGUAGE_SPANISH_ARGENTINA, LANGUAGE_SPANISH_BOLIVIA,
    LANGUAGE_SPANISH_CHILE, LANGUAGE_SPANISH_COLOMBIA, LANGUAGE_SPANISH_COSTARICA,
    LANGUAGE_SPANISH_DOMINICAN_REPUBLIC, LANGUAGE_SPANISH_ECUADOR, LANGUAGE_SPANISH_EL_SALVADOR,
    LANGUAGE_SPANISH_GUATEMALA, LANGUAGE_SPANISH_HONDURAS, LANGUAGE_SPANISH_LATIN_AMERICA,
    LANGUAGE_SPANISH_MEXICAN, LANGUAGE_SPANISH_MODERN, LANGUAGE_SPANISH_NICARAGUA,
    LANGUAGE_SPANISH_PANAMA, LANGUAGE_SPANISH_PARAGUAY, LANGUAGE_SPANISH_PERU,
    LANGUAGE_SPANISH_PUERTO_RICO, LANGUAGE_SPANISH_UNITED_STATES, LANGUAGE_SPANISH_URUGUAY,
    LANGUAGE_SPANISH_VENEZUELA, LANGUAGE_SPANISH, LANGUAGE_SWAHILI, LANGUAGE_SWEDISH,
    LANGUAGE_SWEDISH_FINLAND, LANGUAGE_SYRIAC, LANGUAGE_TAJIK, LANGUAGE_TAMAZIGHT_ARABIC,
    LANGUAGE_TAMAZIGHT_LATIN, LANGUAGE_TAMIL, LANGUAGE_TATAR, LANGUAGE_TELUGU, LANGUAGE_THAI,
    LANGUAGE_TIBETAN, LANGUAGE_DZONGKHA, LANGUAGE_TIBETAN_BHUTAN, LANGUAGE_TIGRIGNA_ERITREA,
    LANGUAGE_TIGRIGNA_ETHIOPIA, LANGUAGE_TSONGA, LANGUAGE_TSWANA, LANGUAGE_TURKISH, LANGUAGE_TURKMEN,
    LANGUAGE_UIGHUR_CHINA, LANGUAGE_UKRAINIAN, LANGUAGE_URDU_INDIA, LANGUAGE_URDU_PAKISTAN,
    LANGUAGE_UZBEK_CYRILLIC, LANGUAGE_UZBEK_LATIN, LANGUAGE_VENDA, LANGUAGE_VIETNAMESE, LANGUAGE_WELSH,
    LANGUAGE_WOLOF_SENEGAL, LANGUAGE_XHOSA, LANGUAGE_YAKUT_RUSSIA, LANGUAGE_YI, LANGUAGE_YIDDISH,
    LANGUAGE_YORUBA, LANGUAGE_ZULU,
};

// Java: public class LanguageCode
pub struct LanguageCode;

// Java field: String DEFAULT_LANG
const DEFAULT_LANG: &str = "en_US";
// Java field: Map<String, LanguageType> m
static M: OnceLock<HashMap<&str, LanguageType>> = OnceLock::new();

// Java: public class LanguageCode
impl LanguageCode {
    // Java: LanguageCode()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static String getLanguageCode(LanguageType lang)
    pub fn getLanguageCode(lang: LanguageType) -> &'static str {
        // Java: switch (lang)
        // Java: case LANGUAGE_AFRIKAANS:
        // Java: return "af_ZA";
        if lang == LANGUAGE_AFRIKAANS { return "af_ZA"; }
        // Java: case LANGUAGE_ALBANIAN:
        // Java: return "sq_AL";
        if lang == LANGUAGE_ALBANIAN { return "sq_AL"; }
        // Java: case LANGUAGE_ALSATIAN_FRANCE:
        // Java: return "gsw_FR";
        if lang == LANGUAGE_ALSATIAN_FRANCE { return "gsw_FR"; }
        // Java: case LANGUAGE_AMHARIC_ETHIOPIA:
        // Java: return "am_ET";
        if lang == LANGUAGE_AMHARIC_ETHIOPIA { return "am_ET"; }
        // Java: case LANGUAGE_ARABIC_ALGERIA:
        // Java: return "ar_DZ";
        if lang == LANGUAGE_ARABIC_ALGERIA { return "ar_DZ"; }
        // Java: case LANGUAGE_ARABIC_BAHRAIN:
        // Java: return "ar_BH";
        if lang == LANGUAGE_ARABIC_BAHRAIN { return "ar_BH"; }
        // Java: case LANGUAGE_ARABIC_EGYPT:
        // Java: return "ar_EG";
        if lang == LANGUAGE_ARABIC_EGYPT { return "ar_EG"; }
        // Java: case LANGUAGE_ARABIC_IRAQ:
        // Java: return "ar_IQ";
        if lang == LANGUAGE_ARABIC_IRAQ { return "ar_IQ"; }
        // Java: case LANGUAGE_ARABIC_JORDAN:
        // Java: return "ar_JO";
        if lang == LANGUAGE_ARABIC_JORDAN { return "ar_JO"; }
        // Java: case LANGUAGE_ARABIC_KUWAIT:
        // Java: return "ar_KW";
        if lang == LANGUAGE_ARABIC_KUWAIT { return "ar_KW"; }
        // Java: case LANGUAGE_ARABIC_LEBANON:
        // Java: return "ar_LB";
        if lang == LANGUAGE_ARABIC_LEBANON { return "ar_LB"; }
        // Java: case LANGUAGE_ARABIC_LIBYA:
        // Java: return "ar_LY";
        if lang == LANGUAGE_ARABIC_LIBYA { return "ar_LY"; }
        // Java: case LANGUAGE_ARABIC_MOROCCO:
        // Java: return "ar_MA";
        if lang == LANGUAGE_ARABIC_MOROCCO { return "ar_MA"; }
        // Java: case LANGUAGE_ARABIC_OMAN:
        // Java: return "ar_OM";
        if lang == LANGUAGE_ARABIC_OMAN { return "ar_OM"; }
        // Java: case LANGUAGE_ARABIC_QATAR:
        // Java: return "ar_QA";
        if lang == LANGUAGE_ARABIC_QATAR { return "ar_QA"; }
        // Java: case LANGUAGE_ARABIC_SAUDI_ARABIA:
        // Java: return "ar_SA";
        if lang == LANGUAGE_ARABIC_SAUDI_ARABIA { return "ar_SA"; }
        // Java: case LANGUAGE_ARABIC_SYRIA:
        // Java: return "ar_SY";
        if lang == LANGUAGE_ARABIC_SYRIA { return "ar_SY"; }
        // Java: case LANGUAGE_ARABIC_TUNISIA:
        // Java: return "ar_TN";
        if lang == LANGUAGE_ARABIC_TUNISIA { return "ar_TN"; }
        // Java: case LANGUAGE_ARABIC_UAE:
        // Java: return "ar_AE";
        if lang == LANGUAGE_ARABIC_UAE { return "ar_AE"; }
        // Java: case LANGUAGE_ARABIC_YEMEN:
        // Java: return "ar_YE";
        if lang == LANGUAGE_ARABIC_YEMEN { return "ar_YE"; }
        // Java: case LANGUAGE_ARMENIAN:
        // Java: return "hy_AM";
        if lang == LANGUAGE_ARMENIAN { return "hy_AM"; }
        // Java: case LANGUAGE_ASSAMESE:
        // Java: return "as_IN";
        if lang == LANGUAGE_ASSAMESE { return "as_IN"; }
        // Java: case LANGUAGE_AZERI_CYRILLIC:
        // Java: return "az_Cyrl";
        if lang == LANGUAGE_AZERI_CYRILLIC { return "az_Cyrl"; }
        // Java: case LANGUAGE_AZERI_LATIN:
        // Java: return "az_AZ";
        if lang == LANGUAGE_AZERI_LATIN { return "az_AZ"; }
        // Java: case LANGUAGE_BASHKIR_RUSSIA:
        // Java: return "ba_RU";
        if lang == LANGUAGE_BASHKIR_RUSSIA { return "ba_RU"; }
        // Java: case LANGUAGE_BASQUE:
        // Java: return "eu_ES";
        if lang == LANGUAGE_BASQUE { return "eu_ES"; }
        // Java: case LANGUAGE_BELARUSIAN:
        // Java: return "be_BY";
        if lang == LANGUAGE_BELARUSIAN { return "be_BY"; }
        // Java: case LANGUAGE_BENGALI:
        // Java: return "bn_IN";
        if lang == LANGUAGE_BENGALI { return "bn_IN"; }
        // Java: case LANGUAGE_BENGALI_BANGLADESH:
        // Java: return "bn_BD";
        if lang == LANGUAGE_BENGALI_BANGLADESH { return "bn_BD"; }
        // Java: case LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA:
        // Java: return "bs_BA";
        if lang == LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA { return "bs_BA"; }
        // Java: case LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA:
        // Java: return "bs_BA";
        if lang == LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA { return "bs_BA"; }
        // Java: case LANGUAGE_BRETON_FRANCE:
        // Java: return "br_FR";
        if lang == LANGUAGE_BRETON_FRANCE { return "br_FR"; }
        // Java: case LANGUAGE_BULGARIAN:
        // Java: return "bg_BG";
        if lang == LANGUAGE_BULGARIAN { return "bg_BG"; }
        // Java: case LANGUAGE_BURMESE:
        // Java: return "my_MM";
        if lang == LANGUAGE_BURMESE { return "my_MM"; }
        // Java: case LANGUAGE_CATALAN:
        // Java: return "ca_ES";
        if lang == LANGUAGE_CATALAN { return "ca_ES"; }
        // Java: case LANGUAGE_CHEROKEE_UNITED_STATES:
        // Java: return "chr_US";
        if lang == LANGUAGE_CHEROKEE_UNITED_STATES { return "chr_US"; }
        // Java: case LANGUAGE_CHINESE_HONGKONG:
        // Java: return "zh_HK";
        if lang == LANGUAGE_CHINESE_HONGKONG { return "zh_HK"; }
        // Java: case LANGUAGE_CHINESE_MACAU:
        // Java: return "zh_MO";
        if lang == LANGUAGE_CHINESE_MACAU { return "zh_MO"; }
        // Java: case LANGUAGE_CHINESE_SIMPLIFIED:
        // Java: return "zh_CN";
        if lang == LANGUAGE_CHINESE_SIMPLIFIED { return "zh_CN"; }
        // Java: case LANGUAGE_CHINESE_SINGAPORE:
        // Java: return "zh_SP";
        if lang == LANGUAGE_CHINESE_SINGAPORE { return "zh_SP"; }
        // Java: case LANGUAGE_CHINESE_TRADITIONAL:
        // Java: return "zh_TW";
        if lang == LANGUAGE_CHINESE_TRADITIONAL { return "zh_TW"; }
        // Java: case LANGUAGE_CORSICAN_FRANCE:
        // Java: return "co_FR";
        if lang == LANGUAGE_CORSICAN_FRANCE { return "co_FR"; }
        // Java: case LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA:
        // Java: return "hr_BA";
        if lang == LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA { return "hr_BA"; }
        // Java: case LANGUAGE_CZECH:
        // Java: return "cs_CZ";
        if lang == LANGUAGE_CZECH { return "cs_CZ"; }
        // Java: case LANGUAGE_DANISH:
        // Java: return "da_DK";
        if lang == LANGUAGE_DANISH { return "da_DK"; }
        // Java: case LANGUAGE_DARI_AFGHANISTAN:
        // Java: return "prs_AF";
        if lang == LANGUAGE_DARI_AFGHANISTAN { return "prs_AF"; }
        // Java: case LANGUAGE_DHIVEHI:
        // Java: return "dv_DV";
        if lang == LANGUAGE_DHIVEHI { return "dv_DV"; }
        // Java: case LANGUAGE_DUTCH:
        // Java: return "nl_NL";
        if lang == LANGUAGE_DUTCH { return "nl_NL"; }
        // Java: case LANGUAGE_DUTCH_BELGIAN:
        // Java: return "nl_BE";
        if lang == LANGUAGE_DUTCH_BELGIAN { return "nl_BE"; }
        // Java: case LANGUAGE_EDO:
        // Java: return "en_US";
        if lang == LANGUAGE_EDO { return "en_US"; }
        // Java: case LANGUAGE_ENGLISH_AUS:
        // Java: return "en_AU";
        if lang == LANGUAGE_ENGLISH_AUS { return "en_AU"; }
        // Java: case LANGUAGE_ENGLISH_BELIZE:
        // Java: return "en_BZ";
        if lang == LANGUAGE_ENGLISH_BELIZE { return "en_BZ"; }
        // Java: case LANGUAGE_ENGLISH_CAN:
        // Java: return "en_CA";
        if lang == LANGUAGE_ENGLISH_CAN { return "en_CA"; }
        // Java: case LANGUAGE_ENGLISH_CARRIBEAN:
        // Java: return "en_US";
        if lang == LANGUAGE_ENGLISH_CARRIBEAN { return "en_US"; }
        // Java: case LANGUAGE_ENGLISH_EIRE:
        // Java: return "en_IE";
        if lang == LANGUAGE_ENGLISH_EIRE { return "en_IE"; }
        // Java: case LANGUAGE_ENGLISH_HONG_KONG_SAR:
        // Java: return "en_HK";
        if lang == LANGUAGE_ENGLISH_HONG_KONG_SAR { return "en_HK"; }
        // Java: case LANGUAGE_ENGLISH_INDIA:
        // Java: return "en_IN";
        if lang == LANGUAGE_ENGLISH_INDIA { return "en_IN"; }
        // Java: case LANGUAGE_ENGLISH_INDONESIA:
        // Java: return "en_ID";
        if lang == LANGUAGE_ENGLISH_INDONESIA { return "en_ID"; }
        // Java: case LANGUAGE_ENGLISH_JAMAICA:
        // Java: return "en_JM";
        if lang == LANGUAGE_ENGLISH_JAMAICA { return "en_JM"; }
        // Java: case LANGUAGE_ENGLISH_MALAYSIA:
        // Java: return "en_MY";
        if lang == LANGUAGE_ENGLISH_MALAYSIA { return "en_MY"; }
        // Java: case LANGUAGE_ENGLISH_NZ:
        // Java: return "en_NZ";
        if lang == LANGUAGE_ENGLISH_NZ { return "en_NZ"; }
        // Java: case LANGUAGE_ENGLISH_PHILIPPINES:
        // Java: return "en_PH";
        if lang == LANGUAGE_ENGLISH_PHILIPPINES { return "en_PH"; }
        // Java: case LANGUAGE_ENGLISH_SAFRICA:
        // Java: return "en_ZA";
        if lang == LANGUAGE_ENGLISH_SAFRICA { return "en_ZA"; }
        // Java: case LANGUAGE_ENGLISH_SINGAPORE:
        // Java: return "en_SG";
        if lang == LANGUAGE_ENGLISH_SINGAPORE { return "en_SG"; }
        // Java: case LANGUAGE_ENGLISH_TRINIDAD:
        // Java: return "en_TT";
        if lang == LANGUAGE_ENGLISH_TRINIDAD { return "en_TT"; }
        // Java: case LANGUAGE_ENGLISH_UK:
        // Java: return "en_GB";
        if lang == LANGUAGE_ENGLISH_UK { return "en_GB"; }
        // Java: case LANGUAGE_ENGLISH_US:
        // Java: return "en_US";
        if lang == LANGUAGE_ENGLISH_US { return "en_US"; }
        // Java: case LANGUAGE_ENGLISH_ZIMBABWE:
        // Java: return "en_ZW";
        if lang == LANGUAGE_ENGLISH_ZIMBABWE { return "en_ZW"; }
        // Java: case LANGUAGE_ESTONIAN:
        // Java: return "et_EE";
        if lang == LANGUAGE_ESTONIAN { return "et_EE"; }
        // Java: case LANGUAGE_FAEROESE:
        // Java: return "fo_FO";
        if lang == LANGUAGE_FAEROESE { return "fo_FO"; }
        // Java: case LANGUAGE_FARSI:
        // Java: return "fa_IR";
        if lang == LANGUAGE_FARSI { return "fa_IR"; }
        // Java: case LANGUAGE_FILIPINO:
        // Java: return "fil_PH";
        if lang == LANGUAGE_FILIPINO { return "fil_PH"; }
        // Java: case LANGUAGE_FINNISH:
        // Java: return "fi_FI";
        if lang == LANGUAGE_FINNISH { return "fi_FI"; }
        // Java: case LANGUAGE_FRENCH:
        // Java: return "fr_FR";
        if lang == LANGUAGE_FRENCH { return "fr_FR"; }
        // Java: case LANGUAGE_FRENCH_BELGIAN:
        // Java: return "fr_BE";
        if lang == LANGUAGE_FRENCH_BELGIAN { return "fr_BE"; }
        // Java: case LANGUAGE_FRENCH_CAMEROON:
        // Java: return "fr_CM";
        if lang == LANGUAGE_FRENCH_CAMEROON { return "fr_CM"; }
        // Java: case LANGUAGE_FRENCH_CANADIAN:
        // Java: return "fr_CA";
        if lang == LANGUAGE_FRENCH_CANADIAN { return "fr_CA"; }
        // Java: case LANGUAGE_FRENCH_COTE_D_IVOIRE:
        // Java: return "fr_CI";
        if lang == LANGUAGE_FRENCH_COTE_D_IVOIRE { return "fr_CI"; }
        // Java: case LANGUAGE_FRENCH_HAITI:
        // Java: return "fr_HT";
        if lang == LANGUAGE_FRENCH_HAITI { return "fr_HT"; }
        // Java: case LANGUAGE_FRENCH_LUXEMBOURG:
        // Java: return "fr_LU";
        if lang == LANGUAGE_FRENCH_LUXEMBOURG { return "fr_LU"; }
        // Java: case LANGUAGE_FRENCH_MALI:
        // Java: return "fr_ML";
        if lang == LANGUAGE_FRENCH_MALI { return "fr_ML"; }
        // Java: case LANGUAGE_FRENCH_MONACO:
        // Java: return "fr_MC";
        if lang == LANGUAGE_FRENCH_MONACO { return "fr_MC"; }
        // Java: case LANGUAGE_FRENCH_MOROCCO:
        // Java: return "fr_MA";
        if lang == LANGUAGE_FRENCH_MOROCCO { return "fr_MA"; }
        // Java: case LANGUAGE_FRENCH_NORTH_AFRICA:
        // Java: return "fr_FR";
        if lang == LANGUAGE_FRENCH_NORTH_AFRICA { return "fr_FR"; }
        // Java: case LANGUAGE_FRENCH_REUNION:
        // Java: return "fr_RE";
        if lang == LANGUAGE_FRENCH_REUNION { return "fr_RE"; }
        // Java: case LANGUAGE_FRENCH_SENEGAL:
        // Java: return "fr_SN";
        if lang == LANGUAGE_FRENCH_SENEGAL { return "fr_SN"; }
        // Java: case LANGUAGE_FRENCH_SWISS:
        // Java: return "fr_CH";
        if lang == LANGUAGE_FRENCH_SWISS { return "fr_CH"; }
        // Java: case LANGUAGE_FRENCH_WEST_INDIES:
        // Java: return "fr_FR";
        if lang == LANGUAGE_FRENCH_WEST_INDIES { return "fr_FR"; }
        // Java: case LANGUAGE_FRENCH_ZAIRE:
        // Java: return "fr_ZR";
        if lang == LANGUAGE_FRENCH_ZAIRE { return "fr_ZR"; }
        // Java: case LANGUAGE_FRISIAN_NETHERLANDS:
        // Java: return "fy_NL";
        if lang == LANGUAGE_FRISIAN_NETHERLANDS { return "fy_NL"; }
        // Java: case LANGUAGE_FULFULDE_NIGERIA:
        // Java: return "en_US";
        if lang == LANGUAGE_FULFULDE_NIGERIA { return "en_US"; }
        // Java: case LANGUAGE_GAELIC_IRELAND:
        // Java: return "gd_IE";
        if lang == LANGUAGE_GAELIC_IRELAND { return "gd_IE"; }
        // Java: case LANGUAGE_GAELIC_SCOTLAND:
        // Java: return "gd_GB";
        if lang == LANGUAGE_GAELIC_SCOTLAND { return "gd_GB"; }
        // Java: case LANGUAGE_GALICIAN:
        // Java: return "gl_ES";
        if lang == LANGUAGE_GALICIAN { return "gl_ES"; }
        // Java: case LANGUAGE_GEORGIAN:
        // Java: return "ka_GE";
        if lang == LANGUAGE_GEORGIAN { return "ka_GE"; }
        // Java: case LANGUAGE_GERMAN:
        // Java: return "de_DE";
        if lang == LANGUAGE_GERMAN { return "de_DE"; }
        // Java: case LANGUAGE_GERMAN_AUSTRIAN:
        // Java: return "de_AT";
        if lang == LANGUAGE_GERMAN_AUSTRIAN { return "de_AT"; }
        // Java: case LANGUAGE_GERMAN_LIECHTENSTEIN:
        // Java: return "de_LI";
        if lang == LANGUAGE_GERMAN_LIECHTENSTEIN { return "de_LI"; }
        // Java: case LANGUAGE_GERMAN_LUXEMBOURG:
        // Java: return "de_LU";
        if lang == LANGUAGE_GERMAN_LUXEMBOURG { return "de_LU"; }
        // Java: case LANGUAGE_GERMAN_SWISS:
        // Java: return "de_CH";
        if lang == LANGUAGE_GERMAN_SWISS { return "de_CH"; }
        // Java: case LANGUAGE_GREEK:
        // Java: return "el_GR";
        if lang == LANGUAGE_GREEK { return "el_GR"; }
        // Java: case LANGUAGE_GUARANI_PARAGUAY:
        // Java: return "gn_PY";
        if lang == LANGUAGE_GUARANI_PARAGUAY { return "gn_PY"; }
        // Java: case LANGUAGE_GUJARATI:
        // Java: return "gu_IN";
        if lang == LANGUAGE_GUJARATI { return "gu_IN"; }
        // Java: case LANGUAGE_HAUSA_NIGERIA:
        // Java: return "ha_NE";
        if lang == LANGUAGE_HAUSA_NIGERIA { return "ha_NE"; }
        // Java: case LANGUAGE_HAWAIIAN_UNITED_STATES:
        // Java: return "haw_US";
        if lang == LANGUAGE_HAWAIIAN_UNITED_STATES { return "haw_US"; }
        // Java: case LANGUAGE_HEBREW:
        // Java: return "iw_IL";
        if lang == LANGUAGE_HEBREW { return "iw_IL"; }
        // Java: case LANGUAGE_HINDI:
        // Java: return "hi_IN";
        if lang == LANGUAGE_HINDI { return "hi_IN"; }
        // Java: case LANGUAGE_HUNGARIAN:
        // Java: return "hu_HU";
        if lang == LANGUAGE_HUNGARIAN { return "hu_HU"; }
        // Java: case LANGUAGE_IBIBIO_NIGERIA:
        // Java: return "ibb_NE";
        if lang == LANGUAGE_IBIBIO_NIGERIA { return "ibb_NE"; }
        // Java: case LANGUAGE_ICELANDIC:
        // Java: return "is_IS";
        if lang == LANGUAGE_ICELANDIC { return "is_IS"; }
        // Java: case LANGUAGE_IGBO_NIGERIA:
        // Java: return "ig_NE";
        if lang == LANGUAGE_IGBO_NIGERIA { return "ig_NE"; }
        // Java: case LANGUAGE_INDONESIAN:
        // Java: return "id_ID";
        if lang == LANGUAGE_INDONESIAN { return "id_ID"; }
        // Java: case LANGUAGE_INUKTITUT_LATIN_CANADA:
        // Java: case LANGUAGE_INUKTITUT_SYLLABICS_CANADA:
        // Java: return "iu_CA";
        if lang == LANGUAGE_INUKTITUT_LATIN_CANADA || lang == LANGUAGE_INUKTITUT_SYLLABICS_CANADA { return "iu_CA"; }
        // Java: case LANGUAGE_ITALIAN:
        // Java: return "it_IT";
        if lang == LANGUAGE_ITALIAN { return "it_IT"; }
        // Java: case LANGUAGE_ITALIAN_SWISS:
        // Java: return "it_CH";
        if lang == LANGUAGE_ITALIAN_SWISS { return "it_CH"; }
        // Java: case LANGUAGE_JAPANESE:
        // Java: return "ja_JP";
        if lang == LANGUAGE_JAPANESE { return "ja_JP"; }
        // Java: case LANGUAGE_KALAALLISUT_GREENLAND:
        // Java: return "kl_GL";
        if lang == LANGUAGE_KALAALLISUT_GREENLAND { return "kl_GL"; }
        // Java: case LANGUAGE_KANNADA:
        // Java: return "kn_IN";
        if lang == LANGUAGE_KANNADA { return "kn_IN"; }
        // Java: case LANGUAGE_KANURI_NIGERIA:
        // Java: return "kr_NE";
        if lang == LANGUAGE_KANURI_NIGERIA { return "kr_NE"; }
        // Java: case LANGUAGE_KASHMIRI:
        // Java: return "ks_KS";
        if lang == LANGUAGE_KASHMIRI { return "ks_KS"; }
        // Java: case LANGUAGE_KASHMIRI_INDIA:
        // Java: return "ks_IN";
        if lang == LANGUAGE_KASHMIRI_INDIA { return "ks_IN"; }
        // Java: case LANGUAGE_KAZAK:
        // Java: return "kk_KZ";
        if lang == LANGUAGE_KAZAK { return "kk_KZ"; }
        // Java: case LANGUAGE_KHMER:
        // Java: return "km_KH";
        if lang == LANGUAGE_KHMER { return "km_KH"; }
        // Java: case LANGUAGE_KICHE_GUATEMALA:
        // Java: return "quc_GT";
        if lang == LANGUAGE_KICHE_GUATEMALA { return "quc_GT"; }
        // Java: case LANGUAGE_KINYARWANDA_RWANDA:
        // Java: return "rw_RW";
        if lang == LANGUAGE_KINYARWANDA_RWANDA { return "rw_RW"; }
        // Java: case LANGUAGE_KIRGHIZ:
        // Java: return "ky_KG";
        if lang == LANGUAGE_KIRGHIZ { return "ky_KG"; }
        // Java: case LANGUAGE_KONKANI:
        // Java: return "kok_IN";
        if lang == LANGUAGE_KONKANI { return "kok_IN"; }
        // Java: case LANGUAGE_KOREAN:
        // Java: return "ko_KR";
        if lang == LANGUAGE_KOREAN { return "ko_KR"; }
        // Java: case LANGUAGE_KOREAN_JOHAB:
        // Java: return "ko_KR";
        if lang == LANGUAGE_KOREAN_JOHAB { return "ko_KR"; }
        // Java: case LANGUAGE_LAO:
        // Java: return "lo_LA";
        if lang == LANGUAGE_LAO { return "lo_LA"; }
        // Java: case LANGUAGE_LATIN:
        // Java: return "en_US";
        if lang == LANGUAGE_LATIN { return "en_US"; }
        // Java: case LANGUAGE_LATVIAN:
        // Java: return "lv_LV";
        if lang == LANGUAGE_LATVIAN { return "lv_LV"; }
        // Java: case LANGUAGE_LITHUANIAN:
        // Java: return "lt_LT";
        if lang == LANGUAGE_LITHUANIAN { return "lt_LT"; }
        // Java: case LANGUAGE_LITHUANIAN_CLASSIC:
        // Java: return "lt_LT";
        if lang == LANGUAGE_LITHUANIAN_CLASSIC { return "lt_LT"; }
        // Java: case LANGUAGE_LUXEMBOURGISH_LUXEMBOURG:
        // Java: return "lb_LU";
        if lang == LANGUAGE_LUXEMBOURGISH_LUXEMBOURG { return "lb_LU"; }
        // Java: case LANGUAGE_MACEDONIAN:
        // Java: return "mk_MK";
        if lang == LANGUAGE_MACEDONIAN { return "mk_MK"; }
        // Java: case LANGUAGE_MALAYALAM:
        // Java: return "ml_IN";
        if lang == LANGUAGE_MALAYALAM { return "ml_IN"; }
        // Java: case LANGUAGE_MALAY_BRUNEI_DARUSSALAM:
        // Java: return "ms_BN";
        if lang == LANGUAGE_MALAY_BRUNEI_DARUSSALAM { return "ms_BN"; }
        // Java: case LANGUAGE_MALAY_MALAYSIA:
        // Java: return "ms_MY";
        if lang == LANGUAGE_MALAY_MALAYSIA { return "ms_MY"; }
        // Java: case LANGUAGE_MALTESE:
        // Java: return "mt_MT";
        if lang == LANGUAGE_MALTESE { return "mt_MT"; }
        // Java: case LANGUAGE_MANIPURI:
        // Java: return "mni_IN";
        if lang == LANGUAGE_MANIPURI { return "mni_IN"; }
        // Java: case LANGUAGE_MAORI_NEW_ZEALAND:
        // Java: return "mi_NZ";
        if lang == LANGUAGE_MAORI_NEW_ZEALAND { return "mi_NZ"; }
        // Java: case LANGUAGE_MAPUDUNGUN_CHILE:
        // Java: return "arn_CL";
        if lang == LANGUAGE_MAPUDUNGUN_CHILE { return "arn_CL"; }
        // Java: case LANGUAGE_MARATHI:
        // Java: return "mr_IN";
        if lang == LANGUAGE_MARATHI { return "mr_IN"; }
        // Java: case LANGUAGE_MOHAWK_CANADA:
        // Java: return "moh_CA";
        if lang == LANGUAGE_MOHAWK_CANADA { return "moh_CA"; }
        // Java: case LANGUAGE_MONGOLIAN_MONGOLIAN:
        // Java: return "mn_MN";
        if lang == LANGUAGE_MONGOLIAN_MONGOLIAN { return "mn_MN"; }
        // Java: case LANGUAGE_NEPALI:
        // Java: return "ne_NP";
        if lang == LANGUAGE_NEPALI { return "ne_NP"; }
        // Java: case LANGUAGE_NEPALI_INDIA:
        // Java: return "ne_IN";
        if lang == LANGUAGE_NEPALI_INDIA { return "ne_IN"; }
        // Java: case LANGUAGE_NORWEGIAN_BOKMAL:
        // Java: return "no_NO";
        if lang == LANGUAGE_NORWEGIAN_BOKMAL { return "no_NO"; }
        // Java: case LANGUAGE_NORWEGIAN_NYNORSK:
        // Java: return "no_NO_NY";
        if lang == LANGUAGE_NORWEGIAN_NYNORSK { return "no_NO_NY"; }
        // Java: case LANGUAGE_OCCITAN_FRANCE:
        // Java: return "oc_FR";
        if lang == LANGUAGE_OCCITAN_FRANCE { return "oc_FR"; }
        // Java: case LANGUAGE_ORIYA:
        // Java: return "or_IN";
        if lang == LANGUAGE_ORIYA { return "or_IN"; }
        // Java: case LANGUAGE_OROMO:
        // Java: return "om_KE";
        if lang == LANGUAGE_OROMO { return "om_KE"; }
        // Java: case LANGUAGE_PAPIAMENTU:
        // Java: return "pap_AW";
        if lang == LANGUAGE_PAPIAMENTU { return "pap_AW"; }
        // Java: case LANGUAGE_PASHTO:
        // Java: return "ps_AF";
        if lang == LANGUAGE_PASHTO { return "ps_AF"; }
        // Java: case LANGUAGE_POLISH:
        // Java: return "pl_PL";
        if lang == LANGUAGE_POLISH { return "pl_PL"; }
        // Java: case LANGUAGE_PORTUGUESE:
        // Java: return "pt_PT";
        if lang == LANGUAGE_PORTUGUESE { return "pt_PT"; }
        // Java: case LANGUAGE_PORTUGUESE_BRAZILIAN:
        // Java: return "pt_BR";
        if lang == LANGUAGE_PORTUGUESE_BRAZILIAN { return "pt_BR"; }
        // Java: case LANGUAGE_PUNJABI:
        // Java: return "pa_IN";
        if lang == LANGUAGE_PUNJABI { return "pa_IN"; }
        // Java: case LANGUAGE_PUNJABI_PAKISTAN:
        // Java: return "pa_PK";
        if lang == LANGUAGE_PUNJABI_PAKISTAN { return "pa_PK"; }
        // Java: case LANGUAGE_QUECHUA_BOLIVIA:
        // Java: return "quz_BO";
        if lang == LANGUAGE_QUECHUA_BOLIVIA { return "quz_BO"; }
        // Java: case LANGUAGE_QUECHUA_ECUADOR:
        // Java: return "quz_EC";
        if lang == LANGUAGE_QUECHUA_ECUADOR { return "quz_EC"; }
        // Java: case LANGUAGE_QUECHUA_PERU:
        // Java: return "quz_PE";
        if lang == LANGUAGE_QUECHUA_PERU { return "quz_PE"; }
        // Java: case LANGUAGE_RHAETO_ROMAN:
        // Java: return "rm_RM";
        if lang == LANGUAGE_RHAETO_ROMAN { return "rm_RM"; }
        // Java: case LANGUAGE_ROMANIAN:
        // Java: return "ro_RO";
        if lang == LANGUAGE_ROMANIAN { return "ro_RO"; }
        // Java: case LANGUAGE_ROMANIAN_MOLDOVA:
        // Java: return "ro_MD";
        if lang == LANGUAGE_ROMANIAN_MOLDOVA { return "ro_MD"; }
        // Java: case LANGUAGE_RUSSIAN:
        // Java: return "ru_RU";
        if lang == LANGUAGE_RUSSIAN { return "ru_RU"; }
        // Java: case LANGUAGE_RUSSIAN_MOLDOVA:
        // Java: return "ru_MD";
        if lang == LANGUAGE_RUSSIAN_MOLDOVA { return "ru_MD"; }
        // Java: case LANGUAGE_SAMI_NORTHERN_NORWAY:
        // Java: return "se_NO";
        if lang == LANGUAGE_SAMI_NORTHERN_NORWAY { return "se_NO"; }
        // Java: case LANGUAGE_SAMI_LAPPISH:
        // Java: return "sz";
        if lang == LANGUAGE_SAMI_LAPPISH { return "sz"; }
        // Java: case LANGUAGE_SAMI_INARI:
        // Java: return "smn_FL";
        if lang == LANGUAGE_SAMI_INARI { return "smn_FL"; }
        // Java: case LANGUAGE_SAMI_LULE_NORWAY:
        // Java: return "smj_NO";
        if lang == LANGUAGE_SAMI_LULE_NORWAY { return "smj_NO"; }
        // Java: case LANGUAGE_SAMI_LULE_SWEDEN:
        // Java: return "smj_SE";
        if lang == LANGUAGE_SAMI_LULE_SWEDEN { return "smj_SE"; }
        // Java: case LANGUAGE_SAMI_NORTHERN_FINLAND:
        // Java: return "se_FI";
        if lang == LANGUAGE_SAMI_NORTHERN_FINLAND { return "se_FI"; }
        // Java: case LANGUAGE_SAMI_NORTHERN_SWEDEN:
        // Java: return "se_SE";
        if lang == LANGUAGE_SAMI_NORTHERN_SWEDEN { return "se_SE"; }
        // Java: case LANGUAGE_SAMI_SKOLT:
        // Java: return "sms_FI";
        if lang == LANGUAGE_SAMI_SKOLT { return "sms_FI"; }
        // Java: case LANGUAGE_SAMI_SOUTHERN_NORWAY:
        // Java: return "sma_NO";
        if lang == LANGUAGE_SAMI_SOUTHERN_NORWAY { return "sma_NO"; }
        // Java: case LANGUAGE_SAMI_SOUTHERN_SWEDEN:
        // Java: return "sma_SE";
        if lang == LANGUAGE_SAMI_SOUTHERN_SWEDEN { return "sma_SE"; }
        // Java: case LANGUAGE_SANSKRIT:
        // Java: return "sa_IN";
        if lang == LANGUAGE_SANSKRIT { return "sa_IN"; }
        // Java: case LANGUAGE_SEPEDI:
        // Java: case LANGUAGE_NORTHERNSOTHO:
        // Java: return "nso";
        if lang == LANGUAGE_SEPEDI || lang == LANGUAGE_NORTHERNSOTHO { return "nso"; }
        // Java: case LANGUAGE_SERBIAN_CYRILLIC:
        // Java: return "sr_YU";
        if lang == LANGUAGE_SERBIAN_CYRILLIC { return "sr_YU"; }
        // Java: case LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA:
        // Java: return "sr_BA";
        if lang == LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA { return "sr_BA"; }
        // Java: case LANGUAGE_SERBIAN_LATIN:
        // Java: return "sr_YU";
        if lang == LANGUAGE_SERBIAN_LATIN { return "sr_YU"; }
        // Java: case LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA:
        // Java: return "sr_YU";
        if lang == LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA { return "sr_YU"; }
        // Java: case LANGUAGE_SERBIAN_LATIN_NEUTRAL:
        // Java: return "sr_YU";
        if lang == LANGUAGE_SERBIAN_LATIN_NEUTRAL { return "sr_YU"; }
        // Java: case LANGUAGE_SESOTHO:
        // Java: return "nso_ZA";
        if lang == LANGUAGE_SESOTHO { return "nso_ZA"; }
        // Java: case LANGUAGE_SINDHI:
        // Java: return "sd_IN";
        if lang == LANGUAGE_SINDHI { return "sd_IN"; }
        // Java: case LANGUAGE_SINDHI_PAKISTAN:
        // Java: return "sd_PK";
        if lang == LANGUAGE_SINDHI_PAKISTAN { return "sd_PK"; }
        // Java: case LANGUAGE_SINHALESE_SRI_LANKA:
        // Java: return "si_LK";
        if lang == LANGUAGE_SINHALESE_SRI_LANKA { return "si_LK"; }
        // Java: case LANGUAGE_SLOVAK:
        // Java: return "sk_SK";
        if lang == LANGUAGE_SLOVAK { return "sk_SK"; }
        // Java: case LANGUAGE_SLOVENIAN:
        // Java: return "sl_SI";
        if lang == LANGUAGE_SLOVENIAN { return "sl_SI"; }
        // Java: case LANGUAGE_SOMALI:
        // Java: return "so_SO";
        if lang == LANGUAGE_SOMALI { return "so_SO"; }
        // Java: case LANGUAGE_UPPER_SORBIAN_GERMANY:
        // Java: return "hsb_DE";
        if lang == LANGUAGE_UPPER_SORBIAN_GERMANY { return "hsb_DE"; }
        // Java: case LANGUAGE_LOWER_SORBIAN_GERMANY:
        // Java: return "dsb_DE";
        if lang == LANGUAGE_LOWER_SORBIAN_GERMANY { return "dsb_DE"; }
        // Java: case LANGUAGE_SPANISH_DATED:
        // Java: return "es_ES";
        if lang == LANGUAGE_SPANISH_DATED { return "es_ES"; }
        // Java: case LANGUAGE_SPANISH_ARGENTINA:
        // Java: return "es_AR";
        if lang == LANGUAGE_SPANISH_ARGENTINA { return "es_AR"; }
        // Java: case LANGUAGE_SPANISH_BOLIVIA:
        // Java: return "es_BO";
        if lang == LANGUAGE_SPANISH_BOLIVIA { return "es_BO"; }
        // Java: case LANGUAGE_SPANISH_CHILE:
        // Java: return "es_CL";
        if lang == LANGUAGE_SPANISH_CHILE { return "es_CL"; }
        // Java: case LANGUAGE_SPANISH_COLOMBIA:
        // Java: return "es_CO";
        if lang == LANGUAGE_SPANISH_COLOMBIA { return "es_CO"; }
        // Java: case LANGUAGE_SPANISH_COSTARICA:
        // Java: return "es_CR";
        if lang == LANGUAGE_SPANISH_COSTARICA { return "es_CR"; }
        // Java: case LANGUAGE_SPANISH_DOMINICAN_REPUBLIC:
        // Java: return "es_DO";
        if lang == LANGUAGE_SPANISH_DOMINICAN_REPUBLIC { return "es_DO"; }
        // Java: case LANGUAGE_SPANISH_ECUADOR:
        // Java: return "es_EC";
        if lang == LANGUAGE_SPANISH_ECUADOR { return "es_EC"; }
        // Java: case LANGUAGE_SPANISH_EL_SALVADOR:
        // Java: return "es_SV";
        if lang == LANGUAGE_SPANISH_EL_SALVADOR { return "es_SV"; }
        // Java: case LANGUAGE_SPANISH_GUATEMALA:
        // Java: return "es_GT";
        if lang == LANGUAGE_SPANISH_GUATEMALA { return "es_GT"; }
        // Java: case LANGUAGE_SPANISH_HONDURAS:
        // Java: return "es_HN";
        if lang == LANGUAGE_SPANISH_HONDURAS { return "es_HN"; }
        // Java: case LANGUAGE_SPANISH_LATIN_AMERICA:
        // Java: return "es_US";
        if lang == LANGUAGE_SPANISH_LATIN_AMERICA { return "es_US"; }
        // Java: case LANGUAGE_SPANISH_MEXICAN:
        // Java: return "es_MX";
        if lang == LANGUAGE_SPANISH_MEXICAN { return "es_MX"; }
        // Java: case LANGUAGE_SPANISH_MODERN:
        // Java: return "es_ES";
        if lang == LANGUAGE_SPANISH_MODERN { return "es_ES"; }
        // Java: case LANGUAGE_SPANISH_NICARAGUA:
        // Java: return "es_NI";
        if lang == LANGUAGE_SPANISH_NICARAGUA { return "es_NI"; }
        // Java: case LANGUAGE_SPANISH_PANAMA:
        // Java: return "es_PA";
        if lang == LANGUAGE_SPANISH_PANAMA { return "es_PA"; }
        // Java: case LANGUAGE_SPANISH_PARAGUAY:
        // Java: return "es_PY";
        if lang == LANGUAGE_SPANISH_PARAGUAY { return "es_PY"; }
        // Java: case LANGUAGE_SPANISH_PERU:
        // Java: return "es_PE";
        if lang == LANGUAGE_SPANISH_PERU { return "es_PE"; }
        // Java: case LANGUAGE_SPANISH_PUERTO_RICO:
        // Java: return "es_PR";
        if lang == LANGUAGE_SPANISH_PUERTO_RICO { return "es_PR"; }
        // Java: case LANGUAGE_SPANISH_UNITED_STATES:
        // Java: return "es_US";
        if lang == LANGUAGE_SPANISH_UNITED_STATES { return "es_US"; }
        // Java: case LANGUAGE_SPANISH_URUGUAY:
        // Java: return "es_UY";
        if lang == LANGUAGE_SPANISH_URUGUAY { return "es_UY"; }
        // Java: case LANGUAGE_SPANISH_VENEZUELA:
        // Java: return "es_VE";
        if lang == LANGUAGE_SPANISH_VENEZUELA { return "es_VE"; }
        // Java: case LANGUAGE_SPANISH:
        // Java: return "es_ES";
        if lang == LANGUAGE_SPANISH { return "es_ES"; }
        // Java: case LANGUAGE_SWAHILI:
        // Java: return "sw_KE";
        if lang == LANGUAGE_SWAHILI { return "sw_KE"; }
        // Java: case LANGUAGE_SWEDISH:
        // Java: return "sv_SE";
        if lang == LANGUAGE_SWEDISH { return "sv_SE"; }
        // Java: case LANGUAGE_SWEDISH_FINLAND:
        // Java: return "sv_FI";
        if lang == LANGUAGE_SWEDISH_FINLAND { return "sv_FI"; }
        // Java: case LANGUAGE_SYRIAC:
        // Java: return "syr_SY";
        if lang == LANGUAGE_SYRIAC { return "syr_SY"; }
        // Java: case LANGUAGE_TAJIK:
        // Java: return "tg_TJ";
        if lang == LANGUAGE_TAJIK { return "tg_TJ"; }
        // Java: case LANGUAGE_TAMAZIGHT_ARABIC:
        // Java: return "tzm";
        if lang == LANGUAGE_TAMAZIGHT_ARABIC { return "tzm"; }
        // Java: case LANGUAGE_TAMAZIGHT_LATIN:
        // Java: return "tzm_Latn_DZ";
        if lang == LANGUAGE_TAMAZIGHT_LATIN { return "tzm_Latn_DZ"; }
        // Java: case LANGUAGE_TAMIL:
        // Java: return "ta_IN";
        if lang == LANGUAGE_TAMIL { return "ta_IN"; }
        // Java: case LANGUAGE_TATAR:
        // Java: return "tt_RU";
        if lang == LANGUAGE_TATAR { return "tt_RU"; }
        // Java: case LANGUAGE_TELUGU:
        // Java: return "te_IN";
        if lang == LANGUAGE_TELUGU { return "te_IN"; }
        // Java: case LANGUAGE_THAI:
        // Java: return "th_TH";
        if lang == LANGUAGE_THAI { return "th_TH"; }
        // Java: case LANGUAGE_TIBETAN:
        // Java: return "bo_CN";
        if lang == LANGUAGE_TIBETAN { return "bo_CN"; }
        // Java: case LANGUAGE_DZONGKHA:
        // Java: return "dz_BT";
        if lang == LANGUAGE_DZONGKHA { return "dz_BT"; }
        // Java: case LANGUAGE_TIBETAN_BHUTAN:
        // Java: return "bo_BT";
        if lang == LANGUAGE_TIBETAN_BHUTAN { return "bo_BT"; }
        // Java: case LANGUAGE_TIGRIGNA_ERITREA:
        // Java: return "ti_ER";
        if lang == LANGUAGE_TIGRIGNA_ERITREA { return "ti_ER"; }
        // Java: case LANGUAGE_TIGRIGNA_ETHIOPIA:
        // Java: return "ti_ET";
        if lang == LANGUAGE_TIGRIGNA_ETHIOPIA { return "ti_ET"; }
        // Java: case LANGUAGE_TSONGA:
        // Java: return "ts_ZA";
        if lang == LANGUAGE_TSONGA { return "ts_ZA"; }
        // Java: case LANGUAGE_TSWANA:
        // Java: return "tn_BW";
        if lang == LANGUAGE_TSWANA { return "tn_BW"; }
        // Java: case LANGUAGE_TURKISH:
        // Java: return "tr_TR";
        if lang == LANGUAGE_TURKISH { return "tr_TR"; }
        // Java: case LANGUAGE_TURKMEN:
        // Java: return "tk_TM";
        if lang == LANGUAGE_TURKMEN { return "tk_TM"; }
        // Java: case LANGUAGE_UIGHUR_CHINA:
        // Java: return "ug_CN";
        if lang == LANGUAGE_UIGHUR_CHINA { return "ug_CN"; }
        // Java: case LANGUAGE_UKRAINIAN:
        // Java: return "uk_UA";
        if lang == LANGUAGE_UKRAINIAN { return "uk_UA"; }
        // Java: case LANGUAGE_URDU_INDIA:
        // Java: return "ur_IN";
        if lang == LANGUAGE_URDU_INDIA { return "ur_IN"; }
        // Java: case LANGUAGE_URDU_PAKISTAN:
        // Java: return "ur_PK";
        if lang == LANGUAGE_URDU_PAKISTAN { return "ur_PK"; }
        // Java: case LANGUAGE_UZBEK_CYRILLIC:
        // Java: return "uz_UZ";
        if lang == LANGUAGE_UZBEK_CYRILLIC { return "uz_UZ"; }
        // Java: case LANGUAGE_UZBEK_LATIN:
        // Java: return "uz_UZ";
        if lang == LANGUAGE_UZBEK_LATIN { return "uz_UZ"; }
        // Java: case LANGUAGE_VENDA:
        // Java: return "ven_ZA";
        if lang == LANGUAGE_VENDA { return "ven_ZA"; }
        // Java: case LANGUAGE_VIETNAMESE:
        // Java: return "vi_VN";
        if lang == LANGUAGE_VIETNAMESE { return "vi_VN"; }
        // Java: case LANGUAGE_WELSH:
        // Java: return "cy_GB";
        if lang == LANGUAGE_WELSH { return "cy_GB"; }
        // Java: case LANGUAGE_WOLOF_SENEGAL:
        // Java: return "wo_SN";
        if lang == LANGUAGE_WOLOF_SENEGAL { return "wo_SN"; }
        // Java: case LANGUAGE_XHOSA:
        // Java: return "xh_ZA";
        if lang == LANGUAGE_XHOSA { return "xh_ZA"; }
        // Java: case LANGUAGE_YAKUT_RUSSIA:
        // Java: return "sah_RU";
        if lang == LANGUAGE_YAKUT_RUSSIA { return "sah_RU"; }
        // Java: case LANGUAGE_YI:
        // Java: return "ii_CN";
        if lang == LANGUAGE_YI { return "ii_CN"; }
        // Java: case LANGUAGE_YIDDISH:
        // Java: return "ji";
        if lang == LANGUAGE_YIDDISH { return "ji"; }
        // Java: case LANGUAGE_YORUBA:
        // Java: return "yo_YO";
        if lang == LANGUAGE_YORUBA { return "yo_YO"; }
        // Java: case LANGUAGE_ZULU:
        // Java: return "zu_ZA";
        if lang == LANGUAGE_ZULU { return "zu_ZA"; }
        // Java: default:
        // Java: return "unknown";
        "unknown"
    }

    // Java: static LanguageType getLanguageType(String code)
    pub fn getLanguageType(code: &str) -> LanguageType {
        // Java: if (m == null)
        let m = M.get_or_init(|| {
            // Java: m = new HashMap();
            let mut map = HashMap::new();
            // Java: m.put("sq_AL", LanguageType.LANGUAGE_ALBANIAN);
            map.insert("sq_AL", LANGUAGE_ALBANIAN);
            // Java: m.put("ar_DZ", LanguageType.LANGUAGE_ARABIC_ALGERIA);
            map.insert("ar_DZ", LANGUAGE_ARABIC_ALGERIA);
            // Java: m.put("ar_BH", LanguageType.LANGUAGE_ARABIC_BAHRAIN);
            map.insert("ar_BH", LANGUAGE_ARABIC_BAHRAIN);
            // Java: m.put("ar_EG", LanguageType.LANGUAGE_ARABIC_EGYPT);
            map.insert("ar_EG", LANGUAGE_ARABIC_EGYPT);
            // Java: m.put("ar_IQ", LanguageType.LANGUAGE_ARABIC_IRAQ);
            map.insert("ar_IQ", LANGUAGE_ARABIC_IRAQ);
            // Java: m.put("ar_JO", LanguageType.LANGUAGE_ARABIC_JORDAN);
            map.insert("ar_JO", LANGUAGE_ARABIC_JORDAN);
            // Java: m.put("ar_KW", LanguageType.LANGUAGE_ARABIC_KUWAIT);
            map.insert("ar_KW", LANGUAGE_ARABIC_KUWAIT);
            // Java: m.put("ar_LB", LanguageType.LANGUAGE_ARABIC_LEBANON);
            map.insert("ar_LB", LANGUAGE_ARABIC_LEBANON);
            // Java: m.put("ar_LY", LanguageType.LANGUAGE_ARABIC_LIBYA);
            map.insert("ar_LY", LANGUAGE_ARABIC_LIBYA);
            // Java: m.put("ar_MA", LanguageType.LANGUAGE_ARABIC_MOROCCO);
            map.insert("ar_MA", LANGUAGE_ARABIC_MOROCCO);
            // Java: m.put("ar_OM", LanguageType.LANGUAGE_ARABIC_OMAN);
            map.insert("ar_OM", LANGUAGE_ARABIC_OMAN);
            // Java: m.put("ar_QA", LanguageType.LANGUAGE_ARABIC_QATAR);
            map.insert("ar_QA", LANGUAGE_ARABIC_QATAR);
            // Java: m.put("ar_SA", LanguageType.LANGUAGE_ARABIC_SAUDI_ARABIA);
            map.insert("ar_SA", LANGUAGE_ARABIC_SAUDI_ARABIA);
            // Java: m.put("ar_SY", LanguageType.LANGUAGE_ARABIC_SYRIA);
            map.insert("ar_SY", LANGUAGE_ARABIC_SYRIA);
            // Java: m.put("ar_TN", LanguageType.LANGUAGE_ARABIC_TUNISIA);
            map.insert("ar_TN", LANGUAGE_ARABIC_TUNISIA);
            // Java: m.put("ar_AE", LanguageType.LANGUAGE_ARABIC_UAE);
            map.insert("ar_AE", LANGUAGE_ARABIC_UAE);
            // Java: m.put("ar_YE", LanguageType.LANGUAGE_ARABIC_YEMEN);
            map.insert("ar_YE", LANGUAGE_ARABIC_YEMEN);
            // Java: m.put("be_BY", LanguageType.LANGUAGE_BELARUSIAN);
            map.insert("be_BY", LANGUAGE_BELARUSIAN);
            // Java: m.put("bg_BG", LanguageType.LANGUAGE_BULGARIAN);
            map.insert("bg_BG", LANGUAGE_BULGARIAN);
            // Java: m.put("ca_ES", LanguageType.LANGUAGE_CATALAN);
            map.insert("ca_ES", LANGUAGE_CATALAN);
            // Java: m.put("zh_HK", LanguageType.LANGUAGE_CHINESE_HONGKONG);
            map.insert("zh_HK", LANGUAGE_CHINESE_HONGKONG);
            // Java: m.put("zh_MO", LanguageType.LANGUAGE_CHINESE_MACAU);
            map.insert("zh_MO", LANGUAGE_CHINESE_MACAU);
            // Java: m.put("zh_CN", LanguageType.LANGUAGE_CHINESE_SIMPLIFIED);
            map.insert("zh_CN", LANGUAGE_CHINESE_SIMPLIFIED);
            // Java: m.put("zh_SP", LanguageType.LANGUAGE_CHINESE_SINGAPORE);
            map.insert("zh_SP", LANGUAGE_CHINESE_SINGAPORE);
            // Java: m.put("zh_TW", LanguageType.LANGUAGE_CHINESE_TRADITIONAL);
            map.insert("zh_TW", LANGUAGE_CHINESE_TRADITIONAL);
            // Java: m.put("hr_BA", LanguageType.LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA);
            map.insert("hr_BA", LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA);
            // Java: m.put("cs_CZ", LanguageType.LANGUAGE_CZECH);
            map.insert("cs_CZ", LANGUAGE_CZECH);
            // Java: m.put("da_DK", LanguageType.LANGUAGE_DANISH);
            map.insert("da_DK", LANGUAGE_DANISH);
            // Java: m.put("nl_NL", LanguageType.LANGUAGE_DUTCH);
            map.insert("nl_NL", LANGUAGE_DUTCH);
            // Java: m.put("nl_BE", LanguageType.LANGUAGE_DUTCH_BELGIAN);
            map.insert("nl_BE", LANGUAGE_DUTCH_BELGIAN);
            // Java: m.put("en_AU", LanguageType.LANGUAGE_ENGLISH_AUS);
            map.insert("en_AU", LANGUAGE_ENGLISH_AUS);
            // Java: m.put("en_CA", LanguageType.LANGUAGE_ENGLISH_CAN);
            map.insert("en_CA", LANGUAGE_ENGLISH_CAN);
            // Java: m.put("en_IN", LanguageType.LANGUAGE_ENGLISH_INDIA);
            map.insert("en_IN", LANGUAGE_ENGLISH_INDIA);
            // Java: m.put("en_NZ", LanguageType.LANGUAGE_ENGLISH_NZ);
            map.insert("en_NZ", LANGUAGE_ENGLISH_NZ);
            // Java: m.put("en_ZA", LanguageType.LANGUAGE_ENGLISH_SAFRICA);
            map.insert("en_ZA", LANGUAGE_ENGLISH_SAFRICA);
            // Java: m.put("en_GB", LanguageType.LANGUAGE_ENGLISH_UK);
            map.insert("en_GB", LANGUAGE_ENGLISH_UK);
            // Java: m.put("en_US", LanguageType.LANGUAGE_ENGLISH_US);
            map.insert("en_US", LANGUAGE_ENGLISH_US);
            // Java: m.put("et_EE", LanguageType.LANGUAGE_ESTONIAN);
            map.insert("et_EE", LANGUAGE_ESTONIAN);
            // Java: m.put("fi_FI", LanguageType.LANGUAGE_FINNISH);
            map.insert("fi_FI", LANGUAGE_FINNISH);
            // Java: m.put("fr_FR", LanguageType.LANGUAGE_FRENCH);
            map.insert("fr_FR", LANGUAGE_FRENCH);
            // Java: m.put("fr_BE", LanguageType.LANGUAGE_FRENCH_BELGIAN);
            map.insert("fr_BE", LANGUAGE_FRENCH_BELGIAN);
            // Java: m.put("fr_CA", LanguageType.LANGUAGE_FRENCH_CANADIAN);
            map.insert("fr_CA", LANGUAGE_FRENCH_CANADIAN);
            // Java: m.put("fr_LU", LanguageType.LANGUAGE_FRENCH_LUXEMBOURG);
            map.insert("fr_LU", LANGUAGE_FRENCH_LUXEMBOURG);
            // Java: m.put("fr_CH", LanguageType.LANGUAGE_FRENCH_SWISS);
            map.insert("fr_CH", LANGUAGE_FRENCH_SWISS);
            // Java: m.put("de_DE", LanguageType.LANGUAGE_GERMAN);
            map.insert("de_DE", LANGUAGE_GERMAN);
            // Java: m.put("de_AT", LanguageType.LANGUAGE_GERMAN_AUSTRIAN);
            map.insert("de_AT", LANGUAGE_GERMAN_AUSTRIAN);
            // Java: m.put("de_LU", LanguageType.LANGUAGE_GERMAN_LUXEMBOURG);
            map.insert("de_LU", LANGUAGE_GERMAN_LUXEMBOURG);
            // Java: m.put("de_CH", LanguageType.LANGUAGE_GERMAN_SWISS);
            map.insert("de_CH", LANGUAGE_GERMAN_SWISS);
            // Java: m.put("el_GR", LanguageType.LANGUAGE_GREEK);
            map.insert("el_GR", LANGUAGE_GREEK);
            // Java: m.put("iw_IL", LanguageType.LANGUAGE_HEBREW);
            map.insert("iw_IL", LANGUAGE_HEBREW);
            // Java: m.put("hi_IN", LanguageType.LANGUAGE_HINDI);
            map.insert("hi_IN", LANGUAGE_HINDI);
            // Java: m.put("hu_HU", LanguageType.LANGUAGE_HUNGARIAN);
            map.insert("hu_HU", LANGUAGE_HUNGARIAN);
            // Java: m.put("is_IS", LanguageType.LANGUAGE_ICELANDIC);
            map.insert("is_IS", LANGUAGE_ICELANDIC);
            // Java: m.put("it_IT", LanguageType.LANGUAGE_ITALIAN);
            map.insert("it_IT", LANGUAGE_ITALIAN);
            // Java: m.put("it_CH", LanguageType.LANGUAGE_ITALIAN_SWISS);
            map.insert("it_CH", LANGUAGE_ITALIAN_SWISS);
            // Java: m.put("ja_JP", LanguageType.LANGUAGE_JAPANESE);
            map.insert("ja_JP", LANGUAGE_JAPANESE);
            // Java: m.put("ko_KR", LanguageType.LANGUAGE_KOREAN);
            map.insert("ko_KR", LANGUAGE_KOREAN);
            // Java: m.put("lv_LV", LanguageType.LANGUAGE_LATVIAN);
            map.insert("lv_LV", LANGUAGE_LATVIAN);
            // Java: m.put("lt_LT", LanguageType.LANGUAGE_LITHUANIAN);
            map.insert("lt_LT", LANGUAGE_LITHUANIAN);
            // Java: m.put("mk_MK", LanguageType.LANGUAGE_MACEDONIAN);
            map.insert("mk_MK", LANGUAGE_MACEDONIAN);
            // Java: m.put("no_NO", LanguageType.LANGUAGE_NORWEGIAN_BOKMAL);
            map.insert("no_NO", LANGUAGE_NORWEGIAN_BOKMAL);
            // Java: m.put("no_NO_NY", LanguageType.LANGUAGE_NORWEGIAN_NYNORSK);
            map.insert("no_NO_NY", LANGUAGE_NORWEGIAN_NYNORSK);
            // Java: m.put("pl_PL", LanguageType.LANGUAGE_POLISH);
            map.insert("pl_PL", LANGUAGE_POLISH);
            // Java: m.put("pt_PT", LanguageType.LANGUAGE_PORTUGUESE);
            map.insert("pt_PT", LANGUAGE_PORTUGUESE);
            // Java: m.put("pt_BR", LanguageType.LANGUAGE_PORTUGUESE_BRAZILIAN);
            map.insert("pt_BR", LANGUAGE_PORTUGUESE_BRAZILIAN);
            // Java: m.put("ro_RO", LanguageType.LANGUAGE_ROMANIAN);
            map.insert("ro_RO", LANGUAGE_ROMANIAN);
            // Java: m.put("ru_RU", LanguageType.LANGUAGE_RUSSIAN);
            map.insert("ru_RU", LANGUAGE_RUSSIAN);
            // Java: m.put("sr_YU", LanguageType.LANGUAGE_SERBIAN_CYRILLIC);
            map.insert("sr_YU", LANGUAGE_SERBIAN_CYRILLIC);
            // Java: m.put("sk_SK", LanguageType.LANGUAGE_SLOVAK);
            map.insert("sk_SK", LANGUAGE_SLOVAK);
            // Java: m.put("sl_SI", LanguageType.LANGUAGE_SLOVENIAN);
            map.insert("sl_SI", LANGUAGE_SLOVENIAN);
            // Java: m.put("es_AR", LanguageType.LANGUAGE_SPANISH_ARGENTINA);
            map.insert("es_AR", LANGUAGE_SPANISH_ARGENTINA);
            // Java: m.put("es_BO", LanguageType.LANGUAGE_SPANISH_BOLIVIA);
            map.insert("es_BO", LANGUAGE_SPANISH_BOLIVIA);
            // Java: m.put("es_CL", LanguageType.LANGUAGE_SPANISH_CHILE);
            map.insert("es_CL", LANGUAGE_SPANISH_CHILE);
            // Java: m.put("es_CO", LanguageType.LANGUAGE_SPANISH_COLOMBIA);
            map.insert("es_CO", LANGUAGE_SPANISH_COLOMBIA);
            // Java: m.put("es_CR", LanguageType.LANGUAGE_SPANISH_COSTARICA);
            map.insert("es_CR", LANGUAGE_SPANISH_COSTARICA);
            // Java: m.put("es_DO", LanguageType.LANGUAGE_SPANISH_DOMINICAN_REPUBLIC);
            map.insert("es_DO", LANGUAGE_SPANISH_DOMINICAN_REPUBLIC);
            // Java: m.put("es_EC", LanguageType.LANGUAGE_SPANISH_ECUADOR);
            map.insert("es_EC", LANGUAGE_SPANISH_ECUADOR);
            // Java: m.put("es_SV", LanguageType.LANGUAGE_SPANISH_EL_SALVADOR);
            map.insert("es_SV", LANGUAGE_SPANISH_EL_SALVADOR);
            // Java: m.put("es_GT", LanguageType.LANGUAGE_SPANISH_GUATEMALA);
            map.insert("es_GT", LANGUAGE_SPANISH_GUATEMALA);
            // Java: m.put("es_HN", LanguageType.LANGUAGE_SPANISH_HONDURAS);
            map.insert("es_HN", LANGUAGE_SPANISH_HONDURAS);
            // Java: m.put("es_MX", LanguageType.LANGUAGE_SPANISH_MEXICAN);
            map.insert("es_MX", LANGUAGE_SPANISH_MEXICAN);
            // Java: m.put("es_NI", LanguageType.LANGUAGE_SPANISH_NICARAGUA);
            map.insert("es_NI", LANGUAGE_SPANISH_NICARAGUA);
            // Java: m.put("es_PA", LanguageType.LANGUAGE_SPANISH_PANAMA);
            map.insert("es_PA", LANGUAGE_SPANISH_PANAMA);
            // Java: m.put("es_PY", LanguageType.LANGUAGE_SPANISH_PARAGUAY);
            map.insert("es_PY", LANGUAGE_SPANISH_PARAGUAY);
            // Java: m.put("es_PE", LanguageType.LANGUAGE_SPANISH_PERU);
            map.insert("es_PE", LANGUAGE_SPANISH_PERU);
            // Java: m.put("es_PR", LanguageType.LANGUAGE_SPANISH_PUERTO_RICO);
            map.insert("es_PR", LANGUAGE_SPANISH_PUERTO_RICO);
            // Java: m.put("es_UY", LanguageType.LANGUAGE_SPANISH_URUGUAY);
            map.insert("es_UY", LANGUAGE_SPANISH_URUGUAY);
            // Java: m.put("es_VE", LanguageType.LANGUAGE_SPANISH_VENEZUELA);
            map.insert("es_VE", LANGUAGE_SPANISH_VENEZUELA);
            // Java: m.put("es_ES", LanguageType.LANGUAGE_SPANISH);
            map.insert("es_ES", LANGUAGE_SPANISH);
            // Java: m.put("sv_SE", LanguageType.LANGUAGE_SWEDISH);
            map.insert("sv_SE", LANGUAGE_SWEDISH);
            // Java: m.put("th_TH", LanguageType.LANGUAGE_THAI);
            map.insert("th_TH", LANGUAGE_THAI);
            // Java: m.put("tr_TR", LanguageType.LANGUAGE_TURKISH);
            map.insert("tr_TR", LANGUAGE_TURKISH);
            // Java: m.put("uk_UA", LanguageType.LANGUAGE_UKRAINIAN);
            map.insert("uk_UA", LANGUAGE_UKRAINIAN);
            // Java: m.put("vi_VN", LanguageType.LANGUAGE_VIETNAMESE);
            map.insert("vi_VN", LANGUAGE_VIETNAMESE);
            // Java: m.put("yo_yo", LanguageType.LANGUAGE_YORUBA);
            map.insert("yo_yo", LANGUAGE_YORUBA);
            // Java: m.put("hy_AM", LanguageType.LANGUAGE_ARMENIAN);
            map.insert("hy_AM", LANGUAGE_ARMENIAN);
            // Java: m.put("am_ET", LanguageType.LANGUAGE_AMHARIC_ETHIOPIA);
            map.insert("am_ET", LANGUAGE_AMHARIC_ETHIOPIA);
            // Java: m.put("bn_IN", LanguageType.LANGUAGE_BENGALI);
            map.insert("bn_IN", LANGUAGE_BENGALI);
            // Java: m.put("bn_BD", LanguageType.LANGUAGE_BENGALI_BANGLADESH);
            map.insert("bn_BD", LANGUAGE_BENGALI_BANGLADESH);
            // Java: m.put("bs_BA", LanguageType.LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA);
            map.insert("bs_BA", LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA);
            // Java: m.put("br_FR", LanguageType.LANGUAGE_BRETON_FRANCE);
            map.insert("br_FR", LANGUAGE_BRETON_FRANCE);
            // Java: m.put("en_JM", LanguageType.LANGUAGE_ENGLISH_JAMAICA);
            map.insert("en_JM", LANGUAGE_ENGLISH_JAMAICA);
            // Java: m.put("en_PH", LanguageType.LANGUAGE_ENGLISH_PHILIPPINES);
            map.insert("en_PH", LANGUAGE_ENGLISH_PHILIPPINES);
            // Java: m.put("en_ID", LanguageType.LANGUAGE_ENGLISH_INDONESIA);
            map.insert("en_ID", LANGUAGE_ENGLISH_INDONESIA);
            // Java: m.put("en_SG", LanguageType.LANGUAGE_ENGLISH_SINGAPORE);
            map.insert("en_SG", LANGUAGE_ENGLISH_SINGAPORE);
            // Java: m.put("en_TT", LanguageType.LANGUAGE_ENGLISH_TRINIDAD);
            map.insert("en_TT", LANGUAGE_ENGLISH_TRINIDAD);
            // Java: m.put("en_ZW", LanguageType.LANGUAGE_ENGLISH_ZIMBABWE);
            map.insert("en_ZW", LANGUAGE_ENGLISH_ZIMBABWE);
            // Java: m.put("af_ZA", LanguageType.LANGUAGE_AFRIKAANS);
            map.insert("af_ZA", LANGUAGE_AFRIKAANS);
            // Java: m.put("gsw_FR", LanguageType.LANGUAGE_ALSATIAN_FRANCE);
            map.insert("gsw_FR", LANGUAGE_ALSATIAN_FRANCE);
            // Java: m.put("as_IN", LanguageType.LANGUAGE_ASSAMESE);
            map.insert("as_IN", LANGUAGE_ASSAMESE);
            // Java: m.put("az_Cyrl", LanguageType.LANGUAGE_AZERI_CYRILLIC);
            map.insert("az_Cyrl", LANGUAGE_AZERI_CYRILLIC);
            // Java: m.put("az_AZ", LanguageType.LANGUAGE_AZERI_LATIN);
            map.insert("az_AZ", LANGUAGE_AZERI_LATIN);
            // Java: m.put("ba_RU", LanguageType.LANGUAGE_BASHKIR_RUSSIA);
            map.insert("ba_RU", LANGUAGE_BASHKIR_RUSSIA);
            // Java: m.put("eu_ES", LanguageType.LANGUAGE_BASQUE);
            map.insert("eu_ES", LANGUAGE_BASQUE);
            // Java: m.put("my_MM", LanguageType.LANGUAGE_BURMESE);
            map.insert("my_MM", LANGUAGE_BURMESE);
            // Java: m.put("chr_US", LanguageType.LANGUAGE_CHEROKEE_UNITED_STATES);
            map.insert("chr_US", LANGUAGE_CHEROKEE_UNITED_STATES);
            // Java: m.put("fa_AF", LanguageType.LANGUAGE_DARI_AFGHANISTAN);
            map.insert("fa_AF", LANGUAGE_DARI_AFGHANISTAN);
            // Java: m.put("dv_DV", LanguageType.LANGUAGE_DHIVEHI);
            map.insert("dv_DV", LANGUAGE_DHIVEHI);
            // Java: m.put("en_BZ", LanguageType.LANGUAGE_ENGLISH_BELIZE);
            map.insert("en_BZ", LANGUAGE_ENGLISH_BELIZE);
            // Java: m.put("en_IE", LanguageType.LANGUAGE_ENGLISH_EIRE);
            map.insert("en_IE", LANGUAGE_ENGLISH_EIRE);
            // Java: m.put("en_HK", LanguageType.LANGUAGE_ENGLISH_HONG_KONG_SAR);
            map.insert("en_HK", LANGUAGE_ENGLISH_HONG_KONG_SAR);
            // Java: m.put("fo_FO", LanguageType.LANGUAGE_FAEROESE);
            map.insert("fo_FO", LANGUAGE_FAEROESE);
            // Java: m.put("fa_IR", LanguageType.LANGUAGE_FARSI);
            map.insert("fa_IR", LANGUAGE_FARSI);
            // Java: m.put("fil_PH", LanguageType.LANGUAGE_FILIPINO);
            map.insert("fil_PH", LANGUAGE_FILIPINO);
            // Java: m.put("fr_CI", LanguageType.LANGUAGE_FRENCH_COTE_D_IVOIRE);
            map.insert("fr_CI", LANGUAGE_FRENCH_COTE_D_IVOIRE);
            // Java: m.put("fy_NL", LanguageType.LANGUAGE_FRISIAN_NETHERLANDS);
            map.insert("fy_NL", LANGUAGE_FRISIAN_NETHERLANDS);
            // Java: m.put("gd_IE", LanguageType.LANGUAGE_GAELIC_IRELAND);
            map.insert("gd_IE", LANGUAGE_GAELIC_IRELAND);
            // Java: m.put("gd_GB", LanguageType.LANGUAGE_GAELIC_SCOTLAND);
            map.insert("gd_GB", LANGUAGE_GAELIC_SCOTLAND);
            // Java: m.put("gl_ES", LanguageType.LANGUAGE_GALICIAN);
            map.insert("gl_ES", LANGUAGE_GALICIAN);
            // Java: m.put("ka_GE", LanguageType.LANGUAGE_GEORGIAN);
            map.insert("ka_GE", LANGUAGE_GEORGIAN);
            // Java: m.put("gn_PY", LanguageType.LANGUAGE_GUARANI_PARAGUAY);
            map.insert("gn_PY", LANGUAGE_GUARANI_PARAGUAY);
            // Java: m.put("gu_IN", LanguageType.LANGUAGE_GUJARATI);
            map.insert("gu_IN", LANGUAGE_GUJARATI);
            // Java: m.put("ha_NE", LanguageType.LANGUAGE_HAUSA_NIGERIA);
            map.insert("ha_NE", LANGUAGE_HAUSA_NIGERIA);
            // Java: m.put("haw_US", LanguageType.LANGUAGE_HAWAIIAN_UNITED_STATES);
            map.insert("haw_US", LANGUAGE_HAWAIIAN_UNITED_STATES);
            // Java: m.put("ibb_NE", LanguageType.LANGUAGE_IBIBIO_NIGERIA);
            map.insert("ibb_NE", LANGUAGE_IBIBIO_NIGERIA);
            // Java: m.put("ig_NE", LanguageType.LANGUAGE_IGBO_NIGERIA);
            map.insert("ig_NE", LANGUAGE_IGBO_NIGERIA);
            // Java: m.put("id_ID", LanguageType.LANGUAGE_INDONESIAN);
            map.insert("id_ID", LANGUAGE_INDONESIAN);
            // Java: m.put("iu_CA", LanguageType.LANGUAGE_INUKTITUT_SYLLABICS_CANADA);
            map.insert("iu_CA", LANGUAGE_INUKTITUT_SYLLABICS_CANADA);
            // Java: m.put("kl_GL", LanguageType.LANGUAGE_KALAALLISUT_GREENLAND);
            map.insert("kl_GL", LANGUAGE_KALAALLISUT_GREENLAND);
            // Java: m.put("kn_IN", LanguageType.LANGUAGE_KANNADA);
            map.insert("kn_IN", LANGUAGE_KANNADA);
            // Java: m.put("kr_NE", LanguageType.LANGUAGE_KANURI_NIGERIA);
            map.insert("kr_NE", LANGUAGE_KANURI_NIGERIA);
            // Java: m.put("ks_KS", LanguageType.LANGUAGE_KASHMIRI);
            map.insert("ks_KS", LANGUAGE_KASHMIRI);
            // Java: m.put("ks_IN", LanguageType.LANGUAGE_KASHMIRI_INDIA);
            map.insert("ks_IN", LANGUAGE_KASHMIRI_INDIA);
            // Java: m.put("kk_KZ", LanguageType.LANGUAGE_KAZAK);
            map.insert("kk_KZ", LANGUAGE_KAZAK);
            // Java: m.put("km_KH", LanguageType.LANGUAGE_KHMER);
            map.insert("km_KH", LANGUAGE_KHMER);
            // Java: m.put("quc_GT", LanguageType.LANGUAGE_KICHE_GUATEMALA);
            map.insert("quc_GT", LANGUAGE_KICHE_GUATEMALA);
            // Java: m.put("rw_RW", LanguageType.LANGUAGE_KINYARWANDA_RWANDA);
            map.insert("rw_RW", LANGUAGE_KINYARWANDA_RWANDA);
            // Java: m.put("ky_KG", LanguageType.LANGUAGE_KIRGHIZ);
            map.insert("ky_KG", LANGUAGE_KIRGHIZ);
            // Java: m.put("kok_IN", LanguageType.LANGUAGE_KONKANI);
            map.insert("kok_IN", LANGUAGE_KONKANI);
            // Java: m.put("lo_LA", LanguageType.LANGUAGE_LAO);
            map.insert("lo_LA", LANGUAGE_LAO);
            // Java: m.put("lb_LU", LanguageType.LANGUAGE_LUXEMBOURGISH_LUXEMBOURG);
            map.insert("lb_LU", LANGUAGE_LUXEMBOURGISH_LUXEMBOURG);
            // Java: m.put("ms_BN", LanguageType.LANGUAGE_MALAY_BRUNEI_DARUSSALAM);
            map.insert("ms_BN", LANGUAGE_MALAY_BRUNEI_DARUSSALAM);
            // Java: m.put("ms_MY", LanguageType.LANGUAGE_MALAY_MALAYSIA);
            map.insert("ms_MY", LANGUAGE_MALAY_MALAYSIA);
            // Java: m.put("mt_MT", LanguageType.LANGUAGE_MALTESE);
            map.insert("mt_MT", LANGUAGE_MALTESE);
            // Java: m.put("mni_IN", LanguageType.LANGUAGE_MANIPURI);
            map.insert("mni_IN", LANGUAGE_MANIPURI);
            // Java: m.put("mi_NZ", LanguageType.LANGUAGE_MAORI_NEW_ZEALAND);
            map.insert("mi_NZ", LANGUAGE_MAORI_NEW_ZEALAND);
            // Java: m.put("arn_CL", LanguageType.LANGUAGE_MAPUDUNGUN_CHILE);
            map.insert("arn_CL", LANGUAGE_MAPUDUNGUN_CHILE);
            // Java: m.put("mr_IN", LanguageType.LANGUAGE_MARATHI);
            map.insert("mr_IN", LANGUAGE_MARATHI);
            // Java: m.put("moh_CA", LanguageType.LANGUAGE_MOHAWK_CANADA);
            map.insert("moh_CA", LANGUAGE_MOHAWK_CANADA);
            // Java: m.put("mn_MN", LanguageType.LANGUAGE_MONGOLIAN_MONGOLIAN);
            map.insert("mn_MN", LANGUAGE_MONGOLIAN_MONGOLIAN);
            // Java: m.put("ne_NP", LanguageType.LANGUAGE_NEPALI);
            map.insert("ne_NP", LANGUAGE_NEPALI);
            // Java: m.put("ne_IN", LanguageType.LANGUAGE_NEPALI_INDIA);
            map.insert("ne_IN", LANGUAGE_NEPALI_INDIA);
            // Java: m.put("oc_FR", LanguageType.LANGUAGE_OCCITAN_FRANCE);
            map.insert("oc_FR", LANGUAGE_OCCITAN_FRANCE);
            // Java: m.put("or_IN", LanguageType.LANGUAGE_ORIYA);
            map.insert("or_IN", LANGUAGE_ORIYA);
            // Java: m.put("om_KE", LanguageType.LANGUAGE_OROMO);
            map.insert("om_KE", LANGUAGE_OROMO);
            // Java: m.put("pap_AW", LanguageType.LANGUAGE_PAPIAMENTU);
            map.insert("pap_AW", LANGUAGE_PAPIAMENTU);
            // Java: m.put("ps_AF", LanguageType.LANGUAGE_PASHTO);
            map.insert("ps_AF", LANGUAGE_PASHTO);
            // Java: m.put("pa_IN", LanguageType.LANGUAGE_PUNJABI);
            map.insert("pa_IN", LANGUAGE_PUNJABI);
            // Java: m.put("pa_PK", LanguageType.LANGUAGE_PUNJABI_PAKISTAN);
            map.insert("pa_PK", LANGUAGE_PUNJABI_PAKISTAN);
            // Java: m.put("quz_BO", LanguageType.LANGUAGE_QUECHUA_BOLIVIA);
            map.insert("quz_BO", LANGUAGE_QUECHUA_BOLIVIA);
            // Java: m.put("quz_EC", LanguageType.LANGUAGE_QUECHUA_ECUADOR);
            map.insert("quz_EC", LANGUAGE_QUECHUA_ECUADOR);
            // Java: m.put("quz_PE", LanguageType.LANGUAGE_QUECHUA_PERU);
            map.insert("quz_PE", LANGUAGE_QUECHUA_PERU);
            // Java: m.put("rm_RM", LanguageType.LANGUAGE_RHAETO_ROMAN);
            map.insert("rm_RM", LANGUAGE_RHAETO_ROMAN);
            // Java: m.put("ro_MD", LanguageType.LANGUAGE_ROMANIAN_MOLDOVA);
            map.insert("ro_MD", LANGUAGE_ROMANIAN_MOLDOVA);
            // Java: m.put("ru_MD", LanguageType.LANGUAGE_RUSSIAN_MOLDOVA);
            map.insert("ru_MD", LANGUAGE_RUSSIAN_MOLDOVA);
            // Java: m.put("se_NO", LanguageType.LANGUAGE_SAMI_NORTHERN_NORWAY);
            map.insert("se_NO", LANGUAGE_SAMI_NORTHERN_NORWAY);
            // Java: m.put("sz", LanguageType.LANGUAGE_SAMI_LAPPISH);
            map.insert("sz", LANGUAGE_SAMI_LAPPISH);
            // Java: m.put("smn_FL", LanguageType.LANGUAGE_SAMI_INARI);
            map.insert("smn_FL", LANGUAGE_SAMI_INARI);
            // Java: m.put("smj_NO", LanguageType.LANGUAGE_SAMI_LULE_NORWAY);
            map.insert("smj_NO", LANGUAGE_SAMI_LULE_NORWAY);
            // Java: m.put("smj_SE", LanguageType.LANGUAGE_SAMI_LULE_SWEDEN);
            map.insert("smj_SE", LANGUAGE_SAMI_LULE_SWEDEN);
            // Java: m.put("se_FI", LanguageType.LANGUAGE_SAMI_NORTHERN_FINLAND);
            map.insert("se_FI", LANGUAGE_SAMI_NORTHERN_FINLAND);
            // Java: m.put("se_SE", LanguageType.LANGUAGE_SAMI_NORTHERN_SWEDEN);
            map.insert("se_SE", LANGUAGE_SAMI_NORTHERN_SWEDEN);
            // Java: m.put("sms_FI", LanguageType.LANGUAGE_SAMI_SKOLT);
            map.insert("sms_FI", LANGUAGE_SAMI_SKOLT);
            // Java: m.put("sma_NO", LanguageType.LANGUAGE_SAMI_SOUTHERN_NORWAY);
            map.insert("sma_NO", LANGUAGE_SAMI_SOUTHERN_NORWAY);
            // Java: m.put("sma_SE", LanguageType.LANGUAGE_SAMI_SOUTHERN_SWEDEN);
            map.insert("sma_SE", LANGUAGE_SAMI_SOUTHERN_SWEDEN);
            // Java: m.put("sa_IN", LanguageType.LANGUAGE_SANSKRIT);
            map.insert("sa_IN", LANGUAGE_SANSKRIT);
            // Java: m.put("nso", LanguageType.LANGUAGE_NORTHERNSOTHO);
            map.insert("nso", LANGUAGE_NORTHERNSOTHO);
            // Java: m.put("sr_BA", LanguageType.LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA);
            map.insert("sr_BA", LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA);
            // Java: m.put("nso_ZA", LanguageType.LANGUAGE_SESOTHO);
            map.insert("nso_ZA", LANGUAGE_SESOTHO);
            // Java: m.put("sd_IN", LanguageType.LANGUAGE_SINDHI);
            map.insert("sd_IN", LANGUAGE_SINDHI);
            // Java: m.put("sd_PK", LanguageType.LANGUAGE_SINDHI_PAKISTAN);
            map.insert("sd_PK", LANGUAGE_SINDHI_PAKISTAN);
            // Java: m.put("so_SO", LanguageType.LANGUAGE_SOMALI);
            map.insert("so_SO", LANGUAGE_SOMALI);
            // Java: m.put("hsb_DE", LanguageType.LANGUAGE_UPPER_SORBIAN_GERMANY);
            map.insert("hsb_DE", LANGUAGE_UPPER_SORBIAN_GERMANY);
            // Java: m.put("dsb_DE", LanguageType.LANGUAGE_LOWER_SORBIAN_GERMANY);
            map.insert("dsb_DE", LANGUAGE_LOWER_SORBIAN_GERMANY);
            // Java: m.put("es_US", LanguageType.LANGUAGE_SPANISH_UNITED_STATES);
            map.insert("es_US", LANGUAGE_SPANISH_UNITED_STATES);
            // Java: m.put("sw_KE", LanguageType.LANGUAGE_SWAHILI);
            map.insert("sw_KE", LANGUAGE_SWAHILI);
            // Java: m.put("sv_FI", LanguageType.LANGUAGE_SWEDISH_FINLAND);
            map.insert("sv_FI", LANGUAGE_SWEDISH_FINLAND);
            // Java: m.put("syr_SY", LanguageType.LANGUAGE_SYRIAC);
            map.insert("syr_SY", LANGUAGE_SYRIAC);
            // Java: m.put("tg_TJ", LanguageType.LANGUAGE_TAJIK);
            map.insert("tg_TJ", LANGUAGE_TAJIK);
            // Java: m.put("tzm", LanguageType.LANGUAGE_TAMAZIGHT_ARABIC);
            map.insert("tzm", LANGUAGE_TAMAZIGHT_ARABIC);
            // Java: m.put("tzm_Latn_DZ", LanguageType.LANGUAGE_TAMAZIGHT_LATIN);
            map.insert("tzm_Latn_DZ", LANGUAGE_TAMAZIGHT_LATIN);
            // Java: m.put("ta_IN", LanguageType.LANGUAGE_TAMIL);
            map.insert("ta_IN", LANGUAGE_TAMIL);
            // Java: m.put("tt_RU", LanguageType.LANGUAGE_TATAR);
            map.insert("tt_RU", LANGUAGE_TATAR);
            // Java: m.put("te_IN", LanguageType.LANGUAGE_TELUGU);
            map.insert("te_IN", LANGUAGE_TELUGU);
            // Java: m.put("bo_CN", LanguageType.LANGUAGE_TIBETAN);
            map.insert("bo_CN", LANGUAGE_TIBETAN);
            // Java: m.put("dz_BT", LanguageType.LANGUAGE_DZONGKHA);
            map.insert("dz_BT", LANGUAGE_DZONGKHA);
            // Java: m.put("bo_BT", LanguageType.LANGUAGE_TIBETAN_BHUTAN);
            map.insert("bo_BT", LANGUAGE_TIBETAN_BHUTAN);
            // Java: m.put("ti_ER", LanguageType.LANGUAGE_TIGRIGNA_ERITREA);
            map.insert("ti_ER", LANGUAGE_TIGRIGNA_ERITREA);
            // Java: m.put("ti_ET", LanguageType.LANGUAGE_TIGRIGNA_ETHIOPIA);
            map.insert("ti_ET", LANGUAGE_TIGRIGNA_ETHIOPIA);
            // Java: m.put("ts_ZA", LanguageType.LANGUAGE_TSONGA);
            map.insert("ts_ZA", LANGUAGE_TSONGA);
            // Java: m.put("tn_BW", LanguageType.LANGUAGE_TSWANA);
            map.insert("tn_BW", LANGUAGE_TSWANA);
            // Java: m.put("tk_TM", LanguageType.LANGUAGE_TURKMEN);
            map.insert("tk_TM", LANGUAGE_TURKMEN);
            // Java: m.put("ug_CN", LanguageType.LANGUAGE_UIGHUR_CHINA);
            map.insert("ug_CN", LANGUAGE_UIGHUR_CHINA);
            // Java: m.put("ur_PK", LanguageType.LANGUAGE_URDU_PAKISTAN);
            map.insert("ur_PK", LANGUAGE_URDU_PAKISTAN);
            // Java: m.put("ur_IN", LanguageType.LANGUAGE_URDU_INDIA);
            map.insert("ur_IN", LANGUAGE_URDU_INDIA);
            // Java: m.put("uz_UZ", LanguageType.LANGUAGE_UZBEK_CYRILLIC);
            map.insert("uz_UZ", LANGUAGE_UZBEK_CYRILLIC);
            // Java: m.put("ven_ZA", LanguageType.LANGUAGE_VENDA);
            map.insert("ven_ZA", LANGUAGE_VENDA);
            // Java: m.put("cy_GB", LanguageType.LANGUAGE_WELSH);
            map.insert("cy_GB", LANGUAGE_WELSH);
            // Java: m.put("wo_SN", LanguageType.LANGUAGE_WOLOF_SENEGAL);
            map.insert("wo_SN", LANGUAGE_WOLOF_SENEGAL);
            // Java: m.put("xh_ZA", LanguageType.LANGUAGE_XHOSA);
            map.insert("xh_ZA", LANGUAGE_XHOSA);
            // Java: m.put("sah_RU", LanguageType.LANGUAGE_YAKUT_RUSSIA);
            map.insert("sah_RU", LANGUAGE_YAKUT_RUSSIA);
            // Java: m.put("ii_CN", LanguageType.LANGUAGE_YI);
            map.insert("ii_CN", LANGUAGE_YI);
            // Java: m.put("zu_ZA", LanguageType.LANGUAGE_ZULU);
            map.insert("zu_ZA", LANGUAGE_ZULU);
            // Java: m.put("ji", LanguageType.LANGUAGE_YIDDISH);
            map.insert("ji", LANGUAGE_YIDDISH);
            // Java: m.put("de_LI", LanguageType.LANGUAGE_GERMAN_LIECHTENSTEIN);
            map.insert("de_LI", LANGUAGE_GERMAN_LIECHTENSTEIN);
            // Java: m.put("fr_ZR", LanguageType.LANGUAGE_FRENCH_ZAIRE);
            map.insert("fr_ZR", LANGUAGE_FRENCH_ZAIRE);
            // Java: m.put("fr_SN", LanguageType.LANGUAGE_FRENCH_SENEGAL);
            map.insert("fr_SN", LANGUAGE_FRENCH_SENEGAL);
            // Java: m.put("fr_RE", LanguageType.LANGUAGE_FRENCH_REUNION);
            map.insert("fr_RE", LANGUAGE_FRENCH_REUNION);
            // Java: m.put("fr_MA", LanguageType.LANGUAGE_FRENCH_MOROCCO);
            map.insert("fr_MA", LANGUAGE_FRENCH_MOROCCO);
            // Java: m.put("fr_MC", LanguageType.LANGUAGE_FRENCH_MONACO);
            map.insert("fr_MC", LANGUAGE_FRENCH_MONACO);
            // Java: m.put("fr_ML", LanguageType.LANGUAGE_FRENCH_MALI);
            map.insert("fr_ML", LANGUAGE_FRENCH_MALI);
            // Java: m.put("fr_HT", LanguageType.LANGUAGE_FRENCH_HAITI);
            map.insert("fr_HT", LANGUAGE_FRENCH_HAITI);
            // Java: m.put("fr_CM", LanguageType.LANGUAGE_FRENCH_CAMEROON);
            map.insert("fr_CM", LANGUAGE_FRENCH_CAMEROON);
            // Java: m.put("co_FR", LanguageType.LANGUAGE_CORSICAN_FRANCE);
            map.insert("co_FR", LANGUAGE_CORSICAN_FRANCE);
            map
        });
        // Java: LanguageType ret = (LanguageType)m.get(code);
        let ret = m.get(code).copied();
        // Java: return ret == null ? LanguageType.LANGUAGE_ENGLISH_US : ret;
        ret.unwrap_or(LANGUAGE_ENGLISH_US)
    }
}
