// Java: public interface VariableReferenceExpr extends Expr
pub trait VariableReferenceExpr {
    // Java: abstract String getPrefix()
    fn getPrefix(&self) -> String;

    // Java: abstract String getVariableName()
    fn getVariableName(&self) -> String;

}
