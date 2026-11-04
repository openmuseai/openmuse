// Java: import java.util.List
// Java: import org.dom4j.Branch
// Java: import org.dom4j.CharacterData
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node
// Java: import org.w3c.dom.Attr
// Java: import org.w3c.dom.DOMException
// Java: import org.w3c.dom.Document
// Java: import org.w3c.dom.DocumentType
// Java: import org.w3c.dom.NamedNodeMap
// Java: import org.w3c.dom.NodeList
// Java: import org.w3c.dom.Text

// Java field: NodeList EMPTY_NODE_LIST

// Java: public class DOMNodeHelper
pub struct DOMNodeHelper {
}

// Java: public class EmptyNodeList
pub struct EmptyNodeList {
}

// Java: public class DOMNodeHelper
impl DOMNodeHelper {
    // Java: DOMNodeHelper()
    pub fn new() -> Self {
        // Java: super();
        DOMNodeHelper {}
    }

    // Java: static boolean supports(Node node, String feature, String version)
    pub fn supports() -> bool {
        // Java: return false;
        false
    }

    // Java: static String getNamespaceURI(Node node)
    pub fn getNamespaceURI() -> Option<String> {
        // Java: return null;
        None
    }

    // Java: static String getPrefix(Node node)
    pub fn getPrefix() -> Option<String> {
        // Java: return null;
        None
    }

    // Java: static String getLocalName(Node node)
    pub fn getLocalName() -> Option<String> {
        // Java: return null;
        None
    }

    // Java: static void setPrefix(Node node, String prefix)
    pub fn setPrefix() {
        // Java: notSupported();
        Self::notSupported();
    }

    // Java: static String getNodeValue(Node node)
    pub fn getNodeValue() -> String {
        // Java: return node.getText();
        String::new()
    }

    // Java: static void setNodeValue(Node node, String nodeValue)
    pub fn setNodeValue() {
        // Java: node.setText(nodeValue);
    }

    // Java: static NodeList getChildNodes(Node node)
    pub fn getChildNodes() {
        // Java: return EMPTY_NODE_LIST;
    }

    // Java: static NamedNodeMap getAttributes(Node node)
    pub fn getAttributes() {
        // Java: return null;
    }

    // Java: static Document getOwnerDocument(Node node)
    pub fn getOwnerDocument() {
        // Java: return asDOMDocument(node.getDocument());
    }

    // Java: static boolean hasChildNodes(Node node)
    pub fn hasChildNodes() -> bool {
        // Java: return false;
        false
    }

    // Java: static void normalize(Node node)
    pub fn normalize() {
        // Java: notSupported();
        Self::notSupported();
    }

    // Java: static boolean isSupported(Node n, String feature, String version)
    pub fn isSupported() -> bool {
        // Java: return false;
        false
    }

    // Java: static boolean hasAttributes(Node node)
    pub fn hasAttributes() -> bool {
        // Java: if (node != null && node instanceof Element)
        {
            // Java: return ((Element)node).attributeCount() > 0;
        }
        // Java: else
        {
            // Java: return false;
        }
        false
    }

    // Java: static String getData(CharacterData charData)
    pub fn getData() -> String {
        // Java: return charData.getText();
        String::new()
    }

    // Java: static void setData(CharacterData charData, String data)
    pub fn setData() {
        // Java: charData.setText(data);
    }

    // Java: static int getLength(CharacterData charData)
    pub fn getLength() -> i32 {
        // Java: String text = charData.getText();
        // Java: return text != null ? text.length() : 0;
        0
    }

    // Java: static String substringData(CharacterData charData, int offset, int count)
    pub fn substringData(text: &str, offset: usize, count: usize) -> String {
        // Java: if (count < 0)
        {
            // Java: throw new DOMException((short)1, "Illegal value for count: " + count);
        }
        // Java: else
        {
            // Java: String text = charData.getText();
            // Java: int length = text != null ? text.length() : 0;
            let length = text.len();
            // Java: if (offset >= 0 && offset < length)
            if offset < length
            {
                // Java: return offset + count > length ? text.substring(offset) : text.substring(offset, offset + count);
                let end = std::cmp::min(offset + count, length);
                return text[offset..end].to_string();
            }
            // Java: else
            {
                // Java: throw new DOMException((short)1, "No text at offset: " + offset);
                String::new()
            }
        }
    }

    // Java: static void appendData(CharacterData charData, String arg)
    pub fn appendData() {
        // Java: if (charData.isReadOnly())
        {
            // Java: throw new DOMException((short)7, "CharacterData node is read only: " + charData);
        }
        // Java: else
        {
            // Java: String text = charData.getText();
            // Java: if (text == null)
            {
                // Java: charData.setText(text);
            }
            // Java: else
            {
                // Java: charData.setText(text + arg);
            }
        }
    }

    // Java: static void insertData(CharacterData data, int offset, String arg)
    pub fn insertData() {
        // Java: if (data.isReadOnly())
        {
            // Java: throw new DOMException((short)7, "CharacterData node is read only: " + data);
        }
        // Java: else
        {
            // Java: String text = data.getText();
            // Java: if (text == null)
            {
                // Java: data.setText(arg);
            }
            // Java: else
            {
                // Java: int length = text.length();
                // Java: if (offset < 0 || offset > length)
                {
                    // Java: throw new DOMException((short)1, "No text at offset: " + offset);
                }
                // Java: StringBuffer buffer = new StringBuffer(text);
                // Java: buffer.insert(offset, arg);
                // Java: data.setText(buffer.toString());
            }
        }
    }

    // Java: static void deleteData(CharacterData charData, int offset, int count)
    pub fn deleteData() {
        // Java: if (charData.isReadOnly())
        {
            // Java: throw new DOMException((short)7, "CharacterData node is read only: " + charData);
        }
        // Java: else if (count < 0)
        {
            // Java: throw new DOMException((short)1, "Illegal value for count: " + count);
        }
        // Java: else
        {
            // Java: String text = charData.getText();
            // Java: if (text != null)
            {
                // Java: int length = text.length();
                // Java: if (offset < 0 || offset >= length)
                {
                    // Java: throw new DOMException((short)1, "No text at offset: " + offset);
                }
                // Java: StringBuffer buffer = new StringBuffer(text);
                // Java: buffer.delete(offset, offset + count);
                // Java: charData.setText(buffer.toString());
            }
        }
    }

    // Java: static void replaceData(CharacterData charData, int offset, int count, String arg)
    pub fn replaceData() {
        // Java: if (charData.isReadOnly())
        {
            // Java: throw new DOMException((short)7, "CharacterData node is read only: " + charData);
        }
        // Java: else if (count < 0)
        {
            // Java: throw new DOMException((short)1, "Illegal value for count: " + count);
        }
        // Java: else
        {
            // Java: String text = charData.getText();
            // Java: if (text != null)
            {
                // Java: int length = text.length();
                // Java: if (offset < 0 || offset >= length)
                {
                    // Java: throw new DOMException((short)1, "No text at offset: " + offset);
                }
                // Java: StringBuffer buffer = new StringBuffer(text);
                // Java: buffer.replace(offset, offset + count, arg);
                // Java: charData.setText(buffer.toString());
            }
        }
    }

    // Java: static void appendElementsByTagName(List list, Branch parent, String name)
    pub fn appendElementsByTagName() {
        // Java: boolean isStar = "*".equals(name);
        // Java: int i = 0;
        // Java: for(int size = parent.nodeCount(); i < size; ++i)
        {
            // Java: Node node = parent.node(i);
            // Java: if (node instanceof Element)
            {
                // Java: Element element = (Element)node;
                // Java: if (isStar || name.equals(element.getName()))
                {
                    // Java: list.add(element);
                }
                // Java: appendElementsByTagName(list, element, name);
            }
        }
    }

    // Java: static void appendElementsByTagNameNS(List list, Branch parent, String namespace, String localName)
    pub fn appendElementsByTagNameNS() {
        // Java: boolean isStarNS = "*".equals(namespace);
        // Java: boolean isStar = "*".equals(localName);
        // Java: int i = 0;
        // Java: for(int size = parent.nodeCount(); i < size; ++i)
        {
            // Java: Node node = parent.node(i);
            // Java: if (node instanceof Element)
            {
                // Java: Element element = (Element)node;
                // Java: if ((isStarNS || (namespace == null || namespace.length() == 0) && (element.getNamespaceURI() == null || element.getNamespaceURI().length() == 0) || namespace != null && namespace.equals(element.getNamespaceURI())) && (isStar || localName.equals(element.getName())))
                {
                    // Java: list.add(element);
                }
                // Java: appendElementsByTagNameNS(list, element, namespace, localName);
            }
        }
    }

    // Java: static NodeList createNodeList(final List list)
    pub fn createNodeList() {
        // Java: return new NodeList()
        {
            // Java: public org.w3c.dom.Node item(int index)
            {
                // Java: return index >= this.getLength() ? null : DOMNodeHelper.asDOMNode((Node)list.get(index));
            }
            // Java: public int getLength()
            {
                // Java: return list.size();
            }
        }
    }

    // Java: static Document asDOMDocument(org.dom4j.Document document)
    pub fn asDOMDocument() {
        // Java: if (document == null)
        {
            // Java: return null;
        }
        // Java: else if (document instanceof Document)
        {
            // Java: return (Document)document;
        }
        // Java: else
        {
            // Java: notSupported();
            // Java: return null;
        }
    }

    // Java: static DocumentType asDOMDocumentType(org.dom4j.DocumentType dt)
    pub fn asDOMDocumentType() {
        // Java: if (dt == null)
        {
            // Java: return null;
        }
        // Java: else if (dt instanceof DocumentType)
        {
            // Java: return (DocumentType)dt;
        }
        // Java: else
        {
            // Java: notSupported();
            // Java: return null;
        }
    }

    // Java: static Text asDOMText(CharacterData text)
    pub fn asDOMText() {
        // Java: if (text == null)
        {
            // Java: return null;
        }
        // Java: else if (text instanceof Text)
        {
            // Java: return (Text)text;
        }
        // Java: else
        {
            // Java: notSupported();
            // Java: return null;
        }
    }

    // Java: static Attr asDOMAttr(Node attribute)
    pub fn asDOMAttr() {
        // Java: if (attribute == null)
        {
            // Java: return null;
        }
        // Java: else if (attribute instanceof Attr)
        {
            // Java: return (Attr)attribute;
        }
        // Java: else
        {
            // Java: notSupported();
            // Java: return null;
        }
    }

    // Java: static void notSupported()
    pub fn notSupported() {
        // Java: throw new DOMException((short)9, "Not supported yet");
        panic!("Not supported yet");
    }
}

// Java: public class EmptyNodeList
impl EmptyNodeList {
    // Java: EmptyNodeList()
    pub fn new() -> Self {
        // Java: super();
        EmptyNodeList {}
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return 0;
        0
    }
}
