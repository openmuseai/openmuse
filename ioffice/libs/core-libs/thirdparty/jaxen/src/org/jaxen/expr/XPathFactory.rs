// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;

// Java: public interface XPathFactory
pub trait XPathFactory {
    // Java: abstract XPathExpr createXPath(Expr var1)
    fn createXPath(&self, var1: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::XPathExpr::XPathExpr>, JaxenException>;

    // Java: abstract PathExpr createPathExpr(FilterExpr var1, LocationPath var2)
    fn createPathExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract LocationPath createRelativeLocationPath()
    fn createRelativeLocationPath(&self) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract LocationPath createAbsoluteLocationPath()
    fn createAbsoluteLocationPath(&self) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createOrExpr(Expr var1, Expr var2)
    fn createOrExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createAndExpr(Expr var1, Expr var2)
    fn createAndExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createEqualityExpr(Expr var1, Expr var2, int var3)
    fn createEqualityExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>, var3: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createRelationalExpr(Expr var1, Expr var2, int var3)
    fn createRelationalExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>, var3: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createAdditiveExpr(Expr var1, Expr var2, int var3)
    fn createAdditiveExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>, var3: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract BinaryExpr createMultiplicativeExpr(Expr var1, Expr var2, int var3)
    fn createMultiplicativeExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>, var3: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Expr createUnaryExpr(Expr var1, int var2)
    fn createUnaryExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract UnionExpr createUnionExpr(Expr var1, Expr var2)
    fn createUnionExpr(&self, var1: Box<dyn super::Expr::Expr>, var2: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract FilterExpr createFilterExpr(Expr var1)
    fn createFilterExpr(&self, var1: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract FunctionCallExpr createFunctionCallExpr(String var1, String var2)
    fn createFunctionCallExpr(&self, var1: &str, var2: &str) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract NumberExpr createNumberExpr(int var1)
    fn createNumberExpr_int(&self, var1: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract NumberExpr createNumberExpr(double var1)
    fn createNumberExpr_double(&self, var1: f64) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract LiteralExpr createLiteralExpr(String var1)
    fn createLiteralExpr(&self, var1: &str) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract VariableReferenceExpr createVariableReferenceExpr(String var1, String var2)
    fn createVariableReferenceExpr(&self, var1: &str, var2: &str) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Step createNameStep(int var1, String var2, String var3)
    fn createNameStep(&self, var1: i32, var2: &str, var3: &str) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Step createAllNodeStep(int var1)
    fn createAllNodeStep(&self, var1: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Step createCommentNodeStep(int var1)
    fn createCommentNodeStep(&self, var1: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Step createTextNodeStep(int var1)
    fn createTextNodeStep(&self, var1: i32) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Step createProcessingInstructionNodeStep(int var1, String var2)
    fn createProcessingInstructionNodeStep(&self, var1: i32, var2: &str) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;

    // Java: abstract Predicate createPredicate(Expr var1)
    fn createPredicate(&self, var1: Box<dyn super::Expr::Expr>) -> Result<Box<dyn super::Predicate::Predicate>, JaxenException>;

    // Java: abstract PredicateSet createPredicateSet()
    fn createPredicateSet(&self) -> Result<Box<dyn super::Expr::Expr>, JaxenException>;
}
