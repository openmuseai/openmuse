#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::prop::Prop::Prop;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};

// Java: public class THandler extends XmlSimpleNodeElementHandler
pub struct THandler {
    // Java field: RHandler mRHandler
    pub(crate) mRHandler: Option<Rc<RefCell<super::RHandler::RHandler>>>,
    pub(crate) mRunProp: Option<Rc<RefCell<Prop>>>,
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: boolean mIsRuby
    pub(crate) mIsRuby: bool,
    // Java field: StringBuilder mTmpText
    pub(crate) mTmpText: Option<String>,
    pub(crate) mDocImporter: Option<Rc<dyn IDocumentImporter>>,
}

// Java: public class THandler extends XmlSimpleNodeElementHandler
impl THandler {
    // Java: THandler(IDocumentImporter importer, SubDocType subDocType, RHandler rHandler)
    pub fn new(
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        rHandler: Option<Rc<RefCell<super::RHandler::RHandler>>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mRHandler = rHandler;
        // Java: this.mSubDocType = subDocType;
        // Java: this.mIsRuby = false;
        // Java: this.mTmpText = null;
        THandler {
            mRHandler: rHandler,
            mRunProp: None,
            mSubDocType: subDocType,
            mIsRuby: false,
            mTmpText: None,
            mDocImporter: Some(importer),
        }
    }

    // Java: THandler(IDocumentImporter importer, SubDocType subDocType, RHandler rHandler, boolean isRuby)
    pub fn new_with_ruby(
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        rHandler: Option<Rc<RefCell<super::RHandler::RHandler>>>,
        isRuby: bool,
    ) -> Self {
        // Java: this(importer, subDocType, rHandler);
        let mut handler = Self::new(importer, subDocType, rHandler);
        // Java: this.mIsRuby = isRuby;
        handler.mIsRuby = isRuby;
        handler
    }

    pub fn with_run_prop(mut self, prop: Rc<RefCell<Prop>>) -> Self {
        self.mRunProp = Some(prop);
        self
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onCharacters(char[] buffer, int start, int len)
    pub fn onCharacters_impl(&mut self, buffer: &[char], start: i32, len: i32) {
        // Java: if (this.mIsRuby)
        if self.mIsRuby {
            // Java: if (null == this.mTmpText)
            if self.mTmpText.is_none() {
                // Java: this.mTmpText = new StringBuilder();
                self.mTmpText = Some(String::new());
            }
            // Java: this.mTmpText.append(new String(buffer, start, len));
            let s: String = buffer
                .iter()
                .skip(start as usize)
                .take(len as usize)
                .collect();
            if let Some(ref mut tmp) = self.mTmpText {
                tmp.push_str(&s);
            }
        // Java: else
        } else if let Some(ref importer) = self.mDocImporter {
            // Java: this.mDocImporter.onImportRun(this.mSubDocType, null == this.mRHandler ? null : this.mRHandler.getProp(), buffer, start, len);
            let from_shared = self.mRunProp.as_ref().and_then(|p| {
                let mut slot = p.borrow_mut();
                if slot.isContainProp() {
                    Some(Box::new(slot.getProp().clone_mps()) as Box<dyn Any + Send + Sync>)
                } else {
                    None
                }
            });
            let from_r = self.mRHandler.as_ref().map(|r| {
                Box::new(r.borrow().getProp()) as Box<dyn Any + Send + Sync>
            });
            let prop = from_shared.or(from_r);
            let prop_ref: Option<&dyn Any> = prop.as_ref().map(|b| b.as_ref() as &dyn Any);
            importer.onImportRun_chars(
                self.mSubDocType,
                prop_ref.unwrap_or(&()),
                buffer,
                start,
                len,
            );
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart(
        &self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) {
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: void clearTmpText()
    pub fn clearTmpText(&mut self) {
        // Java: this.mTmpText = null;
        self.mTmpText = None;
    }
}

impl ElementHandlerSimple for THandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        THandler::filterIndex(self)
    }
    fn onStart(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
        _d: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onEnd(
        &mut self,
        _a: &str,
        _b: &str,
        _c: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onStart_with_id(
        &mut self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        THandler::onStart(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        Ok(())
    }
    fn onEnd_with_id(
        &mut self,
        _elementId: i32,
        _namespaceURI: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters(
        &mut self,
        buffer: &[char],
        start: i32,
        len: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.onCharacters_impl(buffer, start, len);
        Ok(())
    }
    fn onCharacters_str(&mut self, _s: &str) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        None
    }
}
