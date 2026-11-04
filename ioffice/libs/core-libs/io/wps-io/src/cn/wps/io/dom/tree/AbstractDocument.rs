// Java: import cn.wps.io.dom.Comment
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Node
// Java: import cn.wps.io.dom.NodeType
// Java: import cn.wps.io.dom.ProcessingInstruction
// Java: import cn.wps.io.dom.io.OutputFormat
// Java: import cn.wps.io.dom.io.XMLWriter
// Java: import java.io.IOException
// Java: import java.io.StringWriter
// Java: import java.util.Map

// Java: public class AbstractDocument extends AbstractBranch implements Document
pub struct AbstractDocument {
    // Java field: String encoding
    pub(crate) encoding: Option<Box<dyn std::any::Any>>,
}

// Java: public class AbstractDocument extends AbstractBranch implements Document
impl AbstractDocument {
    // Java: AbstractDocument()
    pub fn new() -> Self {
      // Java: super();
      AbstractDocument { encoding: None }
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&mut self) -> Option<super::super::NodeType::NodeType> {
      // Java: return NodeType.DOCUMENT_NODE;
      todo!();
    }

    // Java: String getPath(Element context)
    pub fn getPath(&mut self) -> String {
      // Java: return "/";
      todo!();
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&mut self) -> String {
      // Java: return "/";
      todo!();
    }

    // Java: Document getDocument()
    pub fn getDocument(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this;
      todo!();
    }

    // Java: String getXMLEncoding()
    pub fn getXMLEncoding(&mut self) -> String {
      // Java: return null;
      todo!();
    }

    // Java: String getStringValue()
    pub fn getStringValue(&mut self) -> String {
      // Java: Element root = this.getRootElement();
      todo!();
      // Java: return root != null ? root.getStringValue() : "";
      todo!();
    }

    // Java: String asXML()
    pub fn asXML(&mut self) -> String {
      // Java: OutputFormat format = new OutputFormat();
      todo!();
      // Java: format.setEncoding(this.encoding);
      todo!();
      // Java: try
      todo!();
      {
         // Java: StringWriter out = new StringWriter();
      todo!();
         // Java: XMLWriter writer = new XMLWriter(out, format);
      todo!();
         // Java: writer.write((Document)this);
      todo!();
         // Java: writer.flush();
      todo!();
         // Java: return out.toString();
      todo!();
      }
      // Java: catch (IOException e)
      todo!();
      {
         // Java: throw new RuntimeException("IOException while generating textual representation: " + e.getMessage());
      todo!();
      }
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString(&mut self) {
      // Java: super.toString(builder);
      todo!();
      // Java: builder.append(" [Document: name ");
      todo!();
      // Java: builder.append(this.getName());
      todo!();
      // Java: builder.append(']');
      todo!();
    }

    // Java: void normalize()
    pub fn normalize(&mut self) {
      // Java: Element element = this.getRootElement();
      todo!();
      // Java: if (element != null)
      todo!();
      {
         // Java: element.normalize();
      todo!();
      }
    }

    // Java: Document addComment(String comment)
    pub fn addComment(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this;
      todo!();
    }

    // Java: Document addProcessingInstruction(String target, String data)
    pub fn addProcessingInstruction(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this;
      todo!();
    }

    // Java: Document addProcessingInstruction(String target, Map data)
    pub fn addProcessingInstruction_2(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this;
      todo!();
    }

    // Java: Element addElement(String name)
    pub fn addElement(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Element element = DefaultElement.obtain(name);
      todo!();
      // Java: this.add(element);
      todo!();
      // Java: return element;
      todo!();
    }

    // Java: Element addElement(String namespaceURI, String prefix, String localName)
    pub fn addElement_2(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Element element = DefaultElement.obtain(namespaceURI, prefix, localName);
      todo!();
      // Java: this.add(element);
      todo!();
      // Java: return element;
      todo!();
    }

    // Java: void setRootElement(Element rootElement)
    pub fn setRootElement(&mut self) {
      // Java: this.clearContent();
      todo!();
      // Java: if (rootElement != null)
      todo!();
      {
         // Java: super.add(rootElement);
      todo!();
         // Java: this.rootElementAdded(rootElement);
      todo!();
      }
    }

    // Java: void add(Element element)
    pub fn add(&mut self) {
      // Java: super.add(element);
      todo!();
      // Java: this.rootElementAdded(element);
      todo!();
    }

    // Java: boolean remove(Element element)
    pub fn remove(&mut self) -> bool {
      // Java: boolean answer = super.remove(element);
      todo!();
      // Java: Element root = this.getRootElement();
      todo!();
      // Java: if (root != null && answer)
      todo!();
      {
         // Java: this.setRootElement((Element)null);
      todo!();
      }
      // Java: if (answer)
      todo!();
      {
         // Java: element.setDocument((Document)null);
      todo!();
      }
      // Java: return answer;
      todo!();
    }

    // Java: boolean removeRecursively(Node node)
    pub fn removeRecursively(&mut self) -> bool {
      // Java: switch (node.getNodeTypeEnum())
      todo!();
      {
         // Java: case ELEMENT_NODE:
      todo!();
            // Java: Element element = (Element)node;
      todo!();
            // Java: boolean ret = this.remove(element);
      todo!();
            // Java: if (ret)
      todo!();
            {
               // Java: element.removeAllRecursively();
      todo!();
               // Java: element.recycle();
      todo!();
            }
            // Java: return ret;
      todo!();
         // Java: case COMMENT_NODE:
      todo!();
            // Java: return this.remove((Comment)node);
      todo!();
         // Java: case PROCESSING_INSTRUCTION_NODE:
      todo!();
            // Java: return this.remove((ProcessingInstruction)node);
      todo!();
         // Java: default:
      todo!();
            // Java: this.invalidNodeTypeAddException(node);
      todo!();
            // Java: return false;
      todo!();
      }
    }

    // Java: void removeAllRecursively()
    pub fn removeAllRecursively(&mut self) {
      todo!();
    }

    // Java: void childAdded(Node node)
    pub fn childAdded(&mut self) {
      // Java: if (node != null)
      todo!();
      {
         // Java: node.setDocument(this);
      todo!();
      }
    }

    // Java: void childRemoved(Node node)
    pub fn childRemoved(&mut self) {
      // Java: if (node != null)
      todo!();
      {
         // Java: node.setDocument((Document)null);
      todo!();
      }
    }

    // Java: void checkAddElementAllowed(Element element)
    pub fn checkAddElementAllowed(&mut self) {
      // Java: Element root = this.getRootElement();
      todo!();
    }

    // Java: abstract void rootElementAdded(Element var1)
    pub fn rootElementAdded(&mut self) {
      todo!();
    }

    // Java: void setXMLEncoding(String enc)
    pub fn setXMLEncoding(&mut self) {
      // Java: this.encoding = enc;
      todo!();
    }

}
