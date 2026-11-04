// Java: import cn.wps.base.assertion.Assert
// use util::cn::wps::core::runtime::Assert;
// Java: import java.lang.reflect.Method
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.dom4j.Branch
// use crate::org::dom4j::Branch;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentFactory
// use crate::org::dom4j::DocumentFactory;
// Java: import org.dom4j.DocumentType
// use crate::org::dom4j::DocumentType;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.ElementHandler
// use crate::org::dom4j::ElementHandler;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.dtd.AttributeDecl
// use crate::org::dom4j::dtd::AttributeDecl;
// Java: import org.dom4j.dtd.ElementDecl
// use crate::org::dom4j::dtd::ElementDecl;
// Java: import org.dom4j.dtd.ExternalEntityDecl
// use crate::org::dom4j::dtd::ExternalEntityDecl;
// Java: import org.dom4j.dtd.InternalEntityDecl
// use crate::org::dom4j::dtd::InternalEntityDecl;
// Java: import org.dom4j.tree.AbstractElement
// use crate::org::dom4j::tree::AbstractElement;
// Java: import org.dom4j.tree.NamespaceStack
// use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;
// Java: import org.xml.sax.DTDHandler
// use crate::org::xml::sax::DTDHandler;
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
// Java: import org.xml.sax.ext.DeclHandler
// use crate::org::xml::sax::ext::DeclHandler;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;
// Java: import org.xml.sax.helpers.DefaultHandler
// use crate::org::xml::sax::helpers::DefaultHandler;

// Java: public class SAXContentHandler extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler
pub struct SAXContentHandler {
    // Java field: DocumentFactory documentFactory
    documentFactory: i32,
    // Java field: Document document
    pub(crate) document: i32,
    // Java field: ElementStack elementStack
    pub(crate) elementStack: i32,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: i32,
    // Java field: ElementHandler elementHandler
    elementHandler: i32,
    // Java field: Locator locator
    locator: i32,
    // Java field: String entity
    entity: String,
    // Java field: boolean insideDTDSection
    insideDTDSection: bool,
    // Java field: boolean insideCDATASection
    insideCDATASection: bool,
    // Java field: StringBuffer cdataText
    cdataText: i32,
    // Java field: Map availableNamespaceMap
    availableNamespaceMap: i32,
    // Java field: List declaredNamespaceList
    declaredNamespaceList: i32,
    // Java field: List internalDTDDeclarations
    internalDTDDeclarations: i32,
    // Java field: List externalDTDDeclarations
    externalDTDDeclarations: i32,
    // Java field: int declaredNamespaceIndex
    declaredNamespaceIndex: i32,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: i32,
    // Java field: InputSource inputSource
    pub(crate) inputSource: i32,
    // Java field: Element currentElement
    currentElement: i32,
    // Java field: boolean includeInternalDTDDeclarations
    pub(crate) includeInternalDTDDeclarations: bool,
    // Java field: boolean includeExternalDTDDeclarations
    pub(crate) includeExternalDTDDeclarations: bool,
    // Java field: int entityLevel
    entityLevel: i32,
    // Java field: boolean internalDTDsubset
    internalDTDsubset: bool,
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
    // Java field: int mCurrentLevel
    mCurrentLevel: i32,
    // Java field: MCEContext mMCEContext
    mMCEContext: i32,
}

// Java: public class SAXContentHandler extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler
impl SAXContentHandler {
    // Java: SAXContentHandler()
    pub fn new() -> Self {
      // Java: this(DocumentFactory.getInstance());
      Self { documentFactory: 0, document: 0, elementStack: 0, namespaceStack: 0, elementHandler: 0, locator: 0, entity: String::new(), insideDTDSection: false, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, internalDTDDeclarations: 0, externalDTDDeclarations: 0, declaredNamespaceIndex: 0, entityResolver: 0, inputSource: 0, currentElement: 0, includeInternalDTDDeclarations: false, includeExternalDTDDeclarations: false, entityLevel: 0, internalDTDsubset: false, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false, mCurrentLevel: 0, mMCEContext: 0 }
    }

    // Java: SAXContentHandler(DocumentFactory documentFactory)
    pub fn new_with_factory(_documentFactory: i32) -> Self {
      // Java: this(documentFactory, (ElementHandler)null, false);
      Self { documentFactory: _documentFactory, document: 0, elementStack: 0, namespaceStack: 0, elementHandler: 0, locator: 0, entity: String::new(), insideDTDSection: false, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, internalDTDDeclarations: 0, externalDTDDeclarations: 0, declaredNamespaceIndex: 0, entityResolver: 0, inputSource: 0, currentElement: 0, includeInternalDTDDeclarations: false, includeExternalDTDDeclarations: false, entityLevel: 0, internalDTDsubset: false, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false, mCurrentLevel: 0, mMCEContext: 0 }
    }

    // Java: SAXContentHandler(DocumentFactory documentFactory, ElementHandler elementHandler, boolean onlyUseName)
    pub fn new_with_handler(_documentFactory: i32, _elementHandler: i32, _onlyUseName: bool) -> Self {
      // Java: this(documentFactory, elementHandler, (ElementStack)null);
      // Java: this.elementStack = this.createElementStack(onlyUseName);
      Self { documentFactory: _documentFactory, document: 0, elementStack: 0, namespaceStack: 0, elementHandler: _elementHandler, locator: 0, entity: String::new(), insideDTDSection: false, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, internalDTDDeclarations: 0, externalDTDDeclarations: 0, declaredNamespaceIndex: 0, entityResolver: 0, inputSource: 0, currentElement: 0, includeInternalDTDDeclarations: false, includeExternalDTDDeclarations: false, entityLevel: 0, internalDTDsubset: false, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false, mCurrentLevel: 0, mMCEContext: 0 }
    }

    // Java: SAXContentHandler(DocumentFactory documentFactory, ElementHandler elementHandler, ElementStack elementStack)
    pub fn new_full(_documentFactory: i32, _elementHandler: i32, _elementStack: i32) -> Self {
      // Java: super();
      // Java: this.availableNamespaceMap = new HashMap();
      // Java: this.declaredNamespaceList = new ArrayList();
      // Java: this.includeInternalDTDDeclarations = false;
      // Java: this.includeExternalDTDDeclarations = false;
      // Java: this.internalDTDsubset = false;
      // Java: this.mergeAdjacentText = false;
      // Java: this.textInTextBuffer = false;
      // Java: this.ignoreComments = false;
      // Java: this.stripWhitespaceText = false;
      // Java: this.mCurrentLevel = -1;
      // Java: this.documentFactory = documentFactory;
      // Java: this.elementHandler = elementHandler;
      // Java: this.elementStack = elementStack;
      // Java: this.namespaceStack = new NamespaceStack(documentFactory);
      Self { documentFactory: _documentFactory, document: 0, elementStack: _elementStack, namespaceStack: 0, elementHandler: _elementHandler, locator: 0, entity: String::new(), insideDTDSection: false, insideCDATASection: false, cdataText: 0, availableNamespaceMap: 0, declaredNamespaceList: 0, internalDTDDeclarations: 0, externalDTDDeclarations: 0, declaredNamespaceIndex: 0, entityResolver: 0, inputSource: 0, currentElement: 0, includeInternalDTDDeclarations: false, includeExternalDTDDeclarations: false, entityLevel: 0, internalDTDsubset: false, mergeAdjacentText: false, textInTextBuffer: false, ignoreComments: false, textBuffer: 0, stripWhitespaceText: false, mCurrentLevel: -1, mMCEContext: 0 }
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> i32 {
      // Java: if (this.document == null)
      // Java: this.document = this.createDocument();
      // Java: return this.document;
      0
    }

    // Java: void setDocumentLocator(Locator documentLocator)
    pub fn setDocumentLocator(&self, _documentLocator: i32) {
      // Java: this.locator = documentLocator;
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&self, _target: &str, _data: &str) {
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: if (this.currentElement != null)
      // Java: this.currentElement.addProcessingInstruction(target, data);
      // Java: else
      // Java: this.getDocument().addProcessingInstruction(target, data);
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
      // Java: if (this.elementHandler != null && this.elementHandler instanceof DispatchHandler)
      // Java: this.elementStack.setDispatchHandler((DispatchHandler)this.elementHandler);
      // Java: this.namespaceStack.clear();
      // Java: this.declaredNamespaceIndex = 0;
      // Java: if (this.mergeAdjacentText && this.textBuffer == null)
      // Java: this.textBuffer = new StringBuffer();
      // Java: this.textInTextBuffer = false;
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: this.namespaceStack.clear();
      // Java: this.elementStack.clear();
      // Java: this.currentElement = null;
      // Java: this.textBuffer = null;
    }

    // Java: void setMCEContext(MCEContext mceContext)
    pub fn setMCEContext(&self, _mceContext: i32) {
      // Java: this.mMCEContext = mceContext;
    }

    // Java: void startElement(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn startElement(&self, _namespaceURI: &str, _localName: &str, _qualifiedName: &str, _attributes: i32) {
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: ++this.mCurrentLevel;
      // Java: if (!this.mMCEContext.mMceDisabled)
      // Java: boolean skip = this.mMCEContext.shouldSkipStartElement(this.namespaceStack, namespaceURI, localName, attributes, this.mCurrentLevel);
      // Java: if (skip)
      // Java: this.declaredNamespaceIndex = this.namespaceStack.size();
      // Java: return;
      // Java: QName qName = this.namespaceStack.getQName(namespaceURI, localName, qualifiedName);
      // Java: Branch branch = this.currentElement;
      // Java: if (branch == null)
      // Java: branch = this.getDocument();
      // Java: Element element = branch.addElement(qName);
      // Java: this.addDeclaredNamespaces(element);
      // Java: this.addAttributes(element, attributes);
      // Java: this.elementStack.pushElement(element);
      // Java: this.currentElement = element;
      // Java: this.entity = null;
      // Java: if (this.elementHandler != null)
      // Java: this.elementHandler.onStart(this.elementStack);
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&self, _namespaceURI: &str, _localName: &str, _qName: &str) {
      // Java: if (!this.mMCEContext.mMceDisabled)
      // Java: boolean skip = this.mMCEContext.shouldSkipEndElement(this.namespaceStack, namespaceURI, localName, this.mCurrentLevel);
      // Java: if (skip)
      // Java: --this.mCurrentLevel;
      // Java: return;
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: Assert.assertNotNull("currentElement should not be null", this.currentElement);
      // Java: if (this.elementHandler != null)
      // Java: this.elementHandler.onEnd(this.elementStack);
      // Java: this.elementStack.popElement();
      // Java: this.currentElement = this.elementStack.peekElement();
      // Java: --this.mCurrentLevel;
    }

    // Java: void characters(char[] ch, int start, int end)
    pub fn characters(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: if (end != 0)
      // Java: if (this.currentElement != null)
      // Java: if (!this.mMCEContext.mMceDisabled)
      // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
      // Java: if (skip)
      // Java: return;
      // Java: if (this.entity != null)
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: this.currentElement.addEntity(this.entity, new String(ch, start, end));
      // Java: this.entity = null;
      // Java: else if (this.insideCDATASection)
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: this.cdataText.append(new String(ch, start, end));
      // Java: else if (this.mergeAdjacentText)
      // Java: this.textBuffer.append(ch, start, end);
      // Java: this.textInTextBuffer = true;
      // Java: else
      // Java: this.currentElement.addText(new String(ch, start, end));
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
      // Java: this.getDocument().addDocType(name, publicId, systemId);
      // Java: this.insideDTDSection = true;
      // Java: this.internalDTDsubset = true;
    }

    // Java: void endDTD()
    pub fn endDTD(&self) {
      // Java: this.insideDTDSection = false;
      // Java: DocumentType docType = this.getDocument().getDocType();
      // Java: if (docType != null)
      // Java: if (this.internalDTDDeclarations != null)
      // Java: docType.setInternalDeclarations(this.internalDTDDeclarations);
      // Java: if (this.externalDTDDeclarations != null)
      // Java: docType.setExternalDeclarations(this.externalDTDDeclarations);
      // Java: this.internalDTDDeclarations = null;
      // Java: this.externalDTDDeclarations = null;
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&self, _name: &str) {
      // Java: if (!this.mMCEContext.mMceDisabled)
      // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
      // Java: if (skip)
      // Java: return;
      // Java: ++this.entityLevel;
      // Java: this.entity = null;
      // Java: if (!this.insideDTDSection && !this.isIgnorableEntity(name))
      // Java: this.entity = name;
      // Java: this.internalDTDsubset = false;
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&self, _name: &str) {
      // Java: if (!this.mMCEContext.mMceDisabled)
      // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
      // Java: if (skip)
      // Java: return;
      // Java: --this.entityLevel;
      // Java: this.entity = null;
      // Java: if (this.entityLevel == 0)
      // Java: this.internalDTDsubset = true;
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
      // Java: this.insideCDATASection = true;
      // Java: this.cdataText = new StringBuffer();
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
      // Java: this.insideCDATASection = false;
      // Java: this.currentElement.addCDATA(this.cdataText.toString());
    }

    // Java: void comment(char[] ch, int start, int end)
    pub fn comment(&self, _ch: &[char], _start: i32, _end: i32) {
      // Java: if (!this.ignoreComments)
      // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
      // Java: this.completeCurrentTextNode();
      // Java: String text = new String(ch, start, end);
      // Java: if (!this.insideDTDSection && text.length() > 0)
      // Java: if (this.currentElement != null)
      // Java: this.currentElement.addComment(text);
      // Java: else
      // Java: this.getDocument().addComment(text);
    }

    // Java: void elementDecl(String name, String model)
    pub fn elementDecl(&self, _name: &str, _model: &str) {
      // Java: if (this.internalDTDsubset)
      // Java: if (this.includeInternalDTDDeclarations)
      // Java: this.addDTDDeclaration(new ElementDecl(name, model));
      // Java: else if (this.includeExternalDTDDeclarations)
      // Java: this.addExternalDTDDeclaration(new ElementDecl(name, model));
    }

    // Java: void attributeDecl(String eName, String aName, String type, String valueDefault, String val)
    pub fn attributeDecl(&self, _eName: &str, _aName: &str, _type: &str, _valueDefault: &str, _val: &str) {
      // Java: if (this.internalDTDsubset)
      // Java: if (this.includeInternalDTDDeclarations)
      // Java: this.addDTDDeclaration(new AttributeDecl(eName, aName, type, valueDefault, val));
      // Java: else if (this.includeExternalDTDDeclarations)
      // Java: this.addExternalDTDDeclaration(new AttributeDecl(eName, aName, type, valueDefault, val));
    }

    // Java: void internalEntityDecl(String name, String value)
    pub fn internalEntityDecl(&self, _name: &str, _value: &str) {
      // Java: if (this.internalDTDsubset)
      // Java: if (this.includeInternalDTDDeclarations)
      // Java: this.addDTDDeclaration(new InternalEntityDecl(name, value));
      // Java: else if (this.includeExternalDTDDeclarations)
      // Java: this.addExternalDTDDeclaration(new InternalEntityDecl(name, value));
    }

    // Java: void externalEntityDecl(String name, String publicId, String sysId)
    pub fn externalEntityDecl(&self, _name: &str, _publicId: &str, _sysId: &str) {
      // Java: ExternalEntityDecl declaration = new ExternalEntityDecl(name, publicId, sysId);
      // Java: if (this.internalDTDsubset)
      // Java: if (this.includeInternalDTDDeclarations)
      // Java: this.addDTDDeclaration(declaration);
      // Java: else if (this.includeExternalDTDDeclarations)
      // Java: this.addExternalDTDDeclaration(declaration);
    }

    // Java: void notationDecl(String name, String publicId, String systemId)
    pub fn notationDecl(&self, _name: &str, _publicId: &str, _systemId: &str) {
    }

    // Java: void unparsedEntityDecl(String name, String publicId, String systemId, String notationName)
    pub fn unparsedEntityDecl(&self, _name: &str, _publicId: &str, _systemId: &str, _notationName: &str) {
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

    // Java: boolean isIncludeInternalDTDDeclarations()
    pub fn isIncludeInternalDTDDeclarations(&self) -> bool {
      // Java: return this.includeInternalDTDDeclarations;
      false
    }

    // Java: void setIncludeInternalDTDDeclarations(boolean include)
    pub fn setIncludeInternalDTDDeclarations(&self, _include: bool) {
      // Java: this.includeInternalDTDDeclarations = include;
    }

    // Java: boolean isIncludeExternalDTDDeclarations()
    pub fn isIncludeExternalDTDDeclarations(&self) -> bool {
      // Java: return this.includeExternalDTDDeclarations;
      false
    }

    // Java: void setIncludeExternalDTDDeclarations(boolean include)
    pub fn setIncludeExternalDTDDeclarations(&self, _include: bool) {
      // Java: this.includeExternalDTDDeclarations = include;
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
      // Java: String trimed = this.textBuffer.toString().trim();
      // Java: if (trimed.length() > 0)
      // Java: this.currentElement.addText(trimed);
      // Java: if (this.elementHandler != null)
      // Java: this.elementHandler.onCharacters(trimed);
      // Java: else
      // Java: String untrimed = this.textBuffer.toString();
      // Java: this.currentElement.addText(untrimed);
      // Java: if (this.elementHandler != null)
      // Java: this.elementHandler.onCharacters(untrimed);
      // Java: this.textBuffer.setLength(0);
      // Java: this.textInTextBuffer = false;
    }

    // Java: Document createDocument()
    pub fn createDocument(&self) -> i32 {
      // Java: String encoding = this.getEncoding();
      // Java: Document doc = this.documentFactory.createDocument(encoding);
      // Java: doc.setEntityResolver(this.entityResolver);
      // Java: if (this.inputSource != null)
      // Java: doc.setName(this.inputSource.getSystemId());
      // Java: return doc;
      0
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> i32 {
      // Java: if (this.locator == null)
      // Java: return null;
      // Java: else
      // Java: try
      // Java: Method m = this.locator.getClass().getMethod("getEncoding");
      // Java: if (m != null)
      // Java: return (String)m.invoke(this.locator);
      // Java: catch (Exception var2)
      // Java: return null;
      0
    }

    // Java: boolean isIgnorableEntity(String name)
    pub fn isIgnorableEntity(&self, _name: &str) -> bool {
      // Java: return "amp".equals(name) || "apos".equals(name) || "gt".equals(name) || "lt".equals(name) || "quot".equals(name);
      false
    }

    // Java: void addDeclaredNamespaces(Element element)
    pub fn addDeclaredNamespaces(&self, _element: i32) {
      // Java: for(int size = this.namespaceStack.size(); this.declaredNamespaceIndex < size; ++this.declaredNamespaceIndex)
      // Java: Namespace namespace = this.namespaceStack.getNamespace(this.declaredNamespaceIndex);
      // Java: element.add(namespace);
    }

    // Java: void addAttributes(Element element, Attributes attributes)
    pub fn addAttributes(&self, _element: i32, _attributes: i32) {
      // Java: boolean noNamespaceAttributes = false;
      // Java: Assert.assertTrue("element should be instanceof AbstractElement.", element instanceof AbstractElement);
      // Java: int size = attributes.getLength();
      // Java: if (size > 0)
      // Java: element.setAttributes(attributes, size, this.namespaceStack, noNamespaceAttributes, this.mMCEContext);
    }

    // Java: void addDTDDeclaration(Object declaration)
    pub fn addDTDDeclaration(&self, _declaration: i32) {
      // Java: if (this.internalDTDDeclarations == null)
      // Java: this.internalDTDDeclarations = new ArrayList();
      // Java: this.internalDTDDeclarations.add(declaration);
    }

    // Java: void addExternalDTDDeclaration(Object declaration)
    pub fn addExternalDTDDeclaration(&self, _declaration: i32) {
      // Java: if (this.externalDTDDeclarations == null)
      // Java: this.externalDTDDeclarations = new ArrayList();
      // Java: this.externalDTDDeclarations.add(declaration);
    }

    // Java: ElementStack createElementStack(boolean onlyUseName)
    pub fn createElementStack(&self, _onlyUseName: bool) -> i32 {
      // Java: return new ElementStack(onlyUseName);
      0
    }

}

// impl LexicalHandler for SAXContentHandler {

// impl DeclHandler for SAXContentHandler {

// impl DTDHandler for SAXContentHandler {
