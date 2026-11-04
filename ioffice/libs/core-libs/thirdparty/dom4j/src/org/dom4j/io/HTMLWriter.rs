// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.StringWriter
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import java.util.Collections
// Java: import java.util.HashSet
// Java: import java.util.LinkedList
// Java: import java.util.Set
// Java: import org.dom4j.Document
// use crate::org::dom4j::Document;
// Java: import org.dom4j.DocumentException
// use crate::org::dom4j::DocumentException;
// Java: import org.dom4j.DocumentHelper
// use crate::org::dom4j::DocumentHelper;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.Entity
// use crate::org::dom4j::Entity;
// Java: import org.dom4j.NodeType
// use crate::org::dom4j::NodeType;
// Java: import org.xml.sax.SAXException
// use crate::org::xml::sax::SAXException;

// Java field: String lineSeparator
static LINESEPARATOR: i32 = 0;
// Java field: HashSet<String> DEFAULT_PREFORMATTED_TAGS
static DEFAULT_PREFORMATTED_TAGS: i32 = 0;
// Java field: OutputFormat DEFAULT_HTML_FORMAT
static DEFAULT_HTML_FORMAT: i32 = 0;

// Java: public class HTMLWriter extends XMLWriter
pub struct HTMLWriter {
    // Java field: LinkedList<FormatState> formatStack
    formatStack: i32,
    // Java field: String lastText
    lastText: String,
    // Java field: int tagsOuput
    tagsOuput: i32,
    // Java field: int newLineAfterNTags
    newLineAfterNTags: i32,
    // Java field: HashSet<String> preformattedTags
    pub(crate) preformattedTags: i32,
    // Java field: HashSet<String> omitElementCloseSet
    omitElementCloseSet: i32,
}

// Java: public class FormatState
pub struct FormatState {
    // Java field: boolean newlines
    pub(crate) newlines: bool,
    // Java field: boolean trimText
    pub(crate) trimText: bool,
    // Java field: String indent
    pub(crate) indent: String,
}

// Java: public class HTMLWriter extends XMLWriter
impl HTMLWriter {
    // Java: HTMLWriter(Writer writer)
    pub fn new_with_writer(_writer: i32) -> Self {
      // Java: super(writer, DEFAULT_HTML_FORMAT);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: HTMLWriter(Writer writer, OutputFormat format)
    pub fn new_with_writer_format(_writer: i32, _format: i32) -> Self {
      // Java: super(writer, format);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: HTMLWriter()
    pub fn new() -> Self {
      // Java: super(DEFAULT_HTML_FORMAT);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: HTMLWriter(OutputFormat format)
    pub fn new_with_format(_format: i32) -> Self {
      // Java: super(format);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: HTMLWriter(OutputStream out)
    pub fn new_with_stream(_out: i32) -> Self {
      // Java: super(out, DEFAULT_HTML_FORMAT);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: HTMLWriter(OutputStream out, OutputFormat format)
    pub fn new_with_stream_format(_out: i32, _format: i32) -> Self {
      // Java: super(out, format);
      // Java: this.preformattedTags = DEFAULT_PREFORMATTED_TAGS;
      Self { formatStack: 0, lastText: String::new(), tagsOuput: 0, newLineAfterNTags: -1, preformattedTags: 0, omitElementCloseSet: 0 }
    }

    // Java: void startCDATA()
    pub fn startCDATA(&self) {
    }

    // Java: void endCDATA()
    pub fn endCDATA(&self) {
    }

    // Java: void writeCDATA(String text)
    pub fn writeCDATA(&self, _text: &str) {
      // Java: if (this.getOutputFormat().isXHTML())
      {
         // Java: super.writeCDATA(text);
      }
      // Java: else
      {
         // Java: this.writer.write(text);
      }
      // Java: this.lastOutputNodeType = NodeType.CDATA_SECTION_NODE;
    }

    // Java: void writeEntity(Entity entity)
    pub fn writeEntity(&self, _entity: i32) {
      // Java: this.writer.write(entity.getText());
      // Java: this.lastOutputNodeType = NodeType.ENTITY_REFERENCE_NODE;
    }

    // Java: void writeDeclaration()
    pub fn writeDeclaration(&self) {
    }

    // Java: void writeString(String text)
    pub fn writeString(&self, _text: &str) {
      // Java: if (text.equals("\n"))
      {
         // Java: if (!this.formatStack.isEmpty())
         {
            // Java: super.writeString(lineSeparator);
         }
      }
      // Java: else
      {
         // Java: this.lastText = text;
         // Java: if (this.formatStack.isEmpty())
         {
            // Java: super.writeString(text.trim());
         }
         // Java: else
         {
            // Java: super.writeString(text);
         }
      }
    }

    // Java: void writeClose(String qualifiedName)
    pub fn writeClose(&self, _qualifiedName: &str) {
      // Java: if (!this.omitElementClose(qualifiedName))
      {
         // Java: super.writeClose(qualifiedName);
      }
    }

    // Java: void writeEmptyElementClose(String qualifiedName)
    pub fn writeEmptyElementClose(&self, _qualifiedName: &str) {
      // Java: if (this.getOutputFormat().isXHTML())
      {
         // Java: if (this.omitElementClose(qualifiedName))
         {
            // Java: this.writer.write(" />");
         }
         // Java: else
         {
            // Java: super.writeEmptyElementClose(qualifiedName);
         }
      }
      // Java: else if (this.omitElementClose(qualifiedName))
      {
         // Java: this.writer.write(">");
      }
      // Java: else
      {
         // Java: super.writeEmptyElementClose(qualifiedName);
      }
    }

    // Java: boolean omitElementClose(String qualifiedName)
    pub fn omitElementClose(&self, _qualifiedName: &str) -> bool {
      // Java: return this.internalGetOmitElementCloseSet().contains(qualifiedName.toUpperCase());
      false
    }

    // Java: HashSet internalGetOmitElementCloseSet()
    pub fn internalGetOmitElementCloseSet(&self) -> i32 {
      // Java: if (this.omitElementCloseSet == null)
      {
         // Java: this.omitElementCloseSet = new HashSet();
         // Java: this.loadOmitElementCloseSet(this.omitElementCloseSet);
      }
      // Java: return this.omitElementCloseSet;
      0
    }

    // Java: void loadOmitElementCloseSet(Set<String> set)
    pub fn loadOmitElementCloseSet(&self, _set: i32) {
      // Java: set.add("AREA");
      // Java: set.add("BASE");
      // Java: set.add("BR");
      // Java: set.add("COL");
      // Java: set.add("HR");
      // Java: set.add("IMG");
      // Java: set.add("INPUT");
      // Java: set.add("LINK");
      // Java: set.add("META");
      // Java: set.add("P");
      // Java: set.add("PARAM");
    }

    // Java: Set getOmitElementCloseSet()
    pub fn getOmitElementCloseSet(&self) -> i32 {
      // Java: return (Set)this.internalGetOmitElementCloseSet().clone();
      0
    }

    // Java: void setOmitElementCloseSet(Set<String> newSet)
    pub fn setOmitElementCloseSet(&self, _newSet: i32) {
      // Java: this.omitElementCloseSet = new HashSet();
      // Java: if (newSet != null)
      {
         // Java: this.omitElementCloseSet = new HashSet();
         // Java: for(String tag : newSet)
         {
            // Java: if (tag != null)
            {
               // Java: this.omitElementCloseSet.add(tag.toUpperCase());
            }
         }
      }
    }

    // Java: Set<String> getPreformattedTags()
    pub fn getPreformattedTags(&self) -> i32 {
      // Java: return Collections.unmodifiableSet(this.preformattedTags);
      0
    }

    // Java: void setPreformattedTags(Set<String> newSet)
    pub fn setPreformattedTags(&self, _newSet: i32) {
      // Java: this.preformattedTags = new HashSet();
      // Java: if (newSet != null)
      {
         // Java: for(String tag : newSet)
         {
            // Java: if (tag != null)
            {
               // Java: this.preformattedTags.add(tag.toUpperCase());
            }
         }
      }
    }

    // Java: boolean isPreformattedTag(String qualifiedName)
    pub fn isPreformattedTag(&self, _qualifiedName: &str) -> bool {
      // Java: return this.preformattedTags != null && this.preformattedTags.contains(qualifiedName.toUpperCase());
      false
    }

    // Java: void writeElement(Element element)
    pub fn writeElement(&self, _element: i32) {
      // Java: if (this.newLineAfterNTags == -1)
      {
         // Java: this.lazyInitNewLinesAfterNTags();
      }
      // Java: if (this.newLineAfterNTags > 0 && this.tagsOuput > 0 && this.tagsOuput % this.newLineAfterNTags == 0)
      {
         // Java: super.writer.write(lineSeparator);
      }
      // Java: ++this.tagsOuput;
      // Java: String qualifiedName = element.getQualifiedName();
      // Java: String saveLastText = this.lastText;
      // Java: int size = element.nodeCount();
      // Java: if (this.isPreformattedTag(qualifiedName))
      {
         // Java: OutputFormat currentFormat = this.getOutputFormat();
         // Java: boolean saveNewlines = currentFormat.isNewlines();
         // Java: boolean saveTrimText = currentFormat.isTrimText();
         // Java: String currentIndent = currentFormat.getIndent();
         // Java: this.formatStack.addFirst(new FormatState(saveNewlines, saveTrimText, currentIndent));
         // Java: try
         {
            // Java: super.writePrintln();
            // Java: if (saveLastText.trim().length() == 0 && currentIndent != null && currentIndent.length() > 0)
            {
               // Java: super.writer.write(this.justSpaces(saveLastText));
            }
            // Java: currentFormat.setNewlines(false);
            // Java: currentFormat.setTrimText(false);
            // Java: currentFormat.setIndent("");
            // Java: super.writeElement(element);
         }
         // Java: finally
         {
            // Java: FormatState state = (FormatState)this.formatStack.poll();
            // Java: currentFormat.setNewlines(state.isNewlines());
            // Java: currentFormat.setTrimText(state.isTrimText());
            // Java: currentFormat.setIndent(state.getIndent());
         }
      }
      // Java: else
      {
         // Java: super.writeElement(element);
      }
    }

    // Java: String justSpaces(String param1)
    pub fn justSpaces(&self, _param1: &str) -> String {
      // Java: if (param1 == null) return "";
      // Java: StringBuilder sb = new StringBuilder();
      // Java: for (int i = 0; i < param1.length(); i++)
      {
         // Java: sb.append(' ');
      }
      // Java: return sb.toString();
      String::new()
    }

    // Java: void lazyInitNewLinesAfterNTags(&self)
    pub fn lazyInitNewLinesAfterNTags(&self) {
      // Java: if (this.getOutputFormat().isNewlines())
      {
         // Java: this.newLineAfterNTags = 0;
      }
      // Java: else
      {
         // Java: this.newLineAfterNTags = this.getOutputFormat().getNewLineAfterNTags();
      }
    }

    // Java: static String prettyPrintHTML(String html)
    pub fn prettyPrintHTML(_html: &str) -> String {
      // Java: return prettyPrintHTML(html, true, true, false, true);
      String::new()
    }

    // Java: static String prettyPrintXHTML(String html)
    pub fn prettyPrintXHTML(_html: &str) -> String {
      // Java: return prettyPrintHTML(html, true, true, true, false);
      String::new()
    }

    // Java: static String prettyPrintHTML(String html, boolean newlines, boolean trim, boolean isXHTML, boolean expandEmpty)
    pub fn prettyPrintHTML_full(_html: &str, _newlines: bool, _trim: bool, _isXHTML: bool, _expandEmpty: bool) -> String {
      // Java: StringWriter sw = new StringWriter();
      // Java: OutputFormat format = OutputFormat.createPrettyPrint();
      // Java: format.setNewlines(newlines);
      // Java: format.setTrimText(trim);
      // Java: format.setXHTML(isXHTML);
      // Java: format.setExpandEmptyElements(expandEmpty);
      // Java: HTMLWriter writer = new HTMLWriter(sw, format);
      // Java: Document document = DocumentHelper.parseText(html);
      // Java: writer.write(document);
      // Java: writer.flush();
      // Java: return sw.toString();
      String::new()
    }

}

// Java: public class FormatState
impl FormatState {
    // Java: FormatState(boolean newLines, boolean trimText, String indent)
    pub fn new(_newLines: bool, _trimText: bool, _indent: &str) -> Self {
         // Java: super();
         // Java: this.newlines = newLines;
         // Java: this.trimText = trimText;
         // Java: this.indent = indent;
         Self { newlines: _newLines, trimText: _trimText, indent: _indent.to_string() }
    }

    // Java: boolean isNewlines()
    pub fn isNewlines(&self) -> bool {
         // Java: return this.newlines;
         false
    }

    // Java: boolean isTrimText()
    pub fn isTrimText(&self) -> bool {
         // Java: return this.trimText;
         false
    }

    // Java: String getIndent()
    pub fn getIndent(&self) -> String {
         // Java: return this.indent;
         String::new()
    }

}
