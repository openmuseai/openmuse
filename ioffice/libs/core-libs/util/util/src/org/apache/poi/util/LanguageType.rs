#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java field: short LANGUAGE_MASK_PRIMARY
pub const LANGUAGE_MASK_PRIMARY: i16 = 1023;

// Java: public enum LanguageType
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LanguageType {
    // Java field: short value
    value: i16,
    // rust: Java enum identity is distinct even when two constants share a short.
    identity:u16,
}

// Java: public enum LanguageType
impl LanguageType {
    // Java: LanguageType(short value)
    pub const fn new(value: i16) -> Self {
        // Java: this.value = value;
        LanguageType { value, identity:0 }
    }

    // Java: short getValue()
    pub fn getValue(&self) -> i16 {
        // Java: return this.value;
        self.value
    }

    // Java: short getPrimaryValue()
    pub fn getPrimaryValue(&self) -> i16 {
        // Java: return (short)(this.value & 1023);
        (self.value & 1023) as i16
    }
    // rust: literal enum identity adapter for the five duplicate-valued constants.
    const fn alias(value:i16,identity:u16)->Self{Self{value,identity}}
    // java compiler-generated LanguageType.valueOf(String name).
    pub fn valueOf(name:&str)->Self{
        for line in include_str!("LanguageType.names.tsv").lines(){let mut a=line.split('\t');let n=a.next().unwrap();let value=a.next().unwrap();let identity=a.next().unwrap();if n==name{return Self::alias(value.parse().unwrap(),identity.parse().unwrap());}}
        panic!("IllegalArgumentException: No enum constant LanguageType.{name}")
    }
    // java compiler-generated LanguageType.values().
    pub fn values()->Vec<Self>{include_str!("LanguageType.names.tsv").lines().map(|line|{let mut a=line.split('\t');a.next();Self::alias(a.next().unwrap().parse().unwrap(),a.next().unwrap().parse().unwrap())}).collect()}
    // java Enum.name().
    pub fn name(&self)->&'static str{
        for line in include_str!("LanguageType.names.tsv").lines(){let mut a=line.split('\t');let n=a.next().unwrap();let value:i16=a.next().unwrap().parse().unwrap();let identity:u16=a.next().unwrap().parse().unwrap();if value==self.value&&identity==self.identity{return n;}}
        panic!("native LanguageType has no Java enum identity")
    }
}

pub const LANGUAGE_DONTKNOW: LanguageType = LanguageType::new(1023);
pub const LANGUAGE_NONE: LanguageType = LanguageType::new(255);
pub const LANGUAGE_HID_HUMAN_INTERFACE_DEVICE: LanguageType = LanguageType::new(1279);
pub const LANGUAGE_INVARIANT: LanguageType = LanguageType::new(127);
pub const LANGUAGE_AFRIKAANS: LanguageType = LanguageType::new(1078);
pub const LANGUAGE_ALBANIAN: LanguageType = LanguageType::new(1052);
pub const LANGUAGE_ALSATIAN_FRANCE: LanguageType = LanguageType::new(1156);
pub const LANGUAGE_AMHARIC_ETHIOPIA: LanguageType = LanguageType::new(1118);
pub const LANGUAGE_ARABIC_ALGERIA: LanguageType = LanguageType::new(5121);
pub const LANGUAGE_ARABIC_BAHRAIN: LanguageType = LanguageType::new(15361);
pub const LANGUAGE_ARABIC_EGYPT: LanguageType = LanguageType::new(3073);
pub const LANGUAGE_ARABIC_IRAQ: LanguageType = LanguageType::new(2049);
pub const LANGUAGE_ARABIC_JORDAN: LanguageType = LanguageType::new(11265);
pub const LANGUAGE_ARABIC_KUWAIT: LanguageType = LanguageType::new(13313);
pub const LANGUAGE_ARABIC_LEBANON: LanguageType = LanguageType::new(12289);
pub const LANGUAGE_ARABIC_LIBYA: LanguageType = LanguageType::new(4097);
pub const LANGUAGE_ARABIC_MOROCCO: LanguageType = LanguageType::new(6145);
pub const LANGUAGE_ARABIC_OMAN: LanguageType = LanguageType::new(8193);
pub const LANGUAGE_ARABIC_QATAR: LanguageType = LanguageType::new(16385);
pub const LANGUAGE_ARABIC_SAUDI_ARABIA: LanguageType = LanguageType::new(1025);
pub const LANGUAGE_ARABIC_SYRIA: LanguageType = LanguageType::new(10241);
pub const LANGUAGE_ARABIC_TUNISIA: LanguageType = LanguageType::new(7169);
pub const LANGUAGE_ARABIC_UAE: LanguageType = LanguageType::new(14337);
pub const LANGUAGE_ARABIC_YEMEN: LanguageType = LanguageType::new(9217);
pub const LANGUAGE_ARABIC_PRIMARY_ONLY: LanguageType = LanguageType::new(1);
pub const LANGUAGE_ARMENIAN: LanguageType = LanguageType::new(1067);
pub const LANGUAGE_ASSAMESE: LanguageType = LanguageType::new(1101);
pub const LANGUAGE_AZERI: LanguageType = LanguageType::new(44);
pub const LANGUAGE_AZERI_CYRILLIC: LanguageType = LanguageType::new(2092);
pub const LANGUAGE_AZERI_LATIN: LanguageType = LanguageType::new(1068);
pub const LANGUAGE_BASHKIR_RUSSIA: LanguageType = LanguageType::new(1133);
pub const LANGUAGE_BASQUE: LanguageType = LanguageType::new(1069);
pub const LANGUAGE_BELARUSIAN: LanguageType = LanguageType::new(1059);
pub const LANGUAGE_BENGALI: LanguageType = LanguageType::new(1093);
pub const LANGUAGE_BENGALI_BANGLADESH: LanguageType = LanguageType::new(2117);
pub const LANGUAGE_BOSNIAN_LATIN_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::new(5146);
pub const LANGUAGE_BOSNIAN_CYRILLIC_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::new(8218);
pub const LANGUAGE_BOSNIAN_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::alias(5146,1);
pub const LANGUAGE_BRETON_FRANCE: LanguageType = LanguageType::new(1150);
pub const LANGUAGE_BULGARIAN: LanguageType = LanguageType::new(1026);
pub const LANGUAGE_BURMESE: LanguageType = LanguageType::new(1109);
pub const LANGUAGE_CATALAN: LanguageType = LanguageType::new(1027);
pub const LANGUAGE_CHEROKEE_UNITED_STATES: LanguageType = LanguageType::new(1116);
pub const LANGUAGE_CHINESE: LanguageType = LanguageType::new(4);
pub const LANGUAGE_CHINESE_HONGKONG: LanguageType = LanguageType::new(3076);
pub const LANGUAGE_CHINESE_MACAU: LanguageType = LanguageType::new(5124);
pub const LANGUAGE_CHINESE_SIMPLIFIED: LanguageType = LanguageType::new(2052);
pub const LANGUAGE_CHINESE_SINGAPORE: LanguageType = LanguageType::new(4100);
pub const LANGUAGE_CHINESE_TRADITIONAL: LanguageType = LanguageType::new(1028);
pub const LANGUAGE_CORSICAN_FRANCE: LanguageType = LanguageType::new(1155);
pub const LANGUAGE_CROATIAN: LanguageType = LanguageType::new(1050);
pub const LANGUAGE_CROATIAN_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::new(4122);
pub const LANGUAGE_CZECH: LanguageType = LanguageType::new(1029);
pub const LANGUAGE_DANISH: LanguageType = LanguageType::new(1030);
pub const LANGUAGE_DARI_AFGHANISTAN: LanguageType = LanguageType::new(1164);
pub const LANGUAGE_DHIVEHI: LanguageType = LanguageType::new(1125);
pub const LANGUAGE_DUTCH: LanguageType = LanguageType::new(1043);
pub const LANGUAGE_DUTCH_BELGIAN: LanguageType = LanguageType::new(2067);
pub const LANGUAGE_EDO: LanguageType = LanguageType::new(1126);
pub const LANGUAGE_ENGLISH: LanguageType = LanguageType::new(9);
pub const LANGUAGE_ENGLISH_AUS: LanguageType = LanguageType::new(3081);
pub const LANGUAGE_ENGLISH_BELIZE: LanguageType = LanguageType::new(10249);
pub const LANGUAGE_ENGLISH_CAN: LanguageType = LanguageType::new(4105);
pub const LANGUAGE_ENGLISH_CARRIBEAN: LanguageType = LanguageType::new(9225);
pub const LANGUAGE_ENGLISH_EIRE: LanguageType = LanguageType::new(6153);
pub const LANGUAGE_ENGLISH_HONG_KONG_SAR: LanguageType = LanguageType::new(15369);
pub const LANGUAGE_ENGLISH_INDIA: LanguageType = LanguageType::new(16393);
pub const LANGUAGE_ENGLISH_INDONESIA: LanguageType = LanguageType::new(14345);
pub const LANGUAGE_ENGLISH_JAMAICA: LanguageType = LanguageType::new(8201);
pub const LANGUAGE_ENGLISH_MALAYSIA: LanguageType = LanguageType::new(17417);
pub const LANGUAGE_ENGLISH_NZ: LanguageType = LanguageType::new(5129);
pub const LANGUAGE_ENGLISH_PHILIPPINES: LanguageType = LanguageType::new(13321);
pub const LANGUAGE_ENGLISH_SAFRICA: LanguageType = LanguageType::new(7177);
pub const LANGUAGE_ENGLISH_SINGAPORE: LanguageType = LanguageType::new(18441);
pub const LANGUAGE_ENGLISH_TRINIDAD: LanguageType = LanguageType::new(11273);
pub const LANGUAGE_ENGLISH_UK: LanguageType = LanguageType::new(2057);
pub const LANGUAGE_ENGLISH_US: LanguageType = LanguageType::new(1033);
pub const LANGUAGE_ENGLISH_ZIMBABWE: LanguageType = LanguageType::new(12297);
pub const LANGUAGE_ESTONIAN: LanguageType = LanguageType::new(1061);
pub const LANGUAGE_FAEROESE: LanguageType = LanguageType::new(1080);
pub const LANGUAGE_FARSI: LanguageType = LanguageType::new(1065);
pub const LANGUAGE_FILIPINO: LanguageType = LanguageType::new(1124);
pub const LANGUAGE_FINNISH: LanguageType = LanguageType::new(1035);
pub const LANGUAGE_FRENCH: LanguageType = LanguageType::new(1036);
pub const LANGUAGE_FRENCH_BELGIAN: LanguageType = LanguageType::new(2060);
pub const LANGUAGE_FRENCH_CAMEROON: LanguageType = LanguageType::new(11276);
pub const LANGUAGE_FRENCH_CANADIAN: LanguageType = LanguageType::new(3084);
pub const LANGUAGE_FRENCH_COTE_D_IVOIRE: LanguageType = LanguageType::new(12300);
pub const LANGUAGE_FRENCH_HAITI: LanguageType = LanguageType::new(15372);
pub const LANGUAGE_FRENCH_LUXEMBOURG: LanguageType = LanguageType::new(5132);
pub const LANGUAGE_FRENCH_MALI: LanguageType = LanguageType::new(13324);
pub const LANGUAGE_FRENCH_MONACO: LanguageType = LanguageType::new(6156);
pub const LANGUAGE_FRENCH_MOROCCO: LanguageType = LanguageType::new(14348);
pub const LANGUAGE_FRENCH_NORTH_AFRICA: LanguageType = LanguageType::new(-7156);
pub const LANGUAGE_FRENCH_REUNION: LanguageType = LanguageType::new(8204);
pub const LANGUAGE_FRENCH_SENEGAL: LanguageType = LanguageType::new(10252);
pub const LANGUAGE_FRENCH_SWISS: LanguageType = LanguageType::new(4108);
pub const LANGUAGE_FRENCH_WEST_INDIES: LanguageType = LanguageType::new(7180);
pub const LANGUAGE_FRENCH_ZAIRE: LanguageType = LanguageType::new(9228);
pub const LANGUAGE_FRISIAN_NETHERLANDS: LanguageType = LanguageType::new(1122);
pub const LANGUAGE_FULFULDE_NIGERIA: LanguageType = LanguageType::new(1127);
pub const LANGUAGE_GAELIC_IRELAND: LanguageType = LanguageType::new(2108);
pub const LANGUAGE_GAELIC_SCOTLAND: LanguageType = LanguageType::new(1084);
pub const LANGUAGE_GALICIAN: LanguageType = LanguageType::new(1110);
pub const LANGUAGE_GEORGIAN: LanguageType = LanguageType::new(1079);
pub const LANGUAGE_GERMAN: LanguageType = LanguageType::new(1031);
pub const LANGUAGE_GERMAN_AUSTRIAN: LanguageType = LanguageType::new(3079);
pub const LANGUAGE_GERMAN_LIECHTENSTEIN: LanguageType = LanguageType::new(5127);
pub const LANGUAGE_GERMAN_LUXEMBOURG: LanguageType = LanguageType::new(4103);
pub const LANGUAGE_GERMAN_SWISS: LanguageType = LanguageType::new(2055);
pub const LANGUAGE_GREEK: LanguageType = LanguageType::new(1032);
pub const LANGUAGE_GUARANI_PARAGUAY: LanguageType = LanguageType::new(1140);
pub const LANGUAGE_GUJARATI: LanguageType = LanguageType::new(1095);
pub const LANGUAGE_HAUSA_NIGERIA: LanguageType = LanguageType::new(1128);
pub const LANGUAGE_HAWAIIAN_UNITED_STATES: LanguageType = LanguageType::new(1141);
pub const LANGUAGE_HEBREW: LanguageType = LanguageType::new(1037);
pub const LANGUAGE_HINDI: LanguageType = LanguageType::new(57);
pub const LANGUAGE_HINDI_INDI: LanguageType = LanguageType::new(1081);
pub const LANGUAGE_HUNGARIAN: LanguageType = LanguageType::new(1038);
pub const LANGUAGE_IBIBIO_NIGERIA: LanguageType = LanguageType::new(1129);
pub const LANGUAGE_ICELANDIC: LanguageType = LanguageType::new(1039);
pub const LANGUAGE_IGBO_NIGERIA: LanguageType = LanguageType::new(1136);
pub const LANGUAGE_INDONESIAN: LanguageType = LanguageType::new(1057);
pub const LANGUAGE_INUKTITUT_SYLLABICS_CANADA: LanguageType = LanguageType::new(1117);
pub const LANGUAGE_INUKTITUT_LATIN_CANADA: LanguageType = LanguageType::new(2141);
pub const LANGUAGE_ITALIAN: LanguageType = LanguageType::new(1040);
pub const LANGUAGE_ITALIAN_SWISS: LanguageType = LanguageType::new(2064);
pub const LANGUAGE_JAPANESE: LanguageType = LanguageType::new(1041);
pub const LANGUAGE_KALAALLISUT_GREENLAND: LanguageType = LanguageType::new(1135);
pub const LANGUAGE_KANNADA: LanguageType = LanguageType::new(1099);
pub const LANGUAGE_KANURI_NIGERIA: LanguageType = LanguageType::new(1137);
pub const LANGUAGE_KASHMIRI: LanguageType = LanguageType::new(1120);
pub const LANGUAGE_KASHMIRI_INDIA: LanguageType = LanguageType::new(2144);
pub const LANGUAGE_KAZAK: LanguageType = LanguageType::new(1087);
pub const LANGUAGE_KHMER: LanguageType = LanguageType::new(1107);
pub const LANGUAGE_KICHE_GUATEMALA: LanguageType = LanguageType::new(1158);
pub const LANGUAGE_KINYARWANDA_RWANDA: LanguageType = LanguageType::new(1159);
pub const LANGUAGE_KIRGHIZ: LanguageType = LanguageType::new(1088);
pub const LANGUAGE_KONKANI: LanguageType = LanguageType::new(1111);
pub const LANGUAGE_KOREAN: LanguageType = LanguageType::new(1042);
pub const LANGUAGE_KOREAN_JOHAB: LanguageType = LanguageType::new(2066);
pub const LANGUAGE_LAO: LanguageType = LanguageType::new(1108);
pub const LANGUAGE_LATIN: LanguageType = LanguageType::new(1142);
pub const LANGUAGE_LATVIAN: LanguageType = LanguageType::new(1062);
pub const LANGUAGE_LITHUANIAN: LanguageType = LanguageType::new(1063);
pub const LANGUAGE_LITHUANIAN_CLASSIC: LanguageType = LanguageType::new(2087);
pub const LANGUAGE_LUXEMBOURGISH_LUXEMBOURG: LanguageType = LanguageType::new(1134);
pub const LANGUAGE_MACEDONIAN: LanguageType = LanguageType::new(1071);
pub const LANGUAGE_MALAY: LanguageType = LanguageType::new(62);
pub const LANGUAGE_MALAYALAM: LanguageType = LanguageType::new(1100);
pub const LANGUAGE_MALAY_BRUNEI_DARUSSALAM: LanguageType = LanguageType::new(2110);
pub const LANGUAGE_MALAY_MALAYSIA: LanguageType = LanguageType::new(1086);
pub const LANGUAGE_MALTESE: LanguageType = LanguageType::new(1082);
pub const LANGUAGE_MANIPURI: LanguageType = LanguageType::new(1112);
pub const LANGUAGE_MAORI_NEW_ZEALAND: LanguageType = LanguageType::new(1153);
pub const LANGUAGE_MAPUDUNGUN_CHILE: LanguageType = LanguageType::new(1146);
pub const LANGUAGE_MARATHI: LanguageType = LanguageType::new(1102);
pub const LANGUAGE_MOHAWK_CANADA: LanguageType = LanguageType::new(1148);
pub const LANGUAGE_MONGOLIAN: LanguageType = LanguageType::new(1104);
pub const LANGUAGE_MONGOLIAN_MONGOLIAN: LanguageType = LanguageType::new(2128);
pub const LANGUAGE_NEPALI: LanguageType = LanguageType::new(1121);
pub const LANGUAGE_NEPALI_INDIA: LanguageType = LanguageType::new(2145);
pub const LANGUAGE_NORWEGIAN: LanguageType = LanguageType::new(20);
pub const LANGUAGE_NORWEGIAN_BOKMAL: LanguageType = LanguageType::new(1044);
pub const LANGUAGE_NORWEGIAN_NYNORSK: LanguageType = LanguageType::new(2068);
pub const LANGUAGE_OCCITAN_FRANCE: LanguageType = LanguageType::new(1154);
pub const LANGUAGE_ORIYA: LanguageType = LanguageType::new(1096);
pub const LANGUAGE_OROMO: LanguageType = LanguageType::new(1138);
pub const LANGUAGE_PAPIAMENTU: LanguageType = LanguageType::new(1145);
pub const LANGUAGE_PASHTO: LanguageType = LanguageType::new(1123);
pub const LANGUAGE_POLISH: LanguageType = LanguageType::new(1045);
pub const LANGUAGE_PORTUGUESE: LanguageType = LanguageType::new(2070);
pub const LANGUAGE_PORTUGUESE_BRAZILIAN: LanguageType = LanguageType::new(1046);
pub const LANGUAGE_PUNJABI: LanguageType = LanguageType::new(1094);
pub const LANGUAGE_PUNJABI_PAKISTAN: LanguageType = LanguageType::new(2118);
pub const LANGUAGE_QUECHUA_BOLIVIA: LanguageType = LanguageType::new(1131);
pub const LANGUAGE_QUECHUA_ECUADOR: LanguageType = LanguageType::new(2155);
pub const LANGUAGE_QUECHUA_PERU: LanguageType = LanguageType::new(3179);
pub const LANGUAGE_RHAETO_ROMAN: LanguageType = LanguageType::new(1047);
pub const LANGUAGE_ROMANIAN: LanguageType = LanguageType::new(1048);
pub const LANGUAGE_ROMANIAN_MOLDOVA: LanguageType = LanguageType::new(2072);
pub const LANGUAGE_RUSSIAN: LanguageType = LanguageType::new(1049);
pub const LANGUAGE_RUSSIAN_MOLDOVA: LanguageType = LanguageType::new(2073);
pub const LANGUAGE_SAMI_NORTHERN_NORWAY: LanguageType = LanguageType::new(1083);
pub const LANGUAGE_SAMI_LAPPISH: LanguageType = LanguageType::alias(1083,1);
pub const LANGUAGE_SAMI_INARI: LanguageType = LanguageType::new(9275);
pub const LANGUAGE_SAMI_LULE_NORWAY: LanguageType = LanguageType::new(4155);
pub const LANGUAGE_SAMI_LULE_SWEDEN: LanguageType = LanguageType::new(5179);
pub const LANGUAGE_SAMI_NORTHERN_FINLAND: LanguageType = LanguageType::new(3131);
pub const LANGUAGE_SAMI_NORTHERN_SWEDEN: LanguageType = LanguageType::new(2107);
pub const LANGUAGE_SAMI_SKOLT: LanguageType = LanguageType::new(8251);
pub const LANGUAGE_SAMI_SOUTHERN_NORWAY: LanguageType = LanguageType::new(6203);
pub const LANGUAGE_SAMI_SOUTHERN_SWEDEN: LanguageType = LanguageType::new(7227);
pub const LANGUAGE_SANSKRIT: LanguageType = LanguageType::new(1103);
pub const LANGUAGE_SEPEDI: LanguageType = LanguageType::new(1132);
pub const LANGUAGE_NORTHERNSOTHO: LanguageType = LanguageType::alias(1132,1);
pub const LANGUAGE_SERBIAN: LanguageType = LanguageType::new(26);
pub const LANGUAGE_SERBIAN_CYRILLIC: LanguageType = LanguageType::new(3098);
pub const LANGUAGE_SERBIAN_CYRILLIC_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::new(7194);
pub const LANGUAGE_SERBIAN_LATIN: LanguageType = LanguageType::new(2074);
pub const LANGUAGE_SERBIAN_LATIN_BOSNIA_HERZEGOVINA: LanguageType = LanguageType::new(6170);
pub const LANGUAGE_SERBIAN_LATIN_NEUTRAL: LanguageType = LanguageType::new(31770);
pub const LANGUAGE_SESOTHO: LanguageType = LanguageType::new(1072);
pub const LANGUAGE_SINDHI: LanguageType = LanguageType::new(1113);
pub const LANGUAGE_SINDHI_PAKISTAN: LanguageType = LanguageType::new(2137);
pub const LANGUAGE_SINHALESE_SRI_LANKA: LanguageType = LanguageType::new(1115);
pub const LANGUAGE_SLOVAK: LanguageType = LanguageType::new(1051);
pub const LANGUAGE_SLOVENIAN: LanguageType = LanguageType::new(1060);
pub const LANGUAGE_SOMALI: LanguageType = LanguageType::new(1143);
pub const LANGUAGE_UPPER_SORBIAN_GERMANY: LanguageType = LanguageType::new(1070);
pub const LANGUAGE_LOWER_SORBIAN_GERMANY: LanguageType = LanguageType::new(2094);
pub const LANGUAGE_SPANISH_DATED: LanguageType = LanguageType::new(1034);
pub const LANGUAGE_SPANISH_ARGENTINA: LanguageType = LanguageType::new(11274);
pub const LANGUAGE_SPANISH_BOLIVIA: LanguageType = LanguageType::new(16394);
pub const LANGUAGE_SPANISH_CHILE: LanguageType = LanguageType::new(13322);
pub const LANGUAGE_SPANISH_COLOMBIA: LanguageType = LanguageType::new(9226);
pub const LANGUAGE_SPANISH_COSTARICA: LanguageType = LanguageType::new(5130);
pub const LANGUAGE_SPANISH_DOMINICAN_REPUBLIC: LanguageType = LanguageType::new(7178);
pub const LANGUAGE_SPANISH_ECUADOR: LanguageType = LanguageType::new(12298);
pub const LANGUAGE_SPANISH_EL_SALVADOR: LanguageType = LanguageType::new(17418);
pub const LANGUAGE_SPANISH_GUATEMALA: LanguageType = LanguageType::new(4106);
pub const LANGUAGE_SPANISH_HONDURAS: LanguageType = LanguageType::new(18442);
pub const LANGUAGE_SPANISH_LATIN_AMERICA: LanguageType = LanguageType::new(-7158);
pub const LANGUAGE_SPANISH_MEXICAN: LanguageType = LanguageType::new(2058);
pub const LANGUAGE_SPANISH_MODERN: LanguageType = LanguageType::new(3082);
pub const LANGUAGE_SPANISH_NICARAGUA: LanguageType = LanguageType::new(19466);
pub const LANGUAGE_SPANISH_PANAMA: LanguageType = LanguageType::new(6154);
pub const LANGUAGE_SPANISH_PARAGUAY: LanguageType = LanguageType::new(15370);
pub const LANGUAGE_SPANISH_PERU: LanguageType = LanguageType::new(10250);
pub const LANGUAGE_SPANISH_PUERTO_RICO: LanguageType = LanguageType::new(20490);
pub const LANGUAGE_SPANISH_UNITED_STATES: LanguageType = LanguageType::new(21514);
pub const LANGUAGE_SPANISH_URUGUAY: LanguageType = LanguageType::new(14346);
pub const LANGUAGE_SPANISH_VENEZUELA: LanguageType = LanguageType::new(8202);
pub const LANGUAGE_SPANISH: LanguageType = LanguageType::alias(3082,1);
pub const LANGUAGE_SWAHILI: LanguageType = LanguageType::new(1089);
pub const LANGUAGE_SWEDISH: LanguageType = LanguageType::new(1053);
pub const LANGUAGE_SWEDISH_FINLAND: LanguageType = LanguageType::new(2077);
pub const LANGUAGE_SYRIAC: LanguageType = LanguageType::new(1114);
pub const LANGUAGE_TAJIK: LanguageType = LanguageType::new(1064);
pub const LANGUAGE_TAMAZIGHT_ARABIC: LanguageType = LanguageType::new(1119);
pub const LANGUAGE_TAMAZIGHT_LATIN: LanguageType = LanguageType::new(2143);
pub const LANGUAGE_TAMIL: LanguageType = LanguageType::new(1097);
pub const LANGUAGE_TATAR: LanguageType = LanguageType::new(1092);
pub const LANGUAGE_TELUGU: LanguageType = LanguageType::new(1098);
pub const LANGUAGE_THAI: LanguageType = LanguageType::new(1054);
pub const LANGUAGE_TIBETAN: LanguageType = LanguageType::new(1105);
pub const LANGUAGE_DZONGKHA: LanguageType = LanguageType::new(2129);
pub const LANGUAGE_TIBETAN_BHUTAN: LanguageType = LanguageType::alias(2129,1);
pub const LANGUAGE_TIGRIGNA_ERITREA: LanguageType = LanguageType::new(2163);
pub const LANGUAGE_TIGRIGNA_ETHIOPIA: LanguageType = LanguageType::new(1139);
pub const LANGUAGE_TSONGA: LanguageType = LanguageType::new(1073);
pub const LANGUAGE_TSWANA: LanguageType = LanguageType::new(1074);
pub const LANGUAGE_TURKISH: LanguageType = LanguageType::new(1055);
pub const LANGUAGE_TURKMEN: LanguageType = LanguageType::new(1090);
pub const LANGUAGE_UIGHUR_CHINA: LanguageType = LanguageType::new(1152);
pub const LANGUAGE_UKRAINIAN: LanguageType = LanguageType::new(1058);
pub const LANGUAGE_URDU: LanguageType = LanguageType::new(32);
pub const LANGUAGE_URDU_INDIA: LanguageType = LanguageType::new(2080);
pub const LANGUAGE_URDU_PAKISTAN: LanguageType = LanguageType::new(1056);
pub const LANGUAGE_UZBEK_CYRILLIC: LanguageType = LanguageType::new(2115);
pub const LANGUAGE_UZBEK_LATIN: LanguageType = LanguageType::new(1091);
pub const LANGUAGE_VENDA: LanguageType = LanguageType::new(1075);
pub const LANGUAGE_VIETNAMESE: LanguageType = LanguageType::new(1066);
pub const LANGUAGE_WELSH: LanguageType = LanguageType::new(1106);
pub const LANGUAGE_WOLOF_SENEGAL: LanguageType = LanguageType::new(1160);
pub const LANGUAGE_XHOSA: LanguageType = LanguageType::new(1076);
pub const LANGUAGE_YAKUT_RUSSIA: LanguageType = LanguageType::new(1157);
pub const LANGUAGE_YI: LanguageType = LanguageType::new(1144);
pub const LANGUAGE_YIDDISH: LanguageType = LanguageType::new(1085);
pub const LANGUAGE_YORUBA: LanguageType = LanguageType::new(1130);
pub const LANGUAGE_ZULU: LanguageType = LanguageType::new(1077);
pub const LANGUAGE_USER1: LanguageType = LanguageType::new(513);
pub const LANGUAGE_USER2: LanguageType = LanguageType::new(514);
pub const LANGUAGE_USER3: LanguageType = LanguageType::new(515);
pub const LANGUAGE_USER4: LanguageType = LanguageType::new(516);
pub const LANGUAGE_USER5: LanguageType = LanguageType::new(517);
pub const LANGUAGE_USER6: LanguageType = LanguageType::new(518);
pub const LANGUAGE_USER7: LanguageType = LanguageType::new(519);
pub const LANGUAGE_USER8: LanguageType = LanguageType::new(520);
pub const LANGUAGE_USER9: LanguageType = LanguageType::new(521);

// Java field: short LANGUAGE_MASK_SUBLANGUAGE
pub const LANGUAGE_MASK_SUBLANGUAGE: i16 = !LANGUAGE_MASK_PRIMARY;
// Java field: short LANGUAGE_UNSPECIFIED
pub const LANGUAGE_UNSPECIFIED: i16 = 0x0000;
// Java field: short LANGUAGE_NEUTRAL
pub const LANGUAGE_NEUTRAL: i16 = 0x0000;
