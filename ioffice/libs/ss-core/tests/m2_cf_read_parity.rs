use ss_core::org::apache::poi::ss::util::NumberComparer::NumberComparer;
#[test]
fn original_java_number_comparer(){
 let root=format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/java/numbercompare/expected.tsv",env!("CARGO_MANIFEST_DIR"));
 for(line,s)in std::fs::read_to_string(root).unwrap().lines().enumerate(){let v=s.split('\t').collect::<Vec<_>>();let a=f64::from_bits(u64::from_str_radix(v[0],16).unwrap());let b=f64::from_bits(u64::from_str_radix(v[1],16).unwrap());assert_eq!(NumberComparer::compare(a,b),v[2].parse::<i32>().unwrap(),"line {line}: {s}");}
}
#[test]
fn conditional_file_read_evaluate_display(){
 let _lock=CF_SERIAL.lock().unwrap();
 use ss_core::{java_runtime::Rc,cn::wps::moss::app::{KmoBookApi::KmoBookApi,condfmt::{KmoCondFmtEval::KmoCondFmtEval,KmoConditionFormatter::KmoConditionFormatter}},cn::wps::moss::util::KmoRect::KmoRect};
 let api=KmoBookApi::KmoBookApi();api.book().setAutoCalcOn(false);let path=format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/conditional-basic.xlsx",env!("CARGO_MANIFEST_DIR"));assert_eq!(api.open(&path,None,true),1);let sheet=api.book().sheetAt(0);
 assert_eq!(sheet.borrow().getCFormatter().getAllCfRule().len(),5);
 let evaluator=Rc::new(KmoCondFmtEval::new());let task=evaluator.requestEval(sheet.clone(),Rc::new(vec![KmoRect::KmoRect__bounds(0,0,5,3)])).unwrap();task.join().unwrap();
 let sheet=sheet.borrow();let formatter=sheet.getCFormatter();assert_eq!(formatter.getGenerals(0,0).map(|g|g.len()),None);assert_eq!(sheet.getFormattedCellValue(1,0),"0.000");assert_eq!(sheet.getFormattedCellValue(3,0),"20.00");assert_eq!(formatter.getGenerals(3,0).unwrap().len(),1);
 assert_eq!(formatter.getDataBar(0,2).unwrap().getLength(),10);assert_eq!(formatter.getDataBar(5,2).unwrap().getLength(),90);assert_eq!(formatter.getColorScale(0,1).unwrap().getColor(),0xffff0000u32 as i32);assert_eq!(formatter.getColorScale(5,1).unwrap().getColor(),0xff00ff00u32 as i32);assert_eq!(formatter.getIconSet(0,3).unwrap().getIndex(),3);assert_eq!(formatter.getIconSet(5,3).unwrap().getIndex(),1);assert!(!formatter.isShowValue(5,3));
 let xf=sheet.getCellXF__2(3,0).unwrap();let merged=formatter.getMergeXF(3,0,xf.clone(),KmoConditionFormatter::Font);assert_eq!(merged.getFont().unwrap().getBoldWeight(),700);assert_ne!(xf.getFont().unwrap().getBoldWeight(),700);
}

static CF_SERIAL:std::sync::Mutex<()>=std::sync::Mutex::new(());
#[test]
fn original_java_statemachine_4056_vectors(){
 use ss_core::cn::wps::et::ss::statemachine::{StateMachine::StateMachine,RecogniseResultRl::Value};
 let path=format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/java/statemachine/expected.tsv",env!("CARGO_MANIFEST_DIR"));
 for line in std::fs::read_to_string(path).unwrap().lines(){let a=line.split('\t').collect::<Vec<_>>();let input=(0..a[3].len()).step_by(4).map(|i|u16::from_str_radix(&a[3][i..i+4],16).unwrap()).collect::<Vec<_>>();let mut sm=StateMachine::new();sm.setLanguage(a[1].parse().unwrap());sm.setUsing1904(a[2]=="true");let v=sm.parse(&input);if a[4]=="_"{assert!(v.is_none(),"{line}");continue;}let v=v.unwrap_or_else(||panic!("No result: {line}"));assert_eq!(format!("{:?}",v.getValueType().unwrap()),a[4],"{line}");let val=match v.getValue().unwrap(){Value::Number(n)=>format!("{:x}",if n.is_nan(){0x7ff8000000000000}else{n.to_bits()}),Value::Bool(b)=>b.to_string(),Value::Text(s)=>s.iter().map(|c|format!("{c:04x}")).collect()};assert_eq!(val,a[5],"{line}");assert_eq!(v.getSuggestedNumfmt().map_or("_".into(),|s|s.encode_utf16().map(|c|format!("{c:04x}")).collect::<String>()),a[6],"{line}");assert_eq!(v.getBuiltInNumfmtIdx(),a[7].parse::<i32>().unwrap(),"{line}");}
}
#[test]
fn original_java_rtree_query_order_insert_split_remove_condense(){
 use ss_core::{java_runtime::Rc,cn::wps::moss::util::{KmoTreeSpatialIndex::KmoTreeSpatialIndex,KmoRect::KmoRect}};
 let root=format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/java/rtree",env!("CARGO_MANIFEST_DIR"));let expected=std::fs::read_to_string(format!("{root}/expected.tsv")).unwrap();let mut expected=expected.lines();let t=KmoTreeSpatialIndex::new();let mut values=std::collections::HashMap::new();let mut count=0;
 for line in std::fs::read_to_string(format!("{root}/operations.tsv")).unwrap().lines(){let a=line.split('\t').collect::<Vec<_>>();let v=a[1..].iter().map(|x|x.parse::<i32>().unwrap()).collect::<Vec<_>>();let r=KmoRect::KmoRect__bounds(v[1],v[2],v[3],v[4]);match a[0]{"I"=>{let value=Rc::new(v[0]);t.insert(&r,value.clone());values.insert(v[0],value);},"R"=>t.remove(&r,&values[&v[0]]),"C"=>t.clear(),"Q"=>{let mut out=vec![];t.lookup(&r,&mut out);let actual=format!("{line}\t{}",out.iter().map(|v|v.to_string()).collect::<Vec<_>>().join(","));assert_eq!(actual,expected.next().unwrap(),"query {count}");count+=1;},_=>unreachable!()}}
 assert_eq!(count,643);assert!(expected.next().is_none());
}
#[test]
fn undoable_rtree_preserves_each_operation_in_a_single_commit(){
 use ss_core::{java_runtime::Rc,cn::wps::moss::util::{KmoUndoableRTree::KmoUndoableRTree,KmoRect::KmoRect}};
 use kfc::cn::wps::memor4j::KmoAtomStore::{KmoAtomStore,KmoAtomStoreTrait};let _lock=CF_SERIAL.lock().unwrap();KmoAtomStore::enable();KmoAtomStore::clearUp();let t=KmoUndoableRTree::new();let r=KmoRect::KmoRect__bounds(0,0,4,4);let a=Rc::new(1i32);let b=Rc::new(2i32);let call=|f:&mut dyn FnMut(&mut dyn KmoAtomStoreTrait)|{let mut s=KmoAtomStore::sharedStore();f(s.as_mut().unwrap().as_mut());};
 call(&mut |s|s.start());t.insert(&r,a.clone());t.insert(&r,b.clone());t.remove(&r,a.clone());call(&mut |s|assert!(s.commit()));let get=||{let mut out=vec![];t.lookup(&r,&mut out);out};assert_eq!(get().iter().map(|v|**v).collect::<Vec<_>>(),[2]);call(&mut |s|{s.undo();});assert!(get().is_empty());call(&mut |s|{s.redo();});assert!(Rc::ptr_eq(&get()[0],&b));KmoAtomStore::clearUp();
}
#[test]
fn java_rule_array_equality_retains_token_object_identity(){
 use ss_core::{cn::wps::{moss::app::condfmt::rule::Rule::Rule,et::ss::formula::ptg::Ptg::{Ptg,PtgKind}},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion};
 let a=Rule::new(SpreadsheetVersion::EXCEL2007);let b=Rule::new(SpreadsheetVersion::EXCEL2007);a.setFormula1(Some(vec![Ptg::base(PtgKind::Number(1.0))]));b.setFormula1(Some(vec![Ptg::base(PtgKind::Number(1.0))]));assert!(!a.equals(&b));b.setFormula1(a.formula1());assert!(a.equals(&b));assert_eq!(a.hashCode(),b.hashCode());a.setFormula1(Some(vec![Ptg::base(PtgKind::Bool(true))]));b.setFormula1(Some(vec![Ptg::base(PtgKind::Bool(true))]));assert!(a.equals(&b));
}

#[path="../examples/m2_palette_probe.rs"]mod palette_probe;
#[test]fn original_java_palette_1242_vectors(){let _lock=CF_SERIAL.lock().unwrap();let expected=std::fs::read_to_string(format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/java/palette/expected.tsv",env!("CARGO_MANIFEST_DIR"))).unwrap();let expected=expected.lines().collect::<Vec<_>>();let actual=palette_probe::probe();assert_eq!(actual.len(),1242);for(i,(actual,expected))in actual.iter().zip(expected).enumerate(){assert_eq!(actual,expected,"palette vector {i}");}}
#[test]fn java_visual_rule_equality_keeps_original_omitted_base_fields(){
 use ss_core::{cn::wps::moss::app::condfmt::{rule::{KmoIconSet::KmoIconSet,KmoDataBar::KmoDataBar,Rule::RuleObject},KmoCfvo::{KmoCfvo,CfvoTypes}},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,java_runtime::Rc};
 let a=KmoIconSet::new(SpreadsheetVersion::EXCEL2007);let b=KmoIconSet::new(SpreadsheetVersion::EXCEL2007);a.setPriority(1);b.setPriority(9);assert!(a.javaEquals(&b));assert_eq!(a.javaHashCode(),b.javaHashCode());let a=KmoDataBar::new(SpreadsheetVersion::EXCEL2007);let b=KmoDataBar::new(SpreadsheetVersion::EXCEL2007);a.setPriority(1);b.setPriority(9);assert!(a.javaEquals(&b));let a=Rc::new(KmoCfvo::new());let b=Rc::new(KmoCfvo::new());a.rtype.set(Some(CfvoTypes::num));b.rtype.set(Some(CfvoTypes::num));assert!(a.equals(&b));
}
#[test]fn jdk_url_decoder_native_adapter(){
 use kfc_kfc_alpha::kfc::opcr::OpcrRelationship::OpcrRelationship;
 let expected=std::fs::read_to_string(format!("{}/rewrite-review/milestones/m2-xf-display/fixtures/java/url/expected.tsv",env!("CARGO_MANIFEST_DIR"))).unwrap();
 for line in expected.lines(){let (a,b)=line.split_once('\t').unwrap();let parse=|s:&str|String::from_utf16(&(0..s.len()).step_by(4).map(|i|u16::from_str_radix(&s[i..i+4],16).unwrap()).collect::<Vec<_>>()).unwrap();assert_eq!(OpcrRelationship::decodeURL(Some(&parse(a))).unwrap(),parse(b),"{a}");}
 assert_eq!(OpcrRelationship::decodeURL(None),None);
}
