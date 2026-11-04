//! Same-byte observer of Java class pipeline values, independent of display formatting.
use kfc::cn::wps::kfc::numfmt::i18n::KmoLang::KmoLang;
use ss_core::cn::wps::moss::app::{
    xf::KmoXFManager::KmoXFManager, KmoBookApi::KmoBookApi, KmoLocale::KmoLocale,
};
fn main() {
    let locale = std::env::args().nth(2).unwrap_or_else(|| "zh_CN".into());
    KmoLocale::setLocale(locale.clone());
    KmoLang::setLocale(locale);
    KmoXFManager::loadBuiltinFormats();
    let api = KmoBookApi::KmoBookApi();
    let book = api.book();
    book.setAutoCalcOn(false);
    assert_eq!(
        api.open(&std::env::args().nth(1).expect("xlsx path"), None, true),
        1
    );
    let manager = book.getXFManager().borrow();
    let mut records = Vec::new();
    for index in 0..manager.getNumExFormats() {
        let x = manager.getXF(index).unwrap();
        let font = x.getFont().unwrap();
        records.push(serde_json::json!({"index":index,"present":true,"class":if x.getXFType()==1{"KmoStyleXF"}else{"KmoCellXF"},"fontIndex":x.getFontIndex(),"formatIndex":x.getFormatIndex(),"numFormat":x.getNumFormat(),"alignment":x.getAlignment(),"verticalAlignment":x.getVerticalAlignment(),"wrapText":x.getWrapText(),"locked":x.isLocked(),"hidden":x.isHidden(),"borderLeft":x.getBorderLeft(),"borderRight":x.getBorderRight(),"borderTop":x.getBorderTop(),"borderBottom":x.getBorderBottom(),"fontName":font.getFontName(),"fontHeight":font.getFontHeight()}));
    }
    let path = std::env::args().nth(1).unwrap();
    let input = std::fs::read(path).unwrap();
    let session = ss_core::api::SpreadsheetSession::SpreadsheetSession::open_bytes(
        &input,
        ss_core::api::SpreadsheetSession::SpreadsheetLoadConfig::default(),
    );
    println!(
        "{}",
        serde_json::json!({"inputSha256":session.input_sha256(),"xfCount":records.len(),"xfRecords":records})
    );
}
