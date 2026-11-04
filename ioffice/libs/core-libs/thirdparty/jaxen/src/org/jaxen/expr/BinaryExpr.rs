use super::Expr::Expr;

// Java: public interface BinaryExpr extends Expr
pub trait BinaryExpr: Expr {
    // Java: abstract Expr getLHS()
    fn getLHS(&self) -> &dyn Expr;

    // Java: abstract Expr getRHS()
    fn getRHS(&self) -> &dyn Expr;

    // Java: abstract String getOperator()
    fn getOperator(&self) -> String;
}
