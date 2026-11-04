// Java: import java.util.ArrayList
// Java: import java.util.Collections
// Java: import java.util.HashSet
// Java: import java.util.List
// Java: import java.util.Set
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
// Java: import org.jaxen.XPathSyntaxException
use crate::org::jaxen::XPathSyntaxException::XPathSyntaxException;
use crate::org::jaxen::Object;
use super::DefaultBinaryExpr::DefaultBinaryExpr;

// Java field: long serialVersionUID

// Java: public class DefaultUnionExpr extends DefaultBinaryExpr implements UnionExpr
pub struct DefaultUnionExpr {
    pub(crate) base: DefaultBinaryExpr,
}

// Java: public class DefaultUnionExpr extends DefaultBinaryExpr implements UnionExpr
impl DefaultUnionExpr {
    // Java: DefaultUnionExpr(Expr lhs, Expr rhs)
    pub fn new(lhs: Box<dyn super::Expr::Expr>, rhs: Box<dyn super::Expr::Expr>) -> Self {
      // Java: super(lhs, rhs);
      DefaultUnionExpr {
         base: DefaultBinaryExpr::new(lhs, rhs),
      }
    }

    // Java: String getOperator()
    pub fn getOperator(&self) -> String {
      // Java: return "|";
      "|".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return "[(DefaultUnionExpr): " + this.getLHS() + ", " + this.getRHS() + "]";
      format!("[(DefaultUnionExpr): {}, {}]", self.base.getLHS().getText(), self.base.getRHS().getText())
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: List results = new ArrayList();
      // Java: try
      // Java: List lhsResults = (List)this.getLHS().evaluate(context);
      // Java: List rhsResults = (List)this.getRHS().evaluate(context);
      // Java: Set unique = new HashSet();
      // Java: results.addAll(lhsResults);
      // Java: unique.addAll(lhsResults);
      // Java: for(Object each : rhsResults)
      // Java: if (!unique.contains(each))
      // Java: results.add(each);
      // Java: unique.add(each);
      // Java: Collections.sort(results, new NodeComparator(context.getNavigator()));
      // Java: return results;
      // Java: catch (ClassCastException var8)
      // Java: throw new XPathSyntaxException(this.getText(), context.getPosition(), "Unions are only allowed over node-sets");
      todo!("DefaultUnionExpr::evaluate")
    }
}
