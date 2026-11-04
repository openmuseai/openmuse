// Java: import java.util.List

// Java: public interface FunctionCallExpr extends Expr
pub trait FunctionCallExpr: super::Expr::Expr {
    // Java: abstract String getPrefix()
    fn getPrefix(&self) -> String;

    // Java: abstract String getFunctionName()
    fn getFunctionName(&self) -> String;

    // Java: abstract void addParameter(Expr var1)
    fn addParameter(&mut self, var1: Box<dyn super::Expr::Expr>);

    // Java: abstract List getParameters()
    fn getParameters(&self) -> &Vec<Box<dyn super::Expr::Expr>>;
}
