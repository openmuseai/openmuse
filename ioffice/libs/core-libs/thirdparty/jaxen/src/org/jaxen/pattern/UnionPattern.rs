// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use super::Pattern::Pattern;

// Java: public class UnionPattern extends Pattern
pub struct UnionPattern {
    // Java field: Pattern lhs
    pub(crate) lhs: Box<dyn Pattern>,
    // Java field: Pattern rhs
    pub(crate) rhs: Box<dyn Pattern>,
    // Java field: short nodeType
    pub(crate) nodeType: i16,
    // Java field: String matchesNodeName
    pub(crate) matchesNodeName: Option<String>,
}

// Java: public class UnionPattern extends Pattern
impl UnionPattern {
    // Java: UnionPattern()
    pub fn new_empty() -> Self {
      // Java: super();
      todo!("UnionPattern::new_empty")
    }

    // Java: UnionPattern(Pattern lhs, Pattern rhs)
    pub fn new(lhs: Box<dyn Pattern>, rhs: Box<dyn Pattern>) -> Self {
      // Java: super();
      // Java: this.lhs = lhs;
      // Java: this.rhs = rhs;
      // Java: this.init();
      let type1 = lhs.getMatchType();
      let type2 = rhs.getMatchType();
      let nodeType = if type1 == type2 { type1 } else { 0 };
      let name1 = lhs.getMatchesNodeName();
      let name2 = rhs.getMatchesNodeName();
      let matchesNodeName = match (&name1, &name2) {
         (Some(n1), Some(n2)) if n1 == n2 => Some(n1.clone()),
         _ => None,
      };
      UnionPattern { lhs, rhs, nodeType, matchesNodeName }
    }

    // Java: Pattern getLHS()
    pub fn getLHS(&self) -> &dyn Pattern {
      // Java: return this.lhs;
      &*self.lhs
    }

    // Java: void setLHS(Pattern lhs)
    pub fn setLHS(&mut self, lhs: Box<dyn Pattern>) {
      // Java: this.lhs = lhs;
      self.lhs = lhs;
      // Java: this.init();
      self.init();
    }

    // Java: Pattern getRHS()
    pub fn getRHS(&self) -> &dyn Pattern {
      // Java: return this.rhs;
      &*self.rhs
    }

    // Java: void setRHS(Pattern rhs)
    pub fn setRHS(&mut self, rhs: Box<dyn Pattern>) {
      // Java: this.rhs = rhs;
      self.rhs = rhs;
      // Java: this.init();
      self.init();
    }

    // Java: boolean matches(Object node, Context context)
    pub fn matches(&self, node: &Object, context: &Context) -> Result<bool, JaxenException> {
      // Java: return this.lhs.matches(node, context) || this.rhs.matches(node, context);
      Ok(self.lhs.matches(node, context)? || self.rhs.matches(node, context)?)
    }

    // Java: Pattern[] getUnionPatterns()
    pub fn getUnionPatterns(&self) {
      // Java: return new Pattern[]{this.lhs, this.rhs};
    }

    // Java: short getMatchType()
    pub fn getMatchType(&self) -> i16 {
      // Java: return this.nodeType;
      self.nodeType
    }

    // Java: String getMatchesNodeName()
    pub fn getMatchesNodeName(&self) -> &Option<String> {
      // Java: return this.matchesNodeName;
      &self.matchesNodeName
    }

    // Java: Pattern simplify()
    pub fn simplify(&mut self) {
      // Java: this.lhs = this.lhs.simplify();
      // Java: this.rhs = this.rhs.simplify();
      // Java: this.init();
      self.init();
      // Java: return this;
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: return this.lhs.getText() + " | " + this.rhs.getText();
      format!("{} | {}", self.lhs.getText(), self.rhs.getText())
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + "[ lhs: " + this.lhs + " rhs: " + this.rhs + " ]";
      format!("UnionPattern[ lhs: {} rhs: {} ]", self.lhs.getText(), self.rhs.getText())
    }

    // Java: void init()
    fn init(&mut self) {
      // Java: short type1 = this.lhs.getMatchType();
      let type1 = self.lhs.getMatchType();
      // Java: short type2 = this.rhs.getMatchType();
      let type2 = self.rhs.getMatchType();
      // Java: this.nodeType = type1 == type2 ? type1 : 0;
      self.nodeType = if type1 == type2 { type1 } else { 0 };
      // Java: String name1 = this.lhs.getMatchesNodeName();
      let name1 = self.lhs.getMatchesNodeName();
      // Java: String name2 = this.rhs.getMatchesNodeName();
      let name2 = self.rhs.getMatchesNodeName();
      // Java: this.matchesNodeName = null;
      self.matchesNodeName = None;
      // Java: if (name1 != null && name2 != null && name1.equals(name2))
      if let (Some(n1), Some(n2)) = (&name1, &name2) {
         if n1 == n2 {
            // Java: this.matchesNodeName = name1;
            self.matchesNodeName = Some(n1.clone());
         }
      }
    }
}
