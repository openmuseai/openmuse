// Java: cn.wps.moffice.writer.io.reader.dmlReader.TextImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::TextboxInfoTool::TextboxInfoTool;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext, TxbxChains::TxbxChains},
};

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
fn empty_int(attrs: &dyn SaxAttributes, name: &str) -> Option<i32> {
    attrs.getValue_by_uri("", name).and_then(|s| TypeConversion::string2Integer(&s))
}
fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn to_boolean(value: &str) -> bool {
    !(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f"))
}
fn text_mut(text: &Arc<Text>) -> &mut Text {
    unsafe { &mut *(Arc::as_ptr(text) as *mut Text) }
}

// Java: public class TextImporter extends DmlImporterBase
pub struct TextImporter<ShapeImporter, DmlCommonImporter> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private Text mTarget;
    pub mTarget: Option<Arc<Text>>,
}

impl<S, D: IImporterBase> Deref for TextImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for TextImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> TextImporter<S, D> {
    // Java: public TextImporter(...)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self { super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None }
    }

    // Java: public Text result()
    pub fn result(&self) -> Option<Arc<Text>> { self.mTarget.clone() }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_some() { return; }
        let shape = self.mCtxt.borrow().mShape.clone();
        let Some(shape) = shape else { return; };
        let existing = shape.lock().expect("shape").getTextObj()
            .and_then(|arc| arc.downcast::<Text>().ok());
        if let Some(text) = existing {
            self.mTarget = Some(text);
            return;
        }
        let created = shape.lock().expect("shape").createAndAttachText()
            .and_then(|arc| arc.downcast::<Text>().ok());
        if let Some(text) = created {
            text_mut(&text).setShape(Some(shape.clone() as Arc<dyn std::any::Any + Send + Sync>));
            self.mTarget = Some(text);
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) { self.mTarget = None; }

    // Java: public void startTextbox(Attributes attrs)
    pub fn startTextbox(&mut self, attrs: &dyn SaxAttributes) {
        self.initTarget();
        let shape = self.mCtxt.borrow().mShape.clone().expect("mShape");
        let doc = self.mCtxt.borrow().mDoc.clone().expect("mDoc");
        let doc_type = IDocument::getType(doc.borrow().as_ref());
        let shape_id = shape.lock().expect("shape").getShapeID();
        let textbox = IOHelper::getTextboxDocument(doc.borrow().as_ref());
        if let Some(node) = IOHelper::addTextboxTextNode(textbox, &shape.lock().expect("shape")) {
            TextboxInfoTool::collectTextNode(shape_id, node);
        }
        TextboxInfoTool::collect(&shape, doc_type);
        if let Some(id) = empty_int(attrs, "id") {
            let text = self.mTarget.clone().expect("mTarget");
            let mut sub = self.mSubDocCtxt.borrow_mut();
            if sub.mTxbxChains.is_none() {
                sub.mTxbxChains = Some(Rc::new(RefCell::new(TxbxChains::new())));
            }
            sub.mTxbxChains.as_ref().expect("mTxbxChains").borrow_mut().put(id, 0, shape_id, text);
        }
    }

    // Java: public void setLinkedTxbx(Attributes attrs)
    pub fn setLinkedTxbx(&mut self, attrs: &dyn SaxAttributes) {
        self.initTarget();
        let id = empty_int(attrs, "id");
        let seq = empty_int(attrs, "seq");
        if let (Some(id), Some(seq)) = (id, seq) {
            let shape = self.mCtxt.borrow().mShape.clone().expect("mShape");
            let shape_id = shape.lock().expect("shape").getShapeID();
            let text = self.mTarget.clone().expect("mTarget");
            let mut sub = self.mSubDocCtxt.borrow_mut();
            if sub.mTxbxChains.is_none() {
                sub.mTxbxChains = Some(Rc::new(RefCell::new(TxbxChains::new())));
            }
            sub.mTxbxChains.as_ref().expect("mTxbxChains").borrow_mut().put(id, seq, shape_id, text);
        }
    }

    // Java: public void endTextbox()
    pub fn endTextbox(&mut self) {
        self.initTarget();
        let doc = self.mCtxt.borrow().mDoc.clone().expect("mDoc");
        let textbox = IOHelper::getTextboxDocument(doc.borrow().as_ref());
        IOHelper::addTextboxSeparator(textbox);
    }

    // Java: public void importBodyPr(Attributes attrs)
    pub fn importBodyPr(&mut self, attrs: &dyn SaxAttributes) {
        let Some(target) = self.mTarget.clone() else { return; };
        let text = text_mut(&target);
        if let Some(value) = empty_int(attrs, "lIns") {
            text.setMarginLeft(MetricUtility::EMU2Points(value as f32));
        }
        if let Some(value) = empty_int(attrs, "rIns") {
            text.setMarginRight(MetricUtility::EMU2Points(value as f32));
        }
        if let Some(value) = empty_int(attrs, "tIns") {
            text.setMarginTop(MetricUtility::EMU2Points(value as f32));
        }
        if let Some(value) = empty_int(attrs, "bIns") {
            text.setMarginBottom(MetricUtility::EMU2Points(value as f32));
        }
        let _angle = IOHelper::Angle2Integer(main_string(attrs, "rot").as_deref());
        let _vert_overflow = main_string(attrs, "vertOverflow");
        let _horz_overflow = main_string(attrs, "horzOverflow");
        if let Some(value) = main_string(attrs, "vert") {
            text.setTextFlow(convert2_text_flow(&value));
        }
        let east_asian = self.mCtxt.borrow().mNormalEastAsianFlow;
        if east_asian == Some(true) {
            text.setTextFlow(4);
        }
        if let Some(value) = main_string(attrs, "anchor") {
            text.setTextAnchorV(convert2_text_anchor_v(&value));
        }
        if let Some(value) = main_string(attrs, "wrap") {
            text.setWrapText(convert2_wrap_text(&value));
        }
        if let Some(value) = attrs.getValue_by_uri("", "upright") {
            text.setTextRotate(!to_boolean(&value));
        }
    }

    // Java: public void setNormalEastAsianFlow(Attributes attributes)
    pub fn setNormalEastAsianFlow(&mut self, attributes: &dyn SaxAttributes) {
        let value = attributes.getValue_by_uri("", "normalEastAsianFlow").map(|s| to_boolean(&s));
        self.mCtxt.borrow_mut().mNormalEastAsianFlow = value;
    }

    // Java: public void importFitShape2Text(boolean fitShape2Text)
    pub fn importFitShape2Text(&mut self, fitShape2Text: bool) {
        if let Some(target) = &self.mTarget {
            text_mut(target).setFitShape2Text(fitShape2Text);
        }
    }
}

fn convert2_text_flow(value: &str) -> i32 {
    match java_hash(value) {
        -1311178771 => 1,
        3208591 => 0,
        3616049 => 3,
        351583162 => 2,
        _ => 0,
    }
}
fn convert2_text_anchor_v(value: &str) -> i32 {
    match java_hash(value) {
        98 => 2,
        116 => 0,
        98849 => 1,
        _ => 1,
    }
}
fn convert2_wrap_text(value: &str) -> i32 {
    match java_hash(value) {
        -894674659 => 0,
        3387192 => 2,
        _ => 0,
    }
}
