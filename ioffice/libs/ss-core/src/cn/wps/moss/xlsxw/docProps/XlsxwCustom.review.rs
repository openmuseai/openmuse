//! Direct original private method comparisons, without a product review facade.
use super::*;
use serde_json::{json,Value};
use kfc_kfc_alpha::kfc::xpxml::writer::r#impl::MemoryXmlWriter::MemoryXmlWriter;
fn data(v:&Value)->PropertyValue{match v["kind"].as_str().unwrap(){"null"=>PropertyValue::None,"string"=>PropertyValue::Str(v["value"].as_str().unwrap().into()),"bool"=>PropertyValue::Bool(v["value"].as_bool().unwrap()),"int"=>PropertyValue::Int(v["value"].as_i64().unwrap()as i32),"long"=>PropertyValue::Long(v["value"].as_i64().unwrap()),"double"=>PropertyValue::Double(v["value"].as_f64().unwrap()),"date"=>PropertyValue::Date(std::sync::Arc::new(crate::java_runtime::Date::fromTime(v["value"].as_i64().unwrap()))),_=>panic!("input kind")}}
fn exception(e:Box<dyn std::any::Any+Send>)->String{let s=e.downcast_ref::<String>().cloned().or_else(||e.downcast_ref::<&str>().map(|s|s.to_string())).unwrap_or_default();for name in ["NullPointerException","ClassCastException"]{if s.contains(name){return name.into();}}panic!("unexpected native failure {s}")}
#[test]
fn original_android_custom_property_types_nulls_and_partial_xml(){
    let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/docprops-r0b32");let j:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let mut out=vec![];
    for expected in j["properties"].as_array().unwrap(){let input=&expected["input"];let prop=if input["nullProperty"]==true{None}else{Some(Property::new(-1,input["type"].as_i64().unwrap(),data(&input["data"])))};let mut writer=MemoryXmlWriter::new();let r=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||XlsxwCustom::writeProperty(&mut writer,input["name"].as_str(),prop.as_ref(),input["pid"].as_i64().unwrap()as i32)));let result=r.map_or_else(exception,|_|"ok".into());let actual=json!({"input":input,"result":result,"xml":writer.getString()});assert_eq!(actual,*expected);out.push(actual);}
    std::fs::write(p.join("rust-properties.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();
}
