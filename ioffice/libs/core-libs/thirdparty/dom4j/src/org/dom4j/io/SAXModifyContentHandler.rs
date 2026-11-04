// Java: import java.io.IOException
// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.ElementHandler
// use crate::org::dom4j::ElementHandler;
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;
// Java: import org.xml.sax.Locator
// use crate::org::xml::sax::Locator;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;

// Java: public class SAXModifyContentHandler extends SAXContentHandler
pub struct SAXModifyContentHandler {
    // Java field: XMLWriter xmlWriter
    xmlWriter: i32,
}

// Java: public class SAXModifyContentHandler extends SAXContentHandler
impl SAXModifyContentHandler {
    // Java: SAXModifyContentHandler()
    pub fn SAXModifyContentHandler(&self) {
      // Java: super();
      todo!()
    }

    // Java: SAXModifyContentHandler(DocumentFactory documentFactory)
    pub fn SAXModifyContentHandler_factory(&self, _documentFactory: i32) {
      // Java: super(documentFactory);
      todo!()
    }

    // Java: SAXModifyContentHandler(DocumentFactory documentFactory, ElementHandler elementHandler)
    pub fn SAXModifyContentHandler_factory_handler(&self, _documentFactory: i32, _elementHandler: i32) {
      // Java: super(documentFactory, elementHandler, false);
      todo!()
    }

    // Java: SAXModifyContentHandler(DocumentFactory documentFactory, ElementHandler elementHandler, ElementStack elementStack)
    pub fn SAXModifyContentHandler_factory_handler_stack(&self, _documentFactory: i32, _elementHandler: i32, _elementStack: i32) {
      // Java: super(documentFactory, elementHandler, elementStack);
      todo!()
    }

    // Java: void setXMLWriter(XMLWriter writer)
    pub fn setXMLWriter(&self, _writer: i32) {
      // Java: this.xmlWriter = writer;
      todo!()
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
      // Java: super.startCDATA();
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startCDATA();
         todo!()
      }
    }

    // Java: void startDTD(String name, String publicId, String systemId)
    pub fn startDTD(&self, _name: &str, _publicId: &str, _systemId: &str) {
      // Java: super.startDTD(name, publicId, systemId);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startDTD(name, publicId, systemId);
         todo!()
      }
    }

    // Java: void endDTD()
    pub fn endDTD(&self) {
      // Java: super.endDTD();
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.endDTD();
         todo!()
      }
    }

    // Java: void comment(char[] characters, int parm2, int parm3)
    pub fn comment(&self, _characters: &[char], _parm2: i32, _parm3: i32) {
      // Java: super.comment(characters, parm2, parm3);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.comment(characters, parm2, parm3);
         todo!()
      }
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&self, _name: &str) {
      // Java: super.startEntity(name);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startEntity(name);
         todo!()
      }
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
      // Java: super.endCDATA();
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.endCDATA();
         todo!()
      }
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&self, _name: &str) {
      // Java: super.endEntity(name);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.endEntity(name);
         todo!()
      }
    }

    // Java: void unparsedEntityDecl(String name, String publicId, String systemId, String notation)
    pub fn unparsedEntityDecl(&self, _name: &str, _publicId: &str, _systemId: &str, _notation: &str) {
      // Java: super.unparsedEntityDecl(name, publicId, systemId, notation);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.unparsedEntityDecl(name, publicId, systemId, notation);
         todo!()
      }
    }

    // Java: void notationDecl(String name, String publicId, String systemId)
    pub fn notationDecl(&self, _name: &str, _publicId: &str, _systemId: &str) {
      // Java: super.notationDecl(name, publicId, systemId);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.notationDecl(name, publicId, systemId);
         todo!()
      }
    }

    // Java: void startElement(String uri, String localName, String qName, Attributes atts)
    pub fn startElement(&self, _uri: &str, _localName: &str, _qName: &str, _atts: i32) {
      // Java: super.startElement(uri, localName, qName, atts);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startElement(uri, localName, qName, atts);
         todo!()
      }
    }

    // Java: void startDocument()
    pub fn startDocument(&self) {
      // Java: super.startDocument();
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startDocument();
         todo!()
      }
    }

    // Java: void ignorableWhitespace(char[] parm1, int parm2, int parm3)
    pub fn ignorableWhitespace(&self, _parm1: &[char], _parm2: i32, _parm3: i32) {
      // Java: super.ignorableWhitespace(parm1, parm2, parm3);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.ignorableWhitespace(parm1, parm2, parm3);
         todo!()
      }
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&self, _target: &str, _data: &str) {
      // Java: super.processingInstruction(target, data);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.processingInstruction(target, data);
         todo!()
      }
    }

    // Java: void setDocumentLocator(Locator locator)
    pub fn setDocumentLocator(&self, _locator: i32) {
      // Java: super.setDocumentLocator(locator);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.setDocumentLocator(locator);
         todo!()
      }
    }

    // Java: void skippedEntity(String name)
    pub fn skippedEntity(&self, _name: &str) {
      // Java: super.skippedEntity(name);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.skippedEntity(name);
         todo!()
      }
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: super.endDocument();
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.endDocument();
         todo!()
      }
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&self, _prefix: &str, _uri: &str) {
      // Java: super.startPrefixMapping(prefix, uri);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.startPrefixMapping(prefix, uri);
         todo!()
      }
    }

    // Java: void endElement(String uri, String localName, String qName)
    pub fn endElement(&self, _uri: &str, _localName: &str, _qName: &str) {
      // Java: ElementHandler currentHandler = this.getElementStack().getDispatchHandler().getHandler(this.getElementStack().getPath());
      // Java: super.endElement(uri, localName, qName);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: if (currentHandler == null)
         {
            // Java: this.xmlWriter.endElement(uri, localName, qName);
            todo!()
         }
         // Java: else if (currentHandler instanceof SAXModifyElementHandler)
         {
            // Java: SAXModifyElementHandler modifyHandler = (SAXModifyElementHandler)currentHandler;
            // Java: Element modifiedElement = modifyHandler.getModifiedElement();
            // Java: try
            {
               // Java: this.xmlWriter.write(modifiedElement);
               todo!()
            }
            // Java: catch (IOException ex)
            {
               // Java: throw new SAXModifyException(ex);
               todo!()
            }
         }
      }
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&self, _prefix: &str) {
      // Java: super.endPrefixMapping(prefix);
      // Java: if (this.xmlWriter != null)
      {
         // Java: this.xmlWriter.endPrefixMapping(prefix);
         todo!()
      }
    }

    // Java: void characters(char[] parm1, int parm2, int parm3)
    pub fn characters(&self, _parm1: &[char], _parm2: i32, _parm3: i32) {
      // Java: super.characters(parm1, parm2, parm3);
      // Java: if (!this.activeHandlers() && this.xmlWriter != null)
      {
         // Java: this.xmlWriter.characters(parm1, parm2, parm3);
         todo!()
      }
    }

    // Java: XMLWriter getXMLWriter()
    pub fn getXMLWriter(&self) -> i32 {
      // Java: return this.xmlWriter;
      todo!()
    }

    // Java: boolean activeHandlers()
    pub fn activeHandlers(&self) -> bool {
      // Java: DispatchHandler handler = this.getElementStack().getDispatchHandler();
      // Java: return handler.getActiveHandlerCount() > 0;
      todo!()
    }

}
