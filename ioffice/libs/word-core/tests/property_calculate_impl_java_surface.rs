use std::cell::RefCell;
use std::rc::Rc;

use serde_json::{json, Map, Value};
use word_core::cn::wps::moffice::writer::api::DocumentOpen::open_boxed_text_document_from_docx_outcome;
use word_core::cn::wps::moffice::writer::core::Document::SoftTextDocument;
use word_core::cn::wps::moffice::writer::core::property::IPropertyCalculate::IPropertyCalculate;
use word_core::cn::wps::moffice::writer::core::property::PropertyCalculateImpl::PropertyCalculateImpl;
use word_core::cn::wps::moffice::writer::core::property::StyleCache::StyleCache;
use word_core::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
use word_core::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
use util::moffice::property::PropertySet::PropertySet;

fn describe(props: Option<&PropertySet>) -> Value {
    let Some(props) = props else { return json!({}); };
    let mut keys = vec![0; props.size() as usize];
    props.getAll(&mut keys, None);
    keys.sort_unstable();
    let mut scalars = Map::new();
    for key in &keys {
        if let Some(value) = props.getValue(*key) {
            let any = value.as_ref().as_any();
            let scalar = if let Some(v) = any.downcast_ref::<i32>() { Some(json!(v)) }
                else if let Some(v) = any.downcast_ref::<i64>() { Some(json!(v)) }
                else if let Some(v) = any.downcast_ref::<bool>() { Some(json!(v)) }
                else if let Some(v) = any.downcast_ref::<String>() { Some(json!(v)) }
                else if let Some(v) = any.downcast_ref::<f32>() { Some(json!(v)) }
                else if let Some(v) = any.downcast_ref::<f64>() { Some(json!(v)) }
                else { None };
            if let Some(scalar) = scalar { scalars.insert(key.to_string(), scalar); }
        }
    }
    json!({"keys": keys, "scalars": scalars})
}

fn same_numeric_values(rust: &Value, java: &Value) -> bool {
    if let (Some(a), Some(b)) = (rust.as_f64(), java.as_f64()) {
        return a == b;
    }
    match (rust, java) {
        (Value::Object(a), Value::Object(b)) => a.len() == b.len()
            && a.iter().all(|(key, value)| b.get(key).is_some_and(|v| same_numeric_values(value, v))),
        (Value::Array(a), Value::Array(b)) => a.len() == b.len()
            && a.iter().zip(b).all(|(x, y)| same_numeric_values(x, y)),
        _ => rust == java,
    }
}

fn compare_sample(label: &str, docx: &str, java: &str) {
    let bytes = std::fs::read(docx).unwrap();
    let (document, _) = open_boxed_text_document_from_docx_outcome(&bytes, "").unwrap();
    let main = document.getMainDocument().unwrap();
    let text_document: &dyn SoftTextDocument = &*document;
    let cache = Rc::new(RefCell::new(StyleCache::StyleCache(text_document)));
    let calculate = PropertyCalculateImpl::PropertyCalculateImpl(main, cache);
    let expected: Value = serde_json::from_slice(&std::fs::read(java).unwrap()).unwrap();
    let mut actual_rows = Vec::new();
    for row in expected.as_array().unwrap() {
        let cp = row["cp"].as_i64().unwrap() as i32;
        let papx_table = main.getPapxTable().unwrap();
        let chpx_table = main.getChpxTable().unwrap();
        let papx = IPapxTable::seek(&*papx_table, cp).unwrap();
        let chpx = IChpxTable::seek(&*chpx_table, cp).unwrap();
        let actual = json!({
            "cp": cp,
            "rawPapx": describe(word_core::cn::wps::moffice::writer::data::IPapxTable::Entry::getProperty(&*papx)),
            "rawChpx": describe(word_core::cn::wps::moffice::writer::data::IChpxTable::Entry::getProperty(&*chpx)),
            "paragraphStyle": describe(Some(&calculate.getParagraphStyleFormat(
                word_core::cn::wps::moffice::writer::data::IPapxTable::Entry::getProperty(&*papx)
                    .unwrap().getInt__int_int(191, 0), 0))),
            "pap0": describe(Some(&calculate.getPap__entry_int(&*papx, 0))),
            "pap2": describe(Some(&calculate.getPap__entry_int(&*papx, 2))),
            "chp0": describe(Some(&calculate.getChp__entry_int(&*chpx, 0))),
        });
        actual_rows.push(actual);
    }
    if let Ok(dir) = std::env::var("WORD_PARITY_PROPERTY_OUT_DIR") {
        let path = std::path::Path::new(&dir).join(format!("rust-{label}-property-calculate.json"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(path, serde_json::to_vec_pretty(&actual_rows).unwrap()).unwrap();
    }
    for (actual, row) in actual_rows.iter().zip(expected.as_array().unwrap()) {
        let cp = row["cp"].as_i64().unwrap();
        for field in ["rawPapx", "rawChpx", "paragraphStyle", "pap0", "pap2", "chp0"] {
            if !same_numeric_values(&actual[field], &row[field]) {
                panic!("{label} CP{cp} {field}: Rust keys={} Java keys={} Rust scalars={} Java scalars={}",
                    actual[field]["keys"], row[field]["keys"],
                    actual[field]["scalars"], row[field]["scalars"]);
            }
        }
    }
}

#[test]
fn java_old_sample_property_calculation_matches_real_product() {
    let root = env!("CARGO_MANIFEST_DIR");
    compare_sample("old",
        &format!("{root}/../../samples/docx-mvp-render/flutter/assets/sample1/sample2.docx"),
        &format!("{root}/rewrite-review/milestones/layout-java-pipeline/evidence/complex-sample/m3-10u/java-old/koffice_property_calculate.json"),
    );
}

#[test]
fn java_complex_sample_property_calculation_matches_real_product() {
    let root = env!("CARGO_MANIFEST_DIR");
    compare_sample("complex",
        &format!("{root}/../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx"),
        &format!("{root}/rewrite-review/milestones/layout-java-pipeline/evidence/complex-sample/m3-10u/java-complex/koffice_property_calculate.json"),
    );
}
