// Java: import java.io.Serializable
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java: public interface Predicate extends Serializable
pub trait Predicate {
    // Java: abstract Expr getExpr()
    fn getExpr(&self) -> &dyn super::Expr::Expr;

    // Java: abstract void setExpr(Expr var1)
    fn setExpr(&mut self, var1: Box<dyn super::Expr::Expr>);

    // Java: abstract void simplify()
    fn simplify(&mut self);

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract Object evaluate(Context var1)
    fn evaluate(&self, var1: &Context) -> Result<Object, JaxenException>;
}
