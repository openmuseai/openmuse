// Java: import java.util.Iterator
// Java: import java.util.LinkedList
// Java: import org.jaxen.expr.DefaultXPathFactory
use crate::org::jaxen::expr::DefaultXPathFactory::DefaultXPathFactory;
// Java: import org.jaxen.expr.Expr
use crate::org::jaxen::expr::Expr::Expr;
// Java: import org.jaxen.expr.FilterExpr
// Java: import org.jaxen.expr.FunctionCallExpr
// Java: import org.jaxen.expr.LocationPath
// Java: import org.jaxen.expr.Predicate
use crate::org::jaxen::expr::Predicate::Predicate;
// Java: import org.jaxen.expr.Predicated
// Java: import org.jaxen.expr.Step
// Java: import org.jaxen.expr.XPathExpr
use crate::org::jaxen::expr::XPathExpr::XPathExpr;
// Java: import org.jaxen.expr.XPathFactory
use crate::org::jaxen::expr::XPathFactory::XPathFactory;
// Java: import org.jaxen.saxpath.XPathHandler
use crate::org::jaxen::JaxenException::JaxenException;

type StackFrame = Vec<Box<dyn Expr>>;

// Java: public class JaxenHandler implements XPathHandler
pub struct JaxenHandler {
    // Java field: XPathFactory xpathFactory
    pub(crate) xpathFactory: Box<dyn XPathFactory>,
    // Java field: XPathExpr xpath
    pub(crate) xpath: Option<Box<dyn XPathExpr>>,
    // Java field: boolean simplified
    pub(crate) simplified: bool,
    // Java field: LinkedList stack
    pub(crate) stack: Vec<StackFrame>,
}

// Java: public class JaxenHandler implements XPathHandler
impl JaxenHandler {
    // Java: JaxenHandler()
    pub fn new() -> Self {
      // Java: super();
      JaxenHandler {
          xpathFactory: Box::new(DefaultXPathFactory::new()),
          xpath: None,
          simplified: false,
          stack: Vec::new(),
      }
    }

    // Java: void setXPathFactory(XPathFactory xpathFactory)
    pub fn setXPathFactory(&mut self, xpathFactory: Box<dyn XPathFactory>) {
      // Java: this.xpathFactory = xpathFactory;
      self.xpathFactory = xpathFactory;
    }

    // Java: XPathFactory getXPathFactory()
    pub fn getXPathFactory(&self) -> &dyn XPathFactory {
      // Java: return this.xpathFactory;
      &*self.xpathFactory
    }

    // Java: XPathExpr getXPathExpr()
    pub fn getXPathExpr_default(&mut self) -> Option<&dyn XPathExpr> {
      // Java: return this.getXPathExpr(true);
      self.getXPathExpr(true)
    }

    // Java: XPathExpr getXPathExpr(boolean shouldSimplify)
    pub fn getXPathExpr(&mut self, shouldSimplify: bool) -> Option<&dyn XPathExpr> {
      // Java: if (shouldSimplify && !this.simplified)
      if shouldSimplify && !self.simplified {
         // Java: this.xpath.simplify();
         if let Some(ref mut xpath) = self.xpath {
             xpath.simplify();
         }
         // Java: this.simplified = true;
         self.simplified = true;
      }
      // Java: return this.xpath;
      self.xpath.as_deref()
    }

    // Java: void startXPath()
    pub fn startXPath(&mut self) {
      // Java: this.simplified = false;
      self.simplified = false;
      // Java: this.pushFrame();
      self.pushFrame();
    }

    // Java: void endXPath()
    pub fn endXPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.xpath = this.getXPathFactory().createXPath((Expr)this.pop());
      let expr = self.pop();
      self.xpath = Some(self.xpathFactory.createXPath(expr)?);
      // Java: this.popFrame();
      self.popFrame();
      Ok(())
    }

    // Java: void startPathExpr()
    pub fn startPathExpr(&mut self) {
      // Java: this.pushFrame();
      self.pushFrame();
    }

    // Java: void endPathExpr()
    pub fn endPathExpr(&mut self) -> Result<(), JaxenException> {
      // Java: FilterExpr filterExpr;
      // Java: LocationPath locationPath;
      // Java: if (this.stackSize() == 2)
      if self.stackSize() == 2 {
         // Java: locationPath = (LocationPath)this.pop();
         let location_path = self.pop();
         // Java: filterExpr = (FilterExpr)this.pop();
         let filter_expr = self.pop();
         // Java: this.popFrame();
         self.popFrame();
         // Java: this.push(this.getXPathFactory().createPathExpr(filterExpr, locationPath));
         let result = self.xpathFactory.createPathExpr(filter_expr, location_path)?;
         self.push(result);
      }
      // Java: else
      else {
         // Java: Object popped = this.pop();
         let popped = self.pop();
         // Java: if (popped instanceof LocationPath)
         // Java: locationPath = (LocationPath)popped;
         // Java: filterExpr = null;
         // Java: else
         // Java: locationPath = null;
         // Java: filterExpr = (FilterExpr)popped;
         // Java: this.popFrame();
         self.popFrame();
         // Java: this.push(this.getXPathFactory().createPathExpr(filterExpr, locationPath));
         self.push(popped);
      }
      Ok(())
    }

    // Java: void startAbsoluteLocationPath()
    pub fn startAbsoluteLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createAbsoluteLocationPath());
      let path = self.xpathFactory.createAbsoluteLocationPath()?;
      self.push(path);
      Ok(())
    }

    // Java: void endAbsoluteLocationPath()
    pub fn endAbsoluteLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.endLocationPath();
      self.endLocationPath()
    }

    // Java: void startRelativeLocationPath()
    pub fn startRelativeLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createRelativeLocationPath());
      let path = self.xpathFactory.createRelativeLocationPath()?;
      self.push(path);
      Ok(())
    }

    // Java: void endRelativeLocationPath()
    pub fn endRelativeLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.endLocationPath();
      self.endLocationPath()
    }

    // Java: void endLocationPath()
    pub fn endLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: LocationPath path = (LocationPath)this.peekFrame().removeFirst();
      // Java: this.addSteps(path, this.popFrame().iterator());
      // Java: this.push(path);
      todo!("endLocationPath requires LocationPath downcasting")
    }

    // Java: void addSteps(LocationPath locationPath, Iterator stepIter)
    pub fn addSteps(&mut self, _locationPath: &mut dyn Expr, _steps: Vec<Box<dyn Expr>>) {
      // Java: while(stepIter.hasNext())
      // Java: locationPath.addStep((Step)stepIter.next());
      todo!("addSteps requires LocationPath downcasting")
    }

    // Java: void startNameStep(int axis, String prefix, String localName)
    pub fn startNameStep(&mut self, axis: i32, prefix: &str, localName: &str) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createNameStep(axis, prefix, localName));
      let step = self.xpathFactory.createNameStep(axis, prefix, localName)?;
      self.push(step);
      Ok(())
    }

    // Java: void endNameStep()
    pub fn endNameStep(&mut self) {
      // Java: this.endStep();
      self.endStep();
    }

    // Java: void startTextNodeStep(int axis)
    pub fn startTextNodeStep(&mut self, axis: i32) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createTextNodeStep(axis));
      let step = self.xpathFactory.createTextNodeStep(axis)?;
      self.push(step);
      Ok(())
    }

    // Java: void endTextNodeStep()
    pub fn endTextNodeStep(&mut self) {
      // Java: this.endStep();
      self.endStep();
    }

    // Java: void startCommentNodeStep(int axis)
    pub fn startCommentNodeStep(&mut self, axis: i32) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createCommentNodeStep(axis));
      let step = self.xpathFactory.createCommentNodeStep(axis)?;
      self.push(step);
      Ok(())
    }

    // Java: void endCommentNodeStep()
    pub fn endCommentNodeStep(&mut self) {
      // Java: this.endStep();
      self.endStep();
    }

    // Java: void startAllNodeStep(int axis)
    pub fn startAllNodeStep(&mut self, axis: i32) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createAllNodeStep(axis));
      let step = self.xpathFactory.createAllNodeStep(axis)?;
      self.push(step);
      Ok(())
    }

    // Java: void endAllNodeStep()
    pub fn endAllNodeStep(&mut self) {
      // Java: this.endStep();
      self.endStep();
    }

    // Java: void startProcessingInstructionNodeStep(int axis, String name)
    pub fn startProcessingInstructionNodeStep(&mut self, axis: i32, name: &str) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createProcessingInstructionNodeStep(axis, name));
      let step = self.xpathFactory.createProcessingInstructionNodeStep(axis, name)?;
      self.push(step);
      Ok(())
    }

    // Java: void endProcessingInstructionNodeStep()
    pub fn endProcessingInstructionNodeStep(&mut self) {
      // Java: this.endStep();
      self.endStep();
    }

    // Java: void endStep()
    pub fn endStep(&mut self) {
      // Java: Step step = (Step)this.peekFrame().removeFirst();
      // Java: this.addPredicates(step, this.popFrame().iterator());
      // Java: this.push(step);
      todo!("endStep requires Step/Predicated downcasting")
    }

    // Java: void startPredicate()
    pub fn startPredicate(&mut self) {
      // Java: this.pushFrame();
      self.pushFrame();
    }

    // Java: void endPredicate()
    pub fn endPredicate(&mut self) -> Result<(), JaxenException> {
      // Java: Predicate predicate = this.getXPathFactory().createPredicate((Expr)this.pop());
      // Java: this.popFrame();
      // Java: this.push(predicate);
      todo!("endPredicate requires Predicate -> Expr conversion")
    }

    // Java: void startFilterExpr()
    pub fn startFilterExpr(&mut self) {
      // Java: this.pushFrame();
      self.pushFrame();
    }

    // Java: void endFilterExpr()
    pub fn endFilterExpr(&mut self) -> Result<(), JaxenException> {
      // Java: Expr expr = (Expr)this.peekFrame().removeFirst();
      // Java: FilterExpr filter = this.getXPathFactory().createFilterExpr(expr);
      // Java: Iterator predIter = this.popFrame().iterator();
      // Java: this.addPredicates(filter, predIter);
      // Java: this.push(filter);
      todo!("endFilterExpr requires FilterExpr/Predicated downcasting")
    }

    // Java: void addPredicates(Predicated obj, Iterator predIter)
    pub fn addPredicates(&mut self, _obj: &mut dyn Expr, _predicates: Vec<Box<dyn Expr>>) {
      // Java: while(predIter.hasNext())
      // Java: obj.addPredicate((Predicate)predIter.next());
      todo!("addPredicates requires Predicated downcasting")
    }

    // Java: void returnExpr()
    pub fn returnExpr(&mut self) {
      // Java: Expr expr = (Expr)this.pop();
      let expr = self.pop();
      // Java: this.popFrame();
      self.popFrame();
      // Java: this.push(expr);
      self.push(expr);
    }

    // Java: void startOrExpr()
    pub fn startOrExpr(&self) {
    }

    // Java: void endOrExpr(boolean create)
    pub fn endOrExpr(&mut self, create: bool) -> Result<(), JaxenException> {
      // Java: if (create)
      if create {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createOrExpr(lhs, rhs));
         let result = self.xpathFactory.createOrExpr(lhs, rhs)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startAndExpr()
    pub fn startAndExpr(&self) {
    }

    // Java: void endAndExpr(boolean create)
    pub fn endAndExpr(&mut self, create: bool) -> Result<(), JaxenException> {
      // Java: if (create)
      if create {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createAndExpr(lhs, rhs));
         let result = self.xpathFactory.createAndExpr(lhs, rhs)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startEqualityExpr()
    pub fn startEqualityExpr(&self) {
    }

    // Java: void endEqualityExpr(int operator)
    pub fn endEqualityExpr(&mut self, operator: i32) -> Result<(), JaxenException> {
      // Java: if (operator != 0)
      if operator != 0 {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createEqualityExpr(lhs, rhs, operator));
         let result = self.xpathFactory.createEqualityExpr(lhs, rhs, operator)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startRelationalExpr()
    pub fn startRelationalExpr(&self) {
    }

    // Java: void endRelationalExpr(int operator)
    pub fn endRelationalExpr(&mut self, operator: i32) -> Result<(), JaxenException> {
      // Java: if (operator != 0)
      if operator != 0 {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createRelationalExpr(lhs, rhs, operator));
         let result = self.xpathFactory.createRelationalExpr(lhs, rhs, operator)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startAdditiveExpr()
    pub fn startAdditiveExpr(&self) {
    }

    // Java: void endAdditiveExpr(int operator)
    pub fn endAdditiveExpr(&mut self, operator: i32) -> Result<(), JaxenException> {
      // Java: if (operator != 0)
      if operator != 0 {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createAdditiveExpr(lhs, rhs, operator));
         let result = self.xpathFactory.createAdditiveExpr(lhs, rhs, operator)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startMultiplicativeExpr()
    pub fn startMultiplicativeExpr(&self) {
    }

    // Java: void endMultiplicativeExpr(int operator)
    pub fn endMultiplicativeExpr(&mut self, operator: i32) -> Result<(), JaxenException> {
      // Java: if (operator != 0)
      if operator != 0 {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createMultiplicativeExpr(lhs, rhs, operator));
         let result = self.xpathFactory.createMultiplicativeExpr(lhs, rhs, operator)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startUnaryExpr()
    pub fn startUnaryExpr(&self) {
    }

    // Java: void endUnaryExpr(int operator)
    pub fn endUnaryExpr(&mut self, operator: i32) -> Result<(), JaxenException> {
      // Java: if (operator != 0)
      if operator != 0 {
         // Java: this.push(this.getXPathFactory().createUnaryExpr((Expr)this.pop(), operator));
         let expr = self.pop();
         let result = self.xpathFactory.createUnaryExpr(expr, operator)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void startUnionExpr()
    pub fn startUnionExpr(&self) {
    }

    // Java: void endUnionExpr(boolean create)
    pub fn endUnionExpr(&mut self, create: bool) -> Result<(), JaxenException> {
      // Java: if (create)
      if create {
         // Java: Expr rhs = (Expr)this.pop();
         let rhs = self.pop();
         // Java: Expr lhs = (Expr)this.pop();
         let lhs = self.pop();
         // Java: this.push(this.getXPathFactory().createUnionExpr(lhs, rhs));
         let result = self.xpathFactory.createUnionExpr(lhs, rhs)?;
         self.push(result);
      }
      Ok(())
    }

    // Java: void number(int number)
    pub fn number_int(&mut self, number: i32) -> Result<(), JaxenException> {
      // Java: this.push(this.getXPathFactory().createNumberExpr(number));
      let result = self.xpathFactory.createNumberExpr_int(number)?;
      self.push(result);
      Ok(())
    }

    // Java: void number(double number)
    pub fn number_double(&mut self, number: f64) -> Result<(), JaxenException> {
      // Java: this.push(this.getXPathFactory().createNumberExpr(number));
      let result = self.xpathFactory.createNumberExpr_double(number)?;
      self.push(result);
      Ok(())
    }

    // Java: void literal(String literal)
    pub fn literal(&mut self, literal: &str) -> Result<(), JaxenException> {
      // Java: this.push(this.getXPathFactory().createLiteralExpr(literal));
      let result = self.xpathFactory.createLiteralExpr(literal)?;
      self.push(result);
      Ok(())
    }

    // Java: void variableReference(String prefix, String variableName)
    pub fn variableReference(&mut self, prefix: &str, variableName: &str) -> Result<(), JaxenException> {
      // Java: this.push(this.getXPathFactory().createVariableReferenceExpr(prefix, variableName));
      let result = self.xpathFactory.createVariableReferenceExpr(prefix, variableName)?;
      self.push(result);
      Ok(())
    }

    // Java: void startFunction(String prefix, String functionName)
    pub fn startFunction(&mut self, prefix: &str, functionName: &str) -> Result<(), JaxenException> {
      // Java: this.pushFrame();
      self.pushFrame();
      // Java: this.push(this.getXPathFactory().createFunctionCallExpr(prefix, functionName));
      let result = self.xpathFactory.createFunctionCallExpr(prefix, functionName)?;
      self.push(result);
      Ok(())
    }

    // Java: void endFunction()
    pub fn endFunction(&mut self) {
      // Java: FunctionCallExpr function = (FunctionCallExpr)this.peekFrame().removeFirst();
      // Java: this.addParameters(function, this.popFrame().iterator());
      // Java: this.push(function);
      todo!("endFunction requires FunctionCallExpr downcasting")
    }

    // Java: void addParameters(FunctionCallExpr function, Iterator paramIter)
    pub fn addParameters(&mut self, _function: &mut dyn Expr, _params: Vec<Box<dyn Expr>>) {
      // Java: while(paramIter.hasNext())
      // Java: function.addParameter((Expr)paramIter.next());
      todo!("addParameters requires FunctionCallExpr downcasting")
    }

    // Java: int stackSize()
    pub fn stackSize(&self) -> i32 {
      // Java: return this.peekFrame().size();
      self.stack.last().map_or(0, |f| f.len() as i32)
    }

    // Java: void push(Object obj)
    pub fn push(&mut self, obj: Box<dyn Expr>) {
      // Java: this.peekFrame().addLast(obj);
      if let Some(frame) = self.stack.last_mut() {
          frame.push(obj);
      }
    }

    // Java: Object pop()
    pub fn pop(&mut self) -> Box<dyn Expr> {
      // Java: return this.peekFrame().removeLast();
      self.stack.last_mut().and_then(|f| f.pop()).expect("Stack underflow")
    }

    // Java: boolean canPop()
    pub fn canPop(&self) -> bool {
      // Java: return this.peekFrame().size() > 0;
      self.stack.last().map_or(false, |f| !f.is_empty())
    }

    // Java: void pushFrame()
    pub fn pushFrame(&mut self) {
      // Java: this.stack.addLast(new LinkedList());
      self.stack.push(Vec::new());
    }

    // Java: LinkedList popFrame()
    pub fn popFrame(&mut self) -> StackFrame {
      // Java: return (LinkedList)this.stack.removeLast();
      self.stack.pop().expect("Frame stack underflow")
    }

    // Java: LinkedList peekFrame()
    pub fn peekFrame(&self) -> &StackFrame {
      // Java: return (LinkedList)this.stack.getLast();
      self.stack.last().expect("Frame stack empty")
    }
}
