// Java: import cn.wps.base.assertion.Assert
// Java: import java.util.EnumSet
// Java: import org.dom4j.tree.BackedList

// Java field: EnumSet<NodeType> BRANCH_TYPE

// Java: public class NodeHelper
pub struct NodeHelper {
}

// Java: public class NodeHelper
impl NodeHelper {
    // Java: NodeHelper()
    pub fn new() -> Self {
        // Java: super();
        NodeHelper {}
    }

    // Java: static Attribute nodeAsAttribute(Node node)
    pub fn nodeAsAttribute() {
        // Java: if (node.getNodeTypeEnum() == NodeType.ATTRIBUTE_NODE)
        // Java: Assert.assertTrue("node instanceof Attribute should be true", node instanceof Attribute);
        // Java: return (Attribute)node;
        // Java: else
        // Java: return null;
    }

    // Java: static Branch nodeAsBranch(Node node)
    pub fn nodeAsBranch() {
        // Java: if (BRANCH_TYPE.contains(node.getNodeTypeEnum()))
        // Java: Assert.assertTrue("node instanceof Branch should be true", node instanceof Branch);
        // Java: return (Branch)node;
        // Java: else
        // Java: return null;
    }

    // Java: static Document nodeAsDocument(Node node)
    pub fn nodeAsDocument() {
        // Java: if (node.getNodeTypeEnum() == NodeType.DOCUMENT_NODE)
        // Java: Assert.assertTrue("node instanceof Document should be true", node instanceof Document);
        // Java: return (Document)node;
        // Java: else
        // Java: return null;
    }

    // Java: static Element nodeAsElement(Node node)
    pub fn nodeAsElement() {
        // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
        // Java: Assert.assertTrue("node instanceof Element should be true", node instanceof Element);
        // Java: return (Element)node;
        // Java: else
        // Java: return null;
    }

    // Java: static Namespace nodeAsNamespace(Node node)
    pub fn nodeAsNamespace() {
        // Java: if (node.getNodeTypeEnum() == NodeType.NAMESPACE_NODE)
        // Java: Assert.assertTrue("node instanceof Namespace should be true", node instanceof Namespace);
        // Java: return (Namespace)node;
        // Java: else
        // Java: return null;
    }

    // Java: static ProcessingInstruction nodeAsProcessingInstruction(Node node)
    pub fn nodeAsProcessingInstruction() {
        // Java: if (node.getNodeTypeEnum() == NodeType.PROCESSING_INSTRUCTION_NODE)
        // Java: assert node instanceof ProcessingInstruction;
        // Java: return (ProcessingInstruction)node;
        // Java: else
        // Java: return null;
    }

    // Java: static BackedList<Element> appendElementLocal(Node node, BackedList<Element> list)
    pub fn appendElementLocal() {
        // Java: Element element = nodeAsElement(node);
        // Java: if (element != null)
        // Java: list.addLocal(element);
        // Java: return list;
    }

    // Java: static BackedList<Element> appendElementNamedLocal(Node node, BackedList<Element> list, String name)
    pub fn appendElementNamedLocal() {
        // Java: Element element = nodeAsElement(node);
        // Java: if (element != null && name.equals(element.getName()))
        // Java: list.addLocal(element);
        // Java: return list;
    }

    // Java: static BackedList<Element> appendElementQNamedLocal(Node node, BackedList<Element> list, QName qname)
    pub fn appendElementQNamedLocal() {
        // Java: Element element = nodeAsElement(node);
        // Java: if (element != null && qname.equals(element.getQName()))
        // Java: list.addLocal(element);
        // Java: return list;
    }

    // Java: static String getAttributeValue(Attribute attribute, String defaultValue)
    pub fn getAttributeValue(default_value: Option<&str>) -> Option<String> {
        // Java: return attribute == null ? defaultValue : attribute.getValue();
        default_value.map(|s| s.to_string())
    }
}
