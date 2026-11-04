#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
use core_libs_util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.dom.io.WMap
use crate::io::dom::io::WMap::WMap;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.moffice.property.MutablePropertySet
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use super::RHandler::RHandler;
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};

// Java: public class BrHandler extends XmlSimpleNodeElementHandler
pub struct BrHandler {
    // Java field: SubDocType mSubDocType
    pub(crate) mSubDocType: i32,
    // Java field: RHandler mRHandler
    pub(crate) mRHandler: Option<Rc<RefCell<RHandler>>>,
    pub(crate) mDocImporter: Option<Rc<dyn IDocumentImporter>>,
}

// Java: public class BrHandler extends XmlSimpleNodeElementHandler
impl BrHandler {
    // Java: BrHandler(IDocumentImporter importer, SubDocType subDocType, RHandler rHandler)
    pub fn new(
        importer: Rc<dyn IDocumentImporter>,
        subDocType: i32,
        rHandler: Option<Rc<RefCell<RHandler>>>,
    ) -> Self {
        // Java: super(importer);
        // Java: this.mSubDocType = subDocType;
        // Java: this.mRHandler = rHandler;
        BrHandler {
            mSubDocType: subDocType,
            mRHandler: rHandler,
            mDocImporter: Some(importer),
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef {
        Rc::new(RefCell::new(self))
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_impl(
        &self,
        elementId: i32,
        namespaceURI: &str,
        localName: &str,
        qualifiedName: &str,
        attributes: &dyn Any,
    ) {
        // Java: MutablePropertySet dst = null == this.mRHandler ? null : this.mRHandler.getProp();
        let mut dst: MutablePropertySet = match self.mRHandler.as_ref() {
            Some(r) => r.borrow().getProp(),
            None => MutablePropertySet::new(),
        };
        // Java: if (null == dst) { dst = new MutablePropertySet(); }
        // rust: Java 的 `null` 分支由上面的 `None => MutablePropertySet::new()` 覆盖。
        // Java: this.mDocImporter.convertRunProp(dst, namespaceURI, elementId, attributes);
        if let Some(ref importer) = self.mDocImporter {
            importer.convertRunProp(
                &mut dst as &mut dyn Any,
                namespaceURI,
                elementId,
                attributes,
            );
        }
        // Java: this.mDocImporter.onImportRun(this.mSubDocType, dst, convertBreakText(attributes));
        if let Some(ref importer) = self.mDocImporter {
            importer.onImportRun_str(
                self.mSubDocType,
                &dst as &dyn Any,
                &BrHandler::convertBreakText(attributes),
            );
        }
    }

    // Java: private static String convertBreakText(Attributes attributes)
    pub fn convertBreakText(attributes: &dyn Any) -> String {
        // Java: String type = attributes.getValue(WMap.MAIN_NAMESPACING, "type");
        let type_: Option<String> = get_attr(attributes, WMap::MAIN_NAMESPACING(), "type");
        // Java: if (type != null)
        if let Some(type_) = type_ {
            // Java: if (type.equalsIgnoreCase("page")) return "\f";
            if type_.eq_ignore_ascii_case("page") {
                return "\u{000c}".to_string();
            }
            // Java: if (type.equalsIgnoreCase("column")) return "\u000e";
            if type_.eq_ignore_ascii_case("column") {
                return "\u{000e}".to_string();
            }
            // Java: if (type.equalsIgnoreCase("textWrapping")) return "\u000b";
            if type_.eq_ignore_ascii_case("textWrapping") {
                return "\u{000b}".to_string();
            }
            // Java: IOAssert.fail("It should not reach to here.");
            IOAssert::fail__string(&"It should not reach to here.");
        }
        // Java: return "\u000b";
        "\u{000b}".to_string()
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }
}

/// Adapter: the SAX driver hands `attributes` as `&dyn Any` (an `AttributesImpl`
/// downcast target in `SaxDocumentBridge`). Java calls
/// `attributes.getValue(WMap.MAIN_NAMESPACING, "type")` directly.
fn get_attr(attributes: &dyn Any, uri: &str, localName: &str) -> Option<String> {
    use crate::io::opt::AttributesImpl::AttributesImpl;
    if let Some(a) = attributes.downcast_ref::<AttributesImpl>() {
        return a.getValue_by_uri(uri, localName);
    }
    None
}

impl ElementHandlerSimple for BrHandler {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn filterIndex(&self) -> i32 {
        BrHandler::filterIndex(self)
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
        // rust: ★P0/L0-1 —— Java `BrHandler.onStart`。
        //   捕获 panic：`IOAssert.fail` 对未知 `w:br type` 会 panic（Java 抛 SAXException），
        //   与同仓其它 handler 的 swallow 约定一致，避免解析中止。
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            BrHandler::onStart_impl(self, elementId, namespaceURI, localName, qualifiedName, attributes);
        }));
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
        _buffer: &[char],
        _start: i32,
        _len: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn onCharacters_str(&mut self, _s: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Java: XmlSimpleNodeElementHandler.onCharacters(String) is a no-op; `w:br` is empty.
        Ok(())
    }
    fn getElementHandler(&self, _id: i32, _ns: &str) -> Option<ElementHandlerRef> {
        // Java: XmlSimpleNodeElementHandler has no getElementHandler override ⇒ null.
        None
    }
}
