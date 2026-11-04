// Java: public enum NodeType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeType {
    // Java: ANY_NODE((short)0, "Node")
    AnyNode,
    // Java: ELEMENT_NODE((short)1, "Element")
    ElementNode,
    // Java: ATTRIBUTE_NODE((short)2, "Attribute")
    AttributeNode,
    // Java: TEXT_NODE((short)3, "Text")
    TextNode,
    // Java: CDATA_SECTION_NODE((short)4, "CDATA")
    CdataSectionNode,
    // Java: ENTITY_REFERENCE_NODE((short)5, "Entity")
    EntityReferenceNode,
    // Java: ENTITY_NODE((short)6, "Entity")
    EntityNode,
    // Java: PROCESSING_INSTRUCTION_NODE((short)7, "ProcessingInstruction")
    ProcessingInstructionNode,
    // Java: COMMENT_NODE((short)8, "Comment")
    CommentNode,
    // Java: DOCUMENT_NODE((short)9, "Document")
    DocumentNode,
    // Java: DOCUMENT_TYPE_NODE((short)10, "DocumentType")
    DocumentTypeNode,
    // Java: DOCUMENT_FRAGMENT_NODE((short)11, "DocumentFragment")
    DocumentFragmentNode,
    // Java: NOTATION_NODE((short)12, "Notation")
    NotationNode,
    // Java: NAMESPACE_NODE((short)13, "Namespace")
    NamespaceNode,
    // Java: UNKNOWN_NODE((short)14, "Unknown")
    UnknownNode,
    // Java field: short code
    // Java field: String name
    // Java field: NodeType[] byCode
}

impl NodeType {
    // Java: NodeType(short code, String name)
    fn new(code: i16, name: &str) -> Self {
        // Java: this.code = code;
        // Java: this.name = name;
        NodeType::byCode(code).unwrap()
    }

    // Java: short getCode()
    pub fn getCode(&self) -> i16 {
        // Java: return this.code;
        match self {
            NodeType::AnyNode => 0,
            NodeType::ElementNode => 1,
            NodeType::AttributeNode => 2,
            NodeType::TextNode => 3,
            NodeType::CdataSectionNode => 4,
            NodeType::EntityReferenceNode => 5,
            NodeType::EntityNode => 6,
            NodeType::ProcessingInstructionNode => 7,
            NodeType::CommentNode => 8,
            NodeType::DocumentNode => 9,
            NodeType::DocumentTypeNode => 10,
            NodeType::DocumentFragmentNode => 11,
            NodeType::NotationNode => 12,
            NodeType::NamespaceNode => 13,
            NodeType::UnknownNode => 14,
        }
    }

    // Java: String getName()
    pub fn getName(&self) -> &str {
        // Java: return this.name;
        match self {
            NodeType::AnyNode => "Node",
            NodeType::ElementNode => "Element",
            NodeType::AttributeNode => "Attribute",
            NodeType::TextNode => "Text",
            NodeType::CdataSectionNode => "CDATA",
            NodeType::EntityReferenceNode => "Entity",
            NodeType::EntityNode => "Entity",
            NodeType::ProcessingInstructionNode => "ProcessingInstruction",
            NodeType::CommentNode => "Comment",
            NodeType::DocumentNode => "Document",
            NodeType::DocumentTypeNode => "DocumentType",
            NodeType::DocumentFragmentNode => "DocumentFragment",
            NodeType::NotationNode => "Notation",
            NodeType::NamespaceNode => "Namespace",
            NodeType::UnknownNode => "Unknown",
        }
    }

    // Java: static NodeType byCode(short code)
    pub fn byCode(code: i16) -> Option<NodeType> {
        // Java: if (code >= 0 && code < byCode.length)
        if code >= 0 && code <= 14 {
            // Java: NodeType nodeType = byCode[code];
            let node_type = match code {
                0 => NodeType::AnyNode,
                1 => NodeType::ElementNode,
                2 => NodeType::AttributeNode,
                3 => NodeType::TextNode,
                4 => NodeType::CdataSectionNode,
                5 => NodeType::EntityReferenceNode,
                6 => NodeType::EntityNode,
                7 => NodeType::ProcessingInstructionNode,
                8 => NodeType::CommentNode,
                9 => NodeType::DocumentNode,
                10 => NodeType::DocumentTypeNode,
                11 => NodeType::DocumentFragmentNode,
                12 => NodeType::NotationNode,
                13 => NodeType::NamespaceNode,
                14 => NodeType::UnknownNode,
                _ => unreachable!(),
            };
            // Java: assert nodeType != null;
            // Java: return nodeType;
            Some(node_type)
        }
        // Java: else
        else {
            // Java: return null;
            None
        }
    }
}
