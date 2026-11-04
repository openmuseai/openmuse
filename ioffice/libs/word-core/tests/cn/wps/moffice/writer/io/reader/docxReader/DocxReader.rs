// function_id=07_function_docx_read
#![allow(non_snake_case)]

use std::mem::ManuallyDrop;
use std::panic::{catch_unwind, AssertUnwindSafe};

use util_adapter::cn::wps::moffice::io::IDLProvider::IDLProvider;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::DocxReader::{DocxReader, RecXwpf};
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;

struct DummyListener;
impl IOListener for DummyListener {
    // rust: ★M-STAGE2 P1-D1 —— 真实 IOListener（9 方法）补齐此前只对着
    //   3 方法影子写的空实现。
    fn onLoadParas(&mut self, _cp: i32) {}
    fn onFirstLock(&mut self) {}

    fn onFirstUnLock(&mut self) {}

    fn onError(&mut self, _resultCode: i32, _msg: &dyn stubs::java::lang::Object::Object) {}

    fn beginLoadOnlineSecurityDoc(&mut self) {}

    fn onFinish(&mut self) {}

    fn onFinishDumpObjects(&mut self) {}

    fn onHtmlOpenError(&mut self) {}

    fn onFinishVerifyWritePassword(&mut self) {}
}


struct NoteDl(String);
impl IDLProvider for NoteDl {
    fn setOpenForceQuit(&mut self, _var1: bool) {}
    fn isOpenForceQuit(&self) -> bool { false }
    fn isOpenFirstPageForceQuit(&self) -> bool { false }
    fn getReadingLimit(&self) -> Option<i32> { None }
    fn getNotePath(&self) -> String { self.0.clone() }
    fn getWritePassword(&self) -> String { String::new() }
    fn isOpenOnReadOnly(&self) -> bool { false }
}

fn disk_ok() -> RecXwpf {
    let mut d = RecXwpf::new();
    d.chars = None;
    d.properties_none = false;
    d
}

#[test]
fn DR_all_docx_read_matrix() {
    let p1 = catch_unwind(AssertUnwindSafe(|| {
        let _ = DocxReader::DocxReader(None, Some(Box::new(DummyListener)), Some(disk_ok()), None, None, None);
    }));
    assert!(p1.is_err(), "DR1 doc null");
    let p2 = catch_unwind(AssertUnwindSafe(|| {
        let mut doc = TextDocument::TextDocument();
        let _ = DocxReader::DocxReader(Some(&mut doc), None, Some(disk_ok()), None, None, None);
    }));
    assert!(p2.is_err(), "DR1 listener null");

    let mut doc = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut xw = disk_ok();
    xw.chars = None;
    let mut r = DocxReader::DocxReader(
        Some(doc.as_mut()),
        Some(Box::new(DummyListener)),
        Some(xw),
        None,
        None,
        None,
    );
    assert!(r.mImporter.is_some(), "DR2");
    r.read();
    let order = r.order.borrow().clone();
    let gi = order.iter().position(|s| *s == "guess").unwrap();
    let io = order.iter().position(|s| *s == "inIOState").unwrap();
    let lp = order.iter().position(|s| *s == "onLoadParas").unwrap();
    let si = order.iter().position(|s| *s == "setImporter").unwrap();
    let parse = order.iter().position(|s| *s == "parse").unwrap();
    let meta = order.iter().position(|s| *s == "meta").unwrap();
    let fin = order.iter().position(|s| *s == "finish").unwrap();
    let xml = order.iter().position(|s| *s == "importCustomXmls").unwrap();
    assert!(gi < io && io < lp && lp < si, "DR3");
    assert!(si < parse && parse < meta, "DR4");
    assert!(meta < fin && fin < xml, "DR5");
    assert!(doc.isInIOState(), "DR3 inIO");
    let g = doc
        .getMainDocument()
        .and_then(|m| m.getPieceTable())
        .map(|p| p.getGuessedTotalLength())
        .unwrap_or(-1);
    assert_eq!(g, 100, "DR6");
    assert!(
        doc.getMainDocument().map(|m| m.getLength()).unwrap_or(0) > 0,
        "body chain via rec parse"
    );

    let mut doc7 = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut xw7 = disk_ok();
    xw7.chars = Some(42);
    let mut r7 = DocxReader::DocxReader(
        Some(doc7.as_mut()),
        Some(Box::new(DummyListener)),
        Some(xw7),
        None,
        None,
        None,
    );
    r7.read();
    let g7 = doc7
        .getMainDocument()
        .and_then(|m| m.getPieceTable())
        .map(|p| p.getGuessedTotalLength())
        .unwrap_or(-1);
    assert_eq!(g7, 42, "DR7");

    let mut doc8 = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut xw8 = disk_ok();
    xw8.properties_none = true;
    let mut r8 = DocxReader::DocxReader(
        Some(doc8.as_mut()),
        Some(Box::new(DummyListener)),
        Some(xw8),
        None,
        None,
        None,
    );
    r8.importMetaData();

    let mut doc9 = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut xw9 = disk_ok();
    let log9 = xw9.log.clone();
    let mut r9 = DocxReader::DocxReader(
        Some(doc9.as_mut()),
        Some(Box::new(DummyListener)),
        Some(xw9),
        None,
        None,
        Some(Box::new(NoteDl("/n".into()))),
    );
    r9.read();
    assert!(log9.borrow().iter().any(|s| *s == "setNotePath"), "DR9");

    let mut doc10 = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut xw10 = disk_ok();
    let log10 = xw10.log.clone();
    let mut r10 = DocxReader::DocxReader(
        Some(doc10.as_mut()),
        Some(Box::new(DummyListener)),
        Some(xw10),
        None,
        None,
        None,
    );
    r10.read();
    assert!(!log10.borrow().iter().any(|s| *s == "setNotePath"), "DR10");

    let closed = r10.mXWPFDoc.as_ref().unwrap().closed.clone();
    r10.dispose();
    assert!(*closed.borrow(), "DR11");
    assert!(r10.mImporter.is_none(), "DR11 importer");

    let mut doc12 = ManuallyDrop::new(Box::new(TextDocument::TextDocument()));
    let mut r12 = DocxReader::DocxReader(
        Some(doc12.as_mut()),
        Some(Box::new(DummyListener)),
        None,
        None,
        None,
        None,
    );
    r12.dispose();
}

#[test]
#[ignore = "spec: 缺口C 跨crate样式链"]
fn DR_styles_kstyles_from_fixture_docx() {
    panic!("wave0 styles chain");
}
