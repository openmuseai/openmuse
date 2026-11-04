#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.dom.io.DocumentEndOfParseException
use crate::io::dom::io::DocumentEndOfParseException::DocumentEndOfParseException;
// Java: import cn.wps.io.simple.ElementHandlerSimple
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
// Java: import cn.wps.io.simple.ForceQuitException
use crate::io::simple::ForceQuitException::ForceQuitException;
// Java: import cn.wps.io.simple.ReadingLimitException
use crate::io::simple::ReadingLimitException::ReadingLimitException;

// Java: public class ContentHandlerImpl implements ContentHandler
pub struct ContentHandlerImpl {
    // Java field: Stack<ElementHandlerSimple> handlerStack
    handlerStack: Vec<Option<ElementHandlerRef>>,
    // Java field: ElementHandlerSimple rootHandler
    rootHandler: Option<ElementHandlerRef>,
    // Java field: boolean isFirst
    isFirst: bool,
    // Java field: Stack<String> elementStackDebug
    elementStackDebug: Option<Vec<String>>,
}

// Java: public class ContentHandlerImpl implements ContentHandler
impl ContentHandlerImpl {
    // Java: ContentHandlerImpl(ElementHandlerSimple elementHandler)
    pub fn new(elementHandler: ElementHandlerRef) -> Self {
        // Java: super();
        // Java: this.isFirst = true;
        // Java: this.rootHandler = elementHandler;
        ContentHandlerImpl {
            handlerStack: Vec::new(),
            rootHandler: Some(elementHandler),
            isFirst: true,
            elementStackDebug: None,
        }
    }

    // Java: void startElement(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn startElement(&mut self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
        // Java: ElementHandlerSimple handler = null;
        let mut handler: Option<ElementHandlerRef> = None;
        // Java: if (this.isFirst)
        if self.isFirst {
            // Java: handler = this.rootHandler;
            handler = self.rootHandler.clone();
            // Java: this.isFirst = false;
            self.isFirst = false;
        }
        // Java: else
        else {
            // Java: handler = (ElementHandlerSimple)this.handlerStack.peek();
            let peeked = self.handlerStack.last().and_then(|h| h.clone());
            // Java: if (null != handler)
            // Java: handler = handler.getElementHandler(elementId, namespaceURI);
            handler = match peeked {
                Some(h) => h.borrow().getElementHandler(elementId, namespaceURI),
                None => None,
            };
        }
        // Java: this.handlerStack.push(handler);
        self.handlerStack.push(handler.clone());
        // Java: if (handler != null)
        if let Some(ref h) = handler {
        // Java: try
        {
            // Java: handler.onStart(elementId, namespaceURI, localName, qualifiedName, attributes);
            match h.borrow_mut().onStart_with_id(elementId, namespaceURI, localName, qualifiedName, attributes) {
                Ok(()) => {}
                Err(e) => {
                    // Java: catch (ForceQuitException e)
                    if let Some(fq) = e.downcast_ref::<ForceQuitException>() {
                        // Java: throw new cn.wps.moffice.io.ForceQuitException(e.getErrorCode());
                        panic!("ForceQuitException({})", fq.getErrorCode());
                    }
                    // Java: catch (ReadingLimitException var9)
                    if e.downcast_ref::<ReadingLimitException>().is_some() {
                        // Java: throw new cn.wps.moffice.io.ReadingLimitException();
                        panic!("ReadingLimitException");
                    }
                    // Java: catch (SAXException e)
                    {
                        // Java: e.printStackTrace();
                        eprintln!("{:?}", e);
                    }
                }
            }
        }
        }
    }

    // Java: void endElement(int elementId, String namespaceURI, String localName, String qName)
    pub fn endElement(&mut self, elementId: i32, namespaceURI: &str, _localName: &str, _qName: &str) {
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.pop();
        let elementHandler = self.handlerStack.pop().flatten();
        // Java: if (null != elementHandler)
        if let Some(ref eh) = elementHandler {
        // Java: try
        {
            // Java: elementHandler.onEnd(elementId, namespaceURI);
            if let Err(e) = eh.borrow_mut().onEnd_with_id(elementId, namespaceURI) {
                // Java: catch (SAXException e)
                {
                    // Java: e.printStackTrace();
                    eprintln!("{:?}", e);
                }
            }
        }
        }
    }

    // Java: void character(char[] value, int start, int len)
    pub fn character_buf(&mut self, value: &[char], start: i32, len: i32) {
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.peek();
        let elementHandler = self.handlerStack.last().and_then(|h| h.clone());
        // Java: if (null != elementHandler)
        if let Some(ref eh) = elementHandler {
        // Java: try
        {
            // Java: elementHandler.onCharacters(value, start, len);
            if let Err(e) = eh.borrow_mut().onCharacters(value, start, len) {
                // Java: catch (DocumentEndOfParseException e)
                if e.downcast_ref::<DocumentEndOfParseException>().is_some() {
                    // Java: e.printStackTrace();
                    eprintln!("{:?}", e);
                } else {
                    eprintln!("{:?}", e);
                }
            }
        }
        }
    }

    // Java: void character(String persistPath)
    pub fn character_str(&mut self, persistPath: &str) {
        // Java: ElementHandlerSimple elementHandler = (ElementHandlerSimple)this.handlerStack.peek();
        let elementHandler = self.handlerStack.last().and_then(|h| h.clone());
        // Java: if (null != elementHandler)
        if let Some(ref eh) = elementHandler {
        // Java: try
        {
            // Java: elementHandler.onCharacters(persistPath);
            if let Err(e) = eh.borrow_mut().onCharacters_str(persistPath) {
                // Java: catch (DocumentEndOfParseException e)
                if e.downcast_ref::<DocumentEndOfParseException>().is_some() {
                    // Java: e.printStackTrace();
                    eprintln!("{:?}", e);
                } else {
                    eprintln!("{:?}", e);
                }
            }
        }
        }
    }
}

impl super::ContentHandler::ContentHandler for ContentHandlerImpl {
    fn startElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any) {
        ContentHandlerImpl::startElement(self, var1, var2, var3, var4, var5);
    }
    fn endElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str) {
        ContentHandlerImpl::endElement(self, var1, var2, var3, var4);
    }
    fn character_buf(&mut self, var1: &[char], var2: i32, var3: i32) {
        ContentHandlerImpl::character_buf(self, var1, var2, var3);
    }
    fn character_str(&mut self, var1: &str) {
        ContentHandlerImpl::character_str(self, var1);
    }
}
