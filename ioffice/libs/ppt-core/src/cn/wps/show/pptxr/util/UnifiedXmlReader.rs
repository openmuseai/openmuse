// Java: import cn.wps.kfc.xpxml.hlxml.HLXmlReader
use kfc_kfc_alpha::kfc::xpxml::hlxml::HLXmlReader::HLXmlReader;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import java.io.IOException
use std::io;
// Java: import java.io.InputStream
use std::io::Read;

use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use kfc_kfc_alpha::kfc::xpxml::reader::XmlRoAttrValue::XmlRoAttrValue;
use std::cell::RefCell;

use super::MapManager::MapManager;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler as DmlrHandler;

// java public final class UnifiedXmlReader
#[derive(Default)]
pub struct UnifiedXmlReader {
}

// java public final class UnifiedXmlReader
impl UnifiedXmlReader {
    // java public static void read(InputStream stream, ElementHandler handler)
    pub fn read__inputstream_elementhandler(stream: &mut dyn Read, handler: Box<dyn ElementHandler>) -> io::Result<()> {
      // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
      let mut reader: HLXmlReader = HLXmlReader::new(Box::new(MapManager::getInstance().clone()), handler);
      // java reader.parser(stream);
      reader.parser_stream(stream)
    }

    // java public static void read(String srcPath, ElementHandler handler)
    pub fn read__string_elementhandler(srcPath: &str, handler: Box<dyn ElementHandler>) -> io::Result<()> {
      // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
      let mut reader: HLXmlReader = HLXmlReader::new(Box::new(MapManager::getInstance().clone()), handler);
      // java reader.parser(srcPath);
      reader.parser_path(srcPath)
    }

    // java public static void read(String srcPath, int beginPos, int endPos, ElementHandler handler, boolean isBeginAtRoot)
    pub fn read__string_int_int_elementhandler_boolean(srcPath: &str, beginPos: i32, endPos: i32, handler: Box<dyn ElementHandler>, isBeginAtRoot: bool) -> io::Result<()> {
      // java HLXmlReader reader = new HLXmlReader(MapManager.getInstance(), handler);
      let mut reader: HLXmlReader = HLXmlReader::new(Box::new(MapManager::getInstance().clone()), handler);
      // java reader.parser(srcPath, beginPos, endPos, isBeginAtRoot);
      reader.parser_path_offsets_root(srcPath, beginPos, endPos, isBeginAtRoot)
    }

    /// 产品 `dmlr::ElementHandler` 经适配器交给 `HLXmlReader` → `HLHandlerBuilder.label`。
    /// `enterSubElement` 返回 null、或标签 id 为 -1 时，按 Java 跳过整棵子树。
    pub fn read__stream_elementhandler(
        stream: &mut dyn Read,
        handler: &mut dyn DmlrHandler,
    ) -> io::Result<()> {
        // `*mut dyn DmlrHandler` 默认是 `'static`。调用方的 `&mut` 活过本次 `parser_stream`。
        let inner: *mut dyn DmlrHandler = unsafe {
            std::mem::transmute::<*mut (dyn DmlrHandler + '_), *mut dyn DmlrHandler>(handler)
        };
        let adapter: Box<dyn ElementHandler> = Box::new(DmlrRoot { inner });
        let mut reader: HLXmlReader =
            HLXmlReader::new(Box::new(MapManager::getInstance().clone()), adapter);
        reader.parser_stream(stream)
    }
}

fn copy_attrs(attrs: &dyn XmlRoAttr) -> XmlRoAttrList {
    let mut list = XmlRoAttrList::new();
    for i in 0..attrs.count() {
        if let Some(leaf) = attrs.get_attr_at(i) {
            if leaf.is_leaf() {
                list.add_attr(leaf.get_name(), &XmlRoAttrValue::to_string(leaf));
            }
        }
    }
    list
}

/// 根 handler 的生命周期是 `read__stream_elementhandler` 的 `&mut`。
/// `HLHandlerBuilder` 只接受 `'static` 的 `Box<dyn ElementHandler>`，解析结束前指针一直有效。
struct DmlrRoot {
    inner: *mut dyn DmlrHandler,
}

impl ElementHandler for DmlrRoot {
    fn startElement(&self, element: i32, attrs: &dyn XmlRoAttr) {
        let list = copy_attrs(attrs);
        unsafe { (*self.inner).startElement(element, &list) };
    }
    fn endElement(&self, element: i32) {
        unsafe { (*self.inner).endElement(element) };
    }
    fn characters(&self, chars: &str) {
        unsafe { (*self.inner).characters(chars) };
    }
    fn enterSubElement(&self, sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        let child = unsafe { (*self.inner).enterSubElement(sub_element) }?;
        Some(Box::new(DmlrOwned {
            inner: RefCell::new(child),
        }))
    }
}

struct DmlrOwned {
    inner: RefCell<Box<dyn DmlrHandler>>,
}

impl ElementHandler for DmlrOwned {
    fn startElement(&self, element: i32, attrs: &dyn XmlRoAttr) {
        let list = copy_attrs(attrs);
        self.inner.borrow_mut().startElement(element, &list);
    }
    fn endElement(&self, element: i32) {
        self.inner.borrow_mut().endElement(element);
    }
    fn characters(&self, chars: &str) {
        self.inner.borrow_mut().characters(chars);
    }
    fn enterSubElement(&self, sub_element: i32) -> Option<Box<dyn ElementHandler>> {
        let child = self.inner.borrow_mut().enterSubElement(sub_element)?;
        Some(Box::new(DmlrOwned {
            inner: RefCell::new(child),
        }))
    }
}
