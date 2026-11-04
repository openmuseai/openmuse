// Java: import org.jaxen.BaseXPath
use core_libs_jaxen::org::jaxen::BaseXPath::BaseXPath;
// Java: import org.jaxen.JaxenException
use core_libs_jaxen::org::jaxen::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -75510941087659775;

// Java: public class Dom4jXPath extends BaseXPath
pub struct Dom4jXPath {
    pub(crate) base: BaseXPath,
}

// Java: public class Dom4jXPath extends BaseXPath
impl Dom4jXPath {
    // Java: Dom4jXPath(String xpathExpr)
    pub fn new(xpath_expr: &str) -> Result<Self, JaxenException> {
        // Java: super(xpathExpr, DocumentNavigator.getInstance());
        let base = BaseXPath::new(xpath_expr.to_string())?;
        Ok(Dom4jXPath { base })
    }
}
