#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.dom.DocumentException
use crate::io::dom::DocumentException::DocumentException;
// Java: import cn.wps.io.dom.ElementHandler
// Java: import cn.wps.io.dom.io.DispatchHandler
use crate::io::dom::io::DispatchHandler::DispatchHandler;
// Java: import cn.wps.io.dom.io.DocumentEndOfParseException
use crate::io::dom::io::DocumentEndOfParseException::DocumentEndOfParseException;
// Java: import cn.wps.io.dom.io.SAXHelper
use crate::io::dom::io::SAXHelper::SAXHelper;
// Java: import java.io.InputStream
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXParseException
// Java: import org.xml.sax.XMLReader

use super::DispatchHandler2::DispatchHandler2;
use super::ElementHandlerSimple::ElementHandlerRef;
use super::SAXContentHandler2::SAXContentHandler2;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

/// Placeholder for org.xml.sax.XMLReader (no real SAX stack in this crate yet).
struct XmlReaderStub {
    content_handler: Option<SAXContentHandler2>,
}

impl XmlReaderStub {
    fn new() -> Self {
        XmlReaderStub { content_handler: None }
    }
    fn setContentHandler(&mut self, handler: SAXContentHandler2) {
        self.content_handler = Some(handler);
    }
    fn parse(&mut self, _input: &dyn Any) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

// Java: public class SAXSimpleReader
pub struct SAXSimpleReader {
    // Java field: XMLReader xmlReader
    xmlReader: Option<Box<XmlReaderStub>>,
    // Java field: boolean validating
    validating: bool,
    // Java field: String encoding
    encoding: Option<String>,
    // Java field: DispatchHandler2 dispatchHandler
    dispatchHandler: Option<ElementHandlerRef>,
    // Java field: DispatchHandler previousDispatchHandler
    previousDispatchHandler: Option<DispatchHandler>,
}

// Java: public class SAXSimpleReader
impl SAXSimpleReader {
    // Java: SAXSimpleReader()
    pub fn new() -> Self {
        // Java: super();
        SAXSimpleReader {
            xmlReader: None,
            validating: false,
            encoding: None,
            dispatchHandler: None,
            previousDispatchHandler: None,
        }
    }

    // Java: XMLReader createXMLReader()
    fn createXMLReader(&self) -> Result<Box<XmlReaderStub>, DocumentException> {
        // Java: return SAXHelper.createXMLReader(this.isValidating());
        match SAXHelper::createXMLReader(self.isValidating()) {
            Ok(_) => Ok(Box::new(XmlReaderStub::new())),
            Err(e) => Err(DocumentException::new_with_message(e)),
        }
    }

    // Java: boolean isValidating()
    pub fn isValidating(&self) -> bool {
        // Java: return this.validating;
        self.validating
    }

    // Java: void setEncoding(String encoding)
    pub fn setEncoding(&mut self, encoding: &str) {
        // Java: this.encoding = encoding;
        self.encoding = Some(encoding.to_string());
    }

    // Java: void read(InputStream in)
    pub fn read_stream(&mut self, input: Box<dyn Any + Send + Sync>) -> Result<(), DocumentException> {
        // Java: InputSource source = new InputSource(in);
        let source: Box<dyn Any + Send + Sync> = input;
        // Java: if (this.encoding != null)
        if self.encoding.is_some() {
            // Java: source.setEncoding(this.encoding);
            let _ = self.encoding.as_ref();
        }
        // Java: this.read(source);
        self.read_source(source)
    }

    // Java: void read(InputSource in)
    pub fn read_source(&mut self, input: Box<dyn Any + Send + Sync>) -> Result<(), DocumentException> {
        // Java: try
        let dispatch = self.dispatchHandler.clone();
        let result = (|| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Java: XMLReader reader = this.getXMLReader();
            let reader = self.getXMLReader_mut().map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { Box::new(e) })?;
            // Java: SAXContentHandler2 contentHandler = new SAXContentHandler2(this.dispatchHandler);
            let content_handler = SAXContentHandler2::new(dispatch);
            // Java: reader.setContentHandler(contentHandler);
            reader.setContentHandler(content_handler);
            // Java: reader.parse(in);
            reader.parse(input.as_ref())?;
            Ok(())
        })();
        // Java: catch (Exception e)
        match result {
            Ok(()) => Ok(()),
            Err(e) => {
                // Java: if (e instanceof SAXParseException)
                // Java: SAXParseException parseException = (SAXParseException)e;
                // Java: String systemId = parseException.getSystemId();
                // Java: if (systemId == null)
                // Java: systemId = "";
                // Java: String message = "Error on line " + parseException.getLineNumber() + " of document " + systemId + " : " + parseException.getMessage();
                // Java: throw new DocumentException(message, e);
                // Java: if (!(e instanceof DocumentEndOfParseException))
                if e.downcast_ref::<DocumentEndOfParseException>().is_none() {
                    // Java: throw new DocumentException(e.getMessage(), e);
                    return Err(DocumentException::new_with_message_cause(e.to_string(), e));
                }
                Ok(())
            }
        }
    }

    // Java: void addHandler(int index, ElementHandlerSimple handler)
    pub fn addHandler(&mut self, index: i32, handler: ElementHandlerRef) {
        // Java: this.getDispatchHandler().addHandler(index, handler);
        self.getDispatchHandler().addHandler(index, handler);
    }

    // Java: void addPreviousHandler(String name, ElementHandler handler)
    pub fn addPreviousHandler(&mut self, name: &str, handler: i32) {
        // Java: this.getPreviousDispatchHandler().addHandler(name, handler);
        self.getPreviousDispatchHandler().addHandler(name, handler);
    }

    // Java: DispatchHandler getPreviousDispatchHandler()
    fn getPreviousDispatchHandler(&mut self) -> &mut DispatchHandler {
        // Java: if (this.previousDispatchHandler == null)
        if self.previousDispatchHandler.is_none() {
            // Java: this.previousDispatchHandler = new DispatchHandler();
            self.previousDispatchHandler = Some(DispatchHandler::new());
        }
        // Java: return this.previousDispatchHandler;
        self.previousDispatchHandler.as_mut().unwrap()
    }

    // Java: DispatchHandler2 getDispatchHandler()
    fn getDispatchHandler(&mut self) -> std::cell::RefMut<'_, DispatchHandler2> {
        // Java: if (this.dispatchHandler == null)
        if self.dispatchHandler.is_none() {
            // Java: this.dispatchHandler = new DispatchHandler2();
            self.dispatchHandler = Some(Rc::new(RefCell::new(DispatchHandler2::new())));
        }
        // Java: return this.dispatchHandler;
        let dh = self.dispatchHandler.as_ref().unwrap();
        std::cell::RefMut::map(dh.borrow_mut(), |h| {
            h.as_any_mut()
                .downcast_mut::<DispatchHandler2>()
                .expect("dispatchHandler is DispatchHandler2")
        })
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&mut self) -> Result<&Box<XmlReaderStub>, DocumentException> {
        // Java: if (this.xmlReader == null)
        if self.xmlReader.is_none() {
            // Java: this.xmlReader = this.createXMLReader();
            self.xmlReader = Some(self.createXMLReader()?);
        }
        // Java: return this.xmlReader;
        Ok(self.xmlReader.as_ref().unwrap())
    }

    fn getXMLReader_mut(&mut self) -> Result<&mut XmlReaderStub, DocumentException> {
        if self.xmlReader.is_none() {
            self.xmlReader = Some(self.createXMLReader()?);
        }
        Ok(self.xmlReader.as_mut().unwrap().as_mut())
    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
        // Java: if (this.dispatchHandler != null)
        if let Some(ref dh) = self.dispatchHandler {
            // Java: this.dispatchHandler.resetHandlers();
            let mut borrowed = dh.borrow_mut();
            borrowed
                .as_any_mut()
                .downcast_mut::<DispatchHandler2>()
                .expect("dispatchHandler is DispatchHandler2")
                .resetHandlers();
        }
    }
}
