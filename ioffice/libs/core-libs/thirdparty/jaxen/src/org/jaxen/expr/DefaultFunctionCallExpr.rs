// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.jaxen.Context
use crate::org::jaxen::Context::Context;
// Java: import org.jaxen.Function
use crate::org::jaxen::Function::Function;
// Java: import org.jaxen.JaxenException
use crate::org::jaxen::JaxenException::JaxenException;
use crate::org::jaxen::{Object, List};
use super::Expr::Expr;

// Java field: long serialVersionUID

// Java: public class DefaultFunctionCallExpr extends DefaultExpr implements FunctionCallExpr
pub struct DefaultFunctionCallExpr {
    // Java field: String prefix
    pub(crate) prefix: String,
    // Java field: String functionName
    pub(crate) functionName: String,
    // Java field: List parameters
    pub(crate) parameters: Vec<Box<dyn Expr>>,
}

// Java: public class DefaultFunctionCallExpr extends DefaultExpr implements FunctionCallExpr
impl DefaultFunctionCallExpr {
    // Java: DefaultFunctionCallExpr(String prefix, String functionName)
    pub fn new(prefix: String, functionName: String) -> Self {
      // Java: super();
      // Java: this.prefix = prefix;
      // Java: this.functionName = functionName;
      // Java: this.parameters = new ArrayList();
      DefaultFunctionCallExpr {
         prefix,
         functionName,
         parameters: Vec::new(),
      }
    }

    // Java: void addParameter(Expr parameter)
    pub fn addParameter(&mut self, parameter: Box<dyn Expr>) {
      // Java: this.parameters.add(parameter);
      self.parameters.push(parameter);
    }

    // Java: List getParameters()
    pub fn getParameters(&self) -> &Vec<Box<dyn Expr>> {
      // Java: return this.parameters;
      &self.parameters
    }

    // Java: String getPrefix()
    pub fn getPrefix(&self) -> &str {
      // Java: return this.prefix;
      &self.prefix
    }

    // Java: String getFunctionName()
    pub fn getFunctionName(&self) -> &str {
      // Java: return this.functionName;
      &self.functionName
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: String prefix = this.getPrefix();
      let prefix = self.getPrefix();
      // Java: if (prefix != null && prefix.length() > 0)
      if !prefix.is_empty() {
         // Java: buf.append(prefix);
         buf.push_str(prefix);
         // Java: buf.append(":");
         buf.push(':');
      }
      // Java: buf.append(this.getFunctionName());
      buf.push_str(self.getFunctionName());
      // Java: buf.append("(");
      buf.push('(');
      // Java: Iterator paramIter = this.getParameters().iterator();
      let params = self.getParameters();
      // Java: while(paramIter.hasNext())
      for (i, param) in params.iter().enumerate() {
         // Java: Expr eachParam = (Expr)paramIter.next();
         // Java: buf.append(eachParam.getText());
         buf.push_str(&param.getText());
         // Java: if (paramIter.hasNext())
         if i < params.len() - 1 {
            // Java: buf.append(", ");
            buf.push_str(", ");
         }
      }
      // Java: buf.append(")");
      buf.push(')');
      // Java: return buf.toString();
      buf
    }

    // Java: Expr simplify()
    pub fn simplify(&mut self) {
      // Java: List paramExprs = this.getParameters();
      // Java: int paramSize = paramExprs.size();
      // Java: List newParams = new ArrayList(paramSize);
      // Java: for(int i = 0; i < paramSize; ++i)
      // Java: Expr eachParam = (Expr)paramExprs.get(i);
      // Java: newParams.add(eachParam.simplify());
      // Java: this.parameters = newParams;
      // Java: return this;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: String prefix = this.getPrefix();
      let prefix = self.getPrefix();
      // Java: return prefix == null ? "[(DefaultFunctionCallExpr): " + this.getFunctionName() + "(" + this.getParameters() + ") ]" : "[(DefaultFunctionCallExpr): " + this.getPrefix() + ":" + this.getFunctionName() + "(" + this.getParameters() + ") ]";
      if prefix.is_empty() {
         format!("[(DefaultFunctionCallExpr): {}(...) ]", self.getFunctionName())
      } else {
         format!("[(DefaultFunctionCallExpr): {}:{}(...) ]", prefix, self.getFunctionName())
      }
    }

    // Java: Object evaluate(Context context)
    pub fn evaluate(&self, _context: &Context) -> Result<Object, JaxenException> {
      // Java: String namespaceURI = context.translateNamespacePrefixToUri(this.getPrefix());
      // Java: Function func = context.getFunction(namespaceURI, this.getPrefix(), this.getFunctionName());
      // Java: List paramValues = this.evaluateParams(context);
      // Java: return func.call(context, paramValues);
      todo!("DefaultFunctionCallExpr::evaluate")
    }

    // Java: List evaluateParams(Context context)
    pub fn evaluateParams(&self, _context: &Context) -> Result<List, JaxenException> {
      // Java: List paramExprs = this.getParameters();
      // Java: int paramSize = paramExprs.size();
      // Java: List paramValues = new ArrayList(paramSize);
      // Java: for(int i = 0; i < paramSize; ++i)
      // Java: Expr eachParam = (Expr)paramExprs.get(i);
      // Java: Object eachValue = eachParam.evaluate(context);
      // Java: paramValues.add(eachValue);
      // Java: return paramValues;
      todo!("DefaultFunctionCallExpr::evaluateParams")
    }
}
