// Java: import java.io.IOException
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.dom4j.Attribute
// use crate::org::dom4j::Attribute;
// Java: import org.dom4j.Branch
// use crate::org::dom4j::Branch;
// Java: import org.dom4j.CDATA
// use crate::org::dom4j::CDATA;
// Java: import org.dom4j.CharacterData
// use crate::org::dom4j::CharacterData;
// Java: import org.dom4j.Comment
// use crate::org::dom4j::Comment;
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentType
// use crate::org::dom4j::DocumentType;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Entity
// use crate::org::dom4j::Entity;
// Java: import org.dom4j.Namespace
// use crate::org::dom4j::Namespace;
// Java: import org.dom4j.Node
// use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeType
// use crate::org::dom4j::NodeType;
// Java: import org.dom4j.ProcessingInstruction
// use crate::org::dom4j::ProcessingInstruction;
// Java: import org.dom4j.Text
// use crate::org::dom4j::Text;
// Java: import org.dom4j.tree.NamespaceStack
// use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;
// Java: import org.xml.sax.ContentHandler
// use io_wps_io::io::xwpf::element_handler::ContentHandler;
// Java: import org.xml.sax.DTDHandler
// use crate::org::xml::sax::DTDHandler;
// Java: import org.xml.sax.EntityResolver
// use crate::org::xml::sax::EntityResolver;
// Java: import org.xml.sax.ErrorHandler
// use crate::org::xml::sax::ErrorHandler;
// Java: import org.xml.sax.InputSource
// use crate::org::xml::sax::InputSource;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;
// Java: import org.xml.sax.SAXNotRecognizedException
// use crate::org::xml::sax::SAXNotRecognizedException;
// Java: import org.xml.sax.SAXNotSupportedException
// use crate::org::xml::sax::SAXNotSupportedException;
// Java: import org.xml.sax.XMLReader
// use crate::org::xml::sax::XMLReader;
// Java: import org.xml.sax.ext.LexicalHandler
// use crate::org::xml::sax::ext::LexicalHandler;
// Java: import org.xml.sax.helpers.AttributesImpl
// use io_wps_io::cn::wps::io::opt::AttributesImpl;
// Java: import org.xml.sax.helpers.LocatorImpl
// use crate::org::xml::sax::helpers::LocatorImpl;

// Java field: String[] LEXICAL_HANDLER_NAMES
static LEXICAL_HANDLER_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
// Java field: String FEATURE_NAMESPACE_PREFIXES
pub(crate) const FEATURE_NAMESPACE_PREFIXES: &str = "";
// Java field: String FEATURE_NAMESPACES
pub(crate) const FEATURE_NAMESPACES: &str = "";

// Java: public class SAXWriter implements XMLReader
pub struct SAXWriter {
    // Java field: ContentHandler contentHandler
    pub(crate) contentHandler: i32,
    // Java field: DTDHandler dtdHandler
    pub(crate) dtdHandler: i32,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: i32,
    // Java field: ErrorHandler errorHandler
    pub(crate) errorHandler: i32,
    // Java field: LexicalHandler lexicalHandler
    pub(crate) lexicalHandler: i32,
    // Java field: AttributesImpl attributes
    attributes: i32,
    // Java field: Map<String, Boolean> features
    features: i32,
    // Java field: Map<String, Object> properties
    pub(crate) properties: i32,
    // Java field: boolean declareNamespaceAttributes
    pub(crate) declareNamespaceAttributes: bool,
}

// Java: public class SAXWriter implements XMLReader
impl SAXWriter {
    // Java: SAXWriter()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.attributes = new AttributesImpl();
      // Java: this.features = new HashMap();
      // Java: this.properties = new HashMap();
      // Java: this.properties.put("http://xml.org/sax/features/namespace-prefixes", false);
      // Java: this.properties.put("http://xml.org/sax/features/namespace-prefixes", true);
      Self { contentHandler: 0, dtdHandler: 0, entityResolver: 0, errorHandler: 0, lexicalHandler: 0, attributes: 0, features: 0, properties: 0, declareNamespaceAttributes: false }
    }

    // Java: SAXWriter(ContentHandler contentHandler)
    pub fn new_with_handler(_contentHandler: i32) -> Self {
      // Java: this();
      // Java: this.contentHandler = contentHandler;
      Self { contentHandler: _contentHandler, dtdHandler: 0, entityResolver: 0, errorHandler: 0, lexicalHandler: 0, attributes: 0, features: 0, properties: 0, declareNamespaceAttributes: false }
    }

    // Java: SAXWriter(ContentHandler contentHandler, LexicalHandler lexicalHandler)
    pub fn new_with_handler_and_lexical(_contentHandler: i32, _lexicalHandler: i32) -> Self {
      // Java: this();
      // Java: this.contentHandler = contentHandler;
      // Java: this.lexicalHandler = lexicalHandler;
      Self { contentHandler: _contentHandler, dtdHandler: 0, entityResolver: 0, errorHandler: 0, lexicalHandler: _lexicalHandler, attributes: 0, features: 0, properties: 0, declareNamespaceAttributes: false }
    }

    // Java: SAXWriter(ContentHandler contentHandler, LexicalHandler lexicalHandler, EntityResolver entityResolver)
    pub fn new_with_handler_lexical_resolver(_contentHandler: i32, _lexicalHandler: i32, _entityResolver: i32) -> Self {
      // Java: this();
      // Java: this.contentHandler = contentHandler;
      // Java: this.lexicalHandler = lexicalHandler;
      // Java: this.entityResolver = entityResolver;
      Self { contentHandler: _contentHandler, dtdHandler: 0, entityResolver: _entityResolver, errorHandler: 0, lexicalHandler: _lexicalHandler, attributes: 0, features: 0, properties: 0, declareNamespaceAttributes: false }
    }

    // Java: void write(Node node)
    pub fn write(&self, _node: i32) {
      // Java: NodeType nodeType = node.getNodeTypeEnum();
      // Java: switch (nodeType)
      {
         // Java: case ELEMENT_NODE:
            // Java: this.write((Element)node);
            // Java: break;
         // Java: case ATTRIBUTE_NODE:
            // Java: this.write((Node)((Attribute)node));
            // Java: break;
         // Java: case TEXT_NODE:
            // Java: this.write(node.getText());
            // Java: break;
         // Java: case CDATA_SECTION_NODE:
            // Java: this.write((CDATA)node);
            // Java: break;
         // Java: case ENTITY_REFERENCE_NODE:
            // Java: this.write((Entity)node);
            // Java: break;
         // Java: case PROCESSING_INSTRUCTION_NODE:
            // Java: this.write((ProcessingInstruction)node);
            // Java: break;
         // Java: case COMMENT_NODE:
            // Java: this.write((Comment)node);
            // Java: break;
         // Java: case DOCUMENT_NODE:
            // Java: this.write((Document)node);
            // Java: break;
         // Java: case DOCUMENT_TYPE_NODE:
            // Java: this.write((Node)((DocumentType)node));
         // Java: case NAMESPACE_NODE:
            // Java: break;
         // Java: default:
            // Java: throw new SAXException("Invalid node type: " + node);
      }
    }

    // Java: void write(Document document)
    pub fn write_document(&self, _document: i32) {
      // Java: if (document != null)
      {
         // Java: this.checkForNullHandlers();
         // Java: this.documentLocator(document);
         // Java: this.startDocument();
         // Java: this.entityResolver(document);
         // Java: this.dtdHandler(document);
         // Java: this.writeContent(document, new NamespaceStack());
         // Java: this.endDocument();
      }
    }

    // Java: void write(Element element)
    pub fn write_element(&self, _element: i32) {
      // Java: this.write(element, new NamespaceStack());
    }

    // Java: void writeOpen(Element element)
    pub fn writeOpen(&self, _element: i32) {
      // Java: this.startElement(element, (AttributesImpl)null);
    }

    // Java: void writeClose(Element element)
    pub fn writeClose(&self, _element: i32) {
      // Java: this.endElement(element);
    }

    // Java: void write(String text)
    pub fn write_text(&self, _text: &str) {
      // Java: if (text != null)
      {
         // Java: char[] chars = text.toCharArray();
         // Java: this.contentHandler.characters(chars, 0, chars.length);
      }
    }

    // Java: void write(CDATA cdata)
    pub fn write_cdata(&self, _cdata: i32) {
      // Java: String text = cdata.getText();
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.startCDATA();
         // Java: this.write(text);
         // Java: this.lexicalHandler.endCDATA();
      }
      // Java: else
      {
         // Java: this.write(text);
      }
    }

    // Java: void write(Comment comment)
    pub fn write_comment(&self, _comment: i32) {
      // Java: if (this.lexicalHandler != null)
      {
         // Java: String text = comment.getText();
         // Java: char[] chars = text.toCharArray();
         // Java: this.lexicalHandler.comment(chars, 0, chars.length);
      }
    }

    // Java: void write(Entity entity)
    pub fn write_entity(&self, _entity: i32) {
      // Java: String text = entity.getText();
      // Java: if (this.lexicalHandler != null)
      {
         // Java: String name = entity.getName();
         // Java: this.lexicalHandler.startEntity(name);
         // Java: this.write(text);
         // Java: this.lexicalHandler.endEntity(name);
      }
      // Java: else
      {
         // Java: this.write(text);
      }
    }

    // Java: void write(ProcessingInstruction pi)
    pub fn write_pi(&self, _pi: i32) {
      // Java: String target = pi.getTarget();
      // Java: String text = pi.getText();
      // Java: this.contentHandler.processingInstruction(target, text);
    }

    // Java: boolean isDeclareNamespaceAttributes()
    pub fn isDeclareNamespaceAttributes(&self) -> bool {
      // Java: return this.declareNamespaceAttributes;
      false
    }

    // Java: void setDeclareNamespaceAttributes(boolean declareNamespaceAttrs)
    pub fn setDeclareNamespaceAttributes(&self, _declareNamespaceAttrs: bool) {
      // Java: this.declareNamespaceAttributes = declareNamespaceAttrs;
    }

    // Java: ContentHandler getContentHandler()
    pub fn getContentHandler(&self) -> i32 {
      // Java: return this.contentHandler;
      0
    }

    // Java: void setContentHandler(ContentHandler contentHandler)
    pub fn setContentHandler(&self, _contentHandler: i32) {
      // Java: this.contentHandler = contentHandler;
    }

    // Java: DTDHandler getDTDHandler()
    pub fn getDTDHandler(&self) -> i32 {
      // Java: return this.dtdHandler;
      0
    }

    // Java: void setDTDHandler(DTDHandler handler)
    pub fn setDTDHandler(&self, _handler: i32) {
      // Java: this.dtdHandler = handler;
    }

    // Java: ErrorHandler getErrorHandler()
    pub fn getErrorHandler(&self) -> i32 {
      // Java: return this.errorHandler;
      0
    }

    // Java: void setErrorHandler(ErrorHandler errorHandler)
    pub fn setErrorHandler(&self, _errorHandler: i32) {
      // Java: this.errorHandler = errorHandler;
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

    // Java: LexicalHandler getLexicalHandler()
    pub fn getLexicalHandler(&self) -> i32 {
      // Java: return this.lexicalHandler;
      0
    }

    // Java: void setLexicalHandler(LexicalHandler lexicalHandler)
    pub fn setLexicalHandler(&self, _lexicalHandler: i32) {
      // Java: this.lexicalHandler = lexicalHandler;
    }

    // Java: void setXMLReader(XMLReader xmlReader)
    pub fn setXMLReader(&self, _xmlReader: i32) {
      // Java: this.setContentHandler(xmlReader.getContentHandler());
      // Java: this.setDTDHandler(xmlReader.getDTDHandler());
      // Java: this.setEntityResolver(xmlReader.getEntityResolver());
      // Java: this.setErrorHandler(xmlReader.getErrorHandler());
    }

    // Java: boolean getFeature(String name)
    pub fn getFeature(&self, _name: &str) -> bool {
      // Java: Boolean answer = (Boolean)this.features.get(name);
      // Java: return answer != null && answer;
      false
    }

    // Java: void setFeature(String name, boolean value)
    pub fn setFeature(&self, _name: &str, _value: bool) {
      // Java: if ("http://xml.org/sax/features/namespace-prefixes".equals(name))
      {
         // Java: this.setDeclareNamespaceAttributes(value);
      }
      // Java: else if ("http://xml.org/sax/features/namespace-prefixes".equals(name) && !value)
      {
         // Java: String msg = "Namespace feature is always supported in dom4j";
         // Java: throw new SAXNotSupportedException(msg);
      }
      // Java: this.features.put(name, value);
    }

    // Java: void setProperty(String name, Object value)
    pub fn setProperty(&self, _name: &str, _value: i32) {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
         {
            // Java: this.setLexicalHandler((LexicalHandler)value);
            // Java: return;
         }
      }
      // Java: this.properties.put(name, value);
    }

    // Java: Object getProperty(String name)
    pub fn getProperty(&self, _name: &str) -> i32 {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
         {
            // Java: return this.getLexicalHandler();
         }
      }
      // Java: return this.properties.get(name);
      0
    }

    // Java: void parse(String systemId)
    pub fn parse_systemId(&self, _systemId: &str) {
      // Java: throw new SAXNotSupportedException("This XMLReader can only accept <dom4j> InputSource objects");
    }

    // Java: void parse(InputSource input)
    pub fn parse_input(&self, _input: i32) {
      // Java: if (input instanceof DocumentInputSource)
      {
         // Java: DocumentInputSource documentInput = (DocumentInputSource)input;
         // Java: Document document = documentInput.getDocument();
         // Java: this.write(document);
      }
      // Java: else
      {
         // Java: throw new SAXNotSupportedException("This XMLReader can only accept <dom4j> InputSource objects");
      }
    }

    // Java: void writeContent(Branch branch, NamespaceStack namespaceStack)
    pub fn writeContent(&self, _branch: i32, _namespaceStack: i32) {
      // Java: Iterator iter = branch.nodeIterator();
      // Java: while(iter.hasNext())
      {
         // Java: Object object = iter.next();
         // Java: if (object instanceof Element)
         {
            // Java: this.write((Element)object, namespaceStack);
         }
         // Java: else if (object instanceof CharacterData)
         {
            // Java: if (object instanceof Text)
            {
               // Java: Text text = (Text)object;
               // Java: this.write(text.getText());
            }
            // Java: else if (object instanceof CDATA)
            {
               // Java: this.write((CDATA)object);
            }
            // Java: else
            {
               // Java: if (!(object instanceof Comment))
               {
                  // Java: throw new SAXException("Invalid Node in DOM4J content: " + object + " of type: " + object.getClass());
               }
               // Java: this.write((Comment)object);
            }
         }
         // Java: else if (object instanceof String)
         {
            // Java: this.write((String)object);
         }
         // Java: else if (object instanceof Entity)
         {
            // Java: this.write((Entity)object);
         }
         // Java: else if (object instanceof ProcessingInstruction)
         {
            // Java: this.write((ProcessingInstruction)object);
         }
         // Java: else
         {
            // Java: if (!(object instanceof Namespace))
            {
               // Java: throw new SAXException("Invalid Node in DOM4J content: " + object);
            }
            // Java: this.write((Node)((Namespace)object));
         }
      }
    }

    // Java: void documentLocator(Document document)
    pub fn documentLocator(&self, _document: i32) {
      // Java: LocatorImpl locator = new LocatorImpl();
      // Java: String publicID = null;
      // Java: String systemID = null;
      // Java: DocumentType docType = document.getDocType();
      // Java: if (docType != null)
      {
         // Java: publicID = docType.getPublicID();
         // Java: systemID = docType.getSystemID();
      }
      // Java: if (publicID != null)
      {
         // Java: locator.setPublicId(publicID);
      }
      // Java: if (systemID != null)
      {
         // Java: locator.setSystemId(systemID);
      }
      // Java: locator.setLineNumber(-1);
      // Java: locator.setColumnNumber(-1);
      // Java: this.contentHandler.setDocumentLocator(locator);
    }

    // Java: void entityResolver(Document document)
    pub fn entityResolver(&self, _document: i32) {
      // Java: if (this.entityResolver != null)
      {
         // Java: DocumentType docType = document.getDocType();
         // Java: if (docType != null)
         {
            // Java: String publicID = docType.getPublicID();
            // Java: String systemID = docType.getSystemID();
            // Java: if (publicID != null || systemID != null)
            {
               // Java: try
               {
                  // Java: this.entityResolver.resolveEntity(publicID, systemID);
               }
               // Java: catch (IOException e)
               {
                  // Java: throw new SAXException("Could not resolve publicID: " + publicID + " systemID: " + systemID, e);
               }
            }
         }
      }
    }

    // Java: void dtdHandler(Document document)
    pub fn dtdHandler(&self, _document: i32) {
    }

    // Java: void startDocument()
    pub fn startDocument(&self) {
      // Java: this.contentHandler.startDocument();
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: this.contentHandler.endDocument();
    }

    // Java: void write(Element element, NamespaceStack namespaceStack)
    pub fn write_element_ns(&self, _element: i32, _namespaceStack: i32) {
      // Java: int stackSize = namespaceStack.size();
      // Java: AttributesImpl namespaceAttributes = this.startPrefixMapping(element, namespaceStack);
      // Java: this.startElement(element, namespaceAttributes);
      // Java: this.writeContent(element, namespaceStack);
      // Java: this.endElement(element);
      // Java: this.endPrefixMapping(namespaceStack, stackSize);
    }

    // Java: AttributesImpl startPrefixMapping(Element element, NamespaceStack namespaceStack)
    pub fn startPrefixMapping(&self, _element: i32, _namespaceStack: i32) -> i32 {
      // Java: AttributesImpl namespaceAttributes = null;
      // Java: Namespace elementNamespace = element.getNamespace();
      // Java: if (elementNamespace != null && !this.isIgnoreableNamespace(elementNamespace, namespaceStack))
      {
         // Java: namespaceStack.push(elementNamespace);
         // Java: this.contentHandler.startPrefixMapping(elementNamespace.getPrefix(), elementNamespace.getURI());
         // Java: namespaceAttributes = this.addNamespaceAttribute(namespaceAttributes, elementNamespace);
      }
      // Java: List declaredNamespaces = element.declaredNamespaces();
      // Java: int i = 0;
      // Java: for(int size = declaredNamespaces.size(); i < size; ++i)
      {
         // Java: Namespace namespace = (Namespace)declaredNamespaces.get(i);
         // Java: if (!this.isIgnoreableNamespace(namespace, namespaceStack))
         {
            // Java: namespaceStack.push(namespace);
            // Java: this.contentHandler.startPrefixMapping(namespace.getPrefix(), namespace.getURI());
            // Java: namespaceAttributes = this.addNamespaceAttribute(namespaceAttributes, namespace);
         }
      }
      // Java: return namespaceAttributes;
      0
    }

    // Java: void endPrefixMapping(NamespaceStack stack, int stackSize)
    pub fn endPrefixMapping(&self, _stack: i32, _stackSize: i32) {
      // Java: while(stack.size() > stackSize)
      {
         // Java: Namespace namespace = stack.pop();
         // Java: if (namespace != null)
         {
            // Java: this.contentHandler.endPrefixMapping(namespace.getPrefix());
         }
      }
    }

    // Java: void startElement(Element element, AttributesImpl namespaceAttributes)
    pub fn startElement(&self, _element: i32, _namespaceAttributes: i32) {
      // Java: this.contentHandler.startElement(element.getNamespaceURI(), element.getName(), element.getQualifiedName(), this.createAttributes(element, namespaceAttributes));
    }

    // Java: void endElement(Element element)
    pub fn endElement(&self, _element: i32) {
      // Java: this.contentHandler.endElement(element.getNamespaceURI(), element.getName(), element.getQualifiedName());
    }

    // Java: Attributes createAttributes(Element element, Attributes namespaceAttributes)
    pub fn createAttributes(&self, _element: i32, _namespaceAttributes: i32) -> i32 {
      // Java: this.attributes.clear();
      // Java: if (namespaceAttributes != null)
      {
         // Java: this.attributes.setAttributes(namespaceAttributes);
      }
      // Java: Iterator iter = element.attributeIterator();
      // Java: while(iter.hasNext())
      {
         // Java: Attribute attribute = (Attribute)iter.next();
         // Java: this.attributes.addAttribute(attribute.getNamespaceURI(), attribute.getName(), attribute.getQualifiedName(), "CDATA", attribute.getValue());
      }
      // Java: return this.attributes;
      0
    }

    // Java: AttributesImpl addNamespaceAttribute(AttributesImpl attrs, Namespace namespace)
    pub fn addNamespaceAttribute(&self, _attrs: i32, _namespace: i32) -> i32 {
      // Java: if (this.declareNamespaceAttributes)
      {
         // Java: if (attrs == null)
         {
            // Java: attrs = new AttributesImpl();
         }
         // Java: String prefix = namespace.getPrefix();
         // Java: String qualifiedName = "xmlns";
         // Java: if (prefix != null && prefix.length() > 0)
         {
            // Java: qualifiedName = "xmlns:" + prefix;
         }
         // Java: String uri = "";
         // Java: String type = "CDATA";
         // Java: String value = namespace.getURI();
         // Java: attrs.addAttribute(uri, prefix, qualifiedName, type, value);
      }
      // Java: return attrs;
      0
    }

    // Java: boolean isIgnoreableNamespace(Namespace namespace, NamespaceStack namespaceStack)
    pub fn isIgnoreableNamespace(&self, _namespace: i32, _namespaceStack: i32) -> bool {
      // Java: if (!namespace.equals(Namespace.NO_NAMESPACE) && !namespace.equals(Namespace.XML_NAMESPACE))
      {
         // Java: String uri = namespace.getURI();
         // Java: return uri != null && uri.length() > 0 ? namespaceStack.contains(namespace) : true;
      }
      // Java: else
      {
         // Java: return true;
      }
      false
    }

    // Java: void checkForNullHandlers()
    pub fn checkForNullHandlers(&self) {
    }

}
