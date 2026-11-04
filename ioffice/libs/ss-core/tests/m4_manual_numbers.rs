use ss_core::cn::wps::et::ss::{manual::{IntegerRecognisor::IntegerRecognisor,FloatRecognisor::FloatRecognisor,SenticRecognisor::SenticRecognisor},constant::{PercentRecognisor::PercentRecognisor,RecogniseResult::RecogniseResult}};
use serde_json::{json,Value};
#[test]
fn original_android_manual_number_recognisers(){
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/raw-edit-r0b26");
    let java:Value=serde_json::from_slice(&std::fs::read(root.join("java.json")).unwrap()).unwrap();
    let mut actual=vec![];
    for (i,row) in java["manualNumbers"].as_array().unwrap().iter().enumerate(){
        let input=&row["input"];let s:Vec<u16>=input["text"].as_str().unwrap().encode_utf16().collect();let g=input["group"].as_u64().unwrap()as u16;let d=input["decimal"].as_u64().unwrap()as u16;
        let mut results=vec![];
        for kind in 0..5{
            let r=std::panic::catch_unwind(||match kind{
                0=>IntegerRecognisor::getInt(&s,0,s.len(),g,d),
                1=>IntegerRecognisor::getLocalInt(&s,0,s.len(),g,d,false),
                2=>FloatRecognisor::getFloat(&s,0,s.len(),g,d),
                3=>SenticRecognisor::getSentic(&s,0,s.len(),g,d),
                _=>PercentRecognisor::getPercent(&s,0,s.len(),g,d),
            });
            let result=match r{Ok(Some(r))=>observe(&r),Ok(None)=>Value::Null,Err(_)=>json!({"exception":"NumberFormatException"})};
            assert_eq!(result,row["results"][kind],"input {i}, kind {kind}: {input}");results.push(result);
        }
        actual.push(json!({"input":input,"results":results}));
    }
    std::fs::write(root.join("rust-manual-numbers.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();
}
fn observe(r:&RecogniseResult)->Value{json!({"bits":format!("{:016x}",r.getDblValue().unwrap().to_bits()),"format":r.getBuiltInNumfmtIdx(),"grouped":r.hasGroupedFlag(),"sentic":r.hasSenticFlag(),"percentage":r.hasPercentageFlag()})}
