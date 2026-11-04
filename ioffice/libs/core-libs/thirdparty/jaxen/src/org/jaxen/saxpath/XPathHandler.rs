use super::SAXPathException::SAXPathException;

// Java: public interface XPathHandler
pub trait XPathHandler {
    // Java: abstract void startXPath()
    fn startXPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endXPath()
    fn endXPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startPathExpr()
    fn startPathExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endPathExpr()
    fn endPathExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startAbsoluteLocationPath()
    fn startAbsoluteLocationPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endAbsoluteLocationPath()
    fn endAbsoluteLocationPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startRelativeLocationPath()
    fn startRelativeLocationPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endRelativeLocationPath()
    fn endRelativeLocationPath(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startNameStep(int var1, String var2, String var3)
    fn startNameStep(&mut self, var1: i32, var2: &str, var3: &str) -> Result<(), SAXPathException>;
    // Java: abstract void endNameStep()
    fn endNameStep(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startTextNodeStep(int var1)
    fn startTextNodeStep(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void endTextNodeStep()
    fn endTextNodeStep(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startCommentNodeStep(int var1)
    fn startCommentNodeStep(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void endCommentNodeStep()
    fn endCommentNodeStep(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startAllNodeStep(int var1)
    fn startAllNodeStep(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void endAllNodeStep()
    fn endAllNodeStep(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startProcessingInstructionNodeStep(int var1, String var2)
    fn startProcessingInstructionNodeStep(&mut self, var1: i32, var2: &str) -> Result<(), SAXPathException>;
    // Java: abstract void endProcessingInstructionNodeStep()
    fn endProcessingInstructionNodeStep(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startPredicate()
    fn startPredicate(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endPredicate()
    fn endPredicate(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startFilterExpr()
    fn startFilterExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endFilterExpr()
    fn endFilterExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void startOrExpr()
    fn startOrExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endOrExpr(boolean var1)
    fn endOrExpr(&mut self, var1: bool) -> Result<(), SAXPathException>;
    // Java: abstract void startAndExpr()
    fn startAndExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endAndExpr(boolean var1)
    fn endAndExpr(&mut self, var1: bool) -> Result<(), SAXPathException>;
    // Java: abstract void startEqualityExpr()
    fn startEqualityExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endEqualityExpr(int var1)
    fn endEqualityExpr(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void startRelationalExpr()
    fn startRelationalExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endRelationalExpr(int var1)
    fn endRelationalExpr(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void startAdditiveExpr()
    fn startAdditiveExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endAdditiveExpr(int var1)
    fn endAdditiveExpr(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void startMultiplicativeExpr()
    fn startMultiplicativeExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endMultiplicativeExpr(int var1)
    fn endMultiplicativeExpr(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void startUnaryExpr()
    fn startUnaryExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endUnaryExpr(int var1)
    fn endUnaryExpr(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void startUnionExpr()
    fn startUnionExpr(&mut self) -> Result<(), SAXPathException>;
    // Java: abstract void endUnionExpr(boolean var1)
    fn endUnionExpr(&mut self, var1: bool) -> Result<(), SAXPathException>;
    // Java: abstract void number(int var1)
    fn number_int(&mut self, var1: i32) -> Result<(), SAXPathException>;
    // Java: abstract void number(double var1)
    fn number_double(&mut self, var1: f64) -> Result<(), SAXPathException>;
    // Java: abstract void literal(String var1)
    fn literal(&mut self, var1: &str) -> Result<(), SAXPathException>;
    // Java: abstract void variableReference(String var1, String var2)
    fn variableReference(&mut self, var1: &str, var2: &str) -> Result<(), SAXPathException>;
    // Java: abstract void startFunction(String var1, String var2)
    fn startFunction(&mut self, var1: &str, var2: &str) -> Result<(), SAXPathException>;
    // Java: abstract void endFunction()
    fn endFunction(&mut self) -> Result<(), SAXPathException>;
}
