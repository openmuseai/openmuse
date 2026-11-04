// Java: import org.jaxen.function.BooleanFunction
use crate::org::jaxen::function::BooleanFunction::BooleanFunction;
// Java: import org.jaxen.function.CeilingFunction
use crate::org::jaxen::function::CeilingFunction::CeilingFunction;
// Java: import org.jaxen.function.ConcatFunction
use crate::org::jaxen::function::ConcatFunction::ConcatFunction;
// Java: import org.jaxen.function.ContainsFunction
use crate::org::jaxen::function::ContainsFunction::ContainsFunction;
// Java: import org.jaxen.function.CountFunction
use crate::org::jaxen::function::CountFunction::CountFunction;
// Java: import org.jaxen.function.FalseFunction
use crate::org::jaxen::function::FalseFunction::FalseFunction;
// Java: import org.jaxen.function.FloorFunction
use crate::org::jaxen::function::FloorFunction::FloorFunction;
// Java: import org.jaxen.function.IdFunction
use crate::org::jaxen::function::IdFunction::IdFunction;
// Java: import org.jaxen.function.LangFunction
use crate::org::jaxen::function::LangFunction::LangFunction;
// Java: import org.jaxen.function.LastFunction
use crate::org::jaxen::function::LastFunction::LastFunction;
// Java: import org.jaxen.function.LocalNameFunction
use crate::org::jaxen::function::LocalNameFunction::LocalNameFunction;
// Java: import org.jaxen.function.NameFunction
use crate::org::jaxen::function::NameFunction::NameFunction;
// Java: import org.jaxen.function.NamespaceUriFunction
use crate::org::jaxen::function::NamespaceUriFunction::NamespaceUriFunction;
// Java: import org.jaxen.function.NormalizeSpaceFunction
use crate::org::jaxen::function::NormalizeSpaceFunction::NormalizeSpaceFunction;
// Java: import org.jaxen.function.NotFunction
use crate::org::jaxen::function::NotFunction::NotFunction;
// Java: import org.jaxen.function.NumberFunction
use crate::org::jaxen::function::NumberFunction::NumberFunction;
// Java: import org.jaxen.function.PositionFunction
use crate::org::jaxen::function::PositionFunction::PositionFunction;
// Java: import org.jaxen.function.RoundFunction
use crate::org::jaxen::function::RoundFunction::RoundFunction;
// Java: import org.jaxen.function.StartsWithFunction
use crate::org::jaxen::function::StartsWithFunction::StartsWithFunction;
// Java: import org.jaxen.function.StringFunction
use crate::org::jaxen::function::StringFunction::StringFunction;
// Java: import org.jaxen.function.StringLengthFunction
use crate::org::jaxen::function::StringLengthFunction::StringLengthFunction;
// Java: import org.jaxen.function.SubstringAfterFunction
use crate::org::jaxen::function::SubstringAfterFunction::SubstringAfterFunction;
// Java: import org.jaxen.function.SubstringBeforeFunction
use crate::org::jaxen::function::SubstringBeforeFunction::SubstringBeforeFunction;
// Java: import org.jaxen.function.SubstringFunction
use crate::org::jaxen::function::SubstringFunction::SubstringFunction;
// Java: import org.jaxen.function.SumFunction
use crate::org::jaxen::function::SumFunction::SumFunction;
// Java: import org.jaxen.function.TranslateFunction
use crate::org::jaxen::function::TranslateFunction::TranslateFunction;
// Java: import org.jaxen.function.TrueFunction
use crate::org::jaxen::function::TrueFunction::TrueFunction;
// Java: import org.jaxen.function.ext.EndsWithFunction
use crate::org::jaxen::function::ext::EndsWithFunction::EndsWithFunction;
// Java: import org.jaxen.function.ext.EvaluateFunction
use crate::org::jaxen::function::ext::EvaluateFunction::EvaluateFunction;
// Java: import org.jaxen.function.ext.LowerFunction
use crate::org::jaxen::function::ext::LowerFunction::LowerFunction;
// Java: import org.jaxen.function.ext.UpperFunction
use crate::org::jaxen::function::ext::UpperFunction::UpperFunction;
// Java: import org.jaxen.function.xslt.DocumentFunction
use crate::org::jaxen::function::xslt::DocumentFunction::DocumentFunction;

use super::SimpleFunctionContext::SimpleFunctionContext;
use super::FunctionContext::FunctionContext;
use super::Function::Function;
use super::JaxenException::JaxenException;
use super::Context::Context;
use super::{Object, List};
use std::sync::{Arc, OnceLock, Mutex};

// Java field: XPathFunctionContext instance
static INSTANCE: OnceLock<Mutex<XPathFunctionContext>> = OnceLock::new();

// Java: public class XPathFunctionContext extends SimpleFunctionContext
pub struct XPathFunctionContext {
    pub(crate) base: SimpleFunctionContext,
}

// Java: public class XPathFunctionContext extends SimpleFunctionContext
impl XPathFunctionContext {
    // Java: static FunctionContext getInstance()
    pub fn getInstance() -> &'static Mutex<XPathFunctionContext> {
      // Java: return instance;
      INSTANCE.get_or_init(|| Mutex::new(XPathFunctionContext::new()))
    }

    // Java: XPathFunctionContext()
    pub fn new() -> Self {
      // Java: this(true);
      Self::new_with_extensions(true)
    }

    // Java: XPathFunctionContext(boolean includeExtensionFunctions)
    pub fn new_with_extensions(includeExtensionFunctions: bool) -> Self {
      // Java: super();
      let mut ctx = XPathFunctionContext { base: SimpleFunctionContext::new() };
      // Java: this.registerXPathFunctions();
      ctx.registerXPathFunctions();
      // Java: if (includeExtensionFunctions)
      if includeExtensionFunctions {
         // Java: this.registerXSLTFunctions();
         ctx.registerXSLTFunctions();
         // Java: this.registerExtensionFunctions();
         ctx.registerExtensionFunctions();
      }
      ctx
    }

    // Java: void registerXPathFunctions()
    fn registerXPathFunctions(&mut self) {
      // Java: this.registerFunction((String)null, "boolean", new BooleanFunction());
      self.base.registerFunction("", "boolean", Arc::new(BooleanFunction::new()));
      // Java: this.registerFunction((String)null, "ceiling", new CeilingFunction());
      self.base.registerFunction("", "ceiling", Arc::new(CeilingFunction::new()));
      // Java: this.registerFunction((String)null, "concat", new ConcatFunction());
      self.base.registerFunction("", "concat", Arc::new(ConcatFunction::new()));
      // Java: this.registerFunction((String)null, "contains", new ContainsFunction());
      self.base.registerFunction("", "contains", Arc::new(ContainsFunction::new()));
      // Java: this.registerFunction((String)null, "count", new CountFunction());
      self.base.registerFunction("", "count", Arc::new(CountFunction::new()));
      // Java: this.registerFunction((String)null, "false", new FalseFunction());
      self.base.registerFunction("", "false", Arc::new(FalseFunction::new()));
      // Java: this.registerFunction((String)null, "floor", new FloorFunction());
      self.base.registerFunction("", "floor", Arc::new(FloorFunction::new()));
      // Java: this.registerFunction((String)null, "id", new IdFunction());
      self.base.registerFunction("", "id", Arc::new(IdFunction::new()));
      // Java: this.registerFunction((String)null, "lang", new LangFunction());
      self.base.registerFunction("", "lang", Arc::new(LangFunction::new()));
      // Java: this.registerFunction((String)null, "last", new LastFunction());
      self.base.registerFunction("", "last", Arc::new(LastFunction::new()));
      // Java: this.registerFunction((String)null, "local-name", new LocalNameFunction());
      self.base.registerFunction("", "local-name", Arc::new(LocalNameFunction::new()));
      // Java: this.registerFunction((String)null, "name", new NameFunction());
      self.base.registerFunction("", "name", Arc::new(NameFunction::new()));
      // Java: this.registerFunction((String)null, "namespace-uri", new NamespaceUriFunction());
      self.base.registerFunction("", "namespace-uri", Arc::new(NamespaceUriFunction::new()));
      // Java: this.registerFunction((String)null, "normalize-space", new NormalizeSpaceFunction());
      self.base.registerFunction("", "normalize-space", Arc::new(NormalizeSpaceFunction::new()));
      // Java: this.registerFunction((String)null, "not", new NotFunction());
      self.base.registerFunction("", "not", Arc::new(NotFunction::new()));
      // Java: this.registerFunction((String)null, "number", new NumberFunction());
      self.base.registerFunction("", "number", Arc::new(NumberFunction::new()));
      // Java: this.registerFunction((String)null, "position", new PositionFunction());
      self.base.registerFunction("", "position", Arc::new(PositionFunction::new()));
      // Java: this.registerFunction((String)null, "round", new RoundFunction());
      self.base.registerFunction("", "round", Arc::new(RoundFunction::new()));
      // Java: this.registerFunction((String)null, "starts-with", new StartsWithFunction());
      self.base.registerFunction("", "starts-with", Arc::new(StartsWithFunction::new()));
      // Java: this.registerFunction((String)null, "string", new StringFunction());
      self.base.registerFunction("", "string", Arc::new(StringFunction::new()));
      // Java: this.registerFunction((String)null, "string-length", new StringLengthFunction());
      self.base.registerFunction("", "string-length", Arc::new(StringLengthFunction::new()));
      // Java: this.registerFunction((String)null, "substring-after", new SubstringAfterFunction());
      self.base.registerFunction("", "substring-after", Arc::new(SubstringAfterFunction::new()));
      // Java: this.registerFunction((String)null, "substring-before", new SubstringBeforeFunction());
      self.base.registerFunction("", "substring-before", Arc::new(SubstringBeforeFunction::new()));
      // Java: this.registerFunction((String)null, "substring", new SubstringFunction());
      self.base.registerFunction("", "substring", Arc::new(SubstringFunction::new()));
      // Java: this.registerFunction((String)null, "sum", new SumFunction());
      self.base.registerFunction("", "sum", Arc::new(SumFunction::new()));
      // Java: this.registerFunction((String)null, "true", new TrueFunction());
      self.base.registerFunction("", "true", Arc::new(TrueFunction::new()));
      // Java: this.registerFunction((String)null, "translate", new TranslateFunction());
      self.base.registerFunction("", "translate", Arc::new(TranslateFunction::new()));
    }

    // Java: void registerXSLTFunctions()
    fn registerXSLTFunctions(&mut self) {
      // Java: this.registerFunction((String)null, "document", new DocumentFunction());
      self.base.registerFunction("", "document", Arc::new(DocumentFunction::new()));
    }

    // Java: void registerExtensionFunctions()
    fn registerExtensionFunctions(&mut self) {
      // Java: this.registerFunction((String)null, "evaluate", new EvaluateFunction());
      self.base.registerFunction("", "evaluate", Arc::new(EvaluateFunction::new()));
      // Java: this.registerFunction((String)null, "lower-case", new LowerFunction());
      self.base.registerFunction("", "lower-case", Arc::new(LowerFunction::new()));
      // Java: this.registerFunction((String)null, "upper-case", new UpperFunction());
      self.base.registerFunction("", "upper-case", Arc::new(UpperFunction::new()));
      // Java: this.registerFunction((String)null, "ends-with", new EndsWithFunction());
      self.base.registerFunction("", "ends-with", Arc::new(EndsWithFunction::new()));
    }
}

impl FunctionContext for XPathFunctionContext {
    fn getFunction(&self, namespaceURI: &str, prefix: &str, localName: &str) -> Result<Box<dyn Function>, JaxenException> {
        self.base.getFunction_impl(namespaceURI, prefix, localName)
    }
}
