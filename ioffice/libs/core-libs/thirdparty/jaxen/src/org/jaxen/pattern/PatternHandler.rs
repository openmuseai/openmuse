// Java: import java.util.LinkedList
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.JaxenHandler
use crate::org::jaxen::JaxenHandler::JaxenHandler;
// Java: import org.jaxen.expr.Expr
// Java: import org.jaxen.expr.FilterExpr
use super::Pattern::Pattern;
use super::LocationPathPattern::LocationPathPattern;
use super::NodeTypeTest::NodeTypeTest;
use super::NamespaceTest::NamespaceTest;
use super::NameTest::NameTest;
use super::AnyNodeTest::AnyNodeTest;

// Java: public class PatternHandler extends JaxenHandler
pub struct PatternHandler {
    pub(crate) base: JaxenHandler,
    // Java field: Pattern pattern
    pub(crate) pattern: Option<Box<dyn Pattern>>,
}

// Java: public class PatternHandler extends JaxenHandler
impl PatternHandler {
    // Java: PatternHandler()
    pub fn new() -> Self {
      // Java: super();
      PatternHandler {
         base: JaxenHandler::new(),
         pattern: None,
      }
    }

    // Java: Pattern getPattern()
    pub fn getPattern(&mut self) -> &Option<Box<dyn Pattern>> {
      // Java: return this.getPattern(true);
      &self.pattern
    }

    // Java: Pattern getPattern(boolean shouldSimplify)
    pub fn getPattern_with_simplify(&mut self, _shouldSimplify: bool) -> &Option<Box<dyn Pattern>> {
      // Java: if (shouldSimplify && !this.simplified)
      // Java: this.pattern.simplify();
      // Java: this.simplified = true;
      // Java: return this.pattern;
      &self.pattern
    }

    // Java: void endXPath()
    pub fn endXPath(&mut self) {
      // Java: this.pattern = (Pattern)this.pop();
      // Java: System.out.println("stack is: " + this.stack);
      // Java: this.popFrame();
      todo!("PatternHandler::endXPath")
    }

    // Java: void endPathExpr()
    pub fn endPathExpr(&mut self) {
      // Java: LinkedList frame = this.popFrame();
      // Java: System.out.println("endPathExpr(): " + frame);
      // Java: this.push(frame.removeFirst());
      todo!("PatternHandler::endPathExpr")
    }

    // Java: void startAbsoluteLocationPath()
    pub fn startAbsoluteLocationPath(&mut self) {
      // Java: this.pushFrame();
      // Java: this.push(this.createAbsoluteLocationPath());
      todo!("PatternHandler::startAbsoluteLocationPath")
    }

    // Java: void endAbsoluteLocationPath()
    pub fn endAbsoluteLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.endLocationPath();
      todo!("PatternHandler::endAbsoluteLocationPath")
    }

    // Java: void startRelativeLocationPath()
    pub fn startRelativeLocationPath(&mut self) {
      // Java: this.pushFrame();
      // Java: this.push(this.createRelativeLocationPath());
      todo!("PatternHandler::startRelativeLocationPath")
    }

    // Java: void endRelativeLocationPath()
    pub fn endRelativeLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: this.endLocationPath();
      todo!("PatternHandler::endRelativeLocationPath")
    }

    // Java: void endLocationPath()
    pub fn endLocationPath(&mut self) -> Result<(), JaxenException> {
      // Java: LinkedList list = this.popFrame();
      // Java: System.out.println("endLocationPath: " + list);
      // Java: LocationPathPattern locationPath = (LocationPathPattern)list.removeFirst();
      // Java: this.push(locationPath);
      // Java: boolean doneNodeTest = false;
      // Java: while(!list.isEmpty())
      // Java: Object filter = list.removeFirst();
      // Java: if (filter instanceof NodeTest)
      // Java: if (doneNodeTest)
      // Java: LocationPathPattern parent = new LocationPathPattern((NodeTest)filter);
      // Java: locationPath.setParentPattern(parent);
      // Java: locationPath = parent;
      // Java: doneNodeTest = false;
      // Java: else
      // Java: locationPath.setNodeTest((NodeTest)filter);
      // Java: else if (filter instanceof FilterExpr)
      // Java: locationPath.addFilter((FilterExpr)filter);
      // Java: else if (filter instanceof LocationPathPattern)
      // Java: LocationPathPattern parent = (LocationPathPattern)filter;
      // Java: locationPath.setParentPattern(parent);
      // Java: locationPath = parent;
      // Java: doneNodeTest = false;
      todo!("PatternHandler::endLocationPath")
    }

    // Java: void startNameStep(int axis, String prefix, String localName)
    pub fn startNameStep(&mut self, _axis: i32, _prefix: &str, _localName: &str) {
      // Java: this.pushFrame();
      // Java: short nodeType = 1;
      // Java: switch (axis)
      // Java: case 9:
      // Java: nodeType = 2;
      // Java: break;
      // Java: case 10:
      // Java: nodeType = 13;
      // Java: if (prefix != null && prefix.length() > 0 && !prefix.equals("*"))
      // Java: this.push(new NamespaceTest(prefix, nodeType));
      // Java: if (localName != null && localName.length() > 0 && !localName.equals("*"))
      // Java: this.push(new NameTest(localName, nodeType));
      todo!("PatternHandler::startNameStep")
    }

    // Java: void startTextNodeStep(int axis)
    pub fn startTextNodeStep(&mut self, _axis: i32) {
      // Java: this.pushFrame();
      // Java: this.push(new NodeTypeTest((short)3));
      todo!("PatternHandler::startTextNodeStep")
    }

    // Java: void startCommentNodeStep(int axis)
    pub fn startCommentNodeStep(&mut self, _axis: i32) {
      // Java: this.pushFrame();
      // Java: this.push(new NodeTypeTest((short)8));
      todo!("PatternHandler::startCommentNodeStep")
    }

    // Java: void startAllNodeStep(int axis)
    pub fn startAllNodeStep(&mut self, _axis: i32) {
      // Java: this.pushFrame();
      // Java: this.push(AnyNodeTest.getInstance());
      todo!("PatternHandler::startAllNodeStep")
    }

    // Java: void startProcessingInstructionNodeStep(int axis, String name)
    pub fn startProcessingInstructionNodeStep(&mut self, _axis: i32, _name: &str) {
      // Java: this.pushFrame();
      // Java: this.push(new NodeTypeTest((short)7));
      todo!("PatternHandler::startProcessingInstructionNodeStep")
    }

    // Java: void endStep()
    pub fn endStep(&mut self) {
      // Java: LinkedList list = this.popFrame();
      // Java: if (!list.isEmpty())
      // Java: this.push(list.removeFirst());
      // Java: if (!list.isEmpty())
      // Java: System.out.println("List should now be empty!" + list);
      todo!("PatternHandler::endStep")
    }

    // Java: void startUnionExpr()
    pub fn startUnionExpr(&mut self) {
    }

    // Java: void endUnionExpr(boolean create)
    pub fn endUnionExpr(&mut self, _create: bool) -> Result<(), JaxenException> {
      // Java: if (create)
      // Java: Expr rhs = (Expr)this.pop();
      // Java: Expr lhs = (Expr)this.pop();
      // Java: this.push(this.getXPathFactory().createUnionExpr(lhs, rhs));
      todo!("PatternHandler::endUnionExpr")
    }

    // Java: Pattern createAbsoluteLocationPath()
    pub fn createAbsoluteLocationPath(&self) -> LocationPathPattern {
      // Java: return new LocationPathPattern(NodeTypeTest.DOCUMENT_TEST);
      LocationPathPattern::new_with_node_test(Box::new(NodeTypeTest::new(9)))
    }

    // Java: Pattern createRelativeLocationPath()
    pub fn createRelativeLocationPath(&self) -> LocationPathPattern {
      // Java: return new LocationPathPattern();
      LocationPathPattern::new()
    }
}
