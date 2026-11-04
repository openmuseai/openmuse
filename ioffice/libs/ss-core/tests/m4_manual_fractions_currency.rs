use serde_json::{json,Value};
use ss_core::cn::wps::et::ss::{manual::{FractionRecognisor::FractionRecognisor,CurrencyRecognisor::CurrencyRecognisor},constant::{NumfmtCategory::NumfmtCategory,RecogniseResult::RecogniseResult},multlang::LangRes::LangRes};
use util::org::apache::poi::util::LanguageType::LanguageType;
fn decode(v:&Value)->Vec<u16>{let s=v.as_str().unwrap();(0..s.len()).step_by(4).map(|i|u16::from_str_radix(&s[i..i+4],16).unwrap()).collect()}
fn category(v:&Value)->NumfmtCategory{use NumfmtCategory::*;[NONE,GENERAL,NUMBER,ACCOUNTING,CURRENCY,DATE,TIME,DATE_TIME,PERCENTAGE,FRACTION,SCIENTIC,TEXT].into_iter().find(|c|format!("{c:?}")==v.as_str().unwrap()).unwrap()}
fn observe(r:&RecogniseResult)->Value{let flags=(r.hasGroupedFlag()as i32)|((r.hasDateFlag()as i32)<<1)|((r.hasTimeFlag()as i32)<<2)|((r.hasCurrencyFlag()as i32)<<3)|((r.hasErrorFlag()as i32)<<4)|((r.hasSqleadingFlag()as i32)<<5)|((r.hasPercentageFlag()as i32)<<6)|((r.hasFractionFlag()as i32)<<7)|((r.hasHyperlinkFlag()as i32)<<8)|((r.hasSenticFlag()as i32)<<9);json!({"bits":format!("{:016x}",r.getDblValue().unwrap().to_bits()),"flags":flags,"formatIndex":r.getBuiltInNumfmtIdx(),"format":r.getSuggestedNumfmt()})}
#[test]
fn original_android_fractions(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for (i,row)in java["fractions"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let s=decode(&input["text"]);let lang=LangRes::createByLangType(LanguageType::valueOf(input["language"].as_str().unwrap()));let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||FractionRecognisor::getFraction(&s,input["begin"].as_u64().unwrap()as usize,input["end"].as_u64().unwrap()as usize,&lang,category(&input["category"]))));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};assert_eq!(a,*row,"case {i}: {input}");actual.push(a);}
    std::fs::write(p.join("rust-fractions.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
#[test]
fn original_android_currencies(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for (i,row)in java["currencies"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let lower:Vec<u16>=input["currency"].as_str().unwrap().encode_utf16().map(ss_core::java_runtime::Character::toLowerCase).collect();assert_eq!(String::from_utf16(&lower).unwrap(),input["lower"].as_str().unwrap());let s=decode(&input["text"]);let lang=LangRes::createByLangType(LanguageType::valueOf(input["language"].as_str().unwrap()));let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||CurrencyRecognisor::getCurrency(&s,input["begin"].as_u64().unwrap()as usize,input["end"].as_u64().unwrap()as usize,&lang)));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};assert_eq!(a,*row,"case {i}: {input}");actual.push(a);}
    std::fs::write(p.join("rust-currencies.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
