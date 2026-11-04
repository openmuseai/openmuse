use super::super::NodeType::NodeType;

// Java: public class AbstractEntity extends AbstractNode implements Entity
pub struct AbstractEntity {
    pub(crate) name: String,
}

// Java: public class AbstractEntity extends AbstractNode implements Entity
impl AbstractEntity {
    // Java: AbstractEntity()
    pub fn new() -> Self {
        // Java: super();
        AbstractEntity { name: String::new() }
    }

    pub fn getName(&self) -> &str {
        &self.name
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> NodeType {
        // Java: return NodeType.ENTITY_REFERENCE_NODE;
        NodeType::ENTITY_REFERENCE_NODE()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + " [Entity: &" + this.getName() + ";]";
        format!(" [Entity: &{};]", self.getName())
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
        // Java: return "&" + this.getName() + ";";
        format!("&{};", self.getName())
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
        // Java: return "&" + this.getName() + ";";
        format!("&{};", self.getName())
    }
}
