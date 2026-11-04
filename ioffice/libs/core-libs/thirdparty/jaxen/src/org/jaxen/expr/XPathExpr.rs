use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::List;
use super::Expr::Expr;

// Java: public interface XPathExpr extends Serializable
pub trait XPathExpr {
    // Java: abstract Expr getRootExpr()
    fn getRootExpr(&self) -> &dyn Expr;

    // Java: abstract void setRootExpr(Expr var1)
    fn setRootExpr(&mut self, var1: Box<dyn Expr>);

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract void simplify()
    fn simplify(&mut self);

    // Java: abstract List asList(Context var1)
    fn asList(&self, var1: &Context) -> Result<List, JaxenException>;
}
