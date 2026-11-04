use crate::org::jaxen::Context::Context;
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;

// Java: public interface Expr extends Serializable
pub trait Expr {
    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract Expr simplify()
    fn simplify(&self) -> Box<dyn Expr>;

    // Java: abstract Object evaluate(Context var1)
    fn evaluate(&self, var1: &Context) -> Result<Object, JaxenException>;
}
