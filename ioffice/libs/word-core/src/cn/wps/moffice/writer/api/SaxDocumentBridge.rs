//! Bridge `io_wps_io::IDocumentImporter` (`&self`) → word-core `DocumentImporter` (`&mut self`).
//!
//! Product open uses this with a real `XWPFDocument`. `DocxReader` still uses `RecXwpf`
//! so wc-05 read tests stay on the test double.

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use std::panic::{catch_unwind, AssertUnwindSafe};

use io_wps_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::ParagraphSimple;
use io_wps_io::cn::wps::io::xwpf::element_handler::styles::StylePrModel::StylePrModel;
use io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter;
use io_wps_io::cn::wps::io::xwpf::element_handler::numbering::NumLvlType::NumLvlType;
use io_wps_io::cn::wps::io::xwpf::usermodel::XWPFSettings::XWPFSettings;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use util::base::io::SubDocType::SubDocType;

use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::DocumentImporter::DocumentImporter;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

struct EmptyAttrs;

impl Attributes for EmptyAttrs {
    fn getLength(&self) -> i32 {
        0
    }
    fn getValue(&self, _uri: &str, _localName: &str) -> Option<String> {
        None
    }
}

struct AttrAdapter<'a>(&'a AttributesImpl);

impl Attributes for AttrAdapter<'_> {
    fn getLength(&self) -> i32 {
        self.0.getLength() as i32
    }
    fn getValue(&self, uri: &str, localName: &str) -> Option<String> {
        self.0.getValue_by_uri(uri, localName)
    }
}

fn with_attrs<R>(attrs: &dyn Any, f: impl FnOnce(&dyn Attributes) -> R) -> R {
    if let Some(a) = attrs.downcast_ref::<AttributesImpl>() {
        f(&AttrAdapter(a))
    } else {
        f(&EmptyAttrs)
    }
}

pub struct SaxDocumentBridge {
    pub inner: RefCell<DocumentImporter>,
    pub self_weak: std::rc::Weak<SaxDocumentBridge>,
}

impl IImporterBase for SaxDocumentBridge {
    fn mode(&self) -> i32 {
        0
    }
}

impl IDocumentImporter for SaxDocumentBridge {
    // Java numbering handler callbacks preserve the same importer instance as document.xml.
    fn onImportNumberingNumStart(&self, id: i32, attrs: &dyn Any) {
        with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingNumStart(id, attrs));
    }
    fn onImportNumberingNumChild(&self, id: i32, attrs: &dyn Any) {
        with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingNumChild(id, attrs));
    }
    fn onImportNumberingNumEnd(&self) {
        self.inner.borrow_mut().onImportNumberingNumEnd();
    }
    fn onImportNumberingAbstractNumStart(&self, attrs: &dyn Any) {
        with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingAbstractNumStart(attrs));
    }
    fn onImportNumberingAbstractNumChild(&self, id: i32, attrs: &dyn Any) {
        with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingAbstractNumChild(id, attrs));
    }
    fn onImportNumberingAbstractNumEnd(&self) {
        self.inner.borrow_mut().onImportNumberingAbstractNumEnd();
    }
    fn onImportNumberingLevelStart(&self, kind: &dyn Any, attrs: &dyn Any) {
        if let Some(kind) = kind.downcast_ref::<NumLvlType>() {
            with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingLevelStart(kind, attrs));
        }
    }
    fn onImportNumberingLevelChild(&self, kind: &dyn Any, id: i32, attrs: &dyn Any) {
        if let Some(kind) = kind.downcast_ref::<NumLvlType>() {
            with_attrs(attrs, |attrs| self.inner.borrow_mut().onImportNumberingLevelChild(kind, id, attrs));
        }
    }
    fn onImportNumberingLevelEnd(&self, kind: &dyn Any, rpr: &dyn Any, ppr: &dyn Any) {
        if let Some(kind) = kind.downcast_ref::<NumLvlType>() {
            let empty = MutablePropertySet::new();
            self.inner.borrow_mut().onImportNumberingLevelEnd(
                kind,
                rpr.downcast_ref::<MutablePropertySet>().unwrap_or(&empty),
                ppr.downcast_ref::<MutablePropertySet>().unwrap_or(&empty),
            );
        }
    }
    // Java: DocumentImporter.setThemePackagePart(POIXMLDocumentPart packagePart)
    fn setThemePackagePart(&self, packagePart: &dyn Any) {
        if let Some(part) = packagePart.downcast_ref::<Rc<POIXMLDocumentPart>>() {
            self.inner.borrow_mut().setThemePackagePart(part.clone());
        }
    }

    // Java: DocumentImporter.onImportFontSchemeStart(Attributes attributes)
    fn onImportFontSchemeStart(&self, attributes: &dyn Any) {
        with_attrs(attributes, |a| self.inner.borrow_mut().onImportFontSchemeStart(a));
    }

    // Java: DocumentImporter.onImportMajorFontStart()
    fn onImportMajorFontStart(&self) {
        self.inner.borrow_mut().onImportMajorFontStart();
    }

    // Java: DocumentImporter.onImportMinorFontStart()
    fn onImportMinorFontStart(&self) {
        self.inner.borrow_mut().onImportMinorFontStart();
    }

    // Java: DocumentImporter.onImportThemeCSFontStart(Attributes attributes)
    fn onImportThemeCSFontStart(&self, attributes: &dyn Any) {
        with_attrs(attributes, |a| self.inner.borrow_mut().onImportThemeCSFontStart(a));
    }

    // Java: DocumentImporter.onImportThemeEAFontStart(Attributes attributes)
    fn onImportThemeEAFontStart(&self, attributes: &dyn Any) {
        with_attrs(attributes, |a| self.inner.borrow_mut().onImportThemeEAFontStart(a));
    }

    // Java: DocumentImporter.onImportThemeLatinFontStart(Attributes attributes)
    fn onImportThemeLatinFontStart(&self, attributes: &dyn Any) {
        with_attrs(attributes, |a| self.inner.borrow_mut().onImportThemeLatinFontStart(a));
    }

    // Java: DocumentImporter.onImportThemeFontStart(Attributes attributes)
    fn onImportThemeFontStart(&self, attributes: &dyn Any) {
        with_attrs(attributes, |a| self.inner.borrow_mut().onImportThemeFontStart(a));
    }

    // Java: DocumentImporter.onImportSettingsStart(XWPFSettings xwpfSettings)
    fn onImportSettingsStart(&self, settings: &dyn Any) {
        if let Some(settings) = settings.downcast_ref::<XWPFSettings>() {
            self.inner.borrow_mut().onImportSettingsStart(settings);
        }
    }

    // Java: DocumentImporter.onImportSettingsChild(int elementId, Attributes attributes)
    fn onImportSettingsChild(&self, elementId: i32, attributes: &dyn Any) {
        let mut inner = self.inner.borrow_mut();
        with_attrs(attributes, |a| inner.onImportSettingsChild(elementId, a));
    }

    // Java: DocumentImporter.onImportSettingsEnd()
    fn onImportSettingsEnd(&self) {
        self.inner.borrow_mut().onImportSettingsEnd();
    }

    fn onMainDocumentStart(&self, hasHeaderFooter: bool) {
        self.inner.borrow_mut().onMainDocumentStart(hasHeaderFooter);
    }

    fn onMainDocumentEnd(&self) {
        self.inner.borrow_mut().onMainDocumentEnd();
    }

    fn onImportParagraphStart(&self, subDocType: i32, attributes: &dyn Any) {
        let mut inner = self.inner.borrow_mut();
        with_attrs(attributes, |a| inner.onImportParagraphStart(subDocType, a));
    }

    fn onImportParagraphEnd(&self, paragraph: &dyn Any, subDocType: i32, layerTable: i32) {
        let mut inner = self.inner.borrow_mut();
        if let Some(p) = paragraph.downcast_ref::<ParagraphSimple>() {
            inner.onImportParagraphEnd(p, subDocType, layerTable);
        } else {
            let dummy = ParagraphSimple::new_optional(Box::new(()), None);
            inner.onImportParagraphEnd(&dummy, subDocType, layerTable);
        }
    }

    // Java: DocumentImporter.onImportTableStart(SubDocType subDocType, int layerTable)
    fn onImportTableStart(&self, subDocType: i32, layerTable: i32) {
        self.inner.borrow_mut().onImportTableStart(subDocType, layerTable);
    }

    // Java: DocumentImporter.onImportTableCelStart(SubDocType subDocType)
    fn onImportTableCelStart(&self, subDocType: i32) {
        self.inner.borrow_mut().onImportTableCelStart(subDocType);
    }

    // Java: DocumentImporter.onImportTableCellEnd(SubDocType subDocType, int layerTable)
    fn onImportTableCellEnd(&self, subDocType: i32, layerTable: i32) {
        self.inner.borrow_mut().onImportTableCellEnd(subDocType, layerTable);
    }

    // Java: DocumentImporter.onImportTableRowStart(SubDocType, String, MutablePropertySet)
    fn onImportTableRowStart(&self, sub_doc_type: i32, para_id: &str, prop: &dyn Any) {
        let mut owned = prop
            .downcast_ref::<MutablePropertySet>()
            .map(|p| p.clone_mps())
            .unwrap_or_else(MutablePropertySet::new);
        let para = if para_id.is_empty() { None } else { Some(para_id) };
        self.inner
            .borrow_mut()
            .onImportTableRowStart(sub_doc_type, para, &mut owned);
    }

    // Java: DocumentImporter.onImportTableRowEnd(...)
    fn onImportTableRowEnd(
        &self,
        sub_doc_type: i32,
        layer_table: i32,
        num_cells: i32,
        prop: &dyn Any,
        original: &dyn Any,
    ) {
        if std::env::var_os("WORD_TABLE_IMPORT_TRACE").is_some() {
            let borders = prop.downcast_ref::<MutablePropertySet>()
                .and_then(|p| p.getValue(310))
                .and_then(|v| v.as_any().downcast_ref::<crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders>())
                .map(|b| (0..6).filter(|&i| b.getBorder(i).is_some()).count());
            eprintln!("table_row_end cells={num_cells} borders={borders:?}");
        }
        let mut owned = prop
            .downcast_ref::<MutablePropertySet>()
            .map(|p| p.clone_mps())
            .unwrap_or_else(MutablePropertySet::new);
        let original_owned = original
            .downcast_ref::<MutablePropertySet>()
            .map(|p| p.clone_mps())
            .unwrap_or_else(MutablePropertySet::new);
        self.inner.borrow_mut().onImportTableRowEnd(
            sub_doc_type,
            layer_table,
            num_cells,
            &mut owned,
            &original_owned,
        );
    }

    fn onImportRun_chars(
        &self,
        subDocType: i32,
        prop: &dyn Any,
        buf: &[char],
        start: i32,
        len: i32,
    ) {
        let owned: Vec<char> = buf.to_vec();
        let mut inner = self.inner.borrow_mut();
        let mut mps_owned: Option<MutablePropertySet> =
            prop.downcast_ref::<MutablePropertySet>().map(|p| p.clone_mps());
        inner.onImportRun__subdoctype_mutablepropertyset_chararr_int_int(
            subDocType,
            mps_owned.as_mut(),
            &owned,
            start,
            len,
        );
    }

    fn onImportStylesEnd(&self) {
        self.inner.borrow_mut().onImportStylesEnd();
    }

    /// Java: `IDocumentImporter.onImportRun(SubDocType, MutablePropertySet, String)`
    ///
    /// rust: ★P0/L0-3 —— `w:br` 走的是 **String 重载**（`BrHandler.onStart` →
    ///   `onImportRun(subDocType, dst, convertBreakText(attributes))`），而本桥原先
    ///   只实现了 `onImportRun_chars`。`IDocumentImporter::onImportRun_str` 的默认体是
    ///   空实现 ⇒ `BrHandler` 算出的 `'\f'` 被**静默丢弃**，这是分页符进不了 rope 的
    ///   最后一环。此处按 chars 路径同样的属性语义转调到真实 `DocumentImporter`。
    fn onImportRun_str(&self, subDocType: i32, prop: &dyn Any, text: &str) {
        let mut inner = self.inner.borrow_mut();
        let mut mps_owned: Option<MutablePropertySet> =
            prop.downcast_ref::<MutablePropertySet>().map(|p| p.clone_mps());
        inner.onImportRun__subdoctype_mutablepropertyset_string(
            subDocType,
            mps_owned.as_mut(),
            text,
        );
    }

    // Java: ContentHandler.onStart → IDocumentImporter.onImportRunContent
    fn onImportRunContent(&self, subDocType: i32, elementId: i32, prop: &dyn Any, attributes: &dyn Any) {
        // Java: SubDocType mSubDocType is passed unchanged through the handler.
        let sub_doc_type = match subDocType {
            0 => SubDocType::MAIN_DOCUMENT,
            1 => SubDocType::FOOTNOTE_DOCUMENT,
            2 => SubDocType::HEADER_DOCUMENT,
            3 => SubDocType::COMMENT_DOCUMENT,
            4 => SubDocType::ENDNOTE_DOCUMENT,
            5 => SubDocType::TEXTBOX_DOCUMENT,
            6 => SubDocType::HEADERTEXTBOX_DOCUMENT,
            _ => panic!("invalid SubDocType ordinal: {subDocType}"),
        };
        // Java: RHandler.getProp() supplies the run's MutablePropertySet.
        // The Rust importer signature requires &mut; retain the same properties in an owned clone.
        let mut run_prop = prop.downcast_ref::<MutablePropertySet>()
            .map(MutablePropertySet::clone_mps)
            .unwrap_or_else(MutablePropertySet::new);
        with_attrs(attributes, |attrs| {
            self.inner.borrow_mut().onImportRunContent(&sub_doc_type, elementId, &mut run_prop, attrs);
        });
    }

    fn onImportStyleStart(&self, attributes: &dyn Any) {
        let mut inner = self.inner.borrow_mut();
        with_attrs(attributes, |a| inner.onImportStyleStart(a));
    }

    fn onImportStyleChild(&self, elementId: i32, attributes: &dyn Any) {
        let mut inner = self.inner.borrow_mut();
        with_attrs(attributes, |a| inner.onImportStyleChild(elementId, a));
    }

    fn onImportStyleEnd(&self, model: &dyn Any) {
        let Some(m) = model.downcast_ref::<StylePrModel>() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| {
            self.inner.borrow_mut().onImportStyleEnd(m);
        }));
    }

    fn onImportSectProp(&self, prop: &dyn Any, _headFooter: &dyn Any) {
        let Some(mps) = prop.downcast_ref::<MutablePropertySet>() else {
            return;
        };
        let mut clone = mps.clone_mps();
        let _ = catch_unwind(AssertUnwindSafe(|| {
            self.inner
                .borrow_mut()
                .onImportSectProp(&mut clone, None);
        }));
    }

    fn converTableCellProp(&self, prop: &mut dyn Any, element_id: i32, attributes: &dyn Any, cell_index: i32) {
        let Some(mps) = prop.downcast_mut::<MutablePropertySet>() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut inner = self.inner.borrow_mut();
            with_attrs(attributes, |a| {
                inner.converTableCellProp(mps, element_id, a, cell_index);
            });
        }));
    }

    fn convertProp(
        &self,
        propType: i32,
        prop: &mut dyn Any,
        namespaceURI: &str,
        elementId: i32,
        attributes: &dyn Any,
    ) {
        let Some(mps) = prop.downcast_mut::<MutablePropertySet>() else {
            return;
        };
        // Java: IDocumentImporter.convertProp delegates directly to DocumentImporter.convertProp.
        // A failed table property conversion must remain visible to the importer; swallowing it
        // leaves property 310 absent while allowing layout to continue with zero cell borders.
        let mut inner = self.inner.borrow_mut();
        with_attrs(attributes, |a| {
            inner.convertProp(Some(propType), mps, namespaceURI, elementId, a);
        });
        if std::env::var_os("WORD_TABLE_IMPORT_TRACE").is_some() && (propType == 4 || propType == 7) {
            let borders = mps.getValue(310)
                .and_then(|v| v.as_any().downcast_ref::<crate::cn::wps::moffice::writer::data::value::TableBorders::TableBorders>())
                .map(|b| (0..6).filter(|&i| b.getBorder(i).is_some()).count());
            eprintln!("table_prop type={propType} id={elementId} borders={borders:?}");
        }
    }

    fn convertRunProp(&self, prop: &mut dyn Any, namespaceURI: &str, elementId: i32, attributes: &dyn Any) {
        let Some(mps) = prop.downcast_mut::<MutablePropertySet>() else {
            return;
        };
        let _ = catch_unwind(AssertUnwindSafe(|| {
            let mut inner = self.inner.borrow_mut();
            with_attrs(attributes, |a| {
                inner.convertRunProp(mps, namespaceURI, elementId, a);
            });
        }));
    }

    fn correctProps(
        &self,
        elementId: i32,
        prop: &dyn Any,
        _original: Option<&dyn Any>,
    ) -> Box<dyn Any + Send + Sync> {
        let Some(mps) = prop.downcast_ref::<MutablePropertySet>() else {
            return Box::new(());
        };
        let mut clone = mps.clone_mps();
        match catch_unwind(AssertUnwindSafe(|| {
            self.inner
                .borrow_mut()
                .correctProps(elementId, &mut clone, None)
        })) {
            Ok(Some(out)) => Box::new(out),
            Ok(None) => Box::new(clone),
            Err(_) => Box::new(()),
        }
    }

    fn importCustomXmls(&self) {
        self.inner.borrow().importCustomXmls();
    }

    // Java: DocumentImporter.getDmlImporter(SubDocType). The bridge is the IDocumentImporter
    // DmlImporter later calls back into.
    fn getDmlImporter(&self, sub_doc_type: i32) -> Option<io_wps_io::cn::wps::io::xwpf::IDmlImporter::DmlImporterRef> {
        let bridge = self as *const dyn IDocumentImporter;
        let importer_ref = self.self_weak.upgrade().map(|bridge| bridge as Rc<dyn IDocumentImporter>);
        self.inner.borrow_mut().open_dml_importer(sub_doc_type, bridge, importer_ref)
    }

    // Java: DocumentImporter.getVmlImporter(SubDocType).
    fn getVmlImporter(&self, sub_doc_type: i32) -> Option<io_wps_io::cn::wps::io::xwpf::IVmlImporter::VmlImporterRef> {
        self.inner.borrow_mut().open_vml_importer(sub_doc_type)
    }
}
