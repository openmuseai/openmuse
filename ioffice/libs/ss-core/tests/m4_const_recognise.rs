use serde_json::{json,Value};
use ss_core::cn::wps::et::ss::{constant::{ConstRecognisor::ConstRecognisor,RecogniseOption::RecogniseOption,NumfmtCategory::NumfmtCategory,RecogniseResult::RecogniseResult},statemachine::RecogniseResultRl::Value as CellValue};
use util::org::apache::poi::util::LanguageType::LanguageType;
fn decode(v:&Value)->Option<Vec<u16>>{v.as_str().map(|s|(0..s.len()).step_by(4).map(|i|u16::from_str_radix(&s[i..i+4],16).unwrap()).collect())}
fn hex(v:&[u16])->String{v.iter().map(|c|format!("{c:04x}")).collect()}
fn category(v:&Value)->NumfmtCategory{use NumfmtCategory::*;[NONE,GENERAL,NUMBER,ACCOUNTING,CURRENCY,DATE,TIME,DATE_TIME,PERCENTAGE,FRACTION,SCIENTIC,TEXT].into_iter().find(|c|format!("{c:?}")==v.as_str().unwrap()).unwrap()}
fn observe(r:&RecogniseResult)->Value{
    let flags=(r.hasGroupedFlag()as i32)|((r.hasDateFlag()as i32)<<1)|((r.hasTimeFlag()as i32)<<2)|((r.hasCurrencyFlag()as i32)<<3)|((r.hasErrorFlag()as i32)<<4)|((r.hasSqleadingFlag()as i32)<<5)|((r.hasPercentageFlag()as i32)<<6)|((r.hasFractionFlag()as i32)<<7)|((r.hasHyperlinkFlag()as i32)<<8)|((r.hasSenticFlag()as i32)<<9);
    let mut out=json!({"flags":flags,"formatIndex":r.getBuiltInNumfmtIdx(),"format":r.getSuggestedNumfmt()});match r.getValue(){Some(CellValue::Number(v))=>out["bits"]=json!(format!("{:016x}",v.to_bits())),Some(CellValue::Bool(v))=>out["bool"]=json!(v),Some(CellValue::Text(v))=>out["text"]=json!(hex(&v)),None=>out["text"]=Value::Null};out
}
#[test]
fn original_android_const_recognise_and_option_history(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/recognise-r0b27");let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut actual=vec![];let mut recognisor=ConstRecognisor::ConstRecognisor();
    for (i,row)in java["const"].as_array().unwrap().iter().enumerate(){let input=&row["input"];let s=decode(&input["text"]);let language=LanguageType::valueOf(input["language"].as_str().unwrap());if input["reuse"]!=true{recognisor=ConstRecognisor::ConstRecognisor__2(language);}recognisor.preferStateMachine(input["prefer"].as_bool().unwrap());recognisor.setKeepTailZero(input["keep"].as_bool().unwrap());recognisor.setOption(&RecogniseOption::RecogniseOption__2(!input["keep"].as_bool().unwrap(),input["force"].as_bool().unwrap(),input["date1904"].as_bool().unwrap(),category(&input["category"]),language));
        let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||recognisor.recognise(s.as_deref()).unwrap_or_else(|_|panic!("unexpected EvalFailure"))));let mut a=json!({"input":input});match r{Ok(r)=>a["result"]=r.as_ref().map_or(Value::Null,observe),Err(_)=>a["exception"]=json!("NumberFormatException")};let mut expected=row.clone();expected.as_object_mut().unwrap().remove("cachedDateFormat");assert_eq!(a,expected,"case {i}: {input}");actual.push(a);
    }
    std::fs::write(p.join("rust-const.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
