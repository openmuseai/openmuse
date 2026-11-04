// Java: import java.io.BufferedWriter
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.OutputStreamWriter
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.StringTokenizer
// Java: import org.dom4j.Attribute
use crate::org::dom4j::Attribute;
// Java: import org.dom4j.CDATA
use crate::org::dom4j::CDATA;
// Java: import org.dom4j.Comment
use crate::org::dom4j::Comment;
// Java: import org.dom4j.Document
use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentType
use crate::org::dom4j::DocumentType;
// Java: import org.dom4j.Element
use crate::org::dom4j::Element;
// Java: import org.dom4j.Entity
use crate::org::dom4j::Entity;
// Java: import org.dom4j.Namespace
use crate::org::dom4j::Namespace;
// Java: import org.dom4j.Node
use crate::org::dom4j::Node;
// Java: import org.dom4j.NodeType
use crate::org::dom4j::NodeType;
// Java: import org.dom4j.ProcessingInstruction
use crate::org::dom4j::ProcessingInstruction;
// Java: import org.dom4j.Text
use crate::org::dom4j::Text;
// Java: import org.dom4j.tree.NamespaceStack
use crate::org::dom4j::tree::NamespaceStack;
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.Locator
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXNotRecognizedException
// Java: import org.xml.sax.SAXNotSupportedException
// Java: import org.xml.sax.XMLReader
// Java: import org.xml.sax.ext.LexicalHandler
// Java: import org.xml.sax.helpers.XMLFilterImpl

// Java field: String PAD_TEXT
pub(crate) const PAD_TEXT: &str = " ";
// Java field: String[] LEXICAL_HANDLER_NAMES
static LEXICAL_HANDLER_NAMES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
// Java field: OutputFormat DEFAULT_FORMAT
static DEFAULT_FORMAT: std::sync::OnceLock<i32> = std::sync::OnceLock::new();

// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler
pub struct XMLWriter {
    // Java field: boolean resolveEntityRefs
    pub(crate) resolveEntityRefs: bool,
    // Java field: NodeType lastOutputNodeType
    pub(crate) lastOutputNodeType: i32,
    // Java field: boolean lastElementClosed
    lastElementClosed: bool,
    // Java field: boolean preserve
    pub(crate) preserve: bool,
    // Java field: Writer writer
    pub(crate) writer: String,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: i32,
    // Java field: OutputFormat format
    format: i32,
    // Java field: boolean escapeText
    pub(crate) escapeText: bool,
    // Java field: int indentLevel
    indentLevel: i32,
    // Java field: StringBuffer buffer
    buffer: String,
    // Java field: boolean charsAdded
    charsAdded: bool,
    // Java field: char lastChar
    lastChar: char,
    // Java field: boolean autoFlush
    autoFlush: bool,
    // Java field: LexicalHandler lexicalHandler
    pub(crate) lexicalHandler: i32,
    // Java field: boolean showCommentsInDTDs
    showCommentsInDTDs: bool,
    // Java field: boolean inDTD
    inDTD: bool,
    // Java field: Map<String, String> namespacesMap
    namespacesMap: std::collections::HashMap<String, String>,
    // Java field: int maximumAllowedCharacter
    pub(crate) maximumAllowedCharacter: i32,
}

// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler
impl XMLWriter {
    // Java: XMLWriter(Writer writer)
    pub fn XMLWriter_writer(&self, _writer: &str) {
      // Java: this(writer, DEFAULT_FORMAT);
    }

    // Java: XMLWriter(Writer writer, OutputFormat format)
    pub fn XMLWriter_writer_format(&self, _writer: &str, _format: &str) {
      // Java: super();
      // Java: this.resolveEntityRefs = true;
      // Java: this.lastElementClosed = false;
      // Java: this.preserve = false;
      // Java: this.namespaceStack = new NamespaceStack();
      // Java: this.escapeText = true;
      // Java: this.indentLevel = 0;
      // Java: this.buffer = new StringBuffer();
      // Java: this.charsAdded = false;
      // Java: this.writer = writer;
      // Java: this.format = format;
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
    }

    // Java: XMLWriter()
    pub fn XMLWriter_default(&self) {
      // Java: super();
      // Java: this.resolveEntityRefs = true;
      // Java: this.lastElementClosed = false;
      // Java: this.preserve = false;
      // Java: this.namespaceStack = new NamespaceStack();
      // Java: this.escapeText = true;
      // Java: this.indentLevel = 0;
      // Java: this.buffer = new StringBuffer();
      // Java: this.charsAdded = false;
      // Java: this.format = DEFAULT_FORMAT;
      // Java: this.writer = new BufferedWriter(new OutputStreamWriter(System.out, StandardCharsets.UTF_8));
      // Java: this.autoFlush = true;
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
    }

    // Java: XMLWriter(OutputStream out)
    pub fn XMLWriter_out(&self, _out: &str) {
      // Java: super();
      // Java: this.resolveEntityRefs = true;
      // Java: this.lastElementClosed = false;
      // Java: this.preserve = false;
      // Java: this.namespaceStack = new NamespaceStack();
      // Java: this.escapeText = true;
      // Java: this.indentLevel = 0;
      // Java: this.buffer = new StringBuffer();
      // Java: this.charsAdded = false;
      // Java: this.format = DEFAULT_FORMAT;
      // Java: this.writer = this.createWriter(out, this.format.getEncoding());
      // Java: this.autoFlush = true;
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
    }

    // Java: XMLWriter(OutputStream out, OutputFormat format)
    pub fn XMLWriter_out_format(&self, _out: &str, _format: &str) {
      // Java: super();
      // Java: this.resolveEntityRefs = true;
      // Java: this.lastElementClosed = false;
      // Java: this.preserve = false;
      // Java: this.namespaceStack = new NamespaceStack();
      // Java: this.escapeText = true;
      // Java: this.indentLevel = 0;
      // Java: this.buffer = new StringBuffer();
      // Java: this.charsAdded = false;
      // Java: this.format = format;
      // Java: this.writer = this.createWriter(out, format.getEncoding());
      // Java: this.autoFlush = true;
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
    }

    // Java: XMLWriter(OutputFormat format)
    pub fn XMLWriter_format(&self, _format: &str) {
      // Java: super();
      // Java: this.resolveEntityRefs = true;
      // Java: this.lastElementClosed = false;
      // Java: this.preserve = false;
      // Java: this.namespaceStack = new NamespaceStack();
      // Java: this.escapeText = true;
      // Java: this.indentLevel = 0;
      // Java: this.buffer = new StringBuffer();
      // Java: this.charsAdded = false;
      // Java: this.format = format;
      // Java: this.writer = this.createWriter(System.out, format.getEncoding());
      // Java: this.autoFlush = true;
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
    }

    // Java: void setWriter(Writer writer)
    pub fn setWriter(&self, _writer: &str) {
      // Java: this.writer = writer;
      // Java: this.autoFlush = false;
    }

    // Java: void setOutputStream(OutputStream out)
    pub fn setOutputStream(&self, _out: &str) {
      // Java: this.writer = this.createWriter(out, this.format.getEncoding());
      // Java: this.autoFlush = true;
    }

    // Java: boolean isEscapeText()
    pub fn isEscapeText(&self) -> bool {
      // Java: return this.escapeText;
      false
    }

    // Java: void setEscapeText(boolean escapeText)
    pub fn setEscapeText(&self, _escapeText: &str) {
      // Java: this.escapeText = escapeText;
    }

    // Java: void setIndentLevel(int indentLevel)
    pub fn setIndentLevel(&self, _indentLevel: &str) {
      // Java: this.indentLevel = indentLevel;
    }

    // Java: int getMaximumAllowedCharacter()
    pub fn getMaximumAllowedCharacter(&self) -> i32 {
      // Java: if (this.maximumAllowedCharacter == 0)
      {
         // Java: this.maximumAllowedCharacter = this.defaultMaximumAllowedCharacter();
        0
    }
      // Java: return this.maximumAllowedCharacter;
    }

    // Java: void setMaximumAllowedCharacter(int maximumAllowedCharacter)
    pub fn setMaximumAllowedCharacter(&self, _maximumAllowedCharacter: &str) {
      // Java: this.maximumAllowedCharacter = maximumAllowedCharacter;
    }

    // Java: void flush()
    pub fn flush(&self) {
      // Java: this.writer.flush();
    }

    // Java: void close()
    pub fn close(&self) {
      // Java: this.writer.close();
    }

    // Java: void println()
    pub fn println(&self) {
      // Java: this.writer.write(this.format.getLineSeparator());
    }

    // Java: void write(Attribute attribute)
    pub fn write_attribute(&self, _attribute: &str) {
      // Java: this.writeAttribute(attribute);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Document doc)
    pub fn write_document(&self, _doc: &str) {
      // Java: this.writeDeclaration();
      // Java: if (doc.getDocType() != null)
      {
         // Java: this.indent();
         // Java: this.writeDocType(doc.getDocType());
      }
      // Java: int i = 0;
      // Java: for(int size = doc.nodeCount(); i < size; ++i)
      {
         // Java: Node node = doc.node(i);
         // Java: this.writeNode(node);
      }
      // Java: this.writePrintln();
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Element element)
    pub fn write_element(&self, _element: &str) {
      // Java: this.writeElement(element);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(CDATA cdata)
    pub fn write_cdata(&self, _cdata: &str) {
      // Java: this.writeCDATA(cdata.getText());
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Comment comment)
    pub fn write_comment(&self, _comment: &str) {
      // Java: this.writeComment(comment.getText());
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(DocumentType docType)
    pub fn write_doctype(&self, _docType: &str) {
      // Java: this.writeDocType(docType);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Entity entity)
    pub fn write_entity(&self, _entity: &str) {
      // Java: this.writeEntity(entity);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Namespace namespace)
    pub fn write_namespace(&self, _namespace: &str) {
      // Java: this.writeNamespace(namespace);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(ProcessingInstruction processingInstruction)
    pub fn write_pi(&self, _processingInstruction: &str) {
      // Java: this.writeProcessingInstruction(processingInstruction);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(String text)
    pub fn write_string(&self, _text: &str) {
      // Java: this.writeString(text);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Text text)
    pub fn write_text(&self, _text: &str) {
      // Java: this.writeString(text.getText());
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Node node)
    pub fn write_node(&self, _node: &str) {
      // Java: this.writeNode(node);
      // Java: if (this.autoFlush)
      {
         // Java: this.flush();
      }
    }

    // Java: void write(Object object)
    pub fn write_object(&self, _object: &str) {
      // Java: if (object instanceof Node)
      {
         // Java: this.write((Node)object);
      }
      // Java: else if (object instanceof String)
      {
         // Java: this.write((String)object);
      }
      // Java: else if (object instanceof List)
      {
         // Java: List list = (List)object;
         // Java: int i = 0;
         // Java: for(int size = list.size(); i < size; ++i)
         {
            // Java: this.write(list.get(i));
         }
      }
      // Java: else if (object != null)
      {
         // Java: throw new IOException("Invalid object: " + object);
      }
    }

    // Java: void writeOpen(Element element)
    pub fn writeOpen(&self, _element: &str) {
      // Java: this.writer.write("<");
      // Java: this.writer.write(element.getQualifiedName());
      // Java: this.writeAttributes(element);
      // Java: this.writer.write(">");
    }

    // Java: void writeClose(Element element)
    pub fn writeClose(&self, _element: &str) {
      // Java: this.writeClose(element.getQualifiedName());
    }

    // Java: void parse(InputSource source)
    pub fn parse(&self, _source: &str) {
      // Java: this.installLexicalHandler();
      // Java: super.parse(source);
    }

    // Java: void setProperty(String name, Object value)
    pub fn setProperty(&self, _name: &str, _value: &str) {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
         {
            // Java: this.setLexicalHandler((LexicalHandler)value);
            // Java: return;
         }
      }
      // Java: super.setProperty(name, value);
    }

    // Java: Object getProperty(String name)
    pub fn getProperty(&self, _name: &str) {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
         {
            // Java: return this.getLexicalHandler();
         }
      }
      // Java: return super.getProperty(name);
    }

    // Java: void setLexicalHandler(LexicalHandler handler)
    pub fn setLexicalHandler(&self, _handler: &str) {
      // Java: if (handler == null)
      {
         // Java: throw new NullPointerException("Null lexical handler");
      }
      // Java: else
      {
         // Java: this.lexicalHandler = handler;
      }
    }

    // Java: LexicalHandler getLexicalHandler()
    pub fn getLexicalHandler(&self) {
      // Java: return this.lexicalHandler;
    }

    // Java: void setDocumentLocator(Locator locator)
    pub fn setDocumentLocator(&self, _locator: &str) {
      // Java: super.setDocumentLocator(locator);
    }

    // Java: void startDocument()
    pub fn startDocument(&self) {
      // Java: try
      {
         // Java: this.writeDeclaration();
         // Java: super.startDocument();
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
    }

    // Java: void endDocument()
    pub fn endDocument(&self) {
      // Java: super.endDocument();
      // Java: if (this.autoFlush)
      {
         // Java: try
         {
            // Java: this.flush();
         }
         // Java: catch (IOException var2)
         {
         }
      }
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&self, _prefix: &str, _uri: &str) {
      // Java: if (this.namespacesMap == null)
      {
         // Java: this.namespacesMap = new HashMap();
      }
      // Java: this.namespacesMap.put(prefix, uri);
      // Java: super.startPrefixMapping(prefix, uri);
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&self, _prefix: &str) {
      // Java: super.endPrefixMapping(prefix);
    }

    // Java: void startElement(String namespaceURI, String localName, String qName, Attributes attributes)
    pub fn startElement(&self, _namespaceURI: &str, _localName: &str, _qName: &str, _attributes: &str) {
      // Java: try
      {
         // Java: this.charsAdded = false;
         // Java: this.writePrintln();
         // Java: this.indent();
         // Java: this.writer.write("<");
         // Java: this.writer.write(qName);
         // Java: this.writeNamespaces();
         // Java: this.writeAttributes(attributes);
         // Java: this.writer.write(">");
         // Java: ++this.indentLevel;
         // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
         // Java: this.lastElementClosed = false;
         // Java: super.startElement(namespaceURI, localName, qName, attributes);
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&self, _namespaceURI: &str, _localName: &str, _qName: &str) {
      // Java: try
      {
         // Java: this.charsAdded = false;
         // Java: --this.indentLevel;
         // Java: if (this.lastElementClosed)
         {
            // Java: this.writePrintln();
            // Java: this.indent();
         }
         // Java: boolean hadContent = true;
         // Java: if (hadContent)
         {
            // Java: this.writeClose(qName);
         }
         // Java: else
         {
            // Java: this.writeEmptyElementClose(qName);
         }
         // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
         // Java: this.lastElementClosed = true;
         // Java: super.endElement(namespaceURI, localName, qName);
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
    }

    // Java: void characters(char[] ch, int start, int length)
    pub fn characters(&self, _ch: &str, _start: &str, _length: &str) {
      // Java: if (ch != null && ch.length != 0 && length > 0)
      {
         // Java: try
         {
            // Java: String string = String.valueOf(ch, start, length);
            // Java: if (this.escapeText)
            {
               // Java: string = this.escapeElementEntities(string);
            }
            // Java: if (!this.format.isTrimText())
            {
               // Java: this.writer.write(string);
            }
            // Java: else
            {
               // Java: if (this.lastOutputNodeType == NodeType.TEXT_NODE && !this.charsAdded)
               {
                  // Java: this.writer.write(32);
               }
               // Java: else if (this.charsAdded && Character.isWhitespace(this.lastChar))
               {
                  // Java: this.writer.write(32);
               }
               // Java: else if (this.lastOutputNodeType == NodeType.ELEMENT_NODE && this.format.isPadText() && this.lastElementClosed && Character.isWhitespace(ch[0]))
               {
                  // Java: this.writer.write(" ");
               }
               // Java: String delim = "";
               // Java: for(StringTokenizer tokens = new StringTokenizer(string); tokens.hasMoreTokens(); delim = " ")
               {
                  // Java: this.writer.write(delim);
                  // Java: this.writer.write(tokens.nextToken());
               }
            }
            // Java: this.charsAdded = true;
            // Java: this.lastChar = ch[start + length - 1];
            // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
            // Java: super.characters(ch, start, length);
         }
         // Java: catch (IOException e)
         {
            // Java: this.handleException(e);
         }
      }
    }

    // Java: void ignorableWhitespace(char[] ch, int start, int length)
    pub fn ignorableWhitespace(&self, _ch: &str, _start: &str, _length: &str) {
      // Java: super.ignorableWhitespace(ch, start, length);
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&self, _target: &str, _data: &str) {
      // Java: try
      {
         // Java: this.indent();
         // Java: this.writer.write("<?");
         // Java: this.writer.write(target);
         // Java: this.writer.write(" ");
         // Java: this.writer.write(data);
         // Java: this.writer.write("?>");
         // Java: this.writePrintln();
         // Java: this.lastOutputNodeType = NodeType.PROCESSING_INSTRUCTION_NODE;
         // Java: super.processingInstruction(target, data);
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
    }

    // Java: void notationDecl(String name, String publicID, String systemID)
    pub fn notationDecl(&self, _name: &str, _publicID: &str, _systemID: &str) {
      // Java: super.notationDecl(name, publicID, systemID);
    }

    // Java: void unparsedEntityDecl(String name, String publicID, String systemID, String notationName)
    pub fn unparsedEntityDecl(&self, _name: &str, _publicID: &str, _systemID: &str, _notationName: &str) {
      // Java: super.unparsedEntityDecl(name, publicID, systemID, notationName);
    }

    // Java: void startDTD(String name, String publicID, String systemID)
    pub fn startDTD(&self, _name: &str, _publicID: &str, _systemID: &str) {
      // Java: this.inDTD = true;
      // Java: try
      {
         // Java: this.writeDocType(name, publicID, systemID);
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.startDTD(name, publicID, systemID);
      }
    }

    // Java: void endDTD()
    pub fn endDTD(&self) {
      // Java: this.inDTD = false;
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.endDTD();
      }
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
      // Java: try
      {
         // Java: this.writer.write("<![CDATA[");
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.startCDATA();
      }
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
      // Java: try
      {
         // Java: this.writer.write("]]>");
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.endCDATA();
      }
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&self, _name: &str) {
      // Java: try
      {
         // Java: this.writeEntityRef(name);
      }
      // Java: catch (IOException e)
      {
         // Java: this.handleException(e);
      }
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.startEntity(name);
      }
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&self, _name: &str) {
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.endEntity(name);
      }
    }

    // Java: void comment(char[] ch, int start, int length)
    pub fn comment(&self, _ch: &str, _start: &str, _length: &str) {
      // Java: if (this.showCommentsInDTDs || !this.inDTD)
      {
         // Java: try
         {
            // Java: this.charsAdded = false;
            // Java: this.writeComment(new String(ch, start, length));
         }
         // Java: catch (IOException e)
         {
            // Java: this.handleException(e);
         }
      }
      // Java: if (this.lexicalHandler != null)
      {
         // Java: this.lexicalHandler.comment(ch, start, length);
      }
    }

    // Java: void writeElement(Element element)
    pub fn writeElement(&self, _element: &str) {
      // Java: int size = element.nodeCount();
      // Java: String qualifiedName = element.getQualifiedName();
      // Java: this.writePrintln();
      // Java: this.indent();
      // Java: this.writer.write("<");
      // Java: this.writer.write(qualifiedName);
      // Java: int previouslyDeclaredNamespaces = this.namespaceStack.size();
      // Java: Namespace ns = element.getNamespace();
      // Java: if (this.isNamespaceDeclaration(ns))
      {
         // Java: this.namespaceStack.push(ns);
         // Java: this.writeNamespace(ns);
      }
      // Java: boolean textOnly = true;
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: Node node = element.node(i);
         // Java: if (node instanceof Namespace)
         {
            // Java: Namespace additional = (Namespace)node;
            // Java: if (this.isNamespaceDeclaration(additional))
            {
               // Java: this.namespaceStack.push(additional);
               // Java: this.writeNamespace(additional);
            }
         }
         // Java: else if (node instanceof Element)
         {
            // Java: textOnly = false;
         }
         // Java: else if (node instanceof Comment)
         {
            // Java: textOnly = false;
         }
      }
      // Java: this.writeAttributes(element);
      // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
      // Java: if (size <= 0)
      {
         // Java: this.writeEmptyElementClose(qualifiedName);
      }
      // Java: else
      {
         // Java: this.writer.write(">");
         // Java: if (textOnly)
         {
            // Java: this.writeElementContent(element);
         }
         // Java: else
         {
            // Java: ++this.indentLevel;
            // Java: this.writeElementContent(element);
            // Java: --this.indentLevel;
            // Java: this.writePrintln();
            // Java: this.indent();
         }
         // Java: this.writer.write("</");
         // Java: this.writer.write(qualifiedName);
         // Java: this.writer.write(">");
      }
      // Java: while(this.namespaceStack.size() > previouslyDeclaredNamespaces)
      {
         // Java: this.namespaceStack.pop();
      }
      // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
    }

    // Java: boolean isElementSpacePreserved(Element element)
    pub fn isElementSpacePreserved(&self, _element: &str) -> bool {
      // Java: Attribute attr = element.attribute("space");
      // Java: boolean preserveFound = this.preserve;
      // Java: if (attr != null)
      {
         // Java: if ("xml".equals(attr.getNamespacePrefix()) && "preserve".equals(attr.getText()))
         {
            // Java: preserveFound = true;
         }
         // Java: else
         {
            // Java: preserveFound = false;
         }
      }
      // Java: return preserveFound;
      false
    }

    // Java: void writeElementContent(Element element)
    pub fn writeElementContent(&self, _element: &str) {
      // Java: boolean trim = this.format.isTrimText();
      // Java: boolean oldPreserve = this.preserve;
      // Java: if (trim)
      {
         // Java: this.preserve = this.isElementSpacePreserved(element);
         // Java: trim = !this.preserve;
      }
      // Java: if (trim)
      {
         // Java: Text lastTextNode = null;
         // Java: StringBuffer buff = null;
         // Java: boolean textOnly = true;
         // Java: int i = 0;
         // Java: for(int size = element.nodeCount(); i < size; ++i)
         {
            // Java: Node node = element.node(i);
            // Java: if (node instanceof Text)
            {
               // Java: if (lastTextNode == null)
               {
                  // Java: lastTextNode = (Text)node;
               }
               // Java: else
               {
                  // Java: if (buff == null)
                  {
                     // Java: buff = new StringBuffer(lastTextNode.getText());
                  }
                  // Java: buff.append(((Text)node).getText());
               }
            }
            // Java: else
            {
               // Java: if (!textOnly && this.format.isPadText())
               {
                  // Java: char firstChar = 'a';
                  // Java: if (buff != null)
                  {
                     // Java: firstChar = buff.charAt(0);
                  }
                  // Java: else if (lastTextNode != null)
                  {
                     // Java: firstChar = lastTextNode.getText().charAt(0);
                  }
                  // Java: if (Character.isWhitespace(firstChar))
                  {
                     // Java: this.writer.write(" ");
                  }
               }
               // Java: if (lastTextNode != null)
               {
                  // Java: if (buff != null)
                  {
                     // Java: this.writeString(buff.toString());
                     // Java: buff = null;
                  }
                  // Java: else
                  {
                     // Java: this.writeString(lastTextNode.getText());
                  }
                  // Java: if (this.format.isPadText())
                  {
                     // Java: char lastTextChar = 'a';
                     // Java: if (buff != null)
                     {
                        // Java: lastTextChar = buff.charAt(buff.length() - 1);
                     }
                     // Java: else if (lastTextNode != null)
                     {
                        // Java: String txt = lastTextNode.getText();
                        // Java: lastTextChar = txt.charAt(txt.length() - 1);
                     }
                     // Java: if (Character.isWhitespace(lastTextChar))
                     {
                        // Java: this.writer.write(" ");
                     }
                  }
                  // Java: lastTextNode = null;
               }
               // Java: textOnly = false;
               // Java: this.writeNode(node);
            }
         }
         // Java: if (lastTextNode != null)
         {
            // Java: if (!textOnly && this.format.isPadText())
            {
               // Java: i = 97;
               // Java: if (buff != null)
               {
                  // Java: i = buff.charAt(0);
               }
               // Java: else
               {
                  // Java: i = lastTextNode.getText().charAt(0);
               }
               // Java: if (Character.isWhitespace((char)i))
               {
                  // Java: this.writer.write(" ");
               }
            }
            // Java: if (buff != null)
            {
               // Java: this.writeString(buff.toString());
               // Java: StringBuffer var14 = null;
            }
            // Java: else
            {
               // Java: this.writeString(lastTextNode.getText());
            }
            // Java: Object var12 = null;
         }
      }
      // Java: else
      {
         // Java: Node lastTextNode = null;
         // Java: int i = 0;
         // Java: for(int size = element.nodeCount(); i < size; ++i)
         {
            // Java: Node node = element.node(i);
            // Java: if (node instanceof Text)
            {
               // Java: this.writeNode(node);
               // Java: lastTextNode = node;
            }
            // Java: else
            {
               // Java: if (lastTextNode != null && this.format.isPadText())
               {
                  // Java: String txt = lastTextNode.getText();
                  // Java: char lastTextChar = txt.charAt(txt.length() - 1);
                  // Java: if (Character.isWhitespace(lastTextChar))
                  {
                     // Java: this.writer.write(" ");
                  }
               }
               // Java: this.writeNode(node);
               // Java: lastTextNode = null;
            }
         }
      }
      // Java: this.preserve = oldPreserve;
    }

    // Java: void writeCDATA(String text)
    pub fn writeCDATA(&self, _text: &str) {
      // Java: this.writer.write("<![CDATA[");
      // Java: if (text != null)
      {
         // Java: this.writer.write(text);
      }
      // Java: this.writer.write("]]>");
      // Java: this.lastOutputNodeType = NodeType.CDATA_SECTION_NODE;
    }

    // Java: void writeDocType(DocumentType docType)
    pub fn writeDocType(&self, _docType: &str) {
      // Java: if (docType != null)
      {
         // Java: docType.write(this.writer);
         // Java: this.writePrintln();
      }
    }

    // Java: void writeNamespace(Namespace namespace)
    pub fn writeNamespace(&self, _namespace: &str) {
      // Java: if (namespace != null)
      {
         // Java: this.writeNamespace(namespace.getPrefix(), namespace.getURI());
      }
    }

    // Java: void writeNamespaces()
    pub fn writeNamespaces(&self) {
      // Java: if (this.namespacesMap != null)
      {
         // Java: for(Map.Entry entry : this.namespacesMap.entrySet())
         {
            // Java: String prefix = (String)entry.getKey();
            // Java: String uri = (String)entry.getValue();
            // Java: this.writeNamespace(prefix, uri);
         }
         // Java: this.namespacesMap = null;
      }
    }

    // Java: void writeNamespace(String prefix, String uri)
    pub fn writeNamespace_sax(&self, _prefix: &str, _uri: &str) {
      // Java: if (prefix != null && prefix.length() > 0)
      {
         // Java: this.writer.write(" xmlns:");
         // Java: this.writer.write(prefix);
         // Java: this.writer.write("=\"");
      }
      // Java: else
      {
         // Java: this.writer.write(" xmlns=\"");
      }
      // Java: this.writer.write(uri);
      // Java: this.writer.write("\"");
    }

    // Java: void writeProcessingInstruction(ProcessingInstruction pi)
    pub fn writeProcessingInstruction(&self, _pi: &str) {
      // Java: this.writer.write("<?");
      // Java: this.writer.write(pi.getName());
      // Java: this.writer.write(" ");
      // Java: this.writer.write(pi.getText());
      // Java: this.writer.write("?>");
      // Java: this.writePrintln();
      // Java: this.lastOutputNodeType = NodeType.PROCESSING_INSTRUCTION_NODE;
    }

    // Java: void writeString(String text)
    pub fn writeString(&self, _text: &str) {
      // Java: if (text != null && text.length() > 0)
      {
         // Java: if (this.escapeText)
         {
            // Java: text = this.escapeElementEntities(text);
         }
         // Java: if (this.format.isTrimText())
         {
            // Java: boolean first = true;
            // Java: for(StringTokenizer tokenizer = new StringTokenizer(text); tokenizer.hasMoreTokens(); this.lastOutputNodeType = NodeType.TEXT_NODE)
            {
               // Java: String token = tokenizer.nextToken();
               // Java: if (first)
               {
                  // Java: first = false;
                  // Java: if (this.lastOutputNodeType == NodeType.TEXT_NODE)
                  {
                     // Java: this.writer.write(" ");
                  }
               }
               // Java: else
               {
                  // Java: this.writer.write(" ");
               }
               // Java: this.writer.write(token);
            }
         }
         // Java: else
         {
            // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
            // Java: this.writer.write(text);
         }
      }
    }

    // Java: void writeNodeText(Node node)
    pub fn writeNodeText(&self, _node: &str) {
      // Java: String text = node.getText();
      // Java: if (text != null && text.length() > 0)
      {
         // Java: if (this.escapeText)
         {
            // Java: text = this.escapeElementEntities(text);
         }
         // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
         // Java: this.writer.write(text);
      }
    }

    // Java: void writeNode(Node node)
    pub fn writeNode(&self, _node: &str) {
      // Java: NodeType nodeType = node.getNodeTypeEnum();
      // Java: switch (nodeType)
      {
         // Java: case ELEMENT_NODE:
            // Java: this.writeElement((Element)node);
            // Java: break;
         // Java: case ATTRIBUTE_NODE:
            // Java: this.writeAttribute((Attribute)node);
            // Java: break;
         // Java: case TEXT_NODE:
            // Java: this.writeNodeText(node);
            // Java: break;
         // Java: case CDATA_SECTION_NODE:
            // Java: this.writeCDATA(node.getText());
            // Java: break;
         // Java: case ENTITY_REFERENCE_NODE:
            // Java: this.writeEntity((Entity)node);
            // Java: break;
         // Java: case PROCESSING_INSTRUCTION_NODE:
            // Java: this.writeProcessingInstruction((ProcessingInstruction)node);
            // Java: break;
         // Java: case COMMENT_NODE:
            // Java: this.writeComment(node.getText());
            // Java: break;
         // Java: case DOCUMENT_NODE:
            // Java: this.write((Document)node);
            // Java: break;
         // Java: case DOCUMENT_TYPE_NODE:
            // Java: this.writeDocType((DocumentType)node);
         // Java: case NAMESPACE_NODE:
            // Java: break;
         // Java: default:
            // Java: throw new IOException("Invalid node type: " + node);
      }
    }

    // Java: void installLexicalHandler()
    pub fn installLexicalHandler(&self) {
      // Java: XMLReader parent = this.getParent();
      // Java: if (parent == null)
      {
         // Java: throw new NullPointerException("No parent for filter");
      }
      // Java: else
      {
         // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
         {
            // Java: try
            {
               // Java: parent.setProperty(LEXICAL_HANDLER_NAMES[i], this);
               // Java: break;
            }
            // Java: catch (SAXNotRecognizedException var4)
            {
            }
            // Java: catch (SAXNotSupportedException var5)
            {
            }
         }
      }
    }

    // Java: void writeDocType(String name, String publicID, String systemID)
    pub fn writeDocType_sax(&self, _name: &str, _publicID: &str, _systemID: &str) {
      // Java: boolean hasPublic = false;
      // Java: this.writer.write("<!DOCTYPE ");
      // Java: this.writer.write(name);
      // Java: if (publicID != null && !publicID.equals(""))
      {
         // Java: this.writer.write(" PUBLIC \"");
         // Java: this.writer.write(publicID);
         // Java: this.writer.write("\"");
         // Java: hasPublic = true;
      }
      // Java: if (systemID != null && !systemID.equals(""))
      {
         // Java: if (!hasPublic)
         {
            // Java: this.writer.write(" SYSTEM");
         }
         // Java: this.writer.write(" \"");
         // Java: this.writer.write(systemID);
         // Java: this.writer.write("\"");
      }
      // Java: this.writer.write(">");
      // Java: this.writePrintln();
    }

    // Java: void writeEntity(Entity entity)
    pub fn writeEntity(&self, _entity: &str) {
      // Java: if (!this.resolveEntityRefs())
      {
         // Java: this.writeEntityRef(entity.getName());
      }
      // Java: else
      {
         // Java: this.writer.write(entity.getText());
      }
    }

    // Java: void writeEntityRef(String name)
    pub fn writeEntityRef(&self, _name: &str) {
      // Java: this.writer.write("&");
      // Java: this.writer.write(name);
      // Java: this.writer.write(";");
      // Java: this.lastOutputNodeType = NodeType.ENTITY_REFERENCE_NODE;
    }

    // Java: void writeComment(String text)
    pub fn writeComment(&self, _text: &str) {
      // Java: if (this.format.isNewlines())
      {
         // Java: this.println();
         // Java: this.indent();
      }
      // Java: this.writer.write("<!--");
      // Java: this.writer.write(text);
      // Java: this.writer.write("-->");
      // Java: this.lastOutputNodeType = NodeType.COMMENT_NODE;
    }

    // Java: void writeAttributes(Element element)
    pub fn writeAttributes(&self, _element: &str) {
      // Java: int i = 0;
      // Java: for(int size = element.attributeCount(); i < size; ++i)
      {
         // Java: Attribute attribute = element.attribute(i);
         // Java: Namespace ns = attribute.getNamespace();
         // Java: if (ns != null && ns != Namespace.NO_NAMESPACE && ns != Namespace.XML_NAMESPACE)
         {
            // Java: String prefix = ns.getPrefix();
            // Java: String uri = this.namespaceStack.getURI(prefix);
            // Java: if (!ns.getURI().equals(uri))
            {
               // Java: this.writeNamespace(ns);
               // Java: this.namespaceStack.push(ns);
            }
         }
         // Java: String attName = attribute.getName();
         // Java: if (attName.startsWith("xmlns:"))
         {
            // Java: String prefix = attName.substring(6);
            // Java: if (this.namespaceStack.getNamespaceForPrefix(prefix) == null)
            {
               // Java: String uri = attribute.getValue();
               // Java: this.namespaceStack.push(prefix, uri);
               // Java: this.writeNamespace(prefix, uri);
            }
         }
         // Java: else if (attName.equals("xmlns"))
         {
            // Java: if (this.namespaceStack.getDefaultNamespace() == null)
            {
               // Java: String uri = attribute.getValue();
               // Java: this.namespaceStack.push((String)null, uri);
               // Java: this.writeNamespace((String)null, uri);
            }
         }
         // Java: else
         {
            // Java: char quote = this.format.getAttributeQuoteCharacter();
            // Java: this.writer.write(" ");
            // Java: this.writer.write(attribute.getQualifiedName());
            // Java: this.writer.write("=");
            // Java: this.writer.write(quote);
            // Java: this.writeEscapeAttributeEntities(attribute.getValue());
            // Java: this.writer.write(quote);
         }
      }
    }

    // Java: void writeAttribute(Attribute attribute)
    pub fn writeAttribute(&self, _attribute: &str) {
      // Java: this.writer.write(" ");
      // Java: this.writer.write(attribute.getQualifiedName());
      // Java: this.writer.write("=");
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      // Java: this.writer.write(quote);
      // Java: this.writeEscapeAttributeEntities(attribute.getValue());
      // Java: this.writer.write(quote);
      // Java: this.lastOutputNodeType = NodeType.ATTRIBUTE_NODE;
    }

    // Java: void writeAttributes(Attributes attributes)
    pub fn writeAttributes_sax(&self, _attributes: &str) {
      // Java: int i = 0;
      // Java: for(int size = attributes.getLength(); i < size; ++i)
      {
         // Java: this.writeAttribute(attributes, i);
      }
    }

    // Java: void writeAttribute(Attributes attributes, int index)
    pub fn writeAttribute_sax(&self, _attributes: &str, _index: &str) {
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      // Java: this.writer.write(" ");
      // Java: this.writer.write(attributes.getQName(index));
      // Java: this.writer.write("=");
      // Java: this.writer.write(quote);
      // Java: this.writeEscapeAttributeEntities(attributes.getValue(index));
      // Java: this.writer.write(quote);
    }

    // Java: void indent()
    pub fn indent(&self) {
      // Java: String indent = this.format.getIndent();
      // Java: if (indent != null && indent.length() > 0)
      {
         // Java: for(int i = 0; i < this.indentLevel; ++i)
         {
            // Java: this.writer.write(indent);
         }
      }
    }

    // Java: void writePrintln()
    pub fn writePrintln(&self) {
      // Java: if (this.format.isNewlines())
      {
         // Java: this.writer.write(this.format.getLineSeparator());
      }
    }

    // Java: Writer createWriter(OutputStream outStream, String encoding)
    pub fn createWriter(&self, _outStream: &str, _encoding: &str) {
      // Java: return new BufferedWriter(new OutputStreamWriter(outStream, encoding));
    }

    // Java: void writeDeclaration()
    pub fn writeDeclaration(&self) {
      // Java: String encoding = this.format.getEncoding();
      // Java: if (!this.format.isSuppressDeclaration())
      {
         // Java: if (encoding.equals("UTF8"))
         {
            // Java: this.writer.write("<?xml version=\"1.0\"");
            // Java: if (!this.format.isOmitEncoding())
            {
               // Java: this.writer.write(" encoding=\"UTF-8\"");
            }
            // Java: this.writer.write("?>");
         }
         // Java: else
         {
            // Java: this.writer.write("<?xml version=\"1.0\"");
            // Java: if (!this.format.isOmitEncoding())
            {
               // Java: this.writer.write(" encoding=\"" + encoding + "\"");
            }
            // Java: this.writer.write("?>");
         }
         // Java: if (this.format.isNewLineAfterDeclaration())
         {
            // Java: this.println();
         }
      }
    }

    // Java: void writeClose(String qualifiedName)
    pub fn writeClose_qname(&self, _qualifiedName: &str) {
      // Java: this.writer.write("</");
      // Java: this.writer.write(qualifiedName);
      // Java: this.writer.write(">");
    }

    // Java: void writeEmptyElementClose(String qualifiedName)
    pub fn writeEmptyElementClose(&self, _qualifiedName: &str) {
      // Java: if (!this.format.isExpandEmptyElements())
      {
         // Java: this.writer.write("/>");
      }
      // Java: else
      {
         // Java: this.writer.write("></");
         // Java: this.writer.write(qualifiedName);
         // Java: this.writer.write(">");
      }
    }

    // Java: boolean isExpandEmptyElements()
    pub fn isExpandEmptyElements(&self) -> bool {
      // Java: return this.format.isExpandEmptyElements();
      false
    }

    // Java: String escapeElementEntities(String text)
    pub fn escapeElementEntities(&self, _text: &str) -> String {
      // Java: char[] block = null;
      // Java: int last = 0;
      // Java: int size = text.length();
      // Java: int i;
      // Java: for(i = 0; i < size; ++i)
      {
         // Java: String entity = null;
         // Java: char c = text.charAt(i);
         // Java: switch (c)
         {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
               // Java: if (this.preserve)
               {
                  // Java: entity = String.valueOf(c);
               }
               // Java: break;
            // Java: case '&':
               // Java: entity = "&amp;";
               // Java: break;
            // Java: case '<':
               // Java: entity = "&lt;";
               // Java: break;
            // Java: case '>':
               // Java: entity = "&gt;";
               // Java: break;
            // Java: default:
               // Java: if (c < ' ' || this.shouldEncodeChar(c))
               {
                  // Java: entity = "&#" + c + ";";
               }
         }
         // Java: if (entity != null)
         {
            // Java: if (block == null)
            {
               // Java: block = text.toCharArray();
            }
            // Java: this.buffer.append(block, last, i - last);
            // Java: this.buffer.append(entity);
            // Java: last = i + 1;
         }
      }
      // Java: if (last == 0)
      {
         // Java: return text;
      }
      // Java: else
      {
         // Java: if (last < size)
         {
            // Java: if (block == null)
            {
               // Java: block = text.toCharArray();
            }
            // Java: this.buffer.append(block, last, i - last);
         }
         // Java: String answer = this.buffer.toString();
         // Java: this.buffer.setLength(0);
         // Java: return answer;
      }
      String::new()
    }

    // Java: void writeEscapeAttributeEntities(String txt)
    pub fn writeEscapeAttributeEntities(&self, _txt: &str) {
      // Java: if (txt != null)
      {
         // Java: String escapedText = this.escapeAttributeEntities(txt);
         // Java: this.writer.write(escapedText);
      }
    }

    // Java: String escapeAttributeEntities(String text)
    pub fn escapeAttributeEntities(&self, _text: &str) -> String {
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      // Java: char[] block = null;
      // Java: int last = 0;
      // Java: int size = text.length();
      // Java: int i;
      // Java: for(i = 0; i < size; ++i)
      {
         // Java: String entity = null;
         // Java: char c = text.charAt(i);
         // Java: switch (c)
         {
            // Java: case '\t':
            // Java: case '\n':
            // Java: case '\r':
               // Java: break;
            // Java: case '"':
               // Java: if (quote == '"')
               {
                  // Java: entity = "&quot;";
               }
               // Java: break;
            // Java: case '&':
               // Java: entity = "&amp;";
               // Java: break;
            // Java: case '\'':
               // Java: if (quote == '\'')
               {
                  // Java: entity = "&apos;";
               }
               // Java: break;
            // Java: case '<':
               // Java: entity = "&lt;";
               // Java: break;
            // Java: case '>':
               // Java: entity = "&gt;";
               // Java: break;
            // Java: default:
               // Java: if (c < ' ' || this.shouldEncodeChar(c))
               {
                  // Java: entity = "&#" + c + ";";
               }
         }
         // Java: if (entity != null)
         {
            // Java: if (block == null)
            {
               // Java: block = text.toCharArray();
            }
            // Java: this.buffer.append(block, last, i - last);
            // Java: this.buffer.append(entity);
            // Java: last = i + 1;
         }
      }
      // Java: if (last == 0)
      {
         // Java: return text;
      }
      // Java: else
      {
         // Java: if (last < size)
         {
            // Java: if (block == null)
            {
               // Java: block = text.toCharArray();
            }
            // Java: this.buffer.append(block, last, i - last);
         }
         // Java: String answer = this.buffer.toString();
         // Java: this.buffer.setLength(0);
         // Java: return answer;
      }
      String::new()
    }

    // Java: boolean shouldEncodeChar(char c)
    pub fn shouldEncodeChar(&self, _c: &str) -> bool {
      // Java: int max = this.getMaximumAllowedCharacter();
      // Java: return max > 0 && c > max;
      false
    }

    // Java: int defaultMaximumAllowedCharacter()
    pub fn defaultMaximumAllowedCharacter(&self) -> i32 {
      // Java: String encoding = this.format.getEncoding();
      // Java: return encoding != null && encoding.equals("US-ASCII") ? 127 : -1;
      0
    }

    // Java: boolean isNamespaceDeclaration(Namespace ns)
    pub fn isNamespaceDeclaration(&self, _ns: &str) -> bool {
      // Java: if (ns != null && ns != Namespace.XML_NAMESPACE)
      {
         // Java: String uri = ns.getURI();
         // Java: if (uri != null && !this.namespaceStack.contains(ns))
         {
            // Java: return true;
           false
    }
      }
      // Java: return false;
    }

    // Java: void handleException(IOException e)
    pub fn handleException(&self, _e: &str) {
      // Java: throw new SAXException(e);
    }

    // Java: OutputFormat getOutputFormat()
    pub fn getOutputFormat(&self) {
      // Java: return this.format;
    }

    // Java: boolean resolveEntityRefs()
    pub fn resolveEntityRefs(&self) -> bool {
      // Java: return this.resolveEntityRefs;
      false
    }

    // Java: void setResolveEntityRefs(boolean resolve)
    pub fn setResolveEntityRefs(&self, _resolve: &str) {
      // Java: this.resolveEntityRefs = resolve;
    }

}
