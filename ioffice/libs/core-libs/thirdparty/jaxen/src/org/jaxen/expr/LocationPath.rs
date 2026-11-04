// Java: import java.util.List

// Java: public interface LocationPath extends Expr
pub trait LocationPath: super::Expr::Expr {
    // Java: abstract void addStep(Step var1)
    fn addStep(&mut self, var1: Box<dyn super::Expr::Expr>);

    // Java: abstract List getSteps()
    fn getSteps(&self) -> &Vec<Box<dyn super::Expr::Expr>>;

    // Java: abstract boolean isAbsolute()
    fn isAbsolute(&self) -> bool;
}
