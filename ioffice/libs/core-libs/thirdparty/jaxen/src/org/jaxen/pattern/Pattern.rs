// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java field: short ELEMENT_NODE
pub const ELEMENT_NODE: i16 = 1;
// Java field: short ATTRIBUTE_NODE
pub const ATTRIBUTE_NODE: i16 = 2;
// Java field: short TEXT_NODE
pub const TEXT_NODE: i16 = 3;
// Java field: short CDATA_SECTION_NODE
pub const CDATA_SECTION_NODE: i16 = 4;
// Java field: short ENTITY_REFERENCE_NODE
pub const ENTITY_REFERENCE_NODE: i16 = 5;
// Java field: short PROCESSING_INSTRUCTION_NODE
pub const PROCESSING_INSTRUCTION_NODE: i16 = 7;
// Java field: short COMMENT_NODE
pub const COMMENT_NODE: i16 = 8;
// Java field: short DOCUMENT_NODE
pub const DOCUMENT_NODE: i16 = 9;
// Java field: short DOCUMENT_TYPE_NODE
pub const DOCUMENT_TYPE_NODE: i16 = 10;
// Java field: short NAMESPACE_NODE
pub const NAMESPACE_NODE: i16 = 13;
// Java field: short UNKNOWN_NODE
pub const UNKNOWN_NODE: i16 = 14;
// Java field: short MAX_NODE_TYPE
pub const MAX_NODE_TYPE: i16 = 14;
// Java field: short ANY_NODE
pub const ANY_NODE: i16 = 0;
// Java field: short NO_NODE
pub const NO_NODE: i16 = 14;

// Java: public class Pattern
pub trait Pattern {
    // Java: Pattern()
    // Java: super();

    // Java: abstract boolean matches(Object var1, Context var2)
    fn matches(&self, node: &Object, context: &Context) -> Result<bool, JaxenException>;

    // Java: double getPriority()
    fn getPriority(&self) -> f64 {
      // Java: return (double)0.5F;
      0.5
    }

    // Java: Pattern[] getUnionPatterns()
    fn getUnionPatterns(&self) -> Option<Vec<Box<dyn Pattern>>> {
      // Java: return null;
      None
    }

    // Java: short getMatchType()
    fn getMatchType(&self) -> i16 {
      // Java: return 0;
      0
    }

    // Java: String getMatchesNodeName()
    fn getMatchesNodeName(&self) -> Option<String> {
      // Java: return null;
      None
    }

    // Java: Pattern simplify()
    // Java: return this;

    // Java: abstract String getText()
    fn getText(&self) -> String;
}
