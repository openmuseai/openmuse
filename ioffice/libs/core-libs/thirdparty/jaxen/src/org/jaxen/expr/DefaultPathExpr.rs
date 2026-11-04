// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::Object;
use super::Expr::Expr;
use super::LocationPath::LocationPath;
use super::DefaultLocationPath::DefaultLocationPath;

// Java field: long serialVersionUID

// Java: public class DefaultPathExpr extends DefaultExpr implements PathExpr
pub struct DefaultPathExpr {
    // Java field: Expr filterExpr
    pub(crate) filterExpr: Option<Box<dyn Expr>>,
    // Java field: LocationPath locationPath
    pub(crate) locationPath: Option<Box<DefaultLocationPath>>,
}

// Java: public class DefaultPathExpr extends DefaultExpr implements PathExpr
impl DefaultPathExpr {
    // Java: DefaultPathExpr(Expr filterExpr, LocationPath locationPath)
    pub fn new(filterExpr: Option<Box<dyn Expr>>, locationPath: Option<Box<DefaultLocationPath>>) -> Self {
      // Java: super();
      // Java: this.filterExpr = filterExpr;
      // Java: this.locationPath = locationPath;
      DefaultPathExpr { filterExpr, locationPath }
    }

    // Java: Expr getFilterExpr()
    pub fn getFilterExpr(&self) -> Option<&dyn Expr> {
      // Java: return this.filterExpr;
      self.filterExpr.as_deref()
    }

    // Java: void setFilterExpr(Expr filterExpr)
    pub fn setFilterExpr(&mut self, filterExpr: Option<Box<dyn Expr>>) {
      // Java: this.filterExpr = filterExpr;
      self.filterExpr = filterExpr;
    }

    // Java: LocationPath getLocationPath()
    pub fn getLocationPath(&self) -> Option<&DefaultLocationPath> {
      // Java: return this.locationPath;
      self.locationPath.as_deref()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.getLocationPath() != null ? "[(DefaultPathExpr): " + this.getFilterExpr() + ", " + this.getLocationPath() + "]" : "[(DefaultPathExpr): " + this.getFilterExpr() + "]";
      if self.locationPath.is_some() {
         format!("[(DefaultPathExpr): filter + locationPath]")
      } else {
         format!("[(DefaultPathExpr): filter]")
      }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: if (this.getFilterExpr() != null)
      if let Some(ref filter) = self.filterExpr {
         // Java: buf.append(this.getFilterExpr().getText());
         buf.push_str(&filter.getText());
      }
      // Java: if (this.getLocationPath() != null)
      if let Some(ref loc) = self.locationPath {
         // Java: if (!this.getLocationPath().getSteps().isEmpty())
         if !loc.getSteps().is_empty() {
            // Java: buf.append("/");
            buf.push('/');
         }
         // Java: buf.append(this.getLocationPath().getText());
         buf.push_str(&loc.getText());
      }
      // Java: return buf.toString();
      buf
    }

    // Java: Expr simplify()
    pub fn simplify(&mut self) {
      // Java: if (this.getFilterExpr() != null)
      // Java: this.setFilterExpr(this.getFilterExpr().simplify());
      // Java: if (this.getLocationPath() != null)
      // Java: this.getLocationPath().simplify();
      // Java: if (this.getFilterExpr() == null && this.getLocationPath() == null)
      // Java: return null;
      // Java: else if (this.getLocationPath() == null)
      // Java: return this.getFilterExpr();
      // Java: else
      // Java: return (Expr)(this.getFilterExpr() == null ? this.getLocationPath() : this);
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: Object results = null;
      // Java: Context pathContext = null;
      // Java: if (this.getFilterExpr() != null)
      // Java: results = this.getFilterExpr().evaluate(context);
      // Java: pathContext = new Context(context.getContextSupport());
      // Java: pathContext.setNodeSet(convertToList(results));
      // Java: return this.getLocationPath() != null ? this.getLocationPath().evaluate(pathContext) : results;
      todo!("DefaultPathExpr::evaluate")
    }
}
