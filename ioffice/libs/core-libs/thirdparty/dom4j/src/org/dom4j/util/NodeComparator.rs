// Java: import java.util.Comparator
// Java: import java.util.Objects
// Java: import org.dom4j.Attribute
// Java: import org.dom4j.Branch
// Java: import org.dom4j.CDATA
// Java: import org.dom4j.CharacterData
// Java: import org.dom4j.Comment
// Java: import org.dom4j.Document
// Java: import org.dom4j.DocumentType
// Java: import org.dom4j.Element
// Java: import org.dom4j.Entity
// Java: import org.dom4j.Namespace
// Java: import org.dom4j.Node
// Java: import org.dom4j.NodeType
// Java: import org.dom4j.ProcessingInstruction
// Java: import org.dom4j.QName
// Java: import org.dom4j.Text

// Java: public class NodeComparator implements Comparator
pub struct NodeComparator {
}

// Java: public class NodeComparator implements Comparator
impl NodeComparator {
    // Java: NodeComparator()
    pub fn new() -> Self {
        // Java: super();
        NodeComparator {}
    }

    // Java: int compare(Node n1, Node n2)
    pub fn compare(&self) -> i32 {
        // Java: NodeType nodeType1 = n1.getNodeTypeEnum();
        // Java: NodeType nodeType2 = n2.getNodeTypeEnum();
        // Java: if (nodeType1 != nodeType2)
        {
            // Java: return nodeType1.getCode() - nodeType2.getCode();
        }
        // Java: else
        {
            // Java: switch (nodeType1)
            {
                // Java: case ELEMENT_NODE:
                    // Java: return this.compare((Element)n1, (Element)n2);
                // Java: case DOCUMENT_NODE:
                    // Java: return this.compare((Document)n1, (Document)n2);
                // Java: case ATTRIBUTE_NODE:
                    // Java: return this.compare((Attribute)n1, (Attribute)n2);
                // Java: case TEXT_NODE:
                    // Java: return this.compare((CharacterData)((Text)n1), (CharacterData)((Text)n2));
                // Java: case CDATA_SECTION_NODE:
                    // Java: return this.compare((CharacterData)((CDATA)n1), (CharacterData)((CDATA)n2));
                // Java: case ENTITY_REFERENCE_NODE:
                    // Java: return this.compare((Entity)n1, (Entity)n2);
                // Java: case PROCESSING_INSTRUCTION_NODE:
                    // Java: return this.compare((ProcessingInstruction)n1, (ProcessingInstruction)n2);
                // Java: case COMMENT_NODE:
                    // Java: return this.compare((CharacterData)((Comment)n1), (CharacterData)((Comment)n2));
                // Java: case DOCUMENT_TYPE_NODE:
                    // Java: return this.compare((DocumentType)n1, (DocumentType)n2);
                // Java: case NAMESPACE_NODE:
                    // Java: return this.compare((Namespace)n1, (Namespace)n2);
                // Java: default:
                    // Java: throw new RuntimeException("Invalid node types. node1: " + n1 + " and node2: " + n2);
            }
        }
        0
    }

    // Java: int compare(Document n1, Document n2)
    pub fn compare_document(&self) -> i32 {
        // Java: int answer = this.compare(n1.getDocType(), n2.getDocType());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compareContent(n1, n2);
            answer = self.compareContent();
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(Element n1, Element n2)
    pub fn compare_element(&self) -> i32 {
        // Java: int answer = this.compare(n1.getQName(), n2.getQName());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: int c1 = n1.attributeCount();
            // Java: int c2 = n2.attributeCount();
            // Java: answer = c1 - c2;
            // Java: if (answer == 0)
            {
                // Java: for(int i = 0; i < c1; ++i)
                {
                    // Java: Attribute a1 = n1.attribute(i);
                    // Java: Attribute a2 = n2.attribute(a1.getQName());
                    // Java: answer = this.compare(a1, a2);
                    // Java: if (answer != 0)
                    {
                        // Java: return answer;
                    }
                }
                // Java: answer = this.compareContent(n1, n2);
            }
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(Attribute n1, Attribute n2)
    pub fn compare_attribute(&self) -> i32 {
        // Java: int answer = this.compare(n1.getQName(), n2.getQName());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compare(n1.getValue(), n2.getValue());
            answer = 0;
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(QName n1, QName n2)
    pub fn compare_qname(&self) -> i32 {
        // Java: int answer = this.compare(n1.getNamespaceURI(), n2.getNamespaceURI());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compare(n1.getQualifiedName(), n2.getQualifiedName());
            answer = 0;
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(Namespace n1, Namespace n2)
    pub fn compare_namespace(&self) -> i32 {
        // Java: int answer = this.compare(n1.getURI(), n2.getURI());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compare(n1.getPrefix(), n2.getPrefix());
            answer = 0;
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(CharacterData t1, CharacterData t2)
    pub fn compare_character_data(&self) -> i32 {
        // Java: return this.compare(t1.getText(), t2.getText());
        0
    }

    // Java: int compare(DocumentType o1, DocumentType o2)
    pub fn compare_document_type(&self, o1: Option<&str>, o2: Option<&str>) -> i32 {
        // Java: if (o1 == o2)
        if o1 == o2
        {
            // Java: return 0;
            return 0;
        }
        // Java: else if (o1 == null)
        else if o1.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else if (o2 == null)
        else if o2.is_none()
        {
            // Java: return 1;
            return 1;
        }
        // Java: else
        else
        {
            // Java: int answer = this.compare(o1.getPublicID(), o2.getPublicID());
            let mut answer = 0;
            // Java: if (answer == 0)
            if answer == 0
            {
                // Java: answer = this.compare(o1.getSystemID(), o2.getSystemID());
                // Java: if (answer == 0)
                if answer == 0
                {
                    // Java: answer = this.compare(o1.getName(), o2.getName());
                    answer = 0;
                }
            }
            // Java: return answer;
            answer
        }
    }

    // Java: int compare(Entity n1, Entity n2)
    pub fn compare_entity(&self) -> i32 {
        // Java: int answer = this.compare(n1.getName(), n2.getName());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compare(n1.getText(), n2.getText());
            answer = 0;
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(ProcessingInstruction n1, ProcessingInstruction n2)
    pub fn compare_pi(&self) -> i32 {
        // Java: int answer = this.compare(n1.getTarget(), n2.getTarget());
        let mut answer = 0;
        // Java: if (answer == 0)
        if answer == 0
        {
            // Java: answer = this.compare(n1.getText(), n2.getText());
            answer = 0;
        }
        // Java: return answer;
        answer
    }

    // Java: int compareContent(Branch b1, Branch b2)
    pub fn compareContent(&self) -> i32 {
        // Java: int c1 = b1.nodeCount();
        // Java: int c2 = b2.nodeCount();
        // Java: int answer = c1 - c2;
        let answer = 0;
        // Java: if (answer == 0)
        {
            // Java: for(int i = 0; i < c1; ++i)
            {
                // Java: Node n1 = b1.node(i);
                // Java: Node n2 = b2.node(i);
                // Java: answer = this.compare(n1, n2);
                // Java: if (answer != 0)
                {
                    // Java: break;
                }
            }
        }
        // Java: return answer;
        answer
    }

    // Java: int compare(String o1, String o2)
    pub fn compare_str(&self, o1: Option<&str>, o2: Option<&str>) -> i32 {
        // Java: if (Objects.equals(o1, o2))
        if o1 == o2
        {
            // Java: return 0;
            return 0;
        }
        // Java: else if (o1 == null)
        else if o1.is_none()
        {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else
        {
            // Java: return o2 == null ? 1 : o1.compareTo(o2);
            match o2 {
                None => 1,
                Some(s2) => o1.unwrap().cmp(s2) as i32,
            }
        }
    }
}
