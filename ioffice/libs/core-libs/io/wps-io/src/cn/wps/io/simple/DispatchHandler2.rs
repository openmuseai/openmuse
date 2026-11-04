#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.dom.io.DocumentEndOfParseException
// Java: import cn.wps.io.dom.io.Name2IdMap
use crate::io::dom::io::Name2IdMap::Name2IdMap;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Stack
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.SAXException

use super::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class DispatchHandler2 implements ElementHandlerSimple
pub struct DispatchHandler2 {
    // Java field: HashMap<Integer, ElementHandlerSimple> handlers
    handlers: HashMap<i32, ElementHandlerRef>,
    // Java field: Stack<ElementHandlerSimple> handlerStack
    handlerStack: Vec<Option<ElementHandlerRef>>,
    // Java field: boolean isFirst
    isFirst: bool,
}

// Java: public class DispatchHandler2 implements ElementHandlerSimple
impl DispatchHandler2 {
    // Java: DispatchHandler2()
    pub fn new() -> Self {
        // Java: super();
        DispatchHandler2 {
            handlers: HashMap::new(),
            handlerStack: Vec::new(),
            isFirst: true,
        }
    }

    // Java: void addHandler(int handlerIndex, ElementHandlerSimple handler)
    pub fn addHandler(&mut self, handler_index: i32, handler: ElementHandlerRef) {
        // Java: this.handlers.put(handlerIndex, handler);
        self.handlers.insert(handler_index, handler);
    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
        // Java: this.handlers.clear();
        self.handlers.clear();
        // Java: this.handlerStack.clear();
        self.handlerStack.clear();
    }

    // Java: static int getElementId(String namespaceURI, String qualifiedName, String localName)
    fn getElementId(namespace_uri: &str, _qualified_name: &str, local_name: &str) -> i32 {
        // Java: return Name2IdMap.getId3(namespaceURI, localName);
        Name2IdMap::getId3(namespace_uri, local_name)
    }

    // Java: void onStart(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_str(&mut self, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn std::any::Any) {
        // Java: ElementHandlerSimple handler = null;
        let mut handler: Option<ElementHandlerRef> = None;
        // Java: int elementId = getElementId(namespaceURI, qualifiedName, localName);
        let element_id = Self::getElementId(namespace_uri, qualified_name, local_name);
        // Java: if (this.isFirst)
        if self.isFirst {
            // Java: handler = (ElementHandlerSimple)this.handlers.get(elementId);
            handler = self.handlers.get(&element_id).cloned();
            // Java: this.isFirst = false;
            self.isFirst = false;
        // Java: else
        } else {
            // Java: handler = (ElementHandlerSimple)this.handlerStack.peek();
            let peeked = self.handlerStack.last().and_then(|h| h.clone());
            // Java: if (null != handler)
            // Java: handler = handler.getElementHandler(elementId, namespaceURI);
            handler = match peeked {
                Some(h) => h.borrow().getElementHandler(element_id, namespace_uri),
                None => None,
            };
        }
        // Java: if (handler != null)
        if let Some(ref h) = handler {
            // Java: handler.onStart(elementId, namespaceURI, localName, qualifiedName, attributes);
            let _ = h.borrow_mut().onStart_with_id(element_id, namespace_uri, local_name, qualified_name, attributes);
        }
        // Java: this.handlerStack.push(handler);
        self.handlerStack.push(handler);
    }

    // Java: void onEnd(String namespaceURI, String localName, String qName)
    pub fn onEnd_str(&mut self, namespace_uri: &str, local_name: &str, q_name: &str) {
        // Java: int elementId = getElementId(namespaceURI, qName, localName);
        let element_id = Self::getElementId(namespace_uri, q_name, local_name);
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.pop();
        let element_handler = self.handlerStack.pop().flatten();
        // Java: if (null != elementHandler)
        if let Some(ref eh) = element_handler {
            // Java: elementHandler.onEnd(elementId, namespaceURI);
            let _ = eh.borrow_mut().onEnd_with_id(element_id, namespace_uri);
        }
    }

    // Java: void onCharacters(char[] ch, int start, int end)
    pub fn onCharacters_buf(&mut self, ch: &[char], start: i32, end: i32) {
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.peek();
        let element_handler = self.handlerStack.last().and_then(|h| h.clone());
        // Java: if (elementHandler != null)
        if let Some(ref eh) = element_handler {
            // Java: elementHandler.onCharacters(ch, start, end);
            let _ = eh.borrow_mut().onCharacters(ch, start, end);
        }
    }

    // Java: void onCharacters(String persistPath)
    pub fn onCharacters_str(&mut self, persist_path: &str) {
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.peek();
        let element_handler = self.handlerStack.last().and_then(|h| h.clone());
        // Java: if (elementHandler != null)
        if let Some(ref eh) = element_handler {
            // Java: elementHandler.onCharacters(persistPath);
            let _ = eh.borrow_mut().onCharacters_str(persist_path);
        }
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_int(&self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _qualified_name: &str, _attributes: &dyn std::any::Any) {
    }

    // Java: int filterIndex()
    pub fn filterIndex(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, _element_id: i32, _namespace_uri: &str) -> Option<ElementHandlerRef> {
        // Java: return null;
        None
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_int(&self, _element_id: i32, _namespace_uri: &str) {
    }
}

impl ElementHandlerSimple for DispatchHandler2 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn filterIndex(&self) -> i32 {
        DispatchHandler2::filterIndex(self)
    }

    fn onStart(&mut self, var1: &str, var2: &str, var3: &str, var4: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
        self.onStart_str(var1, var2, var3, var4);
        Ok(())
    }

    fn onEnd(&mut self, var1: &str, var2: &str, var3: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.onEnd_str(var1, var2, var3);
        Ok(())
    }

    fn onStart_with_id(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any) -> Result<(), Box<dyn std::error::Error>> {
        self.onStart_int(var1, var2, var3, var4, var5);
        Ok(())
    }

    fn onEnd_with_id(&mut self, var1: i32, var2: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.onEnd_int(var1, var2);
        Ok(())
    }

    fn onCharacters(&mut self, var1: &[char], var2: i32, var3: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.onCharacters_buf(var1, var2, var3);
        Ok(())
    }

    fn onCharacters_str(&mut self, var1: &str) -> Result<(), Box<dyn std::error::Error>> {
        DispatchHandler2::onCharacters_str(self, var1);
        Ok(())
    }

    fn getElementHandler(&self, var1: i32, var2: &str) -> Option<ElementHandlerRef> {
        DispatchHandler2::getElementHandler(self, var1, var2)
    }
}
