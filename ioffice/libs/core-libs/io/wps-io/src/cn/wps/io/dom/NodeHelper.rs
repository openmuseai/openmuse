// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.dom.tree.BackedList
// Java: import java.util.EnumSet

// Java field: EnumSet<NodeType> BRANCH_TYPE
static BRANCH_TYPE: std::sync::OnceLock<std::sync::Mutex<Vec<i32>>> = std::sync::OnceLock::new();

// Java: public class NodeHelper
pub struct NodeHelper {
}

// Java: public class NodeHelper
impl NodeHelper {
    // Java: NodeHelper()
    pub fn new() -> Self {
      // Java: super();
      todo!();
    }

    // Java: static Attribute nodeAsAttribute(Node node)
    pub fn nodeAsAttribute(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (node.getNodeTypeEnum() == NodeType.ATTRIBUTE_NODE)
      todo!();
      {
         // Java: IOAssert.assertTrue("node instanceof Attribute should be true", node instanceof Attribute);
      todo!();
         // Java: return (Attribute)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static Branch nodeAsBranch(Node node)
    pub fn nodeAsBranch(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (BRANCH_TYPE.contains(node.getNodeTypeEnum()))
      todo!();
      {
         // Java: IOAssert.assertTrue("node instanceof Branch should be true", node instanceof Branch);
      todo!();
         // Java: return (Branch)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static Document nodeAsDocument(Node node)
    pub fn nodeAsDocument(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (node.getNodeTypeEnum() == NodeType.DOCUMENT_NODE)
      todo!();
      {
         // Java: IOAssert.assertTrue("node instanceof Document should be true", node instanceof Document);
      todo!();
         // Java: return (Document)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static Element nodeAsElement(Node node)
    pub fn nodeAsElement(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
      todo!();
      {
         // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
      todo!();
         // Java: return (Element)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static Namespace nodeAsNamespace(Node node)
    pub fn nodeAsNamespace(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (node.getNodeTypeEnum() == NodeType.NAMESPACE_NODE)
      todo!();
      {
         // Java: IOAssert.assertTrue("node instanceof Namespace should be true", node instanceof Namespace);
      todo!();
         // Java: return (Namespace)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static ProcessingInstruction nodeAsProcessingInstruction(Node node)
    pub fn nodeAsProcessingInstruction(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (node.getNodeTypeEnum() == NodeType.PROCESSING_INSTRUCTION_NODE)
      todo!();
      {
         // Java: assert node instanceof ProcessingInstruction;
      todo!();
         // Java: return (ProcessingInstruction)node;
      todo!();
      }
      // Java: else
      todo!();
      {
         // Java: return null;
      todo!();
      }
    }

    // Java: static BackedList<Element> appendElementLocal(Node node, BackedList<Element> list)
    pub fn appendElementLocal(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Element element = nodeAsElement(node);
      todo!();
      // Java: if (element != null)
      todo!();
      {
         // Java: list.addLocal(element);
      todo!();
      }
      // Java: return list;
      todo!();
    }

    // Java: static BackedList<Element> appendElementNamedLocal(Node node, BackedList<Element> list, String name)
    pub fn appendElementNamedLocal(&mut self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Element element = nodeAsElement(node);
      todo!();
      // Java: if (element != null && name.equals(element.getName()))
      todo!();
      {
         // Java: list.addLocal(element);
      todo!();
      }
      // Java: return list;
      todo!();
    }

    // Java: static String getAttributeValue(Attribute attribute, String defaultValue)
    pub fn getAttributeValue(&mut self) -> String {
      // Java: return attribute == null ? defaultValue : attribute.getValue();
      todo!();
    }

}
