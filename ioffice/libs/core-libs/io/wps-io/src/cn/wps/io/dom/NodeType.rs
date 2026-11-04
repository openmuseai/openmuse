use std::sync::Arc;

// Java field: NodeType[] byCode
static BY_CODE: std::sync::OnceLock<Vec<Option<NodeType>>> = std::sync::OnceLock::new();

fn init_by_code() -> Vec<Option<NodeType>> {
    let values = NodeType::values();
    let len = values.iter().map(|v| v.code as usize + 1).max().unwrap_or(0);
    let mut table: Vec<Option<NodeType>> = vec![None; len];
    for nt in values {
        let code = nt.code as usize;
        table[code] = Some(nt);
    }
    table
}

// Java: public enum NodeType
#[derive(Clone, Debug)]
pub struct NodeType {
    // Java field: short code
    pub(crate) code: i16,
    // Java field: String name
    pub(crate) name: Arc<String>,
    // Java field: NodeType[] byCode
}

// Java: public enum NodeType
impl NodeType {
    // Java: NodeType(short code, String name)
    pub fn new(code: i16, name: &str) -> Self {
        NodeType {
            // Java: this.code = code;
            code,
            // Java: this.name = name;
            name: Arc::new(name.to_string()),
        }
    }

    // Java: short getCode()
    pub fn getCode(&self) -> i16 {
        // Java: return this.code;
        self.code
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.name;
        &self.name
    }

    // Java: static NodeType byCode(short code)
    pub fn byCode(code: i16) -> Option<NodeType> {
        let by_code = BY_CODE.get_or_init(init_by_code);
        // Java: if (code >= 0 && code < byCode.length)
        if code >= 0 && (code as usize) < by_code.len() {
            // Java: NodeType nodeType = byCode[code];
            let node_type = by_code[code as usize].clone();
            // Java: assert nodeType != null;
            assert!(node_type.is_some());
            // Java: return nodeType;
            node_type
        // Java: else
        } else {
            // Java: return null;
            None
        }
    }

    fn values() -> Vec<NodeType> {
        vec![
            NodeType::new(0, "Node"),
            NodeType::new(1, "Element"),
            NodeType::new(2, "Attribute"),
            NodeType::new(3, "Text"),
            NodeType::new(4, "CDATA"),
            NodeType::new(5, "Entity"),
            NodeType::new(6, "Entity"),
            NodeType::new(7, "ProcessingInstruction"),
            NodeType::new(8, "Comment"),
            NodeType::new(9, "Document"),
            NodeType::new(10, "DocumentType"),
            NodeType::new(11, "DocumentFragment"),
            NodeType::new(12, "Notation"),
            NodeType::new(13, "Namespace"),
            NodeType::new(14, "Unknown"),
        ]
    }

    pub fn ANY_NODE() -> NodeType { NodeType::new(0, "Node") }
    pub fn ELEMENT_NODE() -> NodeType { NodeType::new(1, "Element") }
    pub fn ATTRIBUTE_NODE() -> NodeType { NodeType::new(2, "Attribute") }
    pub fn TEXT_NODE() -> NodeType { NodeType::new(3, "Text") }
    pub fn CDATA_SECTION_NODE() -> NodeType { NodeType::new(4, "CDATA") }
    pub fn ENTITY_REFERENCE_NODE() -> NodeType { NodeType::new(5, "Entity") }
    pub fn ENTITY_NODE() -> NodeType { NodeType::new(6, "Entity") }
    pub fn PROCESSING_INSTRUCTION_NODE() -> NodeType { NodeType::new(7, "ProcessingInstruction") }
    pub fn COMMENT_NODE() -> NodeType { NodeType::new(8, "Comment") }
    pub fn DOCUMENT_NODE() -> NodeType { NodeType::new(9, "Document") }
    pub fn DOCUMENT_TYPE_NODE() -> NodeType { NodeType::new(10, "DocumentType") }
    pub fn DOCUMENT_FRAGMENT_NODE() -> NodeType { NodeType::new(11, "DocumentFragment") }
    pub fn NOTATION_NODE() -> NodeType { NodeType::new(12, "Notation") }
    pub fn NAMESPACE_NODE() -> NodeType { NodeType::new(13, "Namespace") }
    pub fn UNKNOWN_NODE() -> NodeType { NodeType::new(14, "Unknown") }
}
