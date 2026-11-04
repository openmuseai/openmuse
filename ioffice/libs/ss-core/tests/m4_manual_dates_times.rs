use serde_json::{json,Value};
use ss_core::cn::wps::et::ss::{manual::{DateRecognisor::DateRecognisor,TimeRecognisor::TimeRecognisor},constant::RecogniseResult::RecogniseResult,multlang::LangRes::LangRes};
use util::org::apache::poi::util::LanguageType::LanguageType;
fn decode(v:&Value)->Vec<u16>{let s=v.as_str().unwrap();(0..s.len()).step_by(4).map(|i|u16::from_str_radix(&s[i..i+4],16).unwrap()).collect()}
fn observe(r:&RecogniseResult)->Value{let flags=(r.hasGroupedFlag()as i32)|((r.hasDateFlag()as i32)<<1)|((r.hasTimeFlag()as i32)<<2)|((r.hasCurrencyFlag()as i32)<<3)|((r.hasErrorFlag()as i32)<<4)|((r.hasSqleadingFlag()as i32)<<5)|((r.hasPercentageFlag()as i32)<<6)|((r.hasFractionFlag()as i32)<<7)|((r.hasHyperlinkFlag()as i32)<<8)|((r.hasSenticFlag()as i32)<<9);json!({"bits":format!("{:016x}",r.getDblValue().unwrap().to_bits()),"flags":flags,"formatIndex":r.getBuiltInNumfmtIdx(),"format":r.getSuggestedNumfmt()})}
#[test]
fn original_android_date_parts(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for (i,row)in java["dateParts"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let s=decode(&input["text"]);let lang=LangRes::createByLangType(LanguageType::valueOf(input["language"].as_str().unwrap()));let d=input["date1904"].as_bool().unwrap();let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||match input["kind"].as_i64().unwrap(){0=>DateRecognisor::getDMMMY(&s,0,s.len(),d,lang.DATE.MONTHS.as_ref().unwrap()),1=>DateRecognisor::getEnglishDMMMY(&s,0,s.len(),d),_=>DateRecognisor::getEnglishMMMDY(&s,0,s.len(),d)}));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};assert_eq!(a,*row,"case {i}: {input}");actual.push(a);}
    std::fs::write(p.join("rust-date-parts.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
#[test]
fn original_android_dates(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for (i,row)in java["dates"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let s=decode(&input["text"]);let language=LanguageType::valueOf(input["language"].as_str().unwrap());let lang=LangRes::createByLangType(language);let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||DateRecognisor::getDate(&s,input["begin"].as_u64().unwrap()as usize,input["end"].as_u64().unwrap()as usize,input["date1904"].as_bool().unwrap(),language,&lang)));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};assert_eq!(a,*row,"case {i}: {input}");actual.push(a);}
    std::fs::write(p.join("rust-dates.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
#[test]
fn original_android_times(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];
    for (i,row)in java["times"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let s=decode(&input["text"]);let language=LanguageType::valueOf(input["language"].as_str().unwrap());let lang=LangRes::createByLangType(language);let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||TimeRecognisor::getTime(&s,input["begin"].as_u64().unwrap()as usize,input["end"].as_u64().unwrap()as usize,language,&lang)));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};assert_eq!(a,*row,"case {i}: {input}");actual.push(a);}
    std::fs::write(p.join("rust-times.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
