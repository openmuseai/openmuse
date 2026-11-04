use ss_core::cn::wps::et::ss::multlang::{LangRes::LangRes,res::{DateRes as D,SeparatorRes as S}};
use serde_json::{json,Value};
use util::org::apache::poi::util::LanguageType::LanguageType;
#[test]
fn original_java_language_resource_switch_and_constants(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/language-resources-r0b27");
    let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for row in java["languages"].as_array().unwrap(){
        let lang=LanguageType::valueOf(row["name"].as_str().unwrap());assert_eq!(lang.getValue(),row["code"].as_i64().unwrap()as i16);assert_eq!(lang.name(),row["name"].as_str().unwrap());let value=serde_json::to_value(LangRes::createByLangType(lang)).unwrap();
        assert_eq!(value,row["value"],"{}",row["name"]);actual.push(json!({"name":row["name"],"code":row["code"],"value":value}));
    }
    macro_rules! dates{($($n:ident),*)=>{json!({$(stringify!($n):D::$n),*})}}
    let date=dates!(ALSATIAN_FRANCE_MONTHS,ARABIC_1_MONTHS,ARABIC_2_MONTHS,ARABIC_3_MONTHS,GAELIC_IRELAND_MONTHS,OCCITAN_FRANCE_MONTHS,BRETON_FRANCE_MONTHS,GERMAN_MONTHS,RUSSIAN_MONTHS,FRENCH_MONTHS,FRISIAN_NETHERLANDS_MONTHS,DUTCH_MONTHS,LUXEMBOURGISH_LUXEMBOURG_MONTHS,PORTUGUESE_BRAZILIAN_MONTHS,PORTUGUESE_MONTHS,SWEDISH_MONTHS,SPANISH_MONTHS,ITALIAN_MONTHS,CHINESE_MONTHS,HEBREW_MONTHS);
    let sep=json!({"COMMON_SEPERATORS":S::COMMON_SEPERATORS,"AR_SEPERATORS":S::AR_SEPERATORS,"SPACE_GROUPER_SEPERATORS":S::SPACE_GROUPER_SEPERATORS,"DOT_GROUPER_SEPERATORS":S::DOT_GROUPER_SEPERATORS,"QUOT_GROUPER_SEPERATORS":S::QUOT_GROUPER_SEPERATORS});
    assert_eq!(date,java["dateTables"]);assert_eq!(sep,java["separatorTables"]);
    std::fs::write(p.join("rust.json"),serde_json::to_vec_pretty(&json!({"languages":actual,"dateTables":date,"separatorTables":sep})).unwrap()).unwrap();
}
