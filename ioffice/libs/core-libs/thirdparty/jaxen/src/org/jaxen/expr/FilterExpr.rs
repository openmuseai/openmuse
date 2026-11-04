// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;

// Java: public interface FilterExpr extends Expr
pub trait FilterExpr: super::Expr::Expr {
    // Java: abstract boolean asBoolean(Context var1)
    fn asBoolean(&self, var1: &Context) -> Result<bool, JaxenException>;

    // Java: abstract Expr getExpr()
    fn getExpr(&self) -> &dyn super::Expr::Expr;
}
