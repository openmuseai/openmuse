// Java: import java.io.IOException
// Java: import java.io.Serializable
// Java: import java.io.Writer
// Java: import java.util.List
// Java: import org.dom4j.Document
// Java: import org.dom4j.DocumentFactory
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeFilter
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.XPath
// Java: import org.dom4j.rule.Pattern

// Java field: DocumentFactory DOCUMENT_FACTORY

// Java: public class AbstractNode implements Node, Cloneable, Serializable
pub struct AbstractNode {
}

// Java: public class AbstractNode implements Node, Cloneable, Serializable
impl AbstractNode {
    // Java: AbstractNode()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) {
      // Java: return NodeType.UNKNOWN_NODE;
    }

    // Java: short getNodeType()
    pub fn getNodeType(&self) -> i16 {
      // Java: return this.getNodeTypeEnum().getCode();
      0
    }

    // Java: String getNodeTypeName()
    pub fn getNodeTypeName(&self) -> String {
      // Java: return this.getNodeTypeEnum().getName();
      String::new()
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) {
      // Java: Element element = this.getParent();
      // Java: return element != null ? element.getDocument() : null;
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self) {
    }

    // Java: Element getParent()
    pub fn getParent(&self) {
      // Java: return null;
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self) {
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
      // Java: return true;
      false
    }

    // Java: boolean hasContent()
    pub fn hasContent(&self) -> bool {
      // Java: return false;
      false
    }

    // Java: String getPath()
    pub fn getPath(&self) -> String {
      // Java: return this.getPath((Element)null);
      String::new()
    }

    // Java: String getUniquePath()
    pub fn getUniquePath(&self) -> String {
      // Java: return this.getUniquePath((Element)null);
      String::new()
    }

    // Java: AbstractNode clone()
    pub fn clone_node(&self) {
      // Java: if (this.isReadOnly())
      {
         // Java: return this;
      }
      // Java: else
      {
         // Java: try
         {
            // Java: AbstractNode answer = (AbstractNode)super.clone();
            // Java: answer.setParent((Element)null);
            // Java: answer.setDocument((Document)null);
            // Java: return answer;
         }
         // Java: catch (CloneNotSupportedException e)
         {
            // Java: throw new RuntimeException("This should never happen. Caught: ", e);
         }
      }
    }

    // Java: Node detach()
    pub fn detach(&self) {
      // Java: Element parent = this.getParent();
      // Java: if (parent != null)
      {
         // Java: parent.remove((Node)this);
      }
      // Java: else
      {
         // Java: Document document = this.getDocument();
         // Java: if (document != null)
         {
            // Java: document.remove(this);
         }
      }
      // Java: this.setParent((Element)null);
      // Java: this.setDocument((Document)null);
      // Java: return this;
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return null;
      String::new()
    }

    // Java: void setName(String name)
    pub fn setName(&self, _name: &str) {
      // Java: throw new UnsupportedOperationException("This node cannot be modified");
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return null;
      String::new()
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
      // Java: return this.getText();
      String::new()
    }

    // Java: void setText(String text)
    pub fn setText(&self, _text: &str) {
      // Java: throw new UnsupportedOperationException("This node cannot be modified");
    }

    // Java: void write(Writer writer)
    pub fn write(&self) {
      // Java: writer.write(this.asXML());
    }

    // Java: Object selectObject(String xpathExpression)
    pub fn selectObject(&self, _xpath_expression: &str) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: return xpath.evaluate(this);
    }

    // Java: public List<? extends Node> selectNodes(String xpathExpression)
    pub fn selectNodes(&self, _xpath_expression: &str) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: return xpath.selectNodes(this);
    }

    // Java: public List<? extends Node> selectNodes(String xpathExpression, String comparisonXPathExpression)
    pub fn selectNodes_comparison(&self, _xpath_expression: &str, _comparison: &str) {
      // Java: return this.selectNodes(xpathExpression, comparisonXPathExpression, false);
    }

    // Java: public List<? extends Node> selectNodes(String xpathExpression, String comparisonXPathExpression, boolean removeDuplicates)
    pub fn selectNodes_full(&self, _xpath_expression: &str, _comparison: &str, _remove_duplicates: bool) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: XPath sortBy = this.createXPath(comparisonXPathExpression);
      // Java: return xpath.selectNodes(this, sortBy, removeDuplicates);
    }

    // Java: Node selectSingleNode(String xpathExpression)
    pub fn selectSingleNode(&self, _xpath_expression: &str) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: return xpath.selectSingleNode(this);
    }

    // Java: String valueOf(String xpathExpression)
    pub fn valueOf(&self, _xpath_expression: &str) -> String {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: return xpath.valueOf(this);
      String::new()
    }

    // Java: Number numberValueOf(String xpathExpression)
    pub fn numberValueOf(&self, _xpath_expression: &str) {
      // Java: XPath xpath = this.createXPath(xpathExpression);
      // Java: return xpath.numberValueOf(this);
    }

    // Java: boolean matches(String patternText)
    pub fn matches(&self, _pattern_text: &str) -> bool {
      // Java: NodeFilter filter = this.createXPathFilter(patternText);
      // Java: return filter.matches(this);
      false
    }

    // Java: XPath createXPath(String xpathExpression)
    pub fn createXPath(&self, _xpath_expression: &str) {
      // Java: return this.getDocumentFactory().createXPath(xpathExpression);
    }

    // Java: NodeFilter createXPathFilter(String patternText)
    pub fn createXPathFilter(&self, _pattern_text: &str) {
      // Java: return this.getDocumentFactory().createXPathFilter(patternText);
    }

    // Java: Pattern createPattern(String patternText)
    pub fn createPattern(&self, _pattern_text: &str) {
      // Java: return this.getDocumentFactory().createPattern(patternText);
    }

    // Java: Node asXPathResult(Element parent)
    pub fn asXPathResult(&self) {
      // Java: return (Node)(this.supportsParent() ? this : this.createXPathResult(parent));
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuilder builder = new StringBuilder();
      // Java: this.toString(builder);
      // Java: return builder.toString();
      String::new()
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString_builder(&self) {
      // Java: builder.append(super.toString());
    }

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&self) {
      // Java: return DOCUMENT_FACTORY;
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
      // Java: throw new RuntimeException("asXPathResult() not yet implemented fully for: " + this);
    }

}
