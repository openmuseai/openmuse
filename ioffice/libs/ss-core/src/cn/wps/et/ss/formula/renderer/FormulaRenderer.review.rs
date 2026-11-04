use super::*;
use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::{interfaces::{IEvalName::IEvalName,IEvalNameBase::IEvalNameBase,IEvalExtName::IEvalExtName,IEvalExtSheet::IEvalExtSheet},formula::{ptg::Ptg::{Ptg,ptgArray},ptg::ExternSheetNameResolver::ExternSheetNameResolver},dependent::Dependent::Dependent},org::apache::poi::ss::SpreadsheetVersion::{SpreadsheetVersion,CUR_VERSION}};
use serde_json::{Value,json};
struct State{p:Value,events:Rc<RefCell<Vec<Value>>>,sv:SpreadsheetVersion}
impl State{
 fn event(&self,m:&str,args:Value){self.events.borrow_mut().push(json!({"method":m,"args":args}));}
 fn cfg(&self)->&Value{&self.p["book"]}
 fn name(&self)->&Value{&self.cfg()["defined"]}
 fn sheet(&self,key:&str,index:i32)->Option<String>{self.cfg()[key].as_array().unwrap().get(index as usize).expect("ArrayIndexOutOfBoundsException").as_str().map(str::to_string)}
}
struct Book(Rc<State>);
struct Ext(Rc<State>);
struct Name(Rc<State>);
impl IEvalExtSheet for Ext{
 fn getBookType(&self)->i32{self.0.event("ext.getBookType",json!([]));self.0.cfg()["type"].as_i64().unwrap()as i32}
 fn getBookName(&self)->Option<String>{self.0.event("ext.getBookName",json!([]));self.0.cfg()["name"].as_str().map(str::to_string)}
 fn getFirstSheetIndex(&self)->i32{self.0.event("ext.getFirstSheetIndex",json!([]));self.0.cfg()["first"].as_i64().unwrap()as i32}
 fn getLastSheetIndex(&self)->i32{self.0.event("ext.getLastSheetIndex",json!([]));self.0.cfg()["last"].as_i64().unwrap()as i32}
 fn getSheetName(&self,index:i32)->Option<String>{self.0.event("ext.getSheetName",json!([index]));self.0.sheet("sheets",index)}
}
impl IEvalNameBase for Name{
 fn getNameText(&self)->String{self.0.event("name.getNameText",json!([]));self.0.name()["text"].as_str().unwrap().into()}
 fn getSheetIndexOneBased(&self)->i32{self.0.event("name.getSheetIndexOneBased",json!([]));self.0.name()["sheet"].as_i64().unwrap()as i32}
 fn as_any(&self)->&dyn std::any::Any{self}
 fn as_ext_name(&self)->Option<&dyn IEvalExtName>{if self.0.name()["external"]==true{Some(self)}else{None}}
}
impl IEvalName for Name{
 fn isFunctionName(&self)->bool{self.0.event("name.isFunctionName",json!([]));self.0.name()["function"].as_bool().unwrap()}
 fn hasFormula(&self)->bool{panic!("unexpected mock name.hasFormula")}
 fn getNameDefinition(&self)->crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray{panic!("unexpected mock name.getNameDefinition")}
 fn createNamePtg(&self)->Ptg{panic!("unexpected mock name.createNamePtg")}
 fn createNameXPtg(&self,_:i32)->Ptg{panic!("unexpected mock name.createNameXPtg")}
}
impl IEvalExtName for Name{fn getNameDefinition(&self)->Option<Vec<Ptg>>{panic!("unexpected mock external name.getNameDefinition")}}
impl IEvalWorkbook for Book{
 fn getOtherWorkbook(self:Rc<Self>,_:Option<&str>)->Option<Rc<dyn IEvalWorkbook>>{panic!("unexpected mock book.getOtherWorkbook")}
 fn getDependent(&self)->Option<Rc<RefCell<Dependent>>>{panic!("unexpected mock book.getDependent")}
 fn sheetIndex2SheetUid(&self,_:i32)->i32{panic!("unexpected mock book.sheetIndex2SheetUid")}
 fn getNameByNamePtg(&self,index:i32)->Option<Box<dyn IEvalName>>{self.0.event("book.getNameByNamePtg",json!([index]));if self.0.cfg()["nullName"]==true{return None;}assert!(self.0.name()["external"]==false,"ClassCastException: IEvalName proxy boundary");Some(Box::new(Name(self.0.clone())))}
 fn getNameByNameXPtg(&self,s:i32,n:i32)->Option<Box<dyn IEvalNameBase>>{self.0.event("book.getNameByNameXPtg",json!([s,n]));if self.0.cfg()["nullName"]==true{None}else{Some(Box::new(Name(self.0.clone())))}}
 fn getExtSheet(&self,index:i32)->Option<Box<dyn IEvalExtSheet>>{self.0.event("book.getExtSheet",json!([index]));if self.0.cfg()["nullExt"]==true{None}else{Some(Box::new(Ext(self.0.clone())))}}
 fn getSpreadsheetVersion(&self)->SpreadsheetVersion{self.0.event("book.getSpreadsheetVersion",json!([]));self.0.sv}
 fn getSheetName(&self,index:i32)->Option<String>{self.0.event("book.getSheetName",json!([index]));self.0.sheet("locals",index)}
 fn getFileName(&self)->Option<String>{self.0.event("book.getFileName",json!([]));self.0.cfg()["file"].as_str().map(str::to_string)}
}
fn call<T:serde::Serialize>(f:impl FnOnce()->T)->Value{match catch_unwind(AssertUnwindSafe(f)){Ok(v)=>json!({"result":"ok","value":v}),Err(e)=>{let s=e.downcast_ref::<String>().map(String::as_str).or_else(||e.downcast_ref::<&str>().copied()).unwrap_or("");let n=["NullPointerException","IllegalArgumentException","IllegalStateException","RuntimeException","ClassCastException","ArrayIndexOutOfBoundsException","NoSuchElementException","NegativeArraySizeException"].into_iter().find(|n|s.starts_with(n)).unwrap_or(s);json!({"result":n})}}}
fn range(v:&Value)->Option<CellRangeAddressBase>{if v.is_null(){None}else{Some(CellRangeAddressBase::CellRangeAddressBase(v[0].as_i64().unwrap()as i32,v[1].as_i64().unwrap()as i32,v[2].as_i64().unwrap()as i32,v[3].as_i64().unwrap()as i32))}}
fn one(p:&Value)->Value{let sv=if p["version"]==97{SpreadsheetVersion::EXCEL97}else{SpreadsheetVersion::EXCEL2007};*CUR_VERSION.lock().unwrap()=sv;let events=Rc::new(RefCell::new(vec![]));let state=Rc::new(State{p:p.clone(),events:events.clone(),sv});let book=Book(state);let b=if p["nullBook"]==true{None}else{Some(&book as &dyn IEvalWorkbook)};let opt=if p["nullOption"]==true{None}else{let r=RendererOption::createDefaultOption();{let mut o=r.borrow_mut();let a=&p["option"];o.DECIMAL_SEPARATOR=a["decimal"].as_u64().unwrap()as u16;o.PARAMETER_SEPARATOR=a["parameter"].as_u64().unwrap()as u16;o.UNION_SEPARATOR=a["union"].as_u64().unwrap()as u16;o.CONST_ARRAY_SEPARATOR=a["array"].as_u64().unwrap()as u16;o.CONST_ARRAY_ROW_SEPARATOR=a["rows"].as_u64().unwrap()as u16;o.formulaType=a["type"].as_i64().unwrap()as i32;o.ooxmlBookRefs=a["refs"].as_array().map(|a|Rc::new(RefCell::new(a.iter().map(|s|s.as_str().map(str::to_string)).collect::<Vec<_>>().into_boxed_slice())));}Some(r)};let opt=opt.as_ref().map(|o|o.borrow());let result=call(||{if p["mode"]=="resolver"{return Some(ExternSheetNameResolver::prependSheetName(b,p["extIndex"].as_i64().unwrap()as i32,p["cell"].as_str(),opt.as_deref()));}let ps=p["bytes"].as_str().map(|s|{let bytes=(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect::<Vec<_>>();let ps=ptgArray(Ptg::readTokens__stream(p["size"].as_i64().map_or(bytes.len()as i32,|n|n as i32),&bytes,p["external"]==true,sv));for n in p["nullTokens"].as_array().unwrap(){ps.borrow_mut()[n.as_u64().unwrap()as usize]=None;}ps});if p["mode"]=="fallback"{FormulaRenderer::toFormulaStringWithException(ps.as_ref(),b)}else{let f=range(&p["formula"]);let o=range(&p["origin"]);FormulaRenderer::toFormulaString__addresses(b,ps.as_ref(),opt.as_deref(),f.as_ref(),o.as_ref(),p["isName"].as_bool().unwrap())}});let event_values=events.borrow().clone();json!({"call":result,"events":event_values})}
fn compare(a:&Value,b:&Value,path:&str){match(a,b){(Value::Object(x),Value::Object(y))=>{assert_eq!(x.len(),y.len(),"{path}");for(k,v)in x{compare(v,y.get(k).unwrap(),&format!("{path}.{k}"));}},(Value::Array(x),Value::Array(y))=>{assert_eq!(x.len(),y.len(),"{path}");for(i,(a,b))in x.iter().zip(y).enumerate(){compare(a,b,&format!("{path}[{i}]"));}},_=>assert_eq!(a,b,"{path}")}}
#[test]
fn original_android_formula_renderer_and_workbook_dependent_call_order(){let p=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("rewrite-review/milestones/m4-edit-save/evidence/formula-render-r0b57");let input:Value=serde_json::from_slice(&std::fs::read(p.join("inputs.json")).unwrap()).unwrap();let java:Value=serde_json::from_slice(&std::fs::read(p.join("java.json")).unwrap()).unwrap();let actual=json!({"cases":input["cases"].as_array().unwrap().iter().map(one).collect::<Vec<_>>()});*CUR_VERSION.lock().unwrap()=SpreadsheetVersion::EXCEL2007;std::fs::write(p.join("rust.json"),serde_json::to_vec_pretty(&actual).unwrap()).unwrap();compare(&actual,&java,"root");assert!(Ptg::readTokens(8,&[32,0,0,0,0,0,0,0,0,255,127],SpreadsheetVersion::EXCEL2007).is_empty(),"original byte[] overload catches the negative tail RuntimeException subclass");}
