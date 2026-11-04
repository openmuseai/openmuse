// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.Navigator
// Java: import org.jaxen.expr.FilterExpr
use crate::org::jaxen::expr::FilterExpr::FilterExpr;
// Java: import org.jaxen.util.SingletonList
use crate::org::jaxen::Object;
use super::Pattern::Pattern;

// Java: public class LocationPathPattern extends Pattern
pub struct LocationPathPattern {
    // Java field: NodeTest nodeTest
    pub(crate) nodeTest: Box<dyn Pattern>,
    // Java field: Pattern parentPattern
    pub(crate) parentPattern: Option<Box<dyn Pattern>>,
    // Java field: Pattern ancestorPattern
    pub(crate) ancestorPattern: Option<Box<dyn Pattern>>,
    // Java field: List<FilterExpr> filters
    pub(crate) filters: Option<Vec<Box<dyn FilterExpr>>>,
    // Java field: boolean absolute
    pub(crate) absolute: bool,
}

// Java: public class LocationPathPattern extends Pattern
impl LocationPathPattern {
    // Java: LocationPathPattern()
    pub fn new() -> Self {
      // Java: super();
      LocationPathPattern {
         nodeTest: Box::new(super::AnyNodeTest::AnyNodeTest::new()),
         parentPattern: None,
         ancestorPattern: None,
         filters: None,
         absolute: false,
      }
    }

    // Java: LocationPathPattern(NodeTest nodeTest)
    pub fn new_with_node_test(nodeTest: Box<dyn Pattern>) -> Self {
      // Java: super();
      // Java: this.nodeTest = nodeTest;
      LocationPathPattern {
         nodeTest,
         parentPattern: None,
         ancestorPattern: None,
         filters: None,
         absolute: false,
      }
    }

    // Java: Pattern simplify()
    pub fn simplify(&self) {
      // Java: if (this.parentPattern != null)
      // Java: this.parentPattern = this.parentPattern.simplify();
      // Java: if (this.ancestorPattern != null)
      // Java: this.ancestorPattern = this.ancestorPattern.simplify();
      // Java: if (this.filters == null)
      // Java: if (this.parentPattern == null && this.ancestorPattern == null)
      // Java: return this.nodeTest;
      // Java: if (this.parentPattern != null && this.ancestorPattern == null && this.nodeTest instanceof AnyNodeTest)
      // Java: return this.parentPattern;
      // Java: return this;
    }

    // Java: void addFilter(FilterExpr filter)
    pub fn addFilter(&mut self, filter: Box<dyn FilterExpr>) {
      // Java: if (this.filters == null)
      if self.filters.is_none() {
         // Java: this.filters = new ArrayList<FilterExpr>();
         self.filters = Some(Vec::new());
      }
      // Java: this.filters.add(filter);
      self.filters.as_mut().unwrap().push(filter);
    }

    // Java: void setParentPattern(Pattern parentPattern)
    pub fn setParentPattern(&mut self, parentPattern: Box<dyn Pattern>) {
      // Java: this.parentPattern = parentPattern;
      self.parentPattern = Some(parentPattern);
    }

    // Java: void setAncestorPattern(Pattern ancestorPattern)
    pub fn setAncestorPattern(&mut self, ancestorPattern: Box<dyn Pattern>) {
      // Java: this.ancestorPattern = ancestorPattern;
      self.ancestorPattern = Some(ancestorPattern);
    }

    // Java: void setNodeTest(NodeTest nodeTest)
    pub fn setNodeTest(&mut self, nodeTest: Box<dyn Pattern>) -> Result<(), JaxenException> {
      // Java: if (this.nodeTest instanceof AnyNodeTest)
      // Java: this.nodeTest = nodeTest;
      // Java: else
      // Java: throw new JaxenException("Attempt to overwrite nodeTest: " + this.nodeTest + " with: " + nodeTest);
      self.nodeTest = nodeTest;
      Ok(())
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, _node: &Object, _context: &Context) -> Result<bool, JaxenException> {
      // Java: Navigator navigator = context.getNavigator();
      // Java: if (!this.nodeTest.matches(node, context))
      // Java: return false;
      // Java: else
      // Java: if (this.parentPattern != null)
      // Java: Object parent = navigator.getParentNode(node);
      // Java: if (parent == null)
      // Java: return false;
      // Java: if (!this.parentPattern.matches(parent, context))
      // Java: return false;
      // Java: if (this.ancestorPattern != null)
      // Java: for(Object ancestor = navigator.getParentNode(node); !this.ancestorPattern.matches(ancestor, context); ancestor = navigator.getParentNode(ancestor))
      // Java: if (ancestor == null)
      // Java: return false;
      // Java: if (navigator.isDocument(ancestor))
      // Java: return false;
      // Java: if (this.filters == null)
      // Java: return true;
      // Java: else
      // Java: List list = new SingletonList(node);
      // Java: context.setNodeSet(list);
      // Java: boolean answer = true;
      // Java: for(FilterExpr filter : this.filters)
      // Java: if (!filter.asBoolean(context))
      // Java: answer = false;
      // Java: break;
      // Java: context.setNodeSet(list);
      // Java: return answer;
      todo!("LocationPathPattern::matches")
    }

    // Java: double getPriority()
    pub fn getPriority(&self) -> f64 {
      // Java: return this.filters != null ? (double)0.5F : this.nodeTest.getPriority();
      if self.filters.is_some() { 0.5 } else { self.nodeTest.getPriority() }
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return this.nodeTest.getMatchType();
      self.nodeTest.getMatchType()
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: if (this.absolute)
      if self.absolute {
         // Java: buffer.append("/");
         buffer.push_str("/");
      }
      // Java: if (this.ancestorPattern != null)
      if let Some(ref ancestor) = self.ancestorPattern {
         // Java: String text = this.ancestorPattern.getText();
         let text = ancestor.getText();
         // Java: if (text.length() > 0)
         if !text.is_empty() {
            // Java: buffer.append(text);
            buffer.push_str(&text);
            // Java: buffer.append("//");
            buffer.push_str("//");
         }
      }
      // Java: if (this.parentPattern != null)
      if let Some(ref parent) = self.parentPattern {
         // Java: String text = this.parentPattern.getText();
         let text = parent.getText();
         // Java: if (text.length() > 0)
         if !text.is_empty() {
            // Java: buffer.append(text);
            buffer.push_str(&text);
            // Java: buffer.append("/");
            buffer.push_str("/");
         }
      }
      // Java: buffer.append(this.nodeTest.getText());
      buffer.push_str(&self.nodeTest.getText());
      // Java: if (this.filters != null)
      if let Some(ref filters) = self.filters {
         // Java: buffer.append("[");
         buffer.push_str("[");
         // Java: for(FilterExpr filter : this.filters)
         for filter in filters {
            // Java: buffer.append(filter.getText());
            buffer.push_str(&filter.getText());
         }
         // Java: buffer.append("]");
         buffer.push_str("]");
      }
      // Java: return buffer.toString();
      buffer
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + "[ absolute: " + this.absolute + " parent: " + this.parentPattern + " ancestor: " + this.ancestorPattern + " filters: " + this.filters + " nodeTest: " + this.nodeTest + " ]";
      format!("LocationPathPattern[ absolute: {} ]", self.absolute)
    }

    // Java: boolean isAbsolute()
    pub fn isAbsolute(&self) -> bool {
      // Java: return this.absolute;
      self.absolute
    }

    // Java: void setAbsolute(boolean absolute)
    pub fn setAbsolute(&mut self, absolute: bool) {
      // Java: this.absolute = absolute;
      self.absolute = absolute;
    }

    // Java: boolean hasAnyNodeTest()
    pub fn hasAnyNodeTest(&self) -> bool {
      // Java: return this.nodeTest instanceof AnyNodeTest;
      false
    }
}
