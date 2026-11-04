// Java: public interface LiteralExpr extends Expr
pub trait LiteralExpr {
    // Java: abstract String getLiteral()
    fn getLiteral(&self) -> String;

}
