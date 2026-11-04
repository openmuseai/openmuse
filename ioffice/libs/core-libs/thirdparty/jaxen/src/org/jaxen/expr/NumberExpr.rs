// Java: public interface NumberExpr extends Expr
pub trait NumberExpr {
    // Java: abstract Number getNumber()
    fn getNumber(&self) -> f64;
}
