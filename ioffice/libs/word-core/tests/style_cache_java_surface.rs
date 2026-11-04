use std::fs;
use word_core::cn::wps::moffice::writer::api::DocumentOpen;
use word_core::cn::wps::moffice::writer::core::Document::SoftTextDocument;
use word_core::cn::wps::moffice::writer::core::property::StyleCache::StyleCache;
use word_core::cn::wps::moffice::writer::data::DefaultParagraphProperties::DefaultParagraphProperties;
use word_core::cn::wps::moffice::writer::data::DefaultRunProperties::DefaultRunProperties;
use util::moffice::property::PropertySet::PropertySet;

#[test]
fn java_style_cache_composes_defaults_and_reads_style_zero() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/docx-mvp-render/flutter/assets/sample2/机械制造与自动化毕业论文.docx");
    let bytes = fs::read(path).unwrap();
    let (document, _) = DocumentOpen::open_boxed_text_document_from_docx_outcome(&bytes, "").unwrap();
    let styles = document.getStyles().unwrap();
    let doc: &dyn SoftTextDocument = &*document;
    let cache = StyleCache::StyleCache(doc);
    let pap = PropertySet::compose(&DefaultParagraphProperties::getProperties(), &styles.getPPrDefault(), 2);
    let chp = PropertySet::compose(&DefaultRunProperties::getProperties(), &styles.getRPrDefault(), 2);
    assert!(cache.getDefaultPap().equals(Some(&pap)));
    assert!(cache.getDefaultChp().equals(Some(&chp)));
    let mut style = styles.getStyleById__int(0).unwrap();
    assert!(cache.getStyleFormat(0).equals(style.getFinalProperty().as_ref()));
    assert!(cache.getParagraphStyleFormat(0).equals(Some(&cache.getStyleFormat(0))));
    assert!(cache.getRunStyleFormat(0).isEmpty());
}
