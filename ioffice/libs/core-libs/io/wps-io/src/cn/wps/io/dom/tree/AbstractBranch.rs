// Java: import cn.wps.io.dom.Branch
// Java: import cn.wps.io.dom.Comment
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.IllegalAddException
// Java: import cn.wps.io.dom.Node
// Java: import cn.wps.io.dom.ProcessingInstruction
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.StringTokenizer

// Java field: int DEFAULT_CONTENT_LIST_SIZE
pub(crate) const DEFAULT_CONTENT_LIST_SIZE: i32 = 5;

// Java: public class AbstractBranch extends AbstractNode implements Branch
pub struct AbstractBranch {
}

// Java: public class AbstractBranch extends AbstractNode implements Branch
impl AbstractBranch {
    // Java: AbstractBranch()
    pub fn new() -> Self {
      // Java: super();
      AbstractBranch {}
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&mut self) -> bool {
      // Java: return false;
      todo!();
    }

    // Java: boolean hasContent()
    pub fn hasContent(&mut self) -> bool {
      // Java: return this.nodeCount() > 0;
      todo!();
    }

    // Java: String getText()
    pub fn getText(&mut self) -> String {
      // Java: List<Node> content = this.contentList();
      todo!();
      // Java: if (content != null)
      todo!();
      {
         // Java: int size = content.size();
      todo!();
         // Java: if (size >= 1)
      todo!();
         {
            // Java: Node first = (Node)content.get(0);
      todo!();
            // Java: String firstText = this.getContentAsText(first);
      todo!();
            // Java: if (size == 1)
      todo!();
            {
               // Java: return firstText;
      todo!();
            }
            // Java: StringBuilder buffer = new StringBuilder(firstText);
      todo!();
            // Java: for(int i = 1; i < size; ++i)
      todo!();
            {
               // Java: Node node = (Node)content.get(i);
      todo!();
               // Java: buffer.append(this.getContentAsText(node));
      todo!();
            }
            // Java: return buffer.toString();
      todo!();
         }
      }
      // Java: return "";
      todo!();
    }

    // Java: String getContentAsText(Node node)
    pub fn getContentAsText(&mut self) -> String {
      // Java: switch (node.getNodeTypeEnum())
      todo!();
      {
         // Java: case TEXT_NODE:
      todo!();
         // Java: case CDATA_SECTION_NODE:
      todo!();
         // Java: case ENTITY_REFERENCE_NODE:
      todo!();
            // Java: return node.getText();
      todo!();
         // Java: default:
      todo!();
            // Java: return "";
      todo!();
      }
    }

    // Java: String getContentAsStringValue(Node node)
    pub fn getContentAsStringValue(&mut self) -> String {
      // Java: switch (node.getNodeTypeEnum())
      todo!();
      {
         // Java: case TEXT_NODE:
      todo!();
         // Java: case CDATA_SECTION_NODE:
      todo!();
         // Java: case ENTITY_REFERENCE_NODE:
      todo!();
         // Java: case ELEMENT_NODE:
      todo!();
            // Java: return node.getStringValue();
      todo!();
         // Java: default:
      todo!();
            // Java: return "";
      todo!();
      }
    }

    // Java: String getTextTrim()
    pub fn getTextTrim(&mut self) -> String {
      // Java: String text = this.getText();
      todo!();
      // Java: StringBuilder textContent = new StringBuilder();
      todo!();
      // Java: StringTokenizer tokenizer = new StringTokenizer(text);
      todo!();
      // Java: while(tokenizer.hasMoreTokens())
      todo!();
      {
         // Java: String str = tokenizer.nextToken();
      todo!();
         // Java: textContent.append(str);
      todo!();
         // Java: if (tokenizer.hasMoreTokens())
      todo!();
         {
            // Java: textContent.append(" ");
      todo!();
         }
      }
      // Java: return textContent.toString();
      todo!();
    }

    // Java: Element addElement(String name)
    pub fn addElement(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Element element = DefaultElement.obtain(name);
      todo!();
      // Java: this.addNode(element);
      todo!();
      // Java: return element;
      todo!();
    }

    // Java: Element addElement(String namespaceURI, String prefix, String localName)
    pub fn addElement_2(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Element element = DefaultElement.obtain(namespaceURI, prefix, localName);
      todo!();
      // Java: this.addNode(element);
      todo!();
      // Java: return element;
      todo!();
    }

    // Java: void add(Node node)
    pub fn add(&mut self) {
      // Java: switch (node.getNodeTypeEnum())
      todo!();
      {
         // Java: case ELEMENT_NODE:
      todo!();
            // Java: this.add((Element)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case COMMENT_NODE:
      todo!();
            // Java: this.add((Comment)node);
      todo!();
            // Java: break;
      todo!();
         // Java: case PROCESSING_INSTRUCTION_NODE:
      todo!();
            // Java: this.add((ProcessingInstruction)node);
      todo!();
            // Java: break;
      todo!();
         // Java: default:
      todo!();
            // Java: this.invalidNodeTypeAddException(node);
      todo!();
      }
    }

    // Java: boolean remove(Node node)
    pub fn remove(&mut self) -> bool {
      // Java: switch (node.getNodeTypeEnum())
      todo!();
      {
         // Java: case ELEMENT_NODE:
      todo!();
            // Java: return this.remove((Element)node);
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

    // Java: void add(Comment comment)
    pub fn add_2(&mut self) {
      // Java: this.addNode(comment);
      todo!();
    }

    // Java: void add(Element element)
    pub fn add_3(&mut self) {
      // Java: this.addNode(element);
      todo!();
    }

    // Java: void add(ProcessingInstruction pi)
    pub fn add_4(&mut self) {
      // Java: this.addNode(pi);
      todo!();
    }

    // Java: boolean remove(Comment comment)
    pub fn remove_2(&mut self) -> bool {
      // Java: return this.removeNode(comment);
      todo!();
    }

    // Java: boolean remove(Element element)
    pub fn remove_3(&mut self) -> bool {
      // Java: return this.removeNode(element);
      todo!();
    }

    // Java: boolean remove(ProcessingInstruction pi)
    pub fn remove_4(&mut self) -> bool {
      // Java: return this.removeNode(pi);
      todo!();
    }

    // Java: Element elementByID(String elementID)
    pub fn elementByID(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: int i = 0;
      todo!();
      // Java: for(int size = this.nodeCount(); i < size; ++i)
      todo!();
      {
         // Java: Node node = this.node(i);
      todo!();
         // Java: if (node instanceof Element)
      todo!();
         {
            // Java: Element element = (Element)node;
      todo!();
            // Java: String id = this.elementID(element);
      todo!();
            // Java: if (id != null && id.equals(elementID))
      todo!();
            {
               // Java: return element;
      todo!();
            }
            // Java: element = element.elementByID(elementID);
      todo!();
            // Java: if (element != null)
      todo!();
            {
               // Java: return element;
      todo!();
            }
         }
      }
      // Java: return null;
      todo!();
    }

    // Java: void appendContent(Branch branch)
    pub fn appendContent(&mut self) {
      // Java: for(Node node : branch)
      todo!();
      {
         // Java: this.add((Node)node.clone());
      todo!();
      }
    }

    // Java: Node node(int index)
    pub fn node(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: Node node = (Node)this.contentList().get(index);
      todo!();
      // Java: return node;
      todo!();
    }

    // Java: int nodeCount()
    pub fn nodeCount(&mut self) -> i32 {
      // Java: return this.contentList().size();
      todo!();
    }

    // Java: int indexOf(Node node)
    pub fn indexOf(&mut self) -> i32 {
      // Java: return this.contentList().indexOf(node);
      todo!();
    }

    // Java: Iterator<Node> nodeIterator()
    pub fn nodeIterator(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return this.contentList().iterator();
      todo!();
    }

    // Java: Iterator<Node> iterator()
    pub fn iterator(&mut self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return this.nodeIterator();
      todo!();
    }

    // Java: String elementID(Element element)
    pub fn elementID(&mut self) -> String {
      // Java: return element.attributeValue("ID");
      todo!();
    }

    // Java: abstract List<Node> contentList()
    pub fn contentList(&mut self) -> Vec<Box<dyn std::any::Any>> {
      todo!();
    }

    // Java: protected <T extends Node> BackedList<T> createResultList()
    pub fn createResultList(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: return new BackedList<T>(this, this.contentList());
      todo!();
    }

    // Java: abstract void addNode(Node var1)
    pub fn addNode(&mut self) {
      todo!();
    }

    // Java: abstract void addNode(int var1, Node var2)
    pub fn addNode_2(&mut self) {
      todo!();
    }

    // Java: abstract boolean removeNode(Node var1)
    pub fn removeNode(&mut self) -> bool {
      todo!();
    }

    // Java: abstract void childAdded(Node var1)
    pub fn childAdded(&mut self) {
      todo!();
    }

    // Java: abstract void childRemoved(Node var1)
    pub fn childRemoved(&mut self) {
      todo!();
    }

    // Java: void contentRemoved()
    pub fn contentRemoved(&mut self) {
      // Java: for(Node node : this.contentList())
      todo!();
      {
         // Java: this.childRemoved(node);
      todo!();
      }
    }

    // Java: void invalidNodeTypeAddException(Node node)
    pub fn invalidNodeTypeAddException(&mut self) {
      // Java: throw new IllegalAddException("Invalid node type. Cannot add node: " + node + " to this branch: " + this);
      todo!();
    }

}
