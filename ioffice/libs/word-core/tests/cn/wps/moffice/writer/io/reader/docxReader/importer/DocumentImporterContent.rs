// function_id=04_function_di_main_content
// cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_di_main_content
#![allow(non_snake_case)]

use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use word_core::cn::wps::collections::textrope::ITextRope::ITextRope;
use word_core::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::IOListener;
use word_core::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

#[path = "helpers/wc03_word_attrs.rs"]
mod wc03_word_attrs;
use wc03_word_attrs::{MAIN_NS, WordAttrs};

impl Attributes for WordAttrs {
    fn getLength(&self) -> i32 {
        self.pairs.len() as i32
    }
    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.get_value(uri, localName).map(|s| s.to_string())
    }
}

struct RecListener {
    cps: Rc<RefCell<Vec<i32>>>,
}
impl IOListener for RecListener {
    fn onLoadParas(&mut self, cp: i32) {
        self.cps.borrow_mut().push(cp);
    }
    // rust: ★M-STAGE2 P1-D1 —— 真实 IOListener（9 方法）补齐此前只对着
    //   3 方法影子写的空实现。
    fn onFirstLock(&mut self) {}

    fn onFirstUnLock(&mut self) {}

    fn onError(&mut self, _resultCode: i32, _msg: &dyn stubs::java::lang::Object::Object) {}

    fn beginLoadOnlineSecurityDoc(&mut self) {}

    fn onFinish(&mut self) {}

    fn onFinishDumpObjects(&mut self) {}

    fn onHtmlOpenError(&mut self) {}

    fn onFinishVerifyWritePassword(&mut self) {}

}

fn fixture() -> (
    ManuallyDrop<Box<TextDocument>>,
    Rc<RefCell<Vec<i32>>>,
    ManuallyDrop<DocumentImporter>,
) {
    // ★★必须与产品路径同构（见 `api/DocumentOpen.rs:7-8` 的原文注释）：
    //   `TextDocument__boolean(false)` 会在**栈上局部变量** `this` 上调用 `_create()`，
    //   而 `_create()` 会把 `parent.styles` / `parent.textPool` 记成**指向 `this` 内联字段**的
    //   裸指针（`TextDocument.rs:1086-1091`）；随后 `Self` **按值返回** ⇒ 这些指针指向
    //   **已失效的栈帧**（`subdoc_parent` 内的壳、以及 7 个 `SubDocument.mParent` 全部悬垂）。
    //   ⇒ 改用 `bMock=true` 先装箱、再**在堆上就地** `_create()`，使 `mParent` 为堆地址。
    let mut doc = Box::new(TextDocument::TextDocument__boolean(true));
    doc._create();
    let cps = Rc::new(RefCell::new(Vec::new()));
    let listener = RecListener { cps: cps.clone() };
    let di = DocumentImporter::DocumentImporter__textdocument_iolistener_idlprovider_boolean_themecontext_int_xml07context_xml03context(
        doc.as_mut(),
        Some(Box::new(listener)),
        None,
        false,
        None,
        0,
        None,
        None,
    );
    (ManuallyDrop::new(doc), cps, ManuallyDrop::new(di))
}

fn para() -> ParagraphSimple {
    ParagraphSimple::new_optional(Box::new(0i32), None)
}

unsafe fn main_sd(di: &DocumentImporter) -> *mut word_core::cn::wps::moffice::writer::core::SubDocument::SubDocument {
    di.getSubDocumentImporter(0)
        .unwrap()
        .getSubDocumentPtr()
        .expect("main subdoc")
}

fn last_char(di: &DocumentImporter) -> char {
    unsafe {
        let sd = main_sd(di);
        let r = (*sd).getTextRope().expect("rope");
        let n = ITextRope::length(&mut *r);
        ITextRope::charAt(&mut *r, n - 1)
    }
}

fn end_para(di: &mut DocumentImporter) {
    di.onImportParagraphStart(0, &WordAttrs::empty());
    let mut p = para();
    di.onImportParagraphEnd(&p, 0, 0);
}

#[test]
fn D4_all_main_content_matrix() {
    let (mut doc, cps, mut di) = fixture();
    // D4-1
    {
        let attrs = WordAttrs::empty().with(MAIN_NS, "paraId", "p1");
        di.onImportParagraphStart(0, &attrs);
        let pid = di
            .getSubDocumentImporter(0)
            .unwrap()
            .mCurrentParagraph
            .as_ref()
            .and_then(|c| c.mParaId.clone());
        assert_eq!(pid.as_deref(), Some("p1"));
    }
    // D4-2
    {
        di.onImportParagraphStart(0, &WordAttrs::empty());
        let pid = di
            .getSubDocumentImporter(0)
            .unwrap()
            .mCurrentParagraph
            .as_ref()
            .and_then(|c| c.mParaId.clone());
        assert!(pid.is_none());
    }
    // D4-3 .. D4-17
    end_para(&mut di);
        end_para(&mut di);
        assert!(cps.borrow().is_empty(), "D4-4");
        di.callback4Progress(None);
        assert_eq!(di.mCountParagraph, 2, "D4-6");
        end_para(&mut di);
        assert_eq!(cps.borrow().len(), 1, "D4-5");
        end_para(&mut di);
        end_para(&mut di);
        end_para(&mut di);
        assert_eq!(cps.borrow().len(), 2, "D4-5 reset");
        // D4-7..10
        di.onImportParagraphStart(0, &WordAttrs::empty());
        let mut mps = MutablePropertySet::new();
        di.onImportRun__subdoctype_mutablepropertyset_string(0, Some(&mut mps), "ab");
        di.onImportRun__subdoctype_mutablepropertyset_string(0, None, "ab");
        let buf = vec!['a', 'b', 'c'];
        di.onImportRun__subdoctype_mutablepropertyset_chararr_int_int(0, Some(&mut mps), &buf, 1, 2);
        di.onImportRun__subdoctype_mutablepropertyset_chararr_int_int(0, Some(&mut mps), &buf, 0, 0);
        // D4-11
        let orig = MutablePropertySet::new();
        di.onImportTableRowEnd(0, 1, 0, &mut mps, &orig);
        // D4-12
        di.onImportTableRowEnd(0, 1, 1, &mut mps, &orig);
        assert_eq!(last_char(&di), '\u{0007}', "D4-12");
        // D4-13
        di.onImportTableRowEnd(0, 0, 1, &mut mps, &orig);
        assert_eq!(last_char(&di), '\r', "D4-13");
        // D4-14
        mps.setString(282, "pid282".to_string());
        di.onImportTableRowEnd(0, 1, 1, &mut mps, &orig);
        // rust: ★M-STAGE2 P1-D1 —— 门面与语义双重对齐（Java 原文为准）：
        //   1) 方法名用 Java 原名 `getParaEntry(String)`（旧影子自造的 `contains_id` 已删）。
        //   2) 语义纠正：Java 的 `putId(id, type, entry)` 写入的是 `mId2Entry`
        //      （键为 `id.hashCode()`），**不是** `mParagraphIds`；旧影子把两者混为一谈
        //      （`putId` 直接 `ids.insert`）是假装配。`containsParaId` 只反映
        //      `addParaphId` 的写入，故此处不能拿它断言 `putId` 的效果。
        //   3) `getParaEntry` 按 Java 原文会额外校验 `propertySet.contains(282) &&
        //      id.equals(propertySet.getString(282))`，成立才返回条目。
        assert!(doc.getKParagraphsID().getParaEntry(Some("pid282")).is_some(), "D4-14");
        // D4-16
        di.mHasHeaderFooter = false;
        di.onImportSectProp(&mut mps, None);
        unsafe {
            let plc = (*main_sd(&di)).getPLCSection().expect("plc");
            assert!(!plc.isEmpty(), "D4-16");
        }
        // D4-17
        di.onImportPdf(0);
        // D4-15 last: header/footer flag poisons later appends
        di.mHasHeaderFooter = true;
        di.onImportSectProp(&mut mps, None);
}

