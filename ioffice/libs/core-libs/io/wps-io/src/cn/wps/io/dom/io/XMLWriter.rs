// Java: import cn.wps.io.dom.Attribute
// use io_wps_io - Attribute (trait in crate)
// Java: import cn.wps.io.dom.CDATA
// use io_wps_io - CDATA (trait in crate)
// Java: import cn.wps.io.dom.Comment
// use io_wps_io - Comment (trait in crate)
// Java: import cn.wps.io.dom.Document
// use io_wps_io - Document (trait in crate)
// Java: import cn.wps.io.dom.Element
// Element not yet defined as module
// Java: import cn.wps.io.dom.Entity
// use io_wps_io - Entity (trait in crate)
// Java: import cn.wps.io.dom.Namespace
// Namespace not yet defined as module
// Java: import cn.wps.io.dom.Node
// use io_wps_io - Node (trait in crate)
// Java: import cn.wps.io.dom.NodeType
// use io_wps_io - NodeType (in crate)
// Java: import cn.wps.io.dom.ProcessingInstruction
// use io_wps_io - PI (trait in crate)
// Java: import cn.wps.io.dom.Text
// use io_wps_io - Text (trait in crate)
// Java: import cn.wps.io.dom.tree.NamespaceStack
// NamespaceStack not yet defined
// Java: import java.io.BufferedWriter
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.OutputStreamWriter
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.StringTokenizer
// Java: import org.xml.sax.Attributes
// Attributes (SAX) not available
// Java: import org.xml.sax.InputSource
// SAX InputSource not available
// Java: import org.xml.sax.Locator
// SAX Locator not available
// Java: import org.xml.sax.SAXException
// SAX SAXException not available
// Java: import org.xml.sax.SAXNotRecognizedException
// SAX not available
// Java: import org.xml.sax.SAXNotSupportedException
// SAX not available
// Java: import org.xml.sax.XMLReader
// SAX XMLReader not available
// Java: import org.xml.sax.ext.LexicalHandler
// SAX LexicalHandler not available
// Java: import org.xml.sax.helpers.XMLFilterImpl
// SAX XMLFilterImpl not available

// Java field: String PAD_TEXT
const PAD_TEXT: &str = " ";
// Java field: String[] LEXICAL_HANDLER_NAMES
const LEXICAL_HANDLER_NAMES: &[&str] = &["http://xml.org/sax/properties/lexical-handler", "http://xml.org/sax/handlers/LexicalHandler"];
// Java field: OutputFormat DEFAULT_FORMAT
// DEFAULT_FORMAT: handled inline

// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler
pub struct XMLWriter {
    // Java field: boolean resolveEntityRefs
    pub(crate) resolveEntityRefs: bool,
    // Java field: NodeType lastOutputNodeType
    pub(crate) lastOutputNodeType: Option<super::super::NodeType::NodeType>,
    // Java field: boolean lastElementClosed
    lastElementClosed: bool,
    // Java field: boolean preserve
    pub(crate) preserve: bool,
    // Java field: Writer writer
    pub(crate) writer: Option<Box<dyn std::io::Write>>,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: Option<Box<dyn std::any::Any>>,
    // Java field: OutputFormat format
    format: Option<super::OutputFormat::OutputFormat>,
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
    pub(crate) lexicalHandler: Option<Box<dyn std::any::Any>>,
    // Java field: boolean showCommentsInDTDs
    showCommentsInDTDs: bool,
    // Java field: boolean inDTD
    inDTD: bool,
    // Java field: Map<String, String> namespacesMap
    namespacesMap: Option<HashMap<String, String>>,
    // Java field: int maximumAllowedCharacter
    pub(crate) maximumAllowedCharacter: i32,
}

// Java: public class XMLWriter extends XMLFilterImpl implements LexicalHandler
impl XMLWriter {
    // Java: XMLWriter(Writer writer)
    pub fn new() -> Self {
      // Java: this(writer, DEFAULT_FORMAT);
      todo!();
    }

    // Java: XMLWriter(Writer writer, OutputFormat format)
    pub fn new_1() -> Self {
      // Java: super();
      todo!();
      // Java: this.resolveEntityRefs = true;
      todo!();
      // Java: this.lastElementClosed = false;
      todo!();
      // Java: this.preserve = false;
      todo!();
      // Java: this.namespaceStack = new NamespaceStack();
      todo!();
      // Java: this.escapeText = true;
      todo!();
      // Java: this.indentLevel = 0;
      todo!();
      // Java: this.buffer = new StringBuffer();
      todo!();
      // Java: this.charsAdded = false;
      todo!();
      // Java: this.writer = writer;
      todo!();
      // Java: this.format = format;
      todo!();
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
      todo!();
    }

    // Java: XMLWriter()
    pub fn new_2() -> Self {
      // Java: super();
      todo!();
      // Java: this.resolveEntityRefs = true;
      todo!();
      // Java: this.lastElementClosed = false;
      todo!();
      // Java: this.preserve = false;
      todo!();
      // Java: this.namespaceStack = new NamespaceStack();
      todo!();
      // Java: this.escapeText = true;
      todo!();
      // Java: this.indentLevel = 0;
      todo!();
      // Java: this.buffer = new StringBuffer();
      todo!();
      // Java: this.charsAdded = false;
      todo!();
      // Java: this.format = DEFAULT_FORMAT;
      todo!();
      // Java: this.writer = new BufferedWriter(new OutputStreamWriter(System.out));
      todo!();
      // Java: this.autoFlush = true;
      todo!();
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
      todo!();
    }

    // Java: XMLWriter(OutputStream out)
    pub fn new_3() -> Self {
      // Java: super();
      todo!();
      // Java: this.resolveEntityRefs = true;
      todo!();
      // Java: this.lastElementClosed = false;
      todo!();
      // Java: this.preserve = false;
      todo!();
      // Java: this.namespaceStack = new NamespaceStack();
      todo!();
      // Java: this.escapeText = true;
      todo!();
      // Java: this.indentLevel = 0;
      todo!();
      // Java: this.buffer = new StringBuffer();
      todo!();
      // Java: this.charsAdded = false;
      todo!();
      // Java: this.format = DEFAULT_FORMAT;
      todo!();
      // Java: this.writer = this.createWriter(out, this.format.getEncoding());
      todo!();
      // Java: this.autoFlush = true;
      todo!();
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
      todo!();
    }

    // Java: XMLWriter(OutputStream out, OutputFormat format)
    pub fn new_4() -> Self {
      // Java: super();
      todo!();
      // Java: this.resolveEntityRefs = true;
      todo!();
      // Java: this.lastElementClosed = false;
      todo!();
      // Java: this.preserve = false;
      todo!();
      // Java: this.namespaceStack = new NamespaceStack();
      todo!();
      // Java: this.escapeText = true;
      todo!();
      // Java: this.indentLevel = 0;
      todo!();
      // Java: this.buffer = new StringBuffer();
      todo!();
      // Java: this.charsAdded = false;
      todo!();
      // Java: this.format = format;
      todo!();
      // Java: this.writer = this.createWriter(out, format.getEncoding());
      todo!();
      // Java: this.autoFlush = true;
      todo!();
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
      todo!();
    }

    // Java: XMLWriter(OutputFormat format)
    pub fn new_5() -> Self {
      // Java: super();
      todo!();
      // Java: this.resolveEntityRefs = true;
      todo!();
      // Java: this.lastElementClosed = false;
      todo!();
      // Java: this.preserve = false;
      todo!();
      // Java: this.namespaceStack = new NamespaceStack();
      todo!();
      // Java: this.escapeText = true;
      todo!();
      // Java: this.indentLevel = 0;
      todo!();
      // Java: this.buffer = new StringBuffer();
      todo!();
      // Java: this.charsAdded = false;
      todo!();
      // Java: this.format = format;
      todo!();
      // Java: this.writer = this.createWriter(System.out, format.getEncoding());
      todo!();
      // Java: this.autoFlush = true;
      todo!();
      // Java: this.namespaceStack.push(Namespace.NO_NAMESPACE);
      todo!();
    }

    // Java: void setWriter(Writer writer)
    pub fn setWriter(&mut self) {
      // Java: this.writer = writer;
      todo!();
      // Java: this.autoFlush = false;
      todo!();
    }

    // Java: void setOutputStream(OutputStream out)
    pub fn setOutputStream(&mut self) {
      // Java: this.writer = this.createWriter(out, this.format.getEncoding());
      todo!();
      // Java: this.autoFlush = true;
      todo!();
    }

    // Java: boolean isEscapeText()
    pub fn isEscapeText(&mut self) -> bool {
      // Java: return this.escapeText;
      todo!();
    }

    // Java: void setEscapeText(boolean escapeText)
    pub fn setEscapeText(&mut self) {
      // Java: this.escapeText = escapeText;
      todo!();
    }

    // Java: void setIndentLevel(int indentLevel)
    pub fn setIndentLevel(&mut self) {
      // Java: this.indentLevel = indentLevel;
      todo!();
    }

    // Java: int getMaximumAllowedCharacter()
    pub fn getMaximumAllowedCharacter(&mut self) -> i32 {
      // Java: if (this.maximumAllowedCharacter == 0)
      todo!();
      {
         // Java: this.maximumAllowedCharacter = this.defaultMaximumAllowedCharacter();
      todo!();
      }
      // Java: return this.maximumAllowedCharacter;
      todo!();
    }

    // Java: void setMaximumAllowedCharacter(int maximumAllowedCharacter)
    pub fn setMaximumAllowedCharacter(&mut self) {
      // Java: this.maximumAllowedCharacter = maximumAllowedCharacter;
      todo!();
    }

    // Java: void flush()
    pub fn flush(&mut self) {
      // Java: this.writer.flush();
      todo!();
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.writer.close();
      todo!();
    }

    // Java: void println()
    pub fn println(&mut self) {
      // Java: this.writer.write(this.format.getLineSeparator());
      todo!();
    }

    // Java: void write(Attribute attribute)
    pub fn write(&mut self) {
      // Java: this.writeAttribute(attribute);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Document doc)
    pub fn write_document(&mut self) {
      // Java: this.writeDeclaration();
      todo!();
      // Java: int i = 0;
      todo!();
      // Java: for(int size = doc.nodeCount(); i < size; ++i)
      todo!();
      {
         // Java: Node node = doc.node(i);
      todo!();
         // Java: this.writeNode(node);
      todo!();
      }
      // Java: this.writePrintln();
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Element element)
    pub fn write_element(&mut self) {
      // Java: this.writeElement(element);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(CDATA cdata)
    pub fn write_cdata(&mut self) {
      // Java: this.writeCDATA(cdata.getText());
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Comment comment)
    pub fn write_comment(&mut self) {
      // Java: this.writeComment(comment.getText());
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Entity entity)
    pub fn write_entity(&mut self) {
      // Java: this.writeEntity(entity);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Namespace namespace)
    pub fn write_namespace(&mut self) {
      // Java: this.writeNamespace(namespace);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(ProcessingInstruction processingInstruction)
    pub fn write_pi(&mut self) {
      // Java: this.writeProcessingInstruction(processingInstruction);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(String text)
    pub fn write_string(&mut self) {
      // Java: this.writeString(text);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Text text)
    pub fn write_text(&mut self) {
      // Java: this.writeString(text.getText());
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Node node)
    pub fn write_node(&mut self) {
      // Java: this.writeNode(node);
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: this.flush();
      todo!();
      }
    }

    // Java: void write(Object object)
    pub fn write_object(&mut self) {
      // Java: if (object instanceof Node)
      todo!();
      {
         // Java: this.write((Node)object);
      todo!();
      }
      // Java: else if (object instanceof String)
      todo!();
      {
         // Java: this.write((String)object);
      todo!();
      }
      // Java: else if (object instanceof List)
      todo!();
      {
         // Java: List list = (List)object;
      todo!();
         // Java: int i = 0;
      todo!();
         // Java: for(int size = list.size(); i < size; ++i)
      todo!();
         {
            // Java: this.write(list.get(i));
      todo!();
         }
      }
      // Java: else if (object != null)
      todo!();
      {
         // Java: throw new IOException("Invalid object: " + object);
      todo!();
      }
    }

    // Java: void writeOpen(Element element)
    pub fn writeOpen(&mut self) {
      // Java: this.writer.write("<");
      todo!();
      // Java: this.writer.write(element.getQualifiedName());
      todo!();
      // Java: this.writeAttributes(element);
      todo!();
      // Java: this.writer.write(">");
      todo!();
    }

    // Java: void writeClose(Element element)
    pub fn writeClose(&mut self) {
      // Java: this.writeClose(element.getQualifiedName());
      todo!();
    }

    // Java: void parse(InputSource source)
    pub fn parse(&mut self) {
      // Java: this.installLexicalHandler();
      todo!();
      // Java: super.parse(source);
      todo!();
    }

    // Java: void setProperty(String name, Object value)
    pub fn setProperty(&mut self) {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      todo!();
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
      todo!();
         {
            // Java: this.setLexicalHandler((LexicalHandler)value);
      todo!();
            // Java: return;
      todo!();
         }
      }
      // Java: super.setProperty(name, value);
      todo!();
    }

    // Java: Object getProperty(String name)
    pub fn getProperty(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      todo!();
      {
         // Java: if (LEXICAL_HANDLER_NAMES[i].equals(name))
      todo!();
         {
            // Java: return this.getLexicalHandler();
      todo!();
         }
      }
      // Java: return super.getProperty(name);
      todo!();
    }

    // Java: void setLexicalHandler(LexicalHandler handler)
    pub fn setLexicalHandler(&mut self) {
      // Java: if (handler == null)
      todo!();
      {
         // Java: throw new NullPointerException("Null lexical handler");
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.lexicalHandler = handler;
      todo!();
      }
    }

    // Java: LexicalHandler getLexicalHandler()
    pub fn getLexicalHandler(&mut self) -> Option<&dyn std::any::Any> {
      // Java: return this.lexicalHandler;
      todo!();
    }

    // Java: void setDocumentLocator(Locator locator)
    pub fn setDocumentLocator(&mut self) {
      // Java: super.setDocumentLocator(locator);
      todo!();
    }

    // Java: void startDocument()
    pub fn startDocument(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.writeDeclaration();
      todo!();
         // Java: super.startDocument();
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
    }

    // Java: void endDocument()
    pub fn endDocument(&mut self) {
      // Java: super.endDocument();
      todo!();
      // Java: if (this.autoFlush)
      todo!();
      {
         // Java: try
      todo!();
         {
            // Java: this.flush();
      todo!();
         }
         // Java: catch (IOException var2)
      todo!();
         {
         }
      }
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&mut self) {
      // Java: if (this.namespacesMap == null)
      todo!();
      {
         // Java: this.namespacesMap = new HashMap();
      todo!();
      }
      // Java: this.namespacesMap.put(prefix, uri);
      todo!();
      // Java: super.startPrefixMapping(prefix, uri);
      todo!();
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&mut self) {
      // Java: super.endPrefixMapping(prefix);
      todo!();
    }

    // Java: void startElement(String namespaceURI, String localName, String qName, Attributes attributes)
    pub fn startElement(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.charsAdded = false;
      todo!();
         // Java: this.writePrintln();
      todo!();
         // Java: this.indent();
      todo!();
         // Java: this.writer.write("<");
      todo!();
         // Java: this.writer.write(qName);
      todo!();
         // Java: this.writeNamespaces();
      todo!();
         // Java: this.writeAttributes(attributes);
      todo!();
         // Java: this.writer.write(">");
      todo!();
         // Java: ++this.indentLevel;
      todo!();
         // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
      todo!();
         // Java: this.lastElementClosed = false;
      todo!();
         // Java: super.startElement(namespaceURI, localName, qName, attributes);
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.charsAdded = false;
      todo!();
         // Java: --this.indentLevel;
      todo!();
         // Java: if (this.lastElementClosed)
      todo!();
         {
            // Java: this.writePrintln();
      todo!();
            // Java: this.indent();
      todo!();
         }
         // Java: boolean hadContent = true;
      todo!();
         // Java: if (hadContent)
      todo!();
         {
            // Java: this.writeClose(qName);
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: this.writeEmptyElementClose(qName);
      todo!();
         }
         // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
      todo!();
         // Java: this.lastElementClosed = true;
      todo!();
         // Java: super.endElement(namespaceURI, localName, qName);
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
    }

    // Java: void characters(char[] ch, int start, int length)
    pub fn characters(&mut self) {
      // Java: if (ch != null && ch.length != 0 && length > 0)
      todo!();
      {
         // Java: try
      todo!();
         {
            // Java: String string = String.valueOf(ch, start, length);
      todo!();
            // Java: if (this.escapeText)
      todo!();
            {
               // Java: string = this.escapeElementEntities(string);
      todo!();
            }
            // Java: if (!this.format.isTrimText())
      todo!();
            {
               // Java: this.writer.write(string);
      todo!();
            }
            // Java: else
      todo!();
            {
               // Java: if (this.lastOutputNodeType == NodeType.TEXT_NODE && !this.charsAdded)
      todo!();
               {
                  // Java: this.writer.write(32);
      todo!();
               }
               // Java: else if (this.charsAdded && Character.isWhitespace(this.lastChar))
      todo!();
               {
                  // Java: this.writer.write(32);
      todo!();
               }
               // Java: else if (this.lastOutputNodeType == NodeType.ELEMENT_NODE && this.format.isPadText() && this.lastElementClosed && Character.isWhitespace(ch[0]))
      todo!();
               {
                  // Java: this.writer.write(" ");
      todo!();
               }
               // Java: String delim = "";
      todo!();
               // Java: for(StringTokenizer tokens = new StringTokenizer(string); tokens.hasMoreTokens(); delim = " ")
      todo!();
               {
                  // Java: this.writer.write(delim);
      todo!();
                  // Java: this.writer.write(tokens.nextToken());
      todo!();
               }
            }
            // Java: this.charsAdded = true;
      todo!();
            // Java: this.lastChar = ch[start + length - 1];
      todo!();
            // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
      todo!();
            // Java: super.characters(ch, start, length);
      todo!();
         }
         // Java: catch (IOException e)
      todo!();
         {
            // Java: this.handleException(e);
      todo!();
         }
      }
    }

    // Java: void ignorableWhitespace(char[] ch, int start, int length)
    pub fn ignorableWhitespace(&mut self) {
      // Java: super.ignorableWhitespace(ch, start, length);
      todo!();
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.indent();
      todo!();
         // Java: this.writer.write("<?");
      todo!();
         // Java: this.writer.write(target);
      todo!();
         // Java: this.writer.write(" ");
      todo!();
         // Java: this.writer.write(data);
      todo!();
         // Java: this.writer.write("?>");
      todo!();
         // Java: this.writePrintln();
      todo!();
         // Java: this.lastOutputNodeType = NodeType.PROCESSING_INSTRUCTION_NODE;
      todo!();
         // Java: super.processingInstruction(target, data);
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
    }

    // Java: void notationDecl(String name, String publicID, String systemID)
    pub fn notationDecl(&mut self) {
      // Java: super.notationDecl(name, publicID, systemID);
      todo!();
    }

    // Java: void unparsedEntityDecl(String name, String publicID, String systemID, String notationName)
    pub fn unparsedEntityDecl(&mut self) {
      // Java: super.unparsedEntityDecl(name, publicID, systemID, notationName);
      todo!();
    }

    // Java: void startDTD(String name, String publicID, String systemID)
    pub fn startDTD(&mut self) {
      // Java: this.inDTD = true;
      todo!();
      // Java: try
      todo!();
      {
         // Java: this.writeDocType(name, publicID, systemID);
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.startDTD(name, publicID, systemID);
      todo!();
      }
    }

    // Java: void endDTD()
    pub fn endDTD(&mut self) {
      // Java: this.inDTD = false;
      todo!();
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.endDTD();
      todo!();
      }
    }

    // Java: void startCDATA()
    pub fn startCDATA(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.writer.write("<![CDATA[");
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.startCDATA();
      todo!();
      }
    }

    // Java: void endCDATA()
    pub fn endCDATA(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.writer.write("]]>");
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.endCDATA();
      todo!();
      }
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&mut self) {
      // Java: try
      todo!();
      {
         // Java: this.writeEntityRef(name);
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: this.handleException(e);
      todo!();
      }
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.startEntity(name);
      todo!();
      }
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&mut self) {
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.endEntity(name);
      todo!();
      }
    }

    // Java: void comment(char[] ch, int start, int length)
    pub fn comment(&mut self) {
      // Java: if (this.showCommentsInDTDs || !this.inDTD)
      todo!();
      {
         // Java: try
      todo!();
         {
            // Java: this.charsAdded = false;
      todo!();
            // Java: this.writeComment(new String(ch, start, length));
      todo!();
         }
         // Java: catch (IOException e)
      todo!();
         {
            // Java: this.handleException(e);
      todo!();
         }
      }
      // Java: if (this.lexicalHandler != null)
      todo!();
      {
         // Java: this.lexicalHandler.comment(ch, start, length);
      todo!();
      }
    }

    // Java: void writeElement(Element element)
    pub fn writeElement(&mut self) {
      // Java: int size = element.nodeCount();
      todo!();
      // Java: String qualifiedName = element.getQualifiedName();
      todo!();
      // Java: this.writePrintln();
      todo!();
      // Java: this.indent();
      todo!();
      // Java: this.writer.write("<");
      todo!();
      // Java: this.writer.write(qualifiedName);
      todo!();
      // Java: int previouslyDeclaredNamespaces = this.namespaceStack.size();
      todo!();
      // Java: Namespace ns = new Namespace(element.getNamespacePrefix(), element.getNamespaceURI());
      todo!();
      // Java: if (this.isNamespaceDeclaration(ns))
      todo!();
      {
         // Java: this.namespaceStack.push(ns);
      todo!();
         // Java: this.writeNamespace(ns);
      todo!();
      }
      // Java: boolean textOnly = true;
      todo!();
      // Java: for(int i = 0; i < size; ++i)
      todo!();
      {
         // Java: Node node = element.node(i);
      todo!();
         // Java: if (node instanceof Namespace)
      todo!();
         {
            // Java: Namespace additional = (Namespace)node;
      todo!();
            // Java: if (this.isNamespaceDeclaration(additional))
      todo!();
            {
               // Java: this.namespaceStack.push(additional);
      todo!();
               // Java: this.writeNamespace(additional);
      todo!();
            }
         }
         // Java: else if (node instanceof Element)
      todo!();
         {
            // Java: textOnly = false;
      todo!();
         }
         // Java: else if (node instanceof Comment)
      todo!();
         {
            // Java: textOnly = false;
      todo!();
         }
      }
      // Java: this.writeAttributes(element);
      todo!();
      // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
      todo!();
      // Java: if (size <= 0)
      todo!();
      {
         // Java: this.writeEmptyElementClose(qualifiedName);
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.writer.write(">");
      todo!();
         // Java: if (textOnly)
      todo!();
         {
            // Java: this.writeElementContent(element);
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: ++this.indentLevel;
      todo!();
            // Java: this.writeElementContent(element);
      todo!();
            // Java: --this.indentLevel;
      todo!();
            // Java: this.writePrintln();
      todo!();
            // Java: this.indent();
      todo!();
         }
         // Java: this.writer.write("</");
      todo!();
         // Java: this.writer.write(qualifiedName);
      todo!();
         // Java: this.writer.write(">");
      todo!();
      }
      // Java: while(this.namespaceStack.size() > previouslyDeclaredNamespaces)
      todo!();
      {
         // Java: this.namespaceStack.pop();
      todo!();
      }
      // Java: this.lastOutputNodeType = NodeType.ELEMENT_NODE;
      todo!();
    }

    // Java: boolean isElementSpacePreserved(Element element)
    pub fn isElementSpacePreserved(&mut self) -> bool {
      // Java: Attribute attr = element.attribute("space");
      todo!();
      // Java: boolean preserveFound = this.preserve;
      todo!();
      // Java: if (attr != null)
      todo!();
      {
         // Java: if ("xml".equals(attr.getNamespacePrefix()) && "preserve".equals(attr.getText()))
      todo!();
         {
            // Java: preserveFound = true;
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: preserveFound = false;
      todo!();
         }
      }
      // Java: return preserveFound;
      todo!();
    }

    // Java: void writeElementContent(Element element)
    pub fn writeElementContent(&mut self) {
      // Java: boolean trim = this.format.isTrimText();
      todo!();
      // Java: boolean oldPreserve = this.preserve;
      todo!();
      // Java: if (trim)
      todo!();
      {
         // Java: this.preserve = this.isElementSpacePreserved(element);
      todo!();
         // Java: trim = !this.preserve;
      todo!();
      }
      // Java: if (trim)
      todo!();
      {
         // Java: Text lastTextNode = null;
      todo!();
         // Java: StringBuffer buff = null;
      todo!();
         // Java: boolean textOnly = true;
      todo!();
         // Java: int i = 0;
      todo!();
         // Java: for(int size = element.nodeCount(); i < size; ++i)
      todo!();
         {
            // Java: Node node = element.node(i);
      todo!();
            // Java: if (node instanceof Text)
      todo!();
            {
               // Java: if (lastTextNode == null)
      todo!();
               {
                  // Java: lastTextNode = (Text)node;
      todo!();
               }
               // Java: else
      todo!();
               {
                  // Java: if (buff == null)
      todo!();
                  {
                     // Java: buff = new StringBuffer(lastTextNode.getText());
      todo!();
                  }
                  // Java: buff.append(((Text)node).getText());
      todo!();
               }
            }
            // Java: else
      todo!();
            {
               // Java: if (!textOnly && this.format.isPadText())
      todo!();
               {
                  // Java: char firstChar = 'a';
      todo!();
                  // Java: if (buff != null)
      todo!();
                  {
                     // Java: firstChar = buff.charAt(0);
      todo!();
                  }
                  // Java: else if (lastTextNode != null)
      todo!();
                  {
                     // Java: firstChar = lastTextNode.getText().charAt(0);
      todo!();
                  }
                  // Java: if (Character.isWhitespace(firstChar))
      todo!();
                  {
                     // Java: this.writer.write(" ");
      todo!();
                  }
               }
               // Java: if (lastTextNode != null)
      todo!();
               {
                  // Java: if (buff != null)
      todo!();
                  {
                     // Java: this.writeString(buff.toString());
      todo!();
                     // Java: buff = null;
      todo!();
                  }
                  // Java: else
      todo!();
                  {
                     // Java: this.writeString(lastTextNode.getText());
      todo!();
                  }
                  // Java: if (this.format.isPadText())
      todo!();
                  {
                     // Java: char lastTextChar = 'a';
      todo!();
                     // Java: if (buff != null)
      todo!();
                     {
                        // Java: lastTextChar = buff.charAt(buff.length() - 1);
      todo!();
                     }
                     // Java: else if (lastTextNode != null)
      todo!();
                     {
                        // Java: String txt = lastTextNode.getText();
      todo!();
                        // Java: lastTextChar = txt.charAt(txt.length() - 1);
      todo!();
                     }
                     // Java: if (Character.isWhitespace(lastTextChar))
      todo!();
                     {
                        // Java: this.writer.write(" ");
      todo!();
                     }
                  }
                  // Java: lastTextNode = null;
      todo!();
               }
               // Java: textOnly = false;
      todo!();
               // Java: this.writeNode(node);
      todo!();
            }
         }
         // Java: if (lastTextNode != null)
      todo!();
         {
            // Java: if (!textOnly && this.format.isPadText())
      todo!();
            {
               // Java: i = 97;
      todo!();
               // Java: if (buff != null)
      todo!();
               {
                  // Java: i = buff.charAt(0);
      todo!();
               }
               // Java: else
      todo!();
               {
                  // Java: i = lastTextNode.getText().charAt(0);
      todo!();
               }
               // Java: if (Character.isWhitespace((char)i))
      todo!();
               {
                  // Java: this.writer.write(" ");
      todo!();
               }
            }
            // Java: if (buff != null)
      todo!();
            {
               // Java: this.writeString(buff.toString());
      todo!();
               // Java: StringBuffer var14 = null;
      todo!();
            }
            // Java: else
      todo!();
            {
               // Java: this.writeString(lastTextNode.getText());
      todo!();
            }
            // Java: Object var12 = null;
      todo!();
         }
      }
      // Java: else
      todo!();
      {
         // Java: Node lastTextNode = null;
      todo!();
         // Java: int i = 0;
      todo!();
         // Java: for(int size = element.nodeCount(); i < size; ++i)
      todo!();
         {
            // Java: Node node = element.node(i);
      todo!();
            // Java: if (node instanceof Text)
      todo!();
            {
               // Java: this.writeNode(node);
      todo!();
               // Java: lastTextNode = node;
      todo!();
            }
            // Java: else
      todo!();
            {
               // Java: if (lastTextNode != null && this.format.isPadText())
      todo!();
               {
                  // Java: String txt = lastTextNode.getText();
      todo!();
                  // Java: char lastTextChar = txt.charAt(txt.length() - 1);
      todo!();
                  // Java: if (Character.isWhitespace(lastTextChar))
      todo!();
                  {
                     // Java: this.writer.write(" ");
      todo!();
                  }
               }
               // Java: this.writeNode(node);
      todo!();
               // Java: lastTextNode = null;
      todo!();
            }
         }
      }
      // Java: this.preserve = oldPreserve;
      todo!();
    }

    // Java: void writeCDATA(String text)
    pub fn writeCDATA(&mut self) {
      // Java: this.writer.write("<![CDATA[");
      todo!();
      // Java: if (text != null)
      todo!();
      {
         // Java: this.writer.write(text);
      todo!();
      }
      // Java: this.writer.write("]]>");
      todo!();
      // Java: this.lastOutputNodeType = NodeType.CDATA_SECTION_NODE;
      todo!();
    }

    // Java: void writeNamespace(Namespace namespace)
    pub fn writeNamespace(&mut self) {
      // Java: if (namespace != null)
      todo!();
      {
         // Java: this.writeNamespace(namespace.getPrefix(), namespace.getURI());
      todo!();
      }
    }

    // Java: void writeNamespaces()
    pub fn writeNamespaces(&mut self) {
      // Java: if (this.namespacesMap != null)
      todo!();
      {
         // Java: for(Map.Entry entry : this.namespacesMap.entrySet())
      todo!();
         {
            // Java: String prefix = (String)entry.getKey();
      todo!();
            // Java: String uri = (String)entry.getValue();
      todo!();
            // Java: this.writeNamespace(prefix, uri);
      todo!();
         }
         // Java: this.namespacesMap = null;
      todo!();
      }
    }

    // Java: void writeNamespace(String prefix, String uri)
    pub fn writeNamespace_prefix_uri(&mut self) {
      // Java: if (prefix != null && prefix.length() > 0)
      todo!();
      {
         // Java: this.writer.write(" xmlns:");
      todo!();
         // Java: this.writer.write(prefix);
      todo!();
         // Java: this.writer.write("=\"");
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.writer.write(" xmlns=\"");
      todo!();
      }
      // Java: this.writer.write(uri);
      todo!();
      // Java: this.writer.write("\"");
      todo!();
    }

    // Java: void writeProcessingInstruction(ProcessingInstruction pi)
    pub fn writeProcessingInstruction(&mut self) {
      // Java: this.writer.write("<?");
      todo!();
      // Java: this.writer.write(pi.getName());
      todo!();
      // Java: this.writer.write(" ");
      todo!();
      // Java: this.writer.write(pi.getText());
      todo!();
      // Java: this.writer.write("?>");
      todo!();
      // Java: this.writePrintln();
      todo!();
      // Java: this.lastOutputNodeType = NodeType.PROCESSING_INSTRUCTION_NODE;
      todo!();
    }

    // Java: void writeString(String text)
    pub fn writeString(&mut self) {
      // Java: if (text != null && text.length() > 0)
      todo!();
      {
         // Java: if (this.escapeText)
      todo!();
         {
            // Java: text = this.escapeElementEntities(text);
      todo!();
         }
         // Java: if (this.format.isTrimText())
      todo!();
         {
            // Java: boolean first = true;
      todo!();
            // Java: for(StringTokenizer tokenizer = new StringTokenizer(text); tokenizer.hasMoreTokens(); this.lastOutputNodeType = NodeType.TEXT_NODE)
      todo!();
            {
               // Java: String token = tokenizer.nextToken();
      todo!();
               // Java: if (first)
      todo!();
               {
                  // Java: first = false;
      todo!();
                  // Java: if (this.lastOutputNodeType == NodeType.TEXT_NODE)
      todo!();
                  {
                     // Java: this.writer.write(" ");
      todo!();
                  }
               }
               // Java: else
      todo!();
               {
                  // Java: this.writer.write(" ");
      todo!();
               }
               // Java: this.writer.write(token);
      todo!();
            }
         }
         // Java: else
      todo!();
         {
            // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
      todo!();
            // Java: this.writer.write(text);
      todo!();
         }
      }
    }

    // Java: void writeNodeText(Node node)
    pub fn writeNodeText(&mut self) {
      // Java: String text = node.getText();
      todo!();
      // Java: if (text != null && text.length() > 0)
      todo!();
      {
         // Java: if (this.escapeText)
      todo!();
         {
            // Java: text = this.escapeElementEntities(text);
      todo!();
         }
         // Java: this.lastOutputNodeType = NodeType.TEXT_NODE;
      todo!();
         // Java: this.writer.write(text);
      todo!();
      }
    }

    // Java: void writeNode(Node node)
    pub fn writeNode(&mut self) {
      // Java: NodeType nodeType = node.getNodeTypeEnum();
      todo!();
      // Java: switch (nodeType)
      todo!();
      {
         // Java: case ELEMENT_NODE:
      todo!();
            // Java: this.writeElement((Element)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case ATTRIBUTE_NODE:
      todo!();
            // Java: this.writeAttribute((Attribute)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case TEXT_NODE:
      todo!();
            // Java: this.writeNodeText(node);
      todo!();
            // Java: break;
      todo!();
         // Java: case CDATA_SECTION_NODE:
      todo!();
            // Java: this.writeCDATA(node.getText());
      todo!();
            // Java: break;
      todo!();
         // Java: case ENTITY_REFERENCE_NODE:
      todo!();
            // Java: this.writeEntity((Entity)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case PROCESSING_INSTRUCTION_NODE:
      todo!();
            // Java: this.writeProcessingInstruction((ProcessingInstruction)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case COMMENT_NODE:
      todo!();
            // Java: this.writeComment(node.getText());
      todo!();
            // Java: break;
      todo!();
         // Java: case DOCUMENT_NODE:
      todo!();
            // Java: this.write((Document)node);
      todo!();
         // Java: case NAMESPACE_NODE:
      todo!();
            // Java: break;
      todo!();
         // Java: default:
      todo!();
            // Java: throw new IOException("Invalid node type: " + node);
      todo!();
      }
    }

    // Java: void installLexicalHandler()
    pub fn installLexicalHandler(&mut self) {
      // Java: XMLReader parent = this.getParent();
      todo!();
      // Java: if (parent == null)
      todo!();
      {
         // Java: throw new NullPointerException("No parent for filter");
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: for(int i = 0; i < LEXICAL_HANDLER_NAMES.length; ++i)
      todo!();
         {
            // Java: try
      todo!();
            {
               // Java: parent.setProperty(LEXICAL_HANDLER_NAMES[i], this);
      todo!();
               // Java: break;
      todo!();
            }
            // Java: catch (SAXNotRecognizedException var4)
      todo!();
            {
            }
            // Java: catch (SAXNotSupportedException var5)
      todo!();
            {
            }
         }
      }
    }

    // Java: void writeDocType(String name, String publicID, String systemID)
    pub fn writeDocType(&mut self) {
      // Java: boolean hasPublic = false;
      todo!();
      // Java: this.writer.write("<!DOCTYPE ");
      todo!();
      // Java: this.writer.write(name);
      todo!();
      // Java: if (publicID != null && !publicID.equals(""))
      todo!();
      {
         // Java: this.writer.write(" PUBLIC \"");
      todo!();
         // Java: this.writer.write(publicID);
      todo!();
         // Java: this.writer.write("\"");
      todo!();
         // Java: hasPublic = true;
      todo!();
      }
      // Java: if (systemID != null && !systemID.equals(""))
      todo!();
      {
         // Java: if (!hasPublic)
      todo!();
         {
            // Java: this.writer.write(" SYSTEM");
      todo!();
         }
         // Java: this.writer.write(" \"");
      todo!();
         // Java: this.writer.write(systemID);
      todo!();
         // Java: this.writer.write("\"");
      todo!();
      }
      // Java: this.writer.write(">");
      todo!();
      // Java: this.writePrintln();
      todo!();
    }

    // Java: void writeEntity(Entity entity)
    pub fn writeEntity(&mut self) {
      // Java: if (!this.resolveEntityRefs())
      todo!();
      {
         // Java: this.writeEntityRef(entity.getName());
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.writer.write(entity.getText());
      todo!();
      }
    }

    // Java: void writeEntityRef(String name)
    pub fn writeEntityRef(&mut self) {
      // Java: this.writer.write("&");
      todo!();
      // Java: this.writer.write(name);
      todo!();
      // Java: this.writer.write(";");
      todo!();
      // Java: this.lastOutputNodeType = NodeType.ENTITY_REFERENCE_NODE;
      todo!();
    }

    // Java: void writeComment(String text)
    pub fn writeComment(&mut self) {
      // Java: if (this.format.isNewlines())
      todo!();
      {
         // Java: this.println();
      todo!();
         // Java: this.indent();
      todo!();
      }
      // Java: this.writer.write("<!--");
      todo!();
      // Java: this.writer.write(text);
      todo!();
      // Java: this.writer.write("-->");
      todo!();
      // Java: this.lastOutputNodeType = NodeType.COMMENT_NODE;
      todo!();
    }

    // Java: void writeAttributes(Element element)
    pub fn writeAttributes(&mut self) {
      // Java: int i = 0;
      todo!();
      // Java: for(int size = element.attributeCount(); i < size; ++i)
      todo!();
      {
         // Java: Attribute attribute = element.attribute(i);
      todo!();
         // Java: Namespace ns = new Namespace(attribute.getNamespacePrefix(), attribute.getNamespaceURI());
      todo!();
         // Java: if (ns != null && !ns.equals(Namespace.NO_NAMESPACE) && ns.equals(Namespace.XML_NAMESPACE))
      todo!();
         {
            // Java: String prefix = ns.getPrefix();
      todo!();
            // Java: String uri = this.namespaceStack.getURI(prefix);
      todo!();
            // Java: if (!ns.getURI().equals(uri))
      todo!();
            {
               // Java: this.writeNamespace(ns);
      todo!();
               // Java: this.namespaceStack.push(ns);
      todo!();
            }
         }
         // Java: String attName = attribute.getName();
      todo!();
         // Java: if (attName.startsWith("xmlns:"))
      todo!();
         {
            // Java: String prefix = attName.substring(6);
      todo!();
            // Java: if (this.namespaceStack.getNamespaceForPrefix(prefix) == null)
      todo!();
            {
               // Java: String uri = attribute.getValue();
      todo!();
               // Java: this.namespaceStack.push(prefix, uri);
      todo!();
               // Java: this.writeNamespace(prefix, uri);
      todo!();
            }
         }
         // Java: else if (attName.equals("xmlns"))
      todo!();
         {
            // Java: if (this.namespaceStack.getDefaultNamespace() == null)
      todo!();
            {
               // Java: String uri = attribute.getValue();
      todo!();
               // Java: this.namespaceStack.push((String)null, uri);
      todo!();
               // Java: this.writeNamespace((String)null, uri);
      todo!();
            }
         }
         // Java: else
      todo!();
         {
            // Java: char quote = this.format.getAttributeQuoteCharacter();
      todo!();
            // Java: this.writer.write(" ");
      todo!();
            // Java: this.writer.write(attribute.getQualifiedName());
      todo!();
            // Java: this.writer.write("=");
      todo!();
            // Java: this.writer.write(quote);
      todo!();
            // Java: this.writeEscapeAttributeEntities(attribute.getValue());
      todo!();
            // Java: this.writer.write(quote);
      todo!();
         }
      }
    }

    // Java: void writeAttribute(Attribute attribute)
    pub fn writeAttribute(&mut self) {
      // Java: this.writer.write(" ");
      todo!();
      // Java: this.writer.write(attribute.getQualifiedName());
      todo!();
      // Java: this.writer.write("=");
      todo!();
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      todo!();
      // Java: this.writer.write(quote);
      todo!();
      // Java: this.writeEscapeAttributeEntities(attribute.getValue());
      todo!();
      // Java: this.writer.write(quote);
      todo!();
      // Java: this.lastOutputNodeType = NodeType.ATTRIBUTE_NODE;
      todo!();
    }

    // Java: void writeAttributes(Attributes attributes)
    pub fn writeAttributes_sax(&mut self) {
      // Java: int i = 0;
      todo!();
      // Java: for(int size = attributes.getLength(); i < size; ++i)
      todo!();
      {
         // Java: this.writeAttribute(attributes, i);
      todo!();
      }
    }

    // Java: void writeAttribute(Attributes attributes, int index)
    pub fn writeAttribute_sax_index(&mut self) {
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      todo!();
      // Java: this.writer.write(" ");
      todo!();
      // Java: this.writer.write(attributes.getQName(index));
      todo!();
      // Java: this.writer.write("=");
      todo!();
      // Java: this.writer.write(quote);
      todo!();
      // Java: this.writeEscapeAttributeEntities(attributes.getValue(index));
      todo!();
      // Java: this.writer.write(quote);
      todo!();
    }

    // Java: void indent()
    pub fn indent(&mut self) {
      // Java: String indent = this.format.getIndent();
      todo!();
      // Java: if (indent != null && indent.length() > 0)
      todo!();
      {
         // Java: for(int i = 0; i < this.indentLevel; ++i)
      todo!();
         {
            // Java: this.writer.write(indent);
      todo!();
         }
      }
    }

    // Java: void writePrintln()
    pub fn writePrintln(&mut self) {
      // Java: if (this.format.isNewlines())
      todo!();
      {
         // Java: this.writer.write(this.format.getLineSeparator());
      todo!();
      }
    }

    // Java: Writer createWriter(OutputStream outStream, String encoding)
    pub fn createWriter(&mut self) -> Box<dyn std::io::Write> {
      // Java: return new BufferedWriter(new OutputStreamWriter(outStream, encoding));
      todo!();
    }

    // Java: void writeDeclaration()
    pub fn writeDeclaration(&mut self) {
      // Java: String encoding = this.format.getEncoding();
      todo!();
      // Java: if (!this.format.isSuppressDeclaration())
      todo!();
      {
         // Java: if (encoding.equals("UTF8"))
      todo!();
         {
            // Java: this.writer.write("<?xml version=\"1.0\"");
      todo!();
            // Java: if (!this.format.isOmitEncoding())
      todo!();
            {
               // Java: this.writer.write(" encoding=\"UTF-8\"");
      todo!();
            }
            // Java: this.writer.write("?>");
      todo!();
         }
         // Java: else
      todo!();
         {
            // Java: this.writer.write("<?xml version=\"1.0\"");
      todo!();
            // Java: if (!this.format.isOmitEncoding())
      todo!();
            {
               // Java: this.writer.write(" encoding=\"" + encoding + "\"");
      todo!();
            }
            // Java: this.writer.write("?>");
      todo!();
         }
         // Java: if (this.format.isNewLineAfterDeclaration())
      todo!();
         {
            // Java: this.println();
      todo!();
         }
      }
    }

    // Java: void writeClose(String qualifiedName)
    pub fn writeClose_qname(&mut self) {
      // Java: this.writer.write("</");
      todo!();
      // Java: this.writer.write(qualifiedName);
      todo!();
      // Java: this.writer.write(">");
      todo!();
    }

    // Java: void writeEmptyElementClose(String qualifiedName)
    pub fn writeEmptyElementClose(&mut self) {
      // Java: if (!this.format.isExpandEmptyElements())
      todo!();
      {
         // Java: this.writer.write("/>");
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: this.writer.write("></");
      todo!();
         // Java: this.writer.write(qualifiedName);
      todo!();
         // Java: this.writer.write(">");
      todo!();
      }
    }

    // Java: boolean isExpandEmptyElements()
    pub fn isExpandEmptyElements(&mut self) -> bool {
      // Java: return this.format.isExpandEmptyElements();
      todo!();
    }

    // Java: String escapeElementEntities(String textSrc)
    pub fn escapeElementEntities(&mut self) -> String {
      // Java: char[] block = null;
      todo!();
      // Java: int last = 0;
      todo!();
      // Java: String text = textSrc.replaceAll("[\\x00-\\x08\\x0b-\\x0c\\x0e-\\x1f]", "");
      todo!();
      // Java: int size = text.length();
      todo!();
      // Java: int i;
      todo!();
      // Java: for(i = 0; i < size; ++i)
      todo!();
      {
         // Java: String entity = null;
      todo!();
         // Java: char c = text.charAt(i);
      todo!();
         // Java: switch (c)
      todo!();
         {
            // Java: case '\t':
      todo!();
            // Java: case '\n':
      todo!();
            // Java: case '\r':
      todo!();
               // Java: if (this.preserve)
      todo!();
               {
                  // Java: entity = String.valueOf(c);
      todo!();
               }
               // Java: break;
      todo!();
            // Java: case '&':
      todo!();
               // Java: entity = "&amp;";
      todo!();
               // Java: break;
      todo!();
            // Java: case '<':
      todo!();
               // Java: entity = "&lt;";
      todo!();
               // Java: break;
      todo!();
            // Java: case '>':
      todo!();
               // Java: entity = "&gt;";
      todo!();
               // Java: break;
      todo!();
            // Java: default:
      todo!();
               // Java: if (c < ' ' || this.shouldEncodeChar(c))
      todo!();
               {
                  // Java: entity = "&#" + c + ";";
      todo!();
               }
         }
         // Java: if (entity != null)
      todo!();
         {
            // Java: if (block == null)
      todo!();
            {
               // Java: block = text.toCharArray();
      todo!();
            }
            // Java: this.buffer.append(block, last, i - last);
      todo!();
            // Java: this.buffer.append(entity);
      todo!();
            // Java: last = i + 1;
      todo!();
         }
      }
      // Java: if (last == 0)
      todo!();
      {
         // Java: return text;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: if (last < size)
      todo!();
         {
            // Java: if (block == null)
      todo!();
            {
               // Java: block = text.toCharArray();
      todo!();
            }
            // Java: this.buffer.append(block, last, i - last);
      todo!();
         }
         // Java: String answer = this.buffer.toString();
      todo!();
         // Java: this.buffer.setLength(0);
      todo!();
         // Java: return answer;
      todo!();
      }
    }

    // Java: void writeEscapeAttributeEntities(String txt)
    pub fn writeEscapeAttributeEntities(&mut self) {
      // Java: if (txt != null)
      todo!();
      {
         // Java: String escapedText = this.escapeAttributeEntities(txt);
      todo!();
         // Java: this.writer.write(escapedText);
      todo!();
      }
    }

    // Java: String escapeAttributeEntities(String text)
    pub fn escapeAttributeEntities(&mut self) -> String {
      // Java: char quote = this.format.getAttributeQuoteCharacter();
      todo!();
      // Java: char[] block = null;
      todo!();
      // Java: int last = 0;
      todo!();
      // Java: int size = text.length();
      todo!();
      // Java: int i;
      todo!();
      // Java: for(i = 0; i < size; ++i)
      todo!();
      {
         // Java: String entity = null;
      todo!();
         // Java: char c = text.charAt(i);
      todo!();
         // Java: switch (c)
      todo!();
         {
            // Java: case '\t':
      todo!();
            // Java: case '\n':
      todo!();
            // Java: case '\r':
      todo!();
               // Java: break;
      todo!();
            // Java: case '"':
      todo!();
               // Java: if (quote == '"')
      todo!();
               {
                  // Java: entity = "&quot;";
      todo!();
               }
               // Java: break;
      todo!();
            // Java: case '&':
      todo!();
               // Java: entity = "&amp;";
      todo!();
               // Java: break;
      todo!();
            // Java: case '\'':
      todo!();
               // Java: if (quote == '\'')
      todo!();
               {
                  // Java: entity = "&apos;";
      todo!();
               }
               // Java: break;
      todo!();
            // Java: case '<':
      todo!();
               // Java: entity = "&lt;";
      todo!();
               // Java: break;
      todo!();
            // Java: case '>':
      todo!();
               // Java: entity = "&gt;";
      todo!();
               // Java: break;
      todo!();
            // Java: default:
      todo!();
               // Java: if (c < ' ' || this.shouldEncodeChar(c))
      todo!();
               {
                  // Java: entity = "&#" + c + ";";
      todo!();
               }
         }
         // Java: if (entity != null)
      todo!();
         {
            // Java: if (block == null)
      todo!();
            {
               // Java: block = text.toCharArray();
      todo!();
            }
            // Java: this.buffer.append(block, last, i - last);
      todo!();
            // Java: this.buffer.append(entity);
      todo!();
            // Java: last = i + 1;
      todo!();
         }
      }
      // Java: if (last == 0)
      todo!();
      {
         // Java: return text;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: if (last < size)
      todo!();
         {
            // Java: if (block == null)
      todo!();
            {
               // Java: block = text.toCharArray();
      todo!();
            }
            // Java: this.buffer.append(block, last, i - last);
      todo!();
         }
         // Java: String answer = this.buffer.toString();
      todo!();
         // Java: this.buffer.setLength(0);
      todo!();
         // Java: return answer;
      todo!();
      }
    }

    // Java: boolean shouldEncodeChar(char c)
    pub fn shouldEncodeChar(&mut self) -> bool {
      // Java: int max = this.getMaximumAllowedCharacter();
      todo!();
      // Java: return max > 0 && c > max;
      todo!();
    }

    // Java: int defaultMaximumAllowedCharacter()
    pub fn defaultMaximumAllowedCharacter(&mut self) -> i32 {
      // Java: String encoding = this.format.getEncoding();
      todo!();
      // Java: return encoding != null && encoding.equals("US-ASCII") ? 127 : -1;
      todo!();
    }

    // Java: boolean isNamespaceDeclaration(Namespace ns)
    pub fn isNamespaceDeclaration(&mut self) -> bool {
      // Java: if (ns != null && ns != Namespace.XML_NAMESPACE)
      todo!();
      {
         // Java: String uri = ns.getURI();
      todo!();
         // Java: if (uri != null && uri.length() > 0 && !this.namespaceStack.contains(ns))
      todo!();
         {
            // Java: return true;
      todo!();
         }
      }
      // Java: return false;
      todo!();
    }

    // Java: void handleException(IOException e)
    pub fn handleException(&mut self) {
      // Java: throw new SAXException(e);
      todo!();
    }

    // Java: OutputFormat getOutputFormat()
    pub fn getOutputFormat(&mut self) -> &super::OutputFormat::OutputFormat {
      // Java: return this.format;
      todo!();
    }

    // Java: boolean resolveEntityRefs()
    pub fn resolveEntityRefs(&mut self) -> bool {
      // Java: return this.resolveEntityRefs;
      todo!();
    }

    // Java: void setResolveEntityRefs(boolean resolve)
    pub fn setResolveEntityRefs(&mut self) {
      // Java: this.resolveEntityRefs = resolve;
      todo!();
    }

}

// LexicalHandler trait impl omitted (SAX not available in Rust)
