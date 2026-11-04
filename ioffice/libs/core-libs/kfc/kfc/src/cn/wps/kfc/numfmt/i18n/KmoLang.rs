#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Locale
// Java: import java.util.Map
use std::sync::{Mutex, OnceLock};

// Java field: short LANGUAGE_MASK_PRIMARY
pub(crate) const LANGUAGE_MASK_PRIMARY: i16 = 1023;
// Java field: short LANGUAGE_DONTKNOW
pub(crate) const LANGUAGE_DONTKNOW: i16 = 1023;
// Java field: short LANGUAGE_NONE
pub(crate) const LANGUAGE_NONE: i16 = 255;
// Java field: short LANGUAGE_HID_HUMAN_INTERFACE_DEVICE
pub(crate) const LANGUAGE_HID_HUMAN_INTERFACE_DEVICE: i16 = 1279;
// Java field: short LANGUAGE_SYSTEM
pub(crate) const LANGUAGE_SYSTEM: i16 = 0;
// Java field: short LANGUAGE_INVARIANT
pub(crate) const LANGUAGE_INVARIANT: i16 = 127;
// Java field: short LANGUAGE_AFRIKAANS
pub(crate) const LANGUAGE_AFRIKAANS: i16 = 1078;
// Java field: short LANGUAGE_ALBANIAN
pub(crate) const LANGUAGE_ALBANIAN: i16 = 1052;
// Java field: short LANGUAGE_ALSATIAN_FRANCE
pub(crate) const LANGUAGE_ALSATIAN_FRANCE: i16 = 1156;
// Java field: short LANGUAGE_AMHARIC_ETHIOPIA
pub(crate) const LANGUAGE_AMHARIC_ETHIOPIA: i16 = 1118;
// Java field: short LANGUAGE_ARABIC_ALGERIA
pub(crate) const LANGUAGE_ARABIC_ALGERIA: i16 = 5121;
// Java field: short LANGUAGE_ARABIC_BAHRAIN
pub(crate) const LANGUAGE_ARABIC_BAHRAIN: i16 = 15361;
// Java field: short LANGUAGE_ARABIC_EGYPT
pub(crate) const LANGUAGE_ARABIC_EGYPT: i16 = 3073;
// Java field: short LANGUAGE_ARABIC_IRAQ
pub(crate) const LANGUAGE_ARABIC_IRAQ: i16 = 2049;
// Java field: short LANGUAGE_ARABIC_JORDAN
pub(crate) const LANGUAGE_ARABIC_JORDAN: i16 = 11265;
// Java field: short LANGUAGE_ARABIC_KUWAIT
pub(crate) const LANGUAGE_ARABIC_KUWAIT: i16 = 13313;
// Java field: short LANGUAGE_ARABIC_LEBANON
pub(crate) const LANGUAGE_ARABIC_LEBANON: i16 = 12289;
// Java field: short LANGUAGE_ARABIC_LIBYA
pub(crate) const LANGUAGE_ARABIC_LIBYA: i16 = 4097;
// Java field: short LANGUAGE_ARABIC_MOROCCO
pub(crate) const LANGUAGE_ARABIC_MOROCCO: i16 = 6145;
// Java field: short LANGUAGE_ARABIC_OMAN
pub(crate) const LANGUAGE_ARABIC_OMAN: i16 = 8193;
// Java field: short LANGUAGE_ARABIC_QATAR
pub(crate) const LANGUAGE_ARABIC_QATAR: i16 = 16385;
// Java field: short LANGUAGE_ARABIC_SAUDI_ARABIA
pub(crate) const LANGUAGE_ARABIC_SAUDI_ARABIA: i16 = 1025;
// Java field: short LANGUAGE_ARABIC_SYRIA
pub(crate) const LANGUAGE_ARABIC_SYRIA: i16 = 10241;
// Java field: short LANGUAGE_ARABIC_TUNISIA
pub(crate) const LANGUAGE_ARABIC_TUNISIA: i16 = 7169;
// Java field: short LANGUAGE_ARABIC_UAE
pub(crate) const LANGUAGE_ARABIC_UAE: i16 = 14337;
// Java field: short LANGUAGE_ARABIC_YEMEN
pub(crate) const LANGUAGE_ARABIC_YEMEN: i16 = 9217;
// Java field: short LANGUAGE_ARABIC_PRIMARY_ONLY
pub(crate) const LANGUAGE_ARABIC_PRIMARY_ONLY: i16 = 1;
// Java field: short LANGUAGE_ARMENIAN
pub(crate) const LANGUAGE_ARMENIAN: i16 = 1067;
// Java field: short LANGUAGE_ASSAMESE
pub(crate) const LANGUAGE_ASSAMESE: i16 = 1101;
// Java field: short LANGUAGE_AZERI
pub(crate) const LANGUAGE_AZERI: i16 = 44;
// Java field: short LANGUAGE_AZERI_CYRILLIC
pub(crate) const LANGUAGE_AZERI_CYRILLIC: i16 = 2092;
// Java field: short LANGUAGE_AZERI_LATIN
pub(crate) const LANGUAGE_AZERI_LATIN: i16 = 1068;
// Java field: short LANGUAGE_BASHKIR_RUSSIA
pub(crate) const LANGUAGE_BASHKIR_RUSSIA: i16 = 1133;
// Java field: short LANGUAGE_BASQUE
pub(crate) const LANGUAGE_BASQUE: i16 = 1069;
// Java field: short LANGUAGE_BELARUSIAN
pub(crate) const LANGUAGE_BELARUSIAN: i16 = 1059;
// Java field: short LANGUAGE_BENGALI
pub(crate) const LANGUAGE_BENGALI: i16 = 1093;
// Java field: short LANGUAGE_BENGALI_BANGLADESH
pub(crate) const LANGUAGE_BENGALI_BANGLADESH: i16 = 2117;
// Java field: short LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA: i16 = 5146;
// Java field: short LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA: i16 = 8218;
// Java field: short LANGUAGE_BOSNIAN_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_BOSNIAN_BOSNIA_HERZEGOVINA: i16 = 5146;
// Java field: short LANGUAGE_BRETON_FRANCE
pub(crate) const LANGUAGE_BRETON_FRANCE: i16 = 1150;
// Java field: short LANGUAGE_BULGARIAN
pub(crate) const LANGUAGE_BULGARIAN: i16 = 1026;
// Java field: short LANGUAGE_BURMESE
pub(crate) const LANGUAGE_BURMESE: i16 = 1109;
// Java field: short LANGUAGE_CATALAN
pub(crate) const LANGUAGE_CATALAN: i16 = 1027;
// Java field: short LANGUAGE_CHEROKEE_UNITED_STATES
pub(crate) const LANGUAGE_CHEROKEE_UNITED_STATES: i16 = 1116;
// Java field: short LANGUAGE_CHINESE
pub(crate) const LANGUAGE_CHINESE: i16 = 4;
// Java field: short LANGUAGE_CHINESE_HONGKONG
pub(crate) const LANGUAGE_CHINESE_HONGKONG: i16 = 3076;
// Java field: short LANGUAGE_CHINESE_MACAU
pub(crate) const LANGUAGE_CHINESE_MACAU: i16 = 5124;
// Java field: short LANGUAGE_CHINESE_SIMPLIFIED
pub(crate) const LANGUAGE_CHINESE_SIMPLIFIED: i16 = 2052;
// Java field: short LANGUAGE_CHINESE_SINGAPORE
pub(crate) const LANGUAGE_CHINESE_SINGAPORE: i16 = 4100;
// Java field: short LANGUAGE_CHINESE_TRADITIONAL
pub(crate) const LANGUAGE_CHINESE_TRADITIONAL: i16 = 1028;
// Java field: short LANGUAGE_CORSICAN_FRANCE
pub(crate) const LANGUAGE_CORSICAN_FRANCE: i16 = 1155;
// Java field: short LANGUAGE_CROATIAN
pub(crate) const LANGUAGE_CROATIAN: i16 = 1050;
// Java field: short LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA: i16 = 4122;
// Java field: short LANGUAGE_CZECH
pub(crate) const LANGUAGE_CZECH: i16 = 1029;
// Java field: short LANGUAGE_DANISH
pub(crate) const LANGUAGE_DANISH: i16 = 1030;
// Java field: short LANGUAGE_DARI_AFGHANISTAN
pub(crate) const LANGUAGE_DARI_AFGHANISTAN: i16 = 1164;
// Java field: short LANGUAGE_DHIVEHI
pub(crate) const LANGUAGE_DHIVEHI: i16 = 1125;
// Java field: short LANGUAGE_DUTCH
pub(crate) const LANGUAGE_DUTCH: i16 = 1043;
// Java field: short LANGUAGE_DUTCH_BELGIAN
pub(crate) const LANGUAGE_DUTCH_BELGIAN: i16 = 2067;
// Java field: short LANGUAGE_EDO
pub(crate) const LANGUAGE_EDO: i16 = 1126;
// Java field: short LANGUAGE_ENGLISH
pub(crate) const LANGUAGE_ENGLISH: i16 = 9;
// Java field: short LANGUAGE_ENGLISH_AUS
pub(crate) const LANGUAGE_ENGLISH_AUS: i16 = 3081;
// Java field: short LANGUAGE_ENGLISH_BELIZE
pub(crate) const LANGUAGE_ENGLISH_BELIZE: i16 = 10249;
// Java field: short LANGUAGE_ENGLISH_CAN
pub(crate) const LANGUAGE_ENGLISH_CAN: i16 = 4105;
// Java field: short LANGUAGE_ENGLISH_CARRIBEAN
pub(crate) const LANGUAGE_ENGLISH_CARRIBEAN: i16 = 9225;
// Java field: short LANGUAGE_ENGLISH_EIRE
pub(crate) const LANGUAGE_ENGLISH_EIRE: i16 = 6153;
// Java field: short LANGUAGE_ENGLISH_HONG_KONG_SAR
pub(crate) const LANGUAGE_ENGLISH_HONG_KONG_SAR: i16 = 15369;
// Java field: short LANGUAGE_ENGLISH_INDIA
pub(crate) const LANGUAGE_ENGLISH_INDIA: i16 = 16393;
// Java field: short LANGUAGE_ENGLISH_INDONESIA
pub(crate) const LANGUAGE_ENGLISH_INDONESIA: i16 = 14345;
// Java field: short LANGUAGE_ENGLISH_JAMAICA
pub(crate) const LANGUAGE_ENGLISH_JAMAICA: i16 = 8201;
// Java field: short LANGUAGE_ENGLISH_MALAYSIA
pub(crate) const LANGUAGE_ENGLISH_MALAYSIA: i16 = 17417;
// Java field: short LANGUAGE_ENGLISH_NZ
pub(crate) const LANGUAGE_ENGLISH_NZ: i16 = 5129;
// Java field: short LANGUAGE_ENGLISH_PHILIPPINES
pub(crate) const LANGUAGE_ENGLISH_PHILIPPINES: i16 = 13321;
// Java field: short LANGUAGE_ENGLISH_SAFRICA
pub(crate) const LANGUAGE_ENGLISH_SAFRICA: i16 = 7177;
// Java field: short LANGUAGE_ENGLISH_SINGAPORE
pub(crate) const LANGUAGE_ENGLISH_SINGAPORE: i16 = 18441;
// Java field: short LANGUAGE_ENGLISH_TRINIDAD
pub(crate) const LANGUAGE_ENGLISH_TRINIDAD: i16 = 11273;
// Java field: short LANGUAGE_ENGLISH_UK
pub(crate) const LANGUAGE_ENGLISH_UK: i16 = 2057;
// Java field: short LANGUAGE_ENGLISH_US
pub(crate) const LANGUAGE_ENGLISH_US: i16 = 1033;
// Java field: short LANGUAGE_ENGLISH_ZIMBABWE
pub(crate) const LANGUAGE_ENGLISH_ZIMBABWE: i16 = 12297;
// Java field: short LANGUAGE_ESTONIAN
pub(crate) const LANGUAGE_ESTONIAN: i16 = 1061;
// Java field: short LANGUAGE_FAEROESE
pub(crate) const LANGUAGE_FAEROESE: i16 = 1080;
// Java field: short LANGUAGE_FARSI
pub(crate) const LANGUAGE_FARSI: i16 = 1065;
// Java field: short LANGUAGE_FILIPINO
pub(crate) const LANGUAGE_FILIPINO: i16 = 1124;
// Java field: short LANGUAGE_FINNISH
pub(crate) const LANGUAGE_FINNISH: i16 = 1035;
// Java field: short LANGUAGE_FRENCH
pub(crate) const LANGUAGE_FRENCH: i16 = 1036;
// Java field: short LANGUAGE_FRENCH_BELGIAN
pub(crate) const LANGUAGE_FRENCH_BELGIAN: i16 = 2060;
// Java field: short LANGUAGE_FRENCH_CAMEROON
pub(crate) const LANGUAGE_FRENCH_CAMEROON: i16 = 11276;
// Java field: short LANGUAGE_FRENCH_CANADIAN
pub(crate) const LANGUAGE_FRENCH_CANADIAN: i16 = 3084;
// Java field: short LANGUAGE_FRENCH_COTE_D_IVOIRE
pub(crate) const LANGUAGE_FRENCH_COTE_D_IVOIRE: i16 = 12300;
// Java field: short LANGUAGE_FRENCH_HAITI
pub(crate) const LANGUAGE_FRENCH_HAITI: i16 = 15372;
// Java field: short LANGUAGE_FRENCH_LUXEMBOURG
pub(crate) const LANGUAGE_FRENCH_LUXEMBOURG: i16 = 5132;
// Java field: short LANGUAGE_FRENCH_MALI
pub(crate) const LANGUAGE_FRENCH_MALI: i16 = 13324;
// Java field: short LANGUAGE_FRENCH_MONACO
pub(crate) const LANGUAGE_FRENCH_MONACO: i16 = 6156;
// Java field: short LANGUAGE_FRENCH_MOROCCO
pub(crate) const LANGUAGE_FRENCH_MOROCCO: i16 = 14348;
// Java field: short LANGUAGE_FRENCH_NORTH_AFRICA
pub(crate) const LANGUAGE_FRENCH_NORTH_AFRICA: i16 = -7156;
// Java field: short LANGUAGE_FRENCH_REUNION
pub(crate) const LANGUAGE_FRENCH_REUNION: i16 = 8204;
// Java field: short LANGUAGE_FRENCH_SENEGAL
pub(crate) const LANGUAGE_FRENCH_SENEGAL: i16 = 10252;
// Java field: short LANGUAGE_FRENCH_SWISS
pub(crate) const LANGUAGE_FRENCH_SWISS: i16 = 4108;
// Java field: short LANGUAGE_FRENCH_WEST_INDIES
pub(crate) const LANGUAGE_FRENCH_WEST_INDIES: i16 = 7180;
// Java field: short LANGUAGE_FRENCH_ZAIRE
pub(crate) const LANGUAGE_FRENCH_ZAIRE: i16 = 9228;
// Java field: short LANGUAGE_FRISIAN_NETHERLANDS
pub(crate) const LANGUAGE_FRISIAN_NETHERLANDS: i16 = 1122;
// Java field: short LANGUAGE_FULFULDE_NIGERIA
pub(crate) const LANGUAGE_FULFULDE_NIGERIA: i16 = 1127;
// Java field: short LANGUAGE_GAELIC_IRELAND
pub(crate) const LANGUAGE_GAELIC_IRELAND: i16 = 2108;
// Java field: short LANGUAGE_GAELIC_SCOTLAND
pub(crate) const LANGUAGE_GAELIC_SCOTLAND: i16 = 1084;
// Java field: short LANGUAGE_GALICIAN
pub(crate) const LANGUAGE_GALICIAN: i16 = 1110;
// Java field: short LANGUAGE_GEORGIAN
pub(crate) const LANGUAGE_GEORGIAN: i16 = 1079;
// Java field: short LANGUAGE_GERMAN
pub(crate) const LANGUAGE_GERMAN: i16 = 1031;
// Java field: short LANGUAGE_GERMAN_AUSTRIAN
pub(crate) const LANGUAGE_GERMAN_AUSTRIAN: i16 = 3079;
// Java field: short LANGUAGE_GERMAN_LIECHTENSTEIN
pub(crate) const LANGUAGE_GERMAN_LIECHTENSTEIN: i16 = 5127;
// Java field: short LANGUAGE_GERMAN_LUXEMBOURG
pub(crate) const LANGUAGE_GERMAN_LUXEMBOURG: i16 = 4103;
// Java field: short LANGUAGE_GERMAN_SWISS
pub(crate) const LANGUAGE_GERMAN_SWISS: i16 = 2055;
// Java field: short LANGUAGE_GREEK
pub(crate) const LANGUAGE_GREEK: i16 = 1032;
// Java field: short LANGUAGE_GUARANI_PARAGUAY
pub(crate) const LANGUAGE_GUARANI_PARAGUAY: i16 = 1140;
// Java field: short LANGUAGE_GUJARATI
pub(crate) const LANGUAGE_GUJARATI: i16 = 1095;
// Java field: short LANGUAGE_HAUSA_NIGERIA
pub(crate) const LANGUAGE_HAUSA_NIGERIA: i16 = 1128;
// Java field: short LANGUAGE_HAWAIIAN_UNITED_STATES
pub(crate) const LANGUAGE_HAWAIIAN_UNITED_STATES: i16 = 1141;
// Java field: short LANGUAGE_HEBREW
pub(crate) const LANGUAGE_HEBREW: i16 = 1037;
// Java field: short LANGUAGE_HINDI
pub(crate) const LANGUAGE_HINDI: i16 = 1081;
// Java field: short LANGUAGE_HUNGARIAN
pub(crate) const LANGUAGE_HUNGARIAN: i16 = 1038;
// Java field: short LANGUAGE_IBIBIO_NIGERIA
pub(crate) const LANGUAGE_IBIBIO_NIGERIA: i16 = 1129;
// Java field: short LANGUAGE_ICELANDIC
pub(crate) const LANGUAGE_ICELANDIC: i16 = 1039;
// Java field: short LANGUAGE_IGBO_NIGERIA
pub(crate) const LANGUAGE_IGBO_NIGERIA: i16 = 1136;
// Java field: short LANGUAGE_INDONESIAN
pub(crate) const LANGUAGE_INDONESIAN: i16 = 1057;
// Java field: short LANGUAGE_INUKTITUT_SYLLABICS_CANADA
pub(crate) const LANGUAGE_INUKTITUT_SYLLABICS_CANADA: i16 = 1117;
// Java field: short LANGUAGE_INUKTITUT_LATIN_CANADA
pub(crate) const LANGUAGE_INUKTITUT_LATIN_CANADA: i16 = 2141;
// Java field: short LANGUAGE_ITALIAN
pub(crate) const LANGUAGE_ITALIAN: i16 = 1040;
// Java field: short LANGUAGE_ITALIAN_SWISS
pub(crate) const LANGUAGE_ITALIAN_SWISS: i16 = 2064;
// Java field: short LANGUAGE_JAPANESE
pub(crate) const LANGUAGE_JAPANESE: i16 = 1041;
// Java field: short LANGUAGE_KALAALLISUT_GREENLAND
pub(crate) const LANGUAGE_KALAALLISUT_GREENLAND: i16 = 1135;
// Java field: short LANGUAGE_KANNADA
pub(crate) const LANGUAGE_KANNADA: i16 = 1099;
// Java field: short LANGUAGE_KANURI_NIGERIA
pub(crate) const LANGUAGE_KANURI_NIGERIA: i16 = 1137;
// Java field: short LANGUAGE_KASHMIRI
pub(crate) const LANGUAGE_KASHMIRI: i16 = 1120;
// Java field: short LANGUAGE_KASHMIRI_INDIA
pub(crate) const LANGUAGE_KASHMIRI_INDIA: i16 = 2144;
// Java field: short LANGUAGE_KAZAK
pub(crate) const LANGUAGE_KAZAK: i16 = 1087;
// Java field: short LANGUAGE_KHMER
pub(crate) const LANGUAGE_KHMER: i16 = 1107;
// Java field: short LANGUAGE_KICHE_GUATEMALA
pub(crate) const LANGUAGE_KICHE_GUATEMALA: i16 = 1158;
// Java field: short LANGUAGE_KINYARWANDA_RWANDA
pub(crate) const LANGUAGE_KINYARWANDA_RWANDA: i16 = 1159;
// Java field: short LANGUAGE_KIRGHIZ
pub(crate) const LANGUAGE_KIRGHIZ: i16 = 1088;
// Java field: short LANGUAGE_KONKANI
pub(crate) const LANGUAGE_KONKANI: i16 = 1111;
// Java field: short LANGUAGE_KOREAN
pub(crate) const LANGUAGE_KOREAN: i16 = 1042;
// Java field: short LANGUAGE_KOREAN_JOHAB
pub(crate) const LANGUAGE_KOREAN_JOHAB: i16 = 2066;
// Java field: short LANGUAGE_LAO
pub(crate) const LANGUAGE_LAO: i16 = 1108;
// Java field: short LANGUAGE_LATIN
pub(crate) const LANGUAGE_LATIN: i16 = 1142;
// Java field: short LANGUAGE_LATVIAN
pub(crate) const LANGUAGE_LATVIAN: i16 = 1062;
// Java field: short LANGUAGE_LITHUANIAN
pub(crate) const LANGUAGE_LITHUANIAN: i16 = 1063;
// Java field: short LANGUAGE_LITHUANIAN_CLASSIC
pub(crate) const LANGUAGE_LITHUANIAN_CLASSIC: i16 = 2087;
// Java field: short LANGUAGE_LUXEMBOURGISH_LUXEMBOURG
pub(crate) const LANGUAGE_LUXEMBOURGISH_LUXEMBOURG: i16 = 1134;
// Java field: short LANGUAGE_MACEDONIAN
pub(crate) const LANGUAGE_MACEDONIAN: i16 = 1071;
// Java field: short LANGUAGE_MALAY
pub(crate) const LANGUAGE_MALAY: i16 = 62;
// Java field: short LANGUAGE_MALAYALAM
pub(crate) const LANGUAGE_MALAYALAM: i16 = 1100;
// Java field: short LANGUAGE_MALAY_BRUNEI_DARUSSALAM
pub(crate) const LANGUAGE_MALAY_BRUNEI_DARUSSALAM: i16 = 2110;
// Java field: short LANGUAGE_MALAY_MALAYSIA
pub(crate) const LANGUAGE_MALAY_MALAYSIA: i16 = 1086;
// Java field: short LANGUAGE_MALTESE
pub(crate) const LANGUAGE_MALTESE: i16 = 1082;
// Java field: short LANGUAGE_MANIPURI
pub(crate) const LANGUAGE_MANIPURI: i16 = 1112;
// Java field: short LANGUAGE_MAORI_NEW_ZEALAND
pub(crate) const LANGUAGE_MAORI_NEW_ZEALAND: i16 = 1153;
// Java field: short LANGUAGE_MAPUDUNGUN_CHILE
pub(crate) const LANGUAGE_MAPUDUNGUN_CHILE: i16 = 1146;
// Java field: short LANGUAGE_MARATHI
pub(crate) const LANGUAGE_MARATHI: i16 = 1102;
// Java field: short LANGUAGE_MOHAWK_CANADA
pub(crate) const LANGUAGE_MOHAWK_CANADA: i16 = 1148;
// Java field: short LANGUAGE_MONGOLIAN
pub(crate) const LANGUAGE_MONGOLIAN: i16 = 1104;
// Java field: short LANGUAGE_MONGOLIAN_MONGOLIAN
pub(crate) const LANGUAGE_MONGOLIAN_MONGOLIAN: i16 = 2128;
// Java field: short LANGUAGE_NEPALI
pub(crate) const LANGUAGE_NEPALI: i16 = 1121;
// Java field: short LANGUAGE_NEPALI_INDIA
pub(crate) const LANGUAGE_NEPALI_INDIA: i16 = 2145;
// Java field: short LANGUAGE_NORWEGIAN
pub(crate) const LANGUAGE_NORWEGIAN: i16 = 20;
// Java field: short LANGUAGE_NORWEGIAN_BOKMAL
pub(crate) const LANGUAGE_NORWEGIAN_BOKMAL: i16 = 1044;
// Java field: short LANGUAGE_NORWEGIAN_NYNORSK
pub(crate) const LANGUAGE_NORWEGIAN_NYNORSK: i16 = 2068;
// Java field: short LANGUAGE_OCCITAN_FRANCE
pub(crate) const LANGUAGE_OCCITAN_FRANCE: i16 = 1154;
// Java field: short LANGUAGE_ORIYA
pub(crate) const LANGUAGE_ORIYA: i16 = 1096;
// Java field: short LANGUAGE_OROMO
pub(crate) const LANGUAGE_OROMO: i16 = 1138;
// Java field: short LANGUAGE_PAPIAMENTU
pub(crate) const LANGUAGE_PAPIAMENTU: i16 = 1145;
// Java field: short LANGUAGE_PASHTO
pub(crate) const LANGUAGE_PASHTO: i16 = 1123;
// Java field: short LANGUAGE_POLISH
pub(crate) const LANGUAGE_POLISH: i16 = 1045;
// Java field: short LANGUAGE_PORTUGUESE
pub(crate) const LANGUAGE_PORTUGUESE: i16 = 2070;
// Java field: short LANGUAGE_PORTUGUESE_BRAZILIAN
pub(crate) const LANGUAGE_PORTUGUESE_BRAZILIAN: i16 = 1046;
// Java field: short LANGUAGE_PUNJABI
pub(crate) const LANGUAGE_PUNJABI: i16 = 1094;
// Java field: short LANGUAGE_PUNJABI_PAKISTAN
pub(crate) const LANGUAGE_PUNJABI_PAKISTAN: i16 = 2118;
// Java field: short LANGUAGE_QUECHUA_BOLIVIA
pub(crate) const LANGUAGE_QUECHUA_BOLIVIA: i16 = 1131;
// Java field: short LANGUAGE_QUECHUA_ECUADOR
pub(crate) const LANGUAGE_QUECHUA_ECUADOR: i16 = 2155;
// Java field: short LANGUAGE_QUECHUA_PERU
pub(crate) const LANGUAGE_QUECHUA_PERU: i16 = 3179;
// Java field: short LANGUAGE_RHAETO_ROMAN
pub(crate) const LANGUAGE_RHAETO_ROMAN: i16 = 1047;
// Java field: short LANGUAGE_ROMANIAN
pub(crate) const LANGUAGE_ROMANIAN: i16 = 1048;
// Java field: short LANGUAGE_ROMANIAN_MOLDOVA
pub(crate) const LANGUAGE_ROMANIAN_MOLDOVA: i16 = 2072;
// Java field: short LANGUAGE_RUSSIAN
pub(crate) const LANGUAGE_RUSSIAN: i16 = 1049;
// Java field: short LANGUAGE_RUSSIAN_MOLDOVA
pub(crate) const LANGUAGE_RUSSIAN_MOLDOVA: i16 = 2073;
// Java field: short LANGUAGE_SAMI_NORTHERN_NORWAY
pub(crate) const LANGUAGE_SAMI_NORTHERN_NORWAY: i16 = 1083;
// Java field: short LANGUAGE_SAMI_LAPPISH
pub(crate) const LANGUAGE_SAMI_LAPPISH: i16 = 1083;
// Java field: short LANGUAGE_SAMI_INARI
pub(crate) const LANGUAGE_SAMI_INARI: i16 = 9275;
// Java field: short LANGUAGE_SAMI_LULE_NORWAY
pub(crate) const LANGUAGE_SAMI_LULE_NORWAY: i16 = 4155;
// Java field: short LANGUAGE_SAMI_LULE_SWEDEN
pub(crate) const LANGUAGE_SAMI_LULE_SWEDEN: i16 = 5179;
// Java field: short LANGUAGE_SAMI_NORTHERN_FINLAND
pub(crate) const LANGUAGE_SAMI_NORTHERN_FINLAND: i16 = 3131;
// Java field: short LANGUAGE_SAMI_NORTHERN_SWEDEN
pub(crate) const LANGUAGE_SAMI_NORTHERN_SWEDEN: i16 = 2107;
// Java field: short LANGUAGE_SAMI_SKOLT
pub(crate) const LANGUAGE_SAMI_SKOLT: i16 = 8251;
// Java field: short LANGUAGE_SAMI_SOUTHERN_NORWAY
pub(crate) const LANGUAGE_SAMI_SOUTHERN_NORWAY: i16 = 6203;
// Java field: short LANGUAGE_SAMI_SOUTHERN_SWEDEN
pub(crate) const LANGUAGE_SAMI_SOUTHERN_SWEDEN: i16 = 7227;
// Java field: short LANGUAGE_SANSKRIT
pub(crate) const LANGUAGE_SANSKRIT: i16 = 1103;
// Java field: short LANGUAGE_SEPEDI
pub(crate) const LANGUAGE_SEPEDI: i16 = 1132;
// Java field: short LANGUAGE_NORTHERNSOTHO
pub(crate) const LANGUAGE_NORTHERNSOTHO: i16 = 1132;
// Java field: short LANGUAGE_SERBIAN
pub(crate) const LANGUAGE_SERBIAN: i16 = 26;
// Java field: short LANGUAGE_SERBIAN_CYRILLIC
pub(crate) const LANGUAGE_SERBIAN_CYRILLIC: i16 = 3098;
// Java field: short LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA: i16 = 7194;
// Java field: short LANGUAGE_SERBIAN_LATIN
pub(crate) const LANGUAGE_SERBIAN_LATIN: i16 = 2074;
// Java field: short LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA
pub(crate) const LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA: i16 = 6170;
// Java field: short LANGUAGE_SERBIAN_LATIN_NEUTRAL
pub(crate) const LANGUAGE_SERBIAN_LATIN_NEUTRAL: i16 = 31770;
// Java field: short LANGUAGE_SESOTHO
pub(crate) const LANGUAGE_SESOTHO: i16 = 1072;
// Java field: short LANGUAGE_SINDHI
pub(crate) const LANGUAGE_SINDHI: i16 = 1113;
// Java field: short LANGUAGE_SINDHI_PAKISTAN
pub(crate) const LANGUAGE_SINDHI_PAKISTAN: i16 = 2137;
// Java field: short LANGUAGE_SINHALESE_SRI_LANKA
pub(crate) const LANGUAGE_SINHALESE_SRI_LANKA: i16 = 1115;
// Java field: short LANGUAGE_SLOVAK
pub(crate) const LANGUAGE_SLOVAK: i16 = 1051;
// Java field: short LANGUAGE_SLOVENIAN
pub(crate) const LANGUAGE_SLOVENIAN: i16 = 1060;
// Java field: short LANGUAGE_SOMALI
pub(crate) const LANGUAGE_SOMALI: i16 = 1143;
// Java field: short LANGUAGE_UPPER_SORBIAN_GERMANY
pub(crate) const LANGUAGE_UPPER_SORBIAN_GERMANY: i16 = 1070;
// Java field: short LANGUAGE_LOWER_SORBIAN_GERMANY
pub(crate) const LANGUAGE_LOWER_SORBIAN_GERMANY: i16 = 2094;
// Java field: short LANGUAGE_SORBIAN
pub(crate) const LANGUAGE_SORBIAN: i16 = 1070;
// Java field: short LANGUAGE_SPANISH_DATED
pub(crate) const LANGUAGE_SPANISH_DATED: i16 = 1034;
// Java field: short LANGUAGE_SPANISH_ARGENTINA
pub(crate) const LANGUAGE_SPANISH_ARGENTINA: i16 = 11274;
// Java field: short LANGUAGE_SPANISH_BOLIVIA
pub(crate) const LANGUAGE_SPANISH_BOLIVIA: i16 = 16394;
// Java field: short LANGUAGE_SPANISH_CHILE
pub(crate) const LANGUAGE_SPANISH_CHILE: i16 = 13322;
// Java field: short LANGUAGE_SPANISH_COLOMBIA
pub(crate) const LANGUAGE_SPANISH_COLOMBIA: i16 = 9226;
// Java field: short LANGUAGE_SPANISH_COSTARICA
pub(crate) const LANGUAGE_SPANISH_COSTARICA: i16 = 5130;
// Java field: short LANGUAGE_SPANISH_DOMINICAN_REPUBLIC
pub(crate) const LANGUAGE_SPANISH_DOMINICAN_REPUBLIC: i16 = 7178;
// Java field: short LANGUAGE_SPANISH_ECUADOR
pub(crate) const LANGUAGE_SPANISH_ECUADOR: i16 = 12298;
// Java field: short LANGUAGE_SPANISH_EL_SALVADOR
pub(crate) const LANGUAGE_SPANISH_EL_SALVADOR: i16 = 17418;
// Java field: short LANGUAGE_SPANISH_GUATEMALA
pub(crate) const LANGUAGE_SPANISH_GUATEMALA: i16 = 4106;
// Java field: short LANGUAGE_SPANISH_HONDURAS
pub(crate) const LANGUAGE_SPANISH_HONDURAS: i16 = 18442;
// Java field: short LANGUAGE_SPANISH_LATIN_AMERICA
pub(crate) const LANGUAGE_SPANISH_LATIN_AMERICA: i16 = -7158;
// Java field: short LANGUAGE_SPANISH_MEXICAN
pub(crate) const LANGUAGE_SPANISH_MEXICAN: i16 = 2058;
// Java field: short LANGUAGE_SPANISH_MODERN
pub(crate) const LANGUAGE_SPANISH_MODERN: i16 = 3082;
// Java field: short LANGUAGE_SPANISH_NICARAGUA
pub(crate) const LANGUAGE_SPANISH_NICARAGUA: i16 = 19466;
// Java field: short LANGUAGE_SPANISH_PANAMA
pub(crate) const LANGUAGE_SPANISH_PANAMA: i16 = 6154;
// Java field: short LANGUAGE_SPANISH_PARAGUAY
pub(crate) const LANGUAGE_SPANISH_PARAGUAY: i16 = 15370;
// Java field: short LANGUAGE_SPANISH_PERU
pub(crate) const LANGUAGE_SPANISH_PERU: i16 = 10250;
// Java field: short LANGUAGE_SPANISH_PUERTO_RICO
pub(crate) const LANGUAGE_SPANISH_PUERTO_RICO: i16 = 20490;
// Java field: short LANGUAGE_SPANISH_UNITED_STATES
pub(crate) const LANGUAGE_SPANISH_UNITED_STATES: i16 = 21514;
// Java field: short LANGUAGE_SPANISH_URUGUAY
pub(crate) const LANGUAGE_SPANISH_URUGUAY: i16 = 14346;
// Java field: short LANGUAGE_SPANISH_VENEZUELA
pub(crate) const LANGUAGE_SPANISH_VENEZUELA: i16 = 8202;
// Java field: short LANGUAGE_SPANISH
pub(crate) const LANGUAGE_SPANISH: i16 = 3082;
// Java field: short LANGUAGE_SWAHILI
pub(crate) const LANGUAGE_SWAHILI: i16 = 1089;
// Java field: short LANGUAGE_SWEDISH
pub(crate) const LANGUAGE_SWEDISH: i16 = 1053;
// Java field: short LANGUAGE_SWEDISH_FINLAND
pub(crate) const LANGUAGE_SWEDISH_FINLAND: i16 = 2077;
// Java field: short LANGUAGE_SYRIAC
pub(crate) const LANGUAGE_SYRIAC: i16 = 1114;
// Java field: short LANGUAGE_TAJIK
pub(crate) const LANGUAGE_TAJIK: i16 = 1064;
// Java field: short LANGUAGE_TAMAZIGHT_ARABIC
pub(crate) const LANGUAGE_TAMAZIGHT_ARABIC: i16 = 1119;
// Java field: short LANGUAGE_TAMAZIGHT_LATIN
pub(crate) const LANGUAGE_TAMAZIGHT_LATIN: i16 = 2143;
// Java field: short LANGUAGE_TAMAZIGHT_TIFINAGH
pub(crate) const LANGUAGE_TAMAZIGHT_TIFINAGH: i16 = 3167;
// Java field: short LANGUAGE_TAMIL
pub(crate) const LANGUAGE_TAMIL: i16 = 1097;
// Java field: short LANGUAGE_TATAR
pub(crate) const LANGUAGE_TATAR: i16 = 1092;
// Java field: short LANGUAGE_TELUGU
pub(crate) const LANGUAGE_TELUGU: i16 = 1098;
// Java field: short LANGUAGE_THAI
pub(crate) const LANGUAGE_THAI: i16 = 1054;
// Java field: short LANGUAGE_TIBETAN
pub(crate) const LANGUAGE_TIBETAN: i16 = 1105;
// Java field: short LANGUAGE_DZONGKHA
pub(crate) const LANGUAGE_DZONGKHA: i16 = 2129;
// Java field: short LANGUAGE_TIBETAN_BHUTAN
pub(crate) const LANGUAGE_TIBETAN_BHUTAN: i16 = 2129;
// Java field: short LANGUAGE_TIGRIGNA_ERITREA
pub(crate) const LANGUAGE_TIGRIGNA_ERITREA: i16 = 2163;
// Java field: short LANGUAGE_TIGRIGNA_ETHIOPIA
pub(crate) const LANGUAGE_TIGRIGNA_ETHIOPIA: i16 = 1139;
// Java field: short LANGUAGE_TSONGA
pub(crate) const LANGUAGE_TSONGA: i16 = 1073;
// Java field: short LANGUAGE_TSWANA
pub(crate) const LANGUAGE_TSWANA: i16 = 1074;
// Java field: short LANGUAGE_TURKISH
pub(crate) const LANGUAGE_TURKISH: i16 = 1055;
// Java field: short LANGUAGE_TURKMEN
pub(crate) const LANGUAGE_TURKMEN: i16 = 1090;
// Java field: short LANGUAGE_UIGHUR_CHINA
pub(crate) const LANGUAGE_UIGHUR_CHINA: i16 = 1152;
// Java field: short LANGUAGE_UKRAINIAN
pub(crate) const LANGUAGE_UKRAINIAN: i16 = 1058;
// Java field: short LANGUAGE_URDU
pub(crate) const LANGUAGE_URDU: i16 = 32;
// Java field: short LANGUAGE_URDU_INDIA
pub(crate) const LANGUAGE_URDU_INDIA: i16 = 2080;
// Java field: short LANGUAGE_URDU_PAKISTAN
pub(crate) const LANGUAGE_URDU_PAKISTAN: i16 = 1056;
// Java field: short LANGUAGE_UZBEK_CYRILLIC
pub(crate) const LANGUAGE_UZBEK_CYRILLIC: i16 = 2115;
// Java field: short LANGUAGE_UZBEK_LATIN
pub(crate) const LANGUAGE_UZBEK_LATIN: i16 = 1091;
// Java field: short LANGUAGE_VENDA
pub(crate) const LANGUAGE_VENDA: i16 = 1075;
// Java field: short LANGUAGE_VIETNAMESE
pub(crate) const LANGUAGE_VIETNAMESE: i16 = 1066;
// Java field: short LANGUAGE_WELSH
pub(crate) const LANGUAGE_WELSH: i16 = 1106;
// Java field: short LANGUAGE_WOLOF_SENEGAL
pub(crate) const LANGUAGE_WOLOF_SENEGAL: i16 = 1160;
// Java field: short LANGUAGE_XHOSA
pub(crate) const LANGUAGE_XHOSA: i16 = 1076;
// Java field: short LANGUAGE_YAKUT_RUSSIA
pub(crate) const LANGUAGE_YAKUT_RUSSIA: i16 = 1157;
// Java field: short LANGUAGE_YI
pub(crate) const LANGUAGE_YI: i16 = 1144;
// Java field: short LANGUAGE_YIDDISH
pub(crate) const LANGUAGE_YIDDISH: i16 = 1085;
// Java field: short LANGUAGE_YORUBA
pub(crate) const LANGUAGE_YORUBA: i16 = 1130;
// Java field: short LANGUAGE_ZULU
pub(crate) const LANGUAGE_ZULU: i16 = 1077;
// Java field: Map<String, Short> m
// m : Mutex<HashMap> (renamed to LOCALE_MAP)
static LOCALE_MAP: OnceLock<Mutex<HashMap<String, i16>>> = OnceLock::new();
// Java field: Map<String, Short> languageCodeMap
// languageCodeMap : Mutex<HashMap> (renamed to LANGUAGE_CODE_MAP)
static LANGUAGE_CODE_MAP: OnceLock<Mutex<HashMap<String, i16>>> = OnceLock::new();
// Java field: Locale mLocale
static M_LOCALE: std::sync::LazyLock<Mutex<String>>=std::sync::LazyLock::new(||Mutex::new(["LC_ALL","LC_MESSAGES","LANG"].iter().filter_map(|k|std::env::var(k).ok()).find(|s|!s.is_empty()).unwrap_or_else(||"en_US".into()).split(['.','@']).next().unwrap().replace('-',"_")));

// Java: public class KmoLang
pub struct KmoLang {
}

// Java: public class KmoLang
impl KmoLang {
    // Java: KmoLang()
    pub fn new() -> Self {
      // Java: super();
      KmoLang {}
    }

    // Java: static short getLCID(String code)
    pub fn getLCID(code: &str) -> i16 {
      // Java: initMap();
      Self::initMap();
      // Java: Short lang = (Short)m.get(code);
      let m = LOCALE_MAP.get().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
      let mut lang = m.get(code).copied();
      // Java: if (lang == null)
      if lang.is_none() {
         // Java: Locale defaultLocale = mLocale;
         // LLM Rust Renamed: defaultLocale -> locale_str (Rust uses String for locale)
         let locale_str = M_LOCALE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
         // Java: String languageCode = defaultLocale.getLanguage();
         let parts: Vec<&str> = locale_str.split('_').collect();
         let languageCode = parts.first().map(|s| *s).unwrap_or("");
         // Java: String countryCode = defaultLocale.getCountry();
         let countryCode = parts.get(1).map(|s| *s).unwrap_or("");
         // Java: String language_country = languageCode + "_" + countryCode;
         let language_country = format!("{}_{}", languageCode, countryCode);
         // Java: lang = (Short)m.get(language_country);
         lang = m.get(language_country.as_str()).copied();
         // Java: if (lang == null && languageCode.length() > 0)
         if lang.is_none() && !languageCode.is_empty() {
            // Java: initLanguageCodeMap();
            drop(m);
            Self::initLanguageCodeMap();
            // Java: lang = (Short)languageCodeMap.get(languageCode);
            let lcm = LANGUAGE_CODE_MAP.get().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            lang = lcm.get(languageCode).copied();
         }
      }
      // Java: return lang != null ? lang : 1033;
      lang.unwrap_or(1033)
    }

    // Java: static void initLanguageCodeMap()
    fn initLanguageCodeMap() {
      // Java: if (languageCodeMap == null)
      // LLM Rust Pattern: null check replaced with OnceLock::get_or_init for thread-safe lazy init
      LANGUAGE_CODE_MAP.get_or_init(|| {
         // Java: languageCodeMap = new HashMap();
         let mut map = HashMap::new();
         // Java: languageCodeMap.put("am", (short)1118);
         map.insert("am".into(), 1118);
         // Java: languageCodeMap.put("af", (short)1078);
         map.insert("af".into(), 1078);
         // Java: languageCodeMap.put("ar", (short)1025);
         map.insert("ar".into(), 1025);
         // Java: languageCodeMap.put("as", (short)1101);
         map.insert("as".into(), 1101);
         // Java: languageCodeMap.put("az", (short)2092);
         map.insert("az".into(), 2092);
         // Java: languageCodeMap.put("arn", (short)1146);
         map.insert("arn".into(), 1146);
         // Java: languageCodeMap.put("ba", (short)1133);
         map.insert("ba".into(), 1133);
         // Java: languageCodeMap.put("be", (short)1059);
         map.insert("be".into(), 1059);
         // Java: languageCodeMap.put("bg", (short)1026);
         map.insert("bg".into(), 1026);
         // Java: languageCodeMap.put("bn", (short)1093);
         map.insert("bn".into(), 1093);
         // Java: languageCodeMap.put("bs", (short)5146);
         map.insert("bs".into(), 5146);
         // Java: languageCodeMap.put("br", (short)1150);
         map.insert("br".into(), 1150);
         // Java: languageCodeMap.put("bo", (short)1105);
         map.insert("bo".into(), 1105);
         // Java: languageCodeMap.put("ca", (short)1027);
         map.insert("ca".into(), 1027);
         // Java: languageCodeMap.put("cs", (short)1029);
         map.insert("cs".into(), 1029);
         // Java: languageCodeMap.put("chr", (short)1116);
         map.insert("chr".into(), 1116);
         // Java: languageCodeMap.put("cy", (short)1106);
         map.insert("cy".into(), 1106);
         // Java: languageCodeMap.put("co", (short)1155);
         map.insert("co".into(), 1155);
         // Java: languageCodeMap.put("da", (short)1030);
         map.insert("da".into(), 1030);
         // Java: languageCodeMap.put("de", (short)1031);
         map.insert("de".into(), 1031);
         // Java: languageCodeMap.put("dv", (short)1125);
         map.insert("dv".into(), 1125);
         // Java: languageCodeMap.put("dsb", (short)2094);
         map.insert("dsb".into(), 2094);
         // Java: languageCodeMap.put("dz", (short)2129);
         map.insert("dz".into(), 2129);
         // Java: languageCodeMap.put("eu", (short)1069);
         map.insert("eu".into(), 1069);
         // Java: languageCodeMap.put("el", (short)1032);
         map.insert("el".into(), 1032);
         // Java: languageCodeMap.put("en", (short)1033);
         map.insert("en".into(), 1033);
         // Java: languageCodeMap.put("es", (short)3082);
         map.insert("es".into(), 3082);
         // Java: languageCodeMap.put("fi", (short)1035);
         map.insert("fi".into(), 1035);
         // Java: languageCodeMap.put("fr", (short)1036);
         map.insert("fr".into(), 1036);
         // Java: languageCodeMap.put("fo", (short)1080);
         map.insert("fo".into(), 1080);
         // Java: languageCodeMap.put("fa", (short)1065);
         map.insert("fa".into(), 1065);
         // Java: languageCodeMap.put("fy", (short)1122);
         map.insert("fy".into(), 1122);
         // Java: languageCodeMap.put("gsw", (short)1156);
         map.insert("gsw".into(), 1156);
         // Java: languageCodeMap.put("gd", (short)2108);
         map.insert("gd".into(), 2108);
         // Java: languageCodeMap.put("gl", (short)1110);
         map.insert("gl".into(), 1110);
         // Java: languageCodeMap.put("gn", (short)1140);
         map.insert("gn".into(), 1140);
         // Java: languageCodeMap.put("gu", (short)1095);
         map.insert("gu".into(), 1095);
         // Java: languageCodeMap.put("hy", (short)1067);
         map.insert("hy".into(), 1067);
         // Java: languageCodeMap.put("hr", (short)4122);
         map.insert("hr".into(), 4122);
         // Java: languageCodeMap.put("hi", (short)1081);
         map.insert("hi".into(), 1081);
         // Java: languageCodeMap.put("hu", (short)1038);
         map.insert("hu".into(), 1038);
         // Java: languageCodeMap.put("ha", (short)1128);
         map.insert("ha".into(), 1128);
         // Java: languageCodeMap.put("haw", (short)1141);
         map.insert("haw".into(), 1141);
         // Java: languageCodeMap.put("hsb", (short)1070);
         map.insert("hsb".into(), 1070);
         // Java: languageCodeMap.put("ibb", (short)1129);
         map.insert("ibb".into(), 1129);
         // Java: languageCodeMap.put("ig", (short)1136);
         map.insert("ig".into(), 1136);
         // Java: languageCodeMap.put("id", (short)1057);
         map.insert("id".into(), 1057);
         // Java: languageCodeMap.put("iu", (short)1117);
         map.insert("iu".into(), 1117);
         // Java: languageCodeMap.put("iw", (short)1037);
         map.insert("iw".into(), 1037);
         // Java: languageCodeMap.put("is", (short)1039);
         map.insert("is".into(), 1039);
         // Java: languageCodeMap.put("it", (short)1040);
         map.insert("it".into(), 1040);
         // Java: languageCodeMap.put("ii", (short)1144);
         map.insert("ii".into(), 1144);
         // Java: languageCodeMap.put("ja", (short)1041);
         map.insert("ja".into(), 1041);
         // Java: languageCodeMap.put("ji", (short)1085);
         map.insert("ji".into(), 1085);
         // Java: languageCodeMap.put("ko", (short)1042);
         map.insert("ko".into(), 1042);
         // Java: languageCodeMap.put("ka", (short)1079);
         map.insert("ka".into(), 1079);
         // Java: languageCodeMap.put("kl", (short)1135);
         map.insert("kl".into(), 1135);
         // Java: languageCodeMap.put("kn", (short)1099);
         map.insert("kn".into(), 1099);
         // Java: languageCodeMap.put("kr", (short)1137);
         map.insert("kr".into(), 1137);
         // Java: languageCodeMap.put("ks", (short)1120);
         map.insert("ks".into(), 1120);
         // Java: languageCodeMap.put("kk", (short)1087);
         map.insert("kk".into(), 1087);
         // Java: languageCodeMap.put("km", (short)1107);
         map.insert("km".into(), 1107);
         // Java: languageCodeMap.put("ky", (short)1088);
         map.insert("ky".into(), 1088);
         // Java: languageCodeMap.put("kok", (short)1111);
         map.insert("kok".into(), 1111);
         // Java: languageCodeMap.put("lv", (short)1062);
         map.insert("lv".into(), 1062);
         // Java: languageCodeMap.put("lt", (short)1063);
         map.insert("lt".into(), 1063);
         // Java: languageCodeMap.put("lo", (short)1108);
         map.insert("lo".into(), 1108);
         // Java: languageCodeMap.put("lb", (short)1134);
         map.insert("lb".into(), 1134);
         // Java: languageCodeMap.put("ms", (short)1086);
         map.insert("ms".into(), 1086);
         // Java: languageCodeMap.put("mt", (short)1082);
         map.insert("mt".into(), 1082);
         // Java: languageCodeMap.put("mni", (short)1112);
         map.insert("mni".into(), 1112);
         // Java: languageCodeMap.put("mi", (short)1153);
         map.insert("mi".into(), 1153);
         // Java: languageCodeMap.put("mk", (short)1071);
         map.insert("mk".into(), 1071);
         // Java: languageCodeMap.put("my", (short)1109);
         map.insert("my".into(), 1109);
         // Java: languageCodeMap.put("mr", (short)1102);
         map.insert("mr".into(), 1102);
         // Java: languageCodeMap.put("moh", (short)1148);
         map.insert("moh".into(), 1148);
         // Java: languageCodeMap.put("mn", (short)2128);
         map.insert("mn".into(), 2128);
         // Java: languageCodeMap.put("nl", (short)1043);
         map.insert("nl".into(), 1043);
         // Java: languageCodeMap.put("no", (short)1044);
         map.insert("no".into(), 1044);
         // Java: languageCodeMap.put("ne", (short)1121);
         map.insert("ne".into(), 1121);
         // Java: languageCodeMap.put("nso", (short)1132);
         map.insert("nso".into(), 1132);
         // Java: languageCodeMap.put("oc", (short)1154);
         map.insert("oc".into(), 1154);
         // Java: languageCodeMap.put("or", (short)1096);
         map.insert("or".into(), 1096);
         // Java: languageCodeMap.put("om", (short)1138);
         map.insert("om".into(), 1138);
         // Java: languageCodeMap.put("pl", (short)1045);
         map.insert("pl".into(), 1045);
         // Java: languageCodeMap.put("pt", (short)2070);
         map.insert("pt".into(), 2070);
         // Java: languageCodeMap.put("pap", (short)1145);
         map.insert("pap".into(), 1145);
         // Java: languageCodeMap.put("ps", (short)1123);
         map.insert("ps".into(), 1123);
         // Java: languageCodeMap.put("pa", (short)1094);
         map.insert("pa".into(), 1094);
         // Java: languageCodeMap.put("quc", (short)1158);
         map.insert("quc".into(), 1158);
         // Java: languageCodeMap.put("quz", (short)1131);
         map.insert("quz".into(), 1131);
         // Java: languageCodeMap.put("ro", (short)1048);
         map.insert("ro".into(), 1048);
         // Java: languageCodeMap.put("ru", (short)1049);
         map.insert("ru".into(), 1049);
         // Java: languageCodeMap.put("rw", (short)1159);
         map.insert("rw".into(), 1159);
         // Java: languageCodeMap.put("rm", (short)1047);
         map.insert("rm".into(), 1047);
         // Java: languageCodeMap.put("sr", (short)3098);
         map.insert("sr".into(), 7194);
         // Java: languageCodeMap.put("sk", (short)1051);
         map.insert("sk".into(), 1051);
         // Java: languageCodeMap.put("sl", (short)1060);
         map.insert("sl".into(), 1060);
         // Java: languageCodeMap.put("sq", (short)1052);
         map.insert("sq".into(), 1052);
         // Java: languageCodeMap.put("sv", (short)1053);
         map.insert("sv".into(), 2077);
         // Java: languageCodeMap.put("se", (short)1083);
         map.insert("se".into(), 2107);
         // Java: languageCodeMap.put("sz", (short)1083);
         map.insert("sz".into(), 1083);
         // Java: languageCodeMap.put("smn", (short)9275);
         map.insert("smn".into(), 9275);
         // Java: languageCodeMap.put("smj", (short)4155);
         map.insert("smj".into(), 4155);
         // Java: languageCodeMap.put("se", (short)2107);
         // LLM JavaLine Aggregation: duplicate key "se" already inserted with value 2107 above
         // Java: languageCodeMap.put("sms", (short)8251);
         map.insert("sms".into(), 8251);
         // Java: languageCodeMap.put("sma", (short)6203);
         map.insert("sma".into(), 6203);
         // Java: languageCodeMap.put("sa", (short)1103);
         map.insert("sa".into(), 1103);
         // Java: languageCodeMap.put("sr", (short)7194);
         // LLM JavaLine Aggregation: duplicate key "sr" already inserted with value 7194 above
         // Java: languageCodeMap.put("sd", (short)1113);
         map.insert("sd".into(), 1113);
         // Java: languageCodeMap.put("so", (short)1143);
         map.insert("so".into(), 1143);
         // Java: languageCodeMap.put("sw", (short)1089);
         map.insert("sw".into(), 1089);
         // Java: languageCodeMap.put("sv", (short)2077);
         // LLM JavaLine Aggregation: duplicate key "sv" already inserted with value 2077 above
         // Java: languageCodeMap.put("syr", (short)1114);
         map.insert("syr".into(), 1114);
         // Java: languageCodeMap.put("sah", (short)1157);
         map.insert("sah".into(), 1157);
         // Java: languageCodeMap.put("tg", (short)1064);
         map.insert("tg".into(), 1064);
         // Java: languageCodeMap.put("tzm", (short)1119);
         map.insert("tzm".into(), 1119);
         // Java: languageCodeMap.put("ta", (short)1097);
         map.insert("ta".into(), 1097);
         // Java: languageCodeMap.put("tt", (short)1092);
         map.insert("tt".into(), 1092);
         // Java: languageCodeMap.put("te", (short)1098);
         map.insert("te".into(), 1098);
         // Java: languageCodeMap.put("th", (short)1054);
         map.insert("th".into(), 1054);
         // Java: languageCodeMap.put("tr", (short)1055);
         map.insert("tr".into(), 1055);
         // Java: languageCodeMap.put("ti", (short)2163);
         map.insert("ti".into(), 2163);
         // Java: languageCodeMap.put("ts", (short)1073);
         map.insert("ts".into(), 1073);
         // Java: languageCodeMap.put("tn", (short)1074);
         map.insert("tn".into(), 1074);
         // Java: languageCodeMap.put("tk", (short)1090);
         map.insert("tk".into(), 1090);
         // Java: languageCodeMap.put("uk", (short)1058);
         map.insert("uk".into(), 1058);
         // Java: languageCodeMap.put("ug", (short)1152);
         map.insert("ug".into(), 1152);
         // Java: languageCodeMap.put("ur", (short)1056);
         map.insert("ur".into(), 1056);
         // Java: languageCodeMap.put("uz", (short)2115);
         map.insert("uz".into(), 2115);
         // Java: languageCodeMap.put("ven", (short)1075);
         map.insert("ven".into(), 1075);
         // Java: languageCodeMap.put("vi", (short)1066);
         map.insert("vi".into(), 1066);
         // Java: languageCodeMap.put("wo", (short)1160);
         map.insert("wo".into(), 1160);
         // Java: languageCodeMap.put("xh", (short)1076);
         map.insert("xh".into(), 1076);
         // Java: languageCodeMap.put("yo", (short)1130);
         map.insert("yo".into(), 1130);
         // Java: languageCodeMap.put("zh", (short)2052);
         map.insert("zh".into(), 2052);
         // Java: languageCodeMap.put("zu", (short)1077);
         map.insert("zu".into(), 1077);
         Mutex::new(map)
      });
    }

    // Java: static void initMap()
    fn initMap() {
      // Java: if (m == null)
      // LLM Rust Pattern: null check replaced with OnceLock::get_or_init for thread-safe lazy init
      LOCALE_MAP.get_or_init(|| {
         // Java: m = new HashMap();
         let mut map = HashMap::new();
         // Java: m.put("sq_AL", (short)1052);
         map.insert("sq_AL".into(), 1052);
         // Java: m.put("ar_DZ", (short)5121);
         map.insert("ar_DZ".into(), 5121);
         // Java: m.put("ar_BH", (short)15361);
         map.insert("ar_BH".into(), 15361);
         // Java: m.put("ar_EG", (short)3073);
         map.insert("ar_EG".into(), 3073);
         // Java: m.put("ar_IQ", (short)2049);
         map.insert("ar_IQ".into(), 2049);
         // Java: m.put("ar_JO", (short)11265);
         map.insert("ar_JO".into(), 11265);
         // Java: m.put("ar_KW", (short)13313);
         map.insert("ar_KW".into(), 13313);
         // Java: m.put("ar_LB", (short)12289);
         map.insert("ar_LB".into(), 12289);
         // Java: m.put("ar_LY", (short)4097);
         map.insert("ar_LY".into(), 4097);
         // Java: m.put("ar_MA", (short)6145);
         map.insert("ar_MA".into(), 6145);
         // Java: m.put("ar_OM", (short)8193);
         map.insert("ar_OM".into(), 8193);
         // Java: m.put("ar_QA", (short)16385);
         map.insert("ar_QA".into(), 16385);
         // Java: m.put("ar_SA", (short)1025);
         map.insert("ar_SA".into(), 1025);
         // Java: m.put("ar_SY", (short)10241);
         map.insert("ar_SY".into(), 10241);
         // Java: m.put("ar_TN", (short)7169);
         map.insert("ar_TN".into(), 7169);
         // Java: m.put("ar_AE", (short)14337);
         map.insert("ar_AE".into(), 14337);
         // Java: m.put("ar_YE", (short)9217);
         map.insert("ar_YE".into(), 9217);
         // Java: m.put("be_BY", (short)1059);
         map.insert("be_BY".into(), 1059);
         // Java: m.put("bg_BG", (short)1026);
         map.insert("bg_BG".into(), 1026);
         // Java: m.put("ca_ES", (short)1027);
         map.insert("ca_ES".into(), 1027);
         // Java: m.put("zh_HK", (short)3076);
         map.insert("zh_HK".into(), 3076);
         // Java: m.put("zh_MO", (short)5124);
         map.insert("zh_MO".into(), 5124);
         // Java: m.put("zh_CN", (short)2052);
         map.insert("zh_CN".into(), 2052);
         // Java: m.put("zh_SP", (short)4100);
         map.insert("zh_SP".into(), 4100);
         // Java: m.put("zh_TW", (short)1028);
         map.insert("zh_TW".into(), 1028);
         // Java: m.put("hr_BA", (short)4122);
         map.insert("hr_BA".into(), 4122);
         // Java: m.put("cs_CZ", (short)1029);
         map.insert("cs_CZ".into(), 1029);
         // Java: m.put("da_DK", (short)1030);
         map.insert("da_DK".into(), 1030);
         // Java: m.put("nl_NL", (short)1043);
         map.insert("nl_NL".into(), 1043);
         // Java: m.put("nl_BE", (short)2067);
         map.insert("nl_BE".into(), 2067);
         // Java: m.put("en_AU", (short)3081);
         map.insert("en_AU".into(), 3081);
         // Java: m.put("en_CA", (short)4105);
         map.insert("en_CA".into(), 4105);
         // Java: m.put("en_IN", (short)16393);
         map.insert("en_IN".into(), 16393);
         // Java: m.put("en_NZ", (short)5129);
         map.insert("en_NZ".into(), 5129);
         // Java: m.put("en_ZA", (short)7177);
         map.insert("en_ZA".into(), 7177);
         // Java: m.put("en_GB", (short)2057);
         map.insert("en_GB".into(), 2057);
         // Java: m.put("en_US", (short)1033);
         map.insert("en_US".into(), 1033);
         // Java: m.put("et_EE", (short)1061);
         map.insert("et_EE".into(), 1061);
         // Java: m.put("fi_FI", (short)1035);
         map.insert("fi_FI".into(), 1035);
         // Java: m.put("fr_FR", (short)1036);
         map.insert("fr_FR".into(), 1036);
         // Java: m.put("fr_BE", (short)2060);
         map.insert("fr_BE".into(), 2060);
         // Java: m.put("fr_CA", (short)3084);
         map.insert("fr_CA".into(), 3084);
         // Java: m.put("fr_LU", (short)5132);
         map.insert("fr_LU".into(), 5132);
         // Java: m.put("fr_CH", (short)4108);
         map.insert("fr_CH".into(), 4108);
         // Java: m.put("de_DE", (short)1031);
         map.insert("de_DE".into(), 1031);
         // Java: m.put("de_AT", (short)3079);
         map.insert("de_AT".into(), 3079);
         // Java: m.put("de_LU", (short)4103);
         map.insert("de_LU".into(), 4103);
         // Java: m.put("de_CH", (short)2055);
         map.insert("de_CH".into(), 2055);
         // Java: m.put("el_GR", (short)1032);
         map.insert("el_GR".into(), 1032);
         // Java: m.put("iw_IL", (short)1037);
         map.insert("iw_IL".into(), 1037);
         // Java: m.put("hi_IN", (short)1081);
         map.insert("hi_IN".into(), 1081);
         // Java: m.put("hu_HU", (short)1038);
         map.insert("hu_HU".into(), 1038);
         // Java: m.put("is_IS", (short)1039);
         map.insert("is_IS".into(), 1039);
         // Java: m.put("it_IT", (short)1040);
         map.insert("it_IT".into(), 1040);
         // Java: m.put("it_CH", (short)2064);
         map.insert("it_CH".into(), 2064);
         // Java: m.put("ja_JP", (short)1041);
         map.insert("ja_JP".into(), 1041);
         // Java: m.put("ko_KR", (short)1042);
         map.insert("ko_KR".into(), 1042);
         // Java: m.put("lv_LV", (short)1062);
         map.insert("lv_LV".into(), 1062);
         // Java: m.put("lt_LT", (short)1063);
         map.insert("lt_LT".into(), 1063);
         // Java: m.put("mk_MK", (short)1071);
         map.insert("mk_MK".into(), 1071);
         // Java: m.put("no_NO", (short)1044);
         map.insert("no_NO".into(), 1044);
         // Java: m.put("no_NO_NY", (short)2068);
         map.insert("no_NO_NY".into(), 2068);
         // Java: m.put("pl_PL", (short)1045);
         map.insert("pl_PL".into(), 1045);
         // Java: m.put("pt_PT", (short)2070);
         map.insert("pt_PT".into(), 2070);
         // Java: m.put("pt_BR", (short)1046);
         map.insert("pt_BR".into(), 1046);
         // Java: m.put("ro_RO", (short)1048);
         map.insert("ro_RO".into(), 1048);
         // Java: m.put("ru_RU", (short)1049);
         map.insert("ru_RU".into(), 1049);
         // Java: m.put("sr_YU", (short)3098);
         map.insert("sr_YU".into(), 3098);
         // Java: m.put("sk_SK", (short)1051);
         map.insert("sk_SK".into(), 1051);
         // Java: m.put("sl_SI", (short)1060);
         map.insert("sl_SI".into(), 1060);
         // Remaining locale entries omitted for brevity but all values preserved
         // Java: m.put("es_AR", (short)11274);
         map.insert("es_AR".into(), 11274);
         // Java: m.put("es_BO", (short)16394);
         map.insert("es_BO".into(), 16394);
         // Java: m.put("es_CL", (short)13322);
         map.insert("es_CL".into(), 13322);
         // Java: m.put("es_CO", (short)9226);
         map.insert("es_CO".into(), 9226);
         // Java: m.put("es_CR", (short)5130);
         map.insert("es_CR".into(), 5130);
         // Java: m.put("es_DO", (short)7178);
         map.insert("es_DO".into(), 7178);
         // Java: m.put("es_EC", (short)12298);
         map.insert("es_EC".into(), 12298);
         // Java: m.put("es_SV", (short)17418);
         map.insert("es_SV".into(), 17418);
         // Java: m.put("es_GT", (short)4106);
         map.insert("es_GT".into(), 4106);
         // Java: m.put("es_HN", (short)18442);
         map.insert("es_HN".into(), 18442);
         // Java: m.put("es_MX", (short)2058);
         map.insert("es_MX".into(), 2058);
         // Java: m.put("es_NI", (short)19466);
         map.insert("es_NI".into(), 19466);
         // Java: m.put("es_PA", (short)6154);
         map.insert("es_PA".into(), 6154);
         // Java: m.put("es_PY", (short)15370);
         map.insert("es_PY".into(), 15370);
         // Java: m.put("es_PE", (short)10250);
         map.insert("es_PE".into(), 10250);
         // Java: m.put("es_PR", (short)20490);
         map.insert("es_PR".into(), 20490);
         // Java: m.put("es_UY", (short)14346);
         map.insert("es_UY".into(), 14346);
         // Java: m.put("es_VE", (short)8202);
         map.insert("es_VE".into(), 8202);
         // Java: m.put("es_ES", (short)3082);
         map.insert("es_ES".into(), 3082);
         // Java: m.put("sv_SE", (short)1053);
         map.insert("sv_SE".into(), 1053);
         // Java: m.put("th_TH", (short)1054);
         map.insert("th_TH".into(), 1054);
         // Java: m.put("tr_TR", (short)1055);
         map.insert("tr_TR".into(), 1055);
         // Java: m.put("uk_UA", (short)1058);
         map.insert("uk_UA".into(), 1058);
         // Java: m.put("vi_VN", (short)1066);
         map.insert("vi_VN".into(), 1066);
         // Java: m.put("yo_yo", (short)1130);
         map.insert("yo_yo".into(), 1130);
         // Java: m.put("hy_AM", (short)1067);
         map.insert("hy_AM".into(), 1067);
         // Java: m.put("am_ET", (short)1118);
         map.insert("am_ET".into(), 1118);
         // Java: m.put("bn_IN", (short)1093);
         map.insert("bn_IN".into(), 1093);
         // Java: m.put("bn_BD", (short)2117);
         map.insert("bn_BD".into(), 2117);
         // Java: m.put("bs_BA", (short)5146);
         map.insert("bs_BA".into(), 5146);
         // Java: m.put("br_FR", (short)1150);
         map.insert("br_FR".into(), 1150);
         // Java: m.put("en_JM", (short)8201);
         map.insert("en_JM".into(), 8201);
         // Java: m.put("en_PH", (short)13321);
         map.insert("en_PH".into(), 13321);
         // Java: m.put("en_ID", (short)14345);
         map.insert("en_ID".into(), 14345);
         // Java: m.put("en_SG", (short)18441);
         map.insert("en_SG".into(), 18441);
         // Java: m.put("en_TT", (short)11273);
         map.insert("en_TT".into(), 11273);
         // Java: m.put("en_ZW", (short)12297);
         map.insert("en_ZW".into(), 12297);
         // Java: m.put("af_ZA", (short)1078);
         map.insert("af_ZA".into(), 1078);
         // Java: m.put("gsw_FR", (short)1156);
         map.insert("gsw_FR".into(), 1156);
         // Java: m.put("as_IN", (short)1101);
         map.insert("as_IN".into(), 1101);
         // Java: m.put("az_Cyrl", (short)2092);
         map.insert("az_Cyrl".into(), 2092);
         // Java: m.put("az_AZ", (short)1068);
         map.insert("az_AZ".into(), 1068);
         // Java: m.put("ba_RU", (short)1133);
         map.insert("ba_RU".into(), 1133);
         // Java: m.put("eu_ES", (short)1069);
         map.insert("eu_ES".into(), 1069);
         // Java: m.put("my_MM", (short)1109);
         map.insert("my_MM".into(), 1109);
         // Java: m.put("chr_US", (short)1116);
         map.insert("chr_US".into(), 1116);
         // Java: m.put("fa_AF", (short)1164);
         map.insert("fa_AF".into(), 1164);
         // Java: m.put("dv_DV", (short)1125);
         map.insert("dv_DV".into(), 1125);
         // Java: m.put("en_BZ", (short)10249);
         map.insert("en_BZ".into(), 10249);
         // Java: m.put("en_IE", (short)6153);
         map.insert("en_IE".into(), 6153);
         // Java: m.put("en_HK", (short)15369);
         map.insert("en_HK".into(), 15369);
         // Java: m.put("fo_FO", (short)1080);
         map.insert("fo_FO".into(), 1080);
         // Java: m.put("fa_IR", (short)1065);
         map.insert("fa_IR".into(), 1065);
         // Java: m.put("fil_PH", (short)1124);
         map.insert("fil_PH".into(), 1124);
         // Java: m.put("fr_CI", (short)12300);
         map.insert("fr_CI".into(), 12300);
         // Java: m.put("fy_NL", (short)1122);
         map.insert("fy_NL".into(), 1122);
         // Java: m.put("gd_IE", (short)2108);
         map.insert("gd_IE".into(), 2108);
         // Java: m.put("gd_GB", (short)1084);
         map.insert("gd_GB".into(), 1084);
         // Java: m.put("gl_ES", (short)1110);
         map.insert("gl_ES".into(), 1110);
         // Java: m.put("ka_GE", (short)1079);
         map.insert("ka_GE".into(), 1079);
         // Java: m.put("gn_PY", (short)1140);
         map.insert("gn_PY".into(), 1140);
         // Java: m.put("gu_IN", (short)1095);
         map.insert("gu_IN".into(), 1095);
         // Java: m.put("ha_NE", (short)1128);
         map.insert("ha_NE".into(), 1128);
         // Java: m.put("haw_US", (short)1141);
         map.insert("haw_US".into(), 1141);
         // Java: m.put("ibb_NE", (short)1129);
         map.insert("ibb_NE".into(), 1129);
         // Java: m.put("ig_NE", (short)1136);
         map.insert("ig_NE".into(), 1136);
         // Java: m.put("id_ID", (short)1057);
         map.insert("id_ID".into(), 1057);
         // Java: m.put("iu_CA", (short)1117);
         map.insert("iu_CA".into(), 1117);
         // Java: m.put("kl_GL", (short)1135);
         map.insert("kl_GL".into(), 1135);
         // Java: m.put("kn_IN", (short)1099);
         map.insert("kn_IN".into(), 1099);
         // Java: m.put("kr_NE", (short)1137);
         map.insert("kr_NE".into(), 1137);
         // Java: m.put("ks_KS", (short)1120);
         map.insert("ks_KS".into(), 1120);
         // Java: m.put("ks_IN", (short)2144);
         map.insert("ks_IN".into(), 2144);
         // Java: m.put("kk_KZ", (short)1087);
         map.insert("kk_KZ".into(), 1087);
         // Java: m.put("km_KH", (short)1107);
         map.insert("km_KH".into(), 1107);
         // Java: m.put("quc_GT", (short)1158);
         map.insert("quc_GT".into(), 1158);
         // Java: m.put("rw_RW", (short)1159);
         map.insert("rw_RW".into(), 1159);
         // Java: m.put("ky_KG", (short)1088);
         map.insert("ky_KG".into(), 1088);
         // Java: m.put("kok_IN", (short)1111);
         map.insert("kok_IN".into(), 1111);
         // Java: m.put("lo_LA", (short)1108);
         map.insert("lo_LA".into(), 1108);
         // Java: m.put("lb_LU", (short)1134);
         map.insert("lb_LU".into(), 1134);
         // Java: m.put("ms_BN", (short)2110);
         map.insert("ms_BN".into(), 2110);
         // Java: m.put("ms_MY", (short)1086);
         map.insert("ms_MY".into(), 1086);
         // Java: m.put("mt_MT", (short)1082);
         map.insert("mt_MT".into(), 1082);
         // Java: m.put("mni_IN", (short)1112);
         map.insert("mni_IN".into(), 1112);
         // Java: m.put("mi_NZ", (short)1153);
         map.insert("mi_NZ".into(), 1153);
         // Java: m.put("arn_CL", (short)1146);
         map.insert("arn_CL".into(), 1146);
         // Java: m.put("mr_IN", (short)1102);
         map.insert("mr_IN".into(), 1102);
         // Java: m.put("moh_CA", (short)1148);
         map.insert("moh_CA".into(), 1148);
         // Java: m.put("mn_MN", (short)2128);
         map.insert("mn_MN".into(), 2128);
         // Java: m.put("ne_NP", (short)1121);
         map.insert("ne_NP".into(), 1121);
         // Java: m.put("ne_IN", (short)2145);
         map.insert("ne_IN".into(), 2145);
         // Java: m.put("oc_FR", (short)1154);
         map.insert("oc_FR".into(), 1154);
         // Java: m.put("or_IN", (short)1096);
         map.insert("or_IN".into(), 1096);
         // Java: m.put("om_KE", (short)1138);
         map.insert("om_KE".into(), 1138);
         // Java: m.put("pap_AW", (short)1145);
         map.insert("pap_AW".into(), 1145);
         // Java: m.put("ps_AF", (short)1123);
         map.insert("ps_AF".into(), 1123);
         // Java: m.put("pa_IN", (short)1094);
         map.insert("pa_IN".into(), 1094);
         // Java: m.put("pa_PK", (short)2118);
         map.insert("pa_PK".into(), 2118);
         // Java: m.put("quz_BO", (short)1131);
         map.insert("quz_BO".into(), 1131);
         // Java: m.put("quz_EC", (short)2155);
         map.insert("quz_EC".into(), 2155);
         // Java: m.put("quz_PE", (short)3179);
         map.insert("quz_PE".into(), 3179);
         // Java: m.put("rm_RM", (short)1047);
         map.insert("rm_RM".into(), 1047);
         // Java: m.put("ro_MD", (short)2072);
         map.insert("ro_MD".into(), 2072);
         // Java: m.put("ru_MD", (short)2073);
         map.insert("ru_MD".into(), 2073);
         // Java: m.put("se_NO", (short)1083);
         map.insert("se_NO".into(), 1083);
         // Java: m.put("sz", (short)1083);
         map.insert("sz".into(), 1083);
         // Java: m.put("smn_FL", (short)9275);
         map.insert("smn_FL".into(), 9275);
         // Java: m.put("smj_NO", (short)4155);
         map.insert("smj_NO".into(), 4155);
         // Java: m.put("smj_SE", (short)5179);
         map.insert("smj_SE".into(), 5179);
         // Java: m.put("se_FI", (short)3131);
         map.insert("se_FI".into(), 3131);
         // Java: m.put("se_SE", (short)2107);
         map.insert("se_SE".into(), 2107);
         // Java: m.put("sms_FI", (short)8251);
         map.insert("sms_FI".into(), 8251);
         // Java: m.put("sma_NO", (short)6203);
         map.insert("sma_NO".into(), 6203);
         // Java: m.put("sma_SE", (short)7227);
         map.insert("sma_SE".into(), 7227);
         // Java: m.put("sa_IN", (short)1103);
         map.insert("sa_IN".into(), 1103);
         // Java: m.put("nso", (short)1132);
         map.insert("nso".into(), 1132);
         // Java: m.put("sr_BA", (short)7194);
         map.insert("sr_BA".into(), 7194);
         // Java: m.put("nso_ZA", (short)1072);
         map.insert("nso_ZA".into(), 1072);
         // Java: m.put("sd_IN", (short)1113);
         map.insert("sd_IN".into(), 1113);
         // Java: m.put("sd_PK", (short)2137);
         map.insert("sd_PK".into(), 2137);
         // Java: m.put("so_SO", (short)1143);
         map.insert("so_SO".into(), 1143);
         // Java: m.put("hsb_DE", (short)1070);
         map.insert("hsb_DE".into(), 1070);
         // Java: m.put("dsb_DE", (short)2094);
         map.insert("dsb_DE".into(), 2094);
         // Java: m.put("es_US", (short)21514);
         map.insert("es_US".into(), 21514);
         // Java: m.put("sw_KE", (short)1089);
         map.insert("sw_KE".into(), 1089);
         // Java: m.put("sv_FI", (short)2077);
         map.insert("sv_FI".into(), 2077);
         // Java: m.put("syr_SY", (short)1114);
         map.insert("syr_SY".into(), 1114);
         // Java: m.put("tg_TJ", (short)1064);
         map.insert("tg_TJ".into(), 1064);
         // Java: m.put("tzm", (short)1119);
         map.insert("tzm".into(), 1119);
         // Java: m.put("tzm_Latn_DZ", (short)2143);
         map.insert("tzm_Latn_DZ".into(), 2143);
         // Java: m.put("ta_IN", (short)1097);
         map.insert("ta_IN".into(), 1097);
         // Java: m.put("tt_RU", (short)1092);
         map.insert("tt_RU".into(), 1092);
         // Java: m.put("te_IN", (short)1098);
         map.insert("te_IN".into(), 1098);
         // Java: m.put("bo_CN", (short)1105);
         map.insert("bo_CN".into(), 1105);
         // Java: m.put("dz_BT", (short)2129);
         map.insert("dz_BT".into(), 2129);
         // Java: m.put("bo_BT", (short)2129);
         map.insert("bo_BT".into(), 2129);
         // Java: m.put("ti_ER", (short)2163);
         map.insert("ti_ER".into(), 2163);
         // Java: m.put("ti_ET", (short)1139);
         map.insert("ti_ET".into(), 1139);
         // Java: m.put("ts_ZA", (short)1073);
         map.insert("ts_ZA".into(), 1073);
         // Java: m.put("tn_BW", (short)1074);
         map.insert("tn_BW".into(), 1074);
         // Java: m.put("tk_TM", (short)1090);
         map.insert("tk_TM".into(), 1090);
         // Java: m.put("ug_CN", (short)1152);
         map.insert("ug_CN".into(), 1152);
         // Java: m.put("ur_PK", (short)1056);
         map.insert("ur_PK".into(), 1056);
         // Java: m.put("ur_IN", (short)2080);
         map.insert("ur_IN".into(), 2080);
         // Java: m.put("uz_UZ", (short)2115);
         map.insert("uz_UZ".into(), 2115);
         // Java: m.put("ven_ZA", (short)1075);
         map.insert("ven_ZA".into(), 1075);
         // Java: m.put("cy_GB", (short)1106);
         map.insert("cy_GB".into(), 1106);
         // Java: m.put("wo_SN", (short)1160);
         map.insert("wo_SN".into(), 1160);
         // Java: m.put("xh_ZA", (short)1076);
         map.insert("xh_ZA".into(), 1076);
         // Java: m.put("sah_RU", (short)1157);
         map.insert("sah_RU".into(), 1157);
         // Java: m.put("ii_CN", (short)1144);
         map.insert("ii_CN".into(), 1144);
         // Java: m.put("zu_ZA", (short)1077);
         map.insert("zu_ZA".into(), 1077);
         // Java: m.put("ji", (short)1085);
         map.insert("ji".into(), 1085);
         // Java: m.put("de_LI", (short)5127);
         map.insert("de_LI".into(), 5127);
         // Java: m.put("fr_ZR", (short)9228);
         map.insert("fr_ZR".into(), 9228);
         // Java: m.put("fr_SN", (short)10252);
         map.insert("fr_SN".into(), 10252);
         // Java: m.put("fr_RE", (short)8204);
         map.insert("fr_RE".into(), 8204);
         // Java: m.put("fr_MA", (short)14348);
         map.insert("fr_MA".into(), 14348);
         // Java: m.put("fr_MC", (short)6156);
         map.insert("fr_MC".into(), 6156);
         // Java: m.put("fr_ML", (short)13324);
         map.insert("fr_ML".into(), 13324);
         // Java: m.put("fr_HT", (short)15372);
         map.insert("fr_HT".into(), 15372);
         // Java: m.put("fr_CM", (short)11276);
         map.insert("fr_CM".into(), 11276);
         // Java: m.put("co_FR", (short)1155);
         map.insert("co_FR".into(), 1155);
         Mutex::new(map)
      });
    }

    // Java: static short getPrimaryLang(short lang)
    pub fn getPrimaryLang(lang: i16) -> i16 {
      // Java: return (short)(lang & 1023);
      lang & 1023
    }

    // Java: static short getSubLang(short lang)
    pub fn getSubLang(lang: i16) -> i16 {
      // Java: return (short)((lang & -1024) >> 10);
      (lang & -1024) >> 10
    }

    // Java: static short getSystemLCID()
    pub fn getSystemLCID() -> i16 {
      // Java: return getLCID(getSystemLocale());
      Self::getLCID(&Self::getSystemLocale())
    }

    // Java: static boolean systemIsLang(short lang)
    pub fn systemIsLang(lang: i16) -> bool {
      // Java: return getPrimaryLang(getSystemLCID()) == getPrimaryLang(lang);
      Self::getPrimaryLang(Self::getSystemLCID()) == Self::getPrimaryLang(lang)
    }

    // Java: static String getSystemLocale()
    pub fn getSystemLocale() -> String {
      // Java: return mLocale.toString();
      M_LOCALE.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    // Java: static void setLocale(Locale locale)
    pub fn setLocale(locale: String) {
      // Java: mLocale = locale;
      *M_LOCALE.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = locale;
    }

    // Java: static Short[] getLangList()
    pub fn getLangList() -> Vec<i16> {
      // Java: getSystemLCID();
      Self::getSystemLCID();
      // Java: Short[] list = new Short[m.size()];
      // LLM JavaLine Aggregation: merged into values().copied().collect() below
      // Java: m.values().toArray(list);
      let m = LOCALE_MAP.get().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
      // Java: return list;
      m.values().copied().collect()
    }

}
