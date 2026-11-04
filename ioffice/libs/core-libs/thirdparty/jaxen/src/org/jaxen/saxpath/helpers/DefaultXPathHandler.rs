use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
use crate::org::jaxen::saxpath::XPathHandler::XPathHandler;

// Java: public class DefaultXPathHandler implements XPathHandler
pub struct DefaultXPathHandler;

impl DefaultXPathHandler {
    // Java: DefaultXPathHandler()
    pub fn new() -> Self {
      // Java: super();
      DefaultXPathHandler
    }
}

// Java: public class DefaultXPathHandler implements XPathHandler
impl XPathHandler for DefaultXPathHandler {
    // Java: void startXPath()
    fn startXPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endXPath()
    fn endXPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startPathExpr()
    fn startPathExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endPathExpr()
    fn endPathExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startAbsoluteLocationPath()
    fn startAbsoluteLocationPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endAbsoluteLocationPath()
    fn endAbsoluteLocationPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startRelativeLocationPath()
    fn startRelativeLocationPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endRelativeLocationPath()
    fn endRelativeLocationPath(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startNameStep(int axis, String prefix, String localName)
    fn startNameStep(&mut self, _axis: i32, _prefix: &str, _localName: &str) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endNameStep()
    fn endNameStep(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startTextNodeStep(int axis)
    fn startTextNodeStep(&mut self, _axis: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endTextNodeStep()
    fn endTextNodeStep(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startCommentNodeStep(int axis)
    fn startCommentNodeStep(&mut self, _axis: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endCommentNodeStep()
    fn endCommentNodeStep(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startAllNodeStep(int axis)
    fn startAllNodeStep(&mut self, _axis: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endAllNodeStep()
    fn endAllNodeStep(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startProcessingInstructionNodeStep(int axis, String name)
    fn startProcessingInstructionNodeStep(&mut self, _axis: i32, _name: &str) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endProcessingInstructionNodeStep()
    fn endProcessingInstructionNodeStep(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startPredicate()
    fn startPredicate(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endPredicate()
    fn endPredicate(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startFilterExpr()
    fn startFilterExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endFilterExpr()
    fn endFilterExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startOrExpr()
    fn startOrExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endOrExpr(boolean create)
    fn endOrExpr(&mut self, _create: bool) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startAndExpr()
    fn startAndExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endAndExpr(boolean create)
    fn endAndExpr(&mut self, _create: bool) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startEqualityExpr()
    fn startEqualityExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endEqualityExpr(int operator)
    fn endEqualityExpr(&mut self, _operator: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startRelationalExpr()
    fn startRelationalExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endRelationalExpr(int operator)
    fn endRelationalExpr(&mut self, _operator: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startAdditiveExpr()
    fn startAdditiveExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endAdditiveExpr(int operator)
    fn endAdditiveExpr(&mut self, _operator: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startMultiplicativeExpr()
    fn startMultiplicativeExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endMultiplicativeExpr(int operator)
    fn endMultiplicativeExpr(&mut self, _operator: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startUnaryExpr()
    fn startUnaryExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endUnaryExpr(int operator)
    fn endUnaryExpr(&mut self, _operator: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startUnionExpr()
    fn startUnionExpr(&mut self) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endUnionExpr(boolean create)
    fn endUnionExpr(&mut self, _create: bool) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void number(int number)
    fn number_int(&mut self, _number: i32) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void number(double number)
    fn number_double(&mut self, _number: f64) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void literal(String literal)
    fn literal(&mut self, _literal: &str) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void variableReference(String prefix, String variableName)
    fn variableReference(&mut self, _prefix: &str, _variableName: &str) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void startFunction(String prefix, String functionName)
    fn startFunction(&mut self, _prefix: &str, _functionName: &str) -> Result<(), SAXPathException> { Ok(()) }
    // Java: void endFunction()
    fn endFunction(&mut self) -> Result<(), SAXPathException> { Ok(()) }
}
