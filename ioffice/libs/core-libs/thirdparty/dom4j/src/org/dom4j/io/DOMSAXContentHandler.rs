// Java: import java.lang.reflect.Method
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.dom4j.Branch
// use crate::org::dom4j::Branch;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.dom.DOMAttribute
// use crate::org::dom4j::dom::DOMAttribute;
// Java: import org.dom4j.dom.DOMCDATA
// use crate::org::dom4j::dom::DOMCDATA;
// Java: import org.dom4j.dom.DOMComment
// use crate::org::dom4j::dom::DOMComment;
// Java: import org.dom4j.dom.DOMDocumentFactory
// use crate::org::dom4j::dom::DOMDocumentFactory;
// Java: import org.dom4j.dom.DOMElement
// use crate::org::dom4j::dom::DOMElement;
// Java: import org.dom4j.dom.DOMText
// use crate::org::dom4j::dom::DOMText;
// Java: import org.dom4j.tree.NamespaceStack
// use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.w3c.dom.ProcessingInstruction
// use io_wps_io::cn::wps::io::dom::ProcessingInstruction;
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;
// Java: import org.xml.sax.EntityResolver
// use crate::org::xml::sax::EntityResolver;
// Java: import org.xml.sax.InputSource
// use crate::org::xml::sax::InputSource;
// Java: import org.xml.sax.Locator
// use crate::org::xml::sax::Locator;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.SAXParseException
// use crate::org::xml::sax::SAXParseException;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;
// Java: import org.xml.sax.helpers.DefaultHandler
// use crate::org::xml::sax::helpers::DefaultHandler;

// Java: public class DOMSAXContentHandler extends DefaultHandler implements LexicalHandler
pub struct DOMSAXContentHandler {
    // Java field: DOMDocumentFactory documentFactory
    documentFactory: i32,
    // Java field: Document document
    document: i32,
    // Java field: ElementStack elementStack
    pub(crate) elementStack: i32,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: i32,
    // Java field: Locator locator
    locator: i32,
    // Java field: boolean insideCDATASection
    insideCDATASection: bool,
    // Java field: StringBuffer cdataText
    cdataText: i32,
    // Java field: Map availableNamespaceMap
    availableNamespaceMap: i32,
    // Java field: List declaredNamespaceList
    declaredNamespaceList: i32,
    // Java field: int declaredNamespaceIndex
    declaredNamespaceIndex: i32,
    // Java field: InputSource inputSource
    pub(crate) inputSource: i32,
    // Java field: Element currentElement
    currentElement: i32,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: i32,
    // Java field: boolean mergeAdjacentText
    pub(crate) mergeAdjacentText: bool,
    // Java field: boolean textInTextBuffer
    textInTextBuffer: bool,
    // Java field: boolean ignoreComments
    pub(crate) ignoreComments: bool,
    // Java field: StringBuffer textBuffer
    textBuffer: i32,
    // Java field: boolean stripWhitespaceText
    pub(crate) stripWhitespaceText: bool,
}

// Java: public class DOMSAXContentHandler extends DefaultHandler implements LexicalHandler
impl DOMSAXContentHandler {
    // Java: DOMSAXContentHandler()
    pub fn new() -> Self {
      // Java: this((DOMDocumentFactory)DOMDocumentFactory.getInstance());
      Self { documentFactory: 0, document: 0, elementStack: 0, namespaceStack: 0, locator: 0, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, declaredNamespaceIndex: 0, inputSource: 0, currentElement: 0, entityResolver: 0, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false }
    }

    // Java: DOMSAXContentHandler(DOMDocumentFactory documentFactory)
    pub fn new_with_factory(_documentFactory: i32) -> Self {
      // Java: super();
      // Java: this.availableNamespaceMap = new HashMap();
      // Java: this.declaredNamespaceList = new ArrayList();
      // Java: this.mergeAdjacentText = false;
      // Java: this.textInTextBuffer = false;
      // Java: this.ignoreComments = false;
      // Java: this.stripWhitespaceText = false;
      // Java: this.documentFactory = documentFactory;
      // Java: this.elementStack = this.createElementStack();
      // Java: this.namespaceStack = new NamespaceStack(documentFactory);
      Self { documentFactory: _documentFactory, document: 0, elementStack: 0, namespaceStack: 0, locator: 0, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, declaredNamespaceIndex: 0, inputSource: 0, currentElement: 0, entityResolver: 0, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false }
    }

    // Java: void setDocumentLocator(Locator documentLocator)
    pub fn setDocumentLocator(&self, _documentLocator: i32) {
      // Java: this.locator = documentLocator;
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&self, _target: &str, _data: &str) {
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      {
         // Java: this.completeCurrentTextNode();
      }
      // Java: ProcessingInstruction pi = (ProcessingInstruction)this.documentFactory.createProcessingInstruction(target, data);
      // Java: if (this.currentElement != null)
      {
         // Java: ((org.w3c.dom.Element)this.currentElement).appendChild(pi);
      }
      // Java: else
      {
         // Java: this.getDocument().appendChild(pi);
      }
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&self, _prefix: &str, _uri: &str) {
      // Java: this.namespaceStack.push(prefix, uri);
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&self, _prefix: &str) {
      // Java: this.namespaceStack.pop(prefix);
      // Java: this.declaredNamespaceIndex = this.namespaceStack.size();
    }

    // Java: void startDocument()
    pub fn startDocument(&self) {
      // Java: this.document = null;
      // Java: this.currentElement = null;
      // Java: this.elementStack.clear();
      // Java: this.namespaceStack.clear();
      // Java: this.declaredNamespaceIndex = 0;
      // Java: if (this.mergeAdjacentText && this.textBuffer == null)
      {
         // Java: this.textBuffer = new StringBuffer();
      }
      // Java: this.textInTextBuffer = false;
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: this.namespaceStack.clear();
      // Java: this.elementStack.clear();
      // Java: this.currentElement = null;
      // Java: this.textBuffer = null;
    }

    // Java: void startElement(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn startElement(&self, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: i32) {
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      {
         // Java: this.completeCurrentTextNode();
      }
      // Java: QName qName = this.namespaceStack.getQName(namespaceURI, localName, qualifiedName);
      // Java: Branch branch = this.currentElement;
      // Java: if (branch == null)
      {
         // Java: branch = (Document)this.getDocument();
      }
      // Java: Element element = new DOMElement(qName);
      // Java: branch.add(element);
      // Java: this.addDeclaredNamespaces(element);
      // Java: this.addAttributes(element, attributes);
      // Java: this.elementStack.pushElement(element);
      // Java: this.currentElement = element;
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&self, _namespaceURI: &str, _localName: &str, _qName: &str) {
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      {
         // Java: this.completeCurrentTextNode();
      }
      // Java: this.elementStack.popElement();
      // Java: this.currentElement = this.elementStack.peekElement();
    }

    // Java: void characters(char[] ch, int start, int end)
    pub fn characters(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: if (end != 0)
      {
         // Java: if (this.currentElement != null)
         {
            // Java: if (this.insideCDATASection)
            {
               // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
               {
                  // Java: this.completeCurrentTextNode();
               }
               // Java: this.cdataText.append(new String(ch, start, end));
            }
            // Java: else if (this.mergeAdjacentText)
            {
               // Java: this.textBuffer.append(ch, start, end);
               // Java: this.textInTextBuffer = true;
            }
            // Java: else
            {
               // Java: DOMText text = new DOMText(new String(ch, start, end));
               // Java: ((DOMElement)this.currentElement).add(text);
            }
         }
      }
    }

    // Java: void warning(SAXParseException exception)
    pub fn warning(&self, _exception: i32) {
    }

    // Java: void error(SAXParseException exception)
    pub fn error(&self, _exception: i32) {
      // Java: throw exception;
    }

    // Java: void fatalError(SAXParseException exception)
    pub fn fatalError(&self, _exception: i32) {
      // Java: throw exception;
    }

    // Java: void startDTD(String name, String publicId, String systemId)
    pub fn startDTD(&self, _name: &str, _publicId: &str, _systemId: &str) {
    }

    // Java: void endDTD()
    pub fn endDTD(&self) {
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&self, _name: &str) {
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&self, _name: &str) {
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
      // Java: this.insideCDATASection = true;
      // Java: this.cdataText = new StringBuffer();
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
      // Java: this.insideCDATASection = false;
      // Java: DOMCDATA cdata = new DOMCDATA(this.cdataText.toString());
      // Java: ((DOMElement)this.currentElement).add(cdata);
    }

    // Java: void comment(char[] ch, int start, int end)
    pub fn comment(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: if (!this.ignoreComments)
      {
         // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
         {
            // Java: this.completeCurrentTextNode();
         }
         // Java: String text = new String(ch, start, end);
         // Java: if (text.length() > 0)
         {
            // Java: DOMComment domComment = new DOMComment(text);
            // Java: if (this.currentElement != null)
            {
               // Java: ((DOMElement)this.currentElement).add(domComment);
            }
            // Java: else
            {
               // Java: this.getDocument().appendChild(domComment);
            }
         }
      }
    }

    // Java: ElementStack getElementStack()
    pub fn getElementStack(&self) -> i32 {
      // Java: return this.elementStack;
      0
    }

    // Java: void setElementStack(ElementStack elementStack)
    pub fn setElementStack(&self, _elementStack: i32) {
      // Java: this.elementStack = elementStack;
    }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) -> i32 {
      // Java: return this.entityResolver;
      0
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&self, _entityResolver: i32) {
      // Java: this.entityResolver = entityResolver;
    }

    // Java: InputSource getInputSource()
    pub fn getInputSource(&self) -> i32 {
      // Java: return this.inputSource;
      0
    }

    // Java: void setInputSource(InputSource inputSource)
    pub fn setInputSource(&self, _inputSource: i32) {
      // Java: this.inputSource = inputSource;
    }

    // Java: boolean isMergeAdjacentText()
    pub fn isMergeAdjacentText(&self) -> bool {
      // Java: return this.mergeAdjacentText;
      false
    }

    // Java: void setMergeAdjacentText(boolean mergeAdjacentText)
    pub fn setMergeAdjacentText(&self, _mergeAdjacentText: bool) {
      // Java: this.mergeAdjacentText = mergeAdjacentText;
    }

    // Java: boolean isStripWhitespaceText()
    pub fn isStripWhitespaceText(&self) -> bool {
      // Java: return this.stripWhitespaceText;
      false
    }

    // Java: void setStripWhitespaceText(boolean stripWhitespaceText)
    pub fn setStripWhitespaceText(&self, _stripWhitespaceText: bool) {
      // Java: this.stripWhitespaceText = stripWhitespaceText;
    }

    // Java: boolean isIgnoreComments()
    pub fn isIgnoreComments(&self) -> bool {
      // Java: return this.ignoreComments;
      false
    }

    // Java: void setIgnoreComments(boolean ignoreComments)
    pub fn setIgnoreComments(&self, _ignoreComments: bool) {
      // Java: this.ignoreComments = ignoreComments;
    }

    // Java: void completeCurrentTextNode()
    pub fn completeCurrentTextNode(&self) {
      // Java: if (this.stripWhitespaceText)
      {
         // Java: boolean whitespace = true;
         // Java: int i = 0;
         // Java: for(int size = this.textBuffer.length(); i < size; ++i)
         {
            // Java: if (!Character.isWhitespace(this.textBuffer.charAt(i)))
            {
               // Java: whitespace = false;
               // Java: break;
            }
         }
         // Java: if (!whitespace)
         {
            // Java: DOMText domText = new DOMText(this.textBuffer.toString());
            // Java: ((DOMElement)this.currentElement).add(domText);
         }
      }
      // Java: else
      {
         // Java: DOMText domText = new DOMText(this.textBuffer.toString());
         // Java: ((DOMElement)this.currentElement).add(domText);
      }
      // Java: this.textBuffer.setLength(0);
      // Java: this.textInTextBuffer = false;
    }

    // Java: Document createDocument()
    pub fn createDocument(&self) -> i32 {
      // Java: String encoding = this.getEncoding();
      // Java: Document doc = this.documentFactory.createDocument(encoding);
      // Java: doc.setEntityResolver(this.entityResolver);
      // Java: if (this.inputSource != null)
      {
         // Java: doc.setName(this.inputSource.getSystemId());
      }
      // Java: return doc;
      0
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> String {
      // Java: if (this.locator == null)
      {
         // Java: return null;
      }
      // Java: else
      {
         // Java: try
         {
            // Java: Method m = this.locator.getClass().getMethod("getEncoding");
            // Java: if (m != null)
            {
               // Java: return (String)m.invoke(this.locator);
            }
         }
         // Java: catch (Exception var2)
         {
         }
         // Java: return null;
      }
      String::new()
    }

    // Java: void addDeclaredNamespaces(Element element)
    pub fn addDeclaredNamespaces(&self, _element: i32) {
      // Java: for(int size = this.namespaceStack.size(); this.declaredNamespaceIndex < size; ++this.declaredNamespaceIndex)
      {
         // Java: Namespace namespace = this.namespaceStack.getNamespace(this.declaredNamespaceIndex);
         // Java: String attributeName = this.attributeNameForNamespace(namespace);
         // Java: ((DOMElement)element).setAttribute(attributeName, namespace.getURI());
      }
    }

    // Java: void addAttributes(Element element, Attributes attributes)
    pub fn addAttributes(&self, _element: i32, _attributes: i32) {
      // Java: int size = attributes.getLength();
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: String attributeQName = attributes.getQName(i);
         // Java: if (!attributeQName.startsWith("xmlns"))
         {
            // Java: String attributeURI = attributes.getURI(i);
            // Java: String attributeLocalName = attributes.getLocalName(i);
            // Java: String attributeValue = attributes.getValue(i);
            // Java: QName qName = this.namespaceStack.getAttributeQName(attributeURI, attributeLocalName, attributeQName);
            // Java: DOMAttribute domAttribute = new DOMAttribute(qName, attributeValue);
            // Java: ((DOMElement)element).setAttributeNode(domAttribute);
         }
      }
    }

    // Java: ElementStack createElementStack()
    pub fn createElementStack(&self) -> i32 {
      // Java: return new ElementStack(false);
      0
    }

    // Java: String attributeNameForNamespace(Namespace namespace)
    pub fn attributeNameForNamespace(&self, _namespace: i32) -> String {
      // Java: String xmlns = "xmlns";
      // Java: String prefix = namespace.getPrefix();
      // Java: return prefix.length() > 0 ? xmlns + ":" + prefix : xmlns;
      String::new()
    }

}

// impl LexicalHandler for DOMSAXContentHandler {
// }
