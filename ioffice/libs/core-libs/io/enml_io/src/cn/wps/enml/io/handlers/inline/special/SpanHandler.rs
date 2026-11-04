#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;
use crate::cn::wps::enml::io::handlers::HandlerPool::HandlerPool;
use crate::cn::wps::enml::io::handlers::XmlDefaultHandler::{ElementHandler, ElementPath};
use crate::cn::wps::enml::io::handlers::inline::InlineElement::InlineElement;

// Java field: String SPAN
pub(crate) const SPAN: &str = "span";
// Java: public class SpanHandler extends InlineElement
pub struct SpanHandler { base: InlineElement }
// Java: public class SpanHandler extends InlineElement
impl SpanHandler {
    // Java: SpanHandler(IENMLDocumentImporter importer, HandlerPool pool)
    pub fn new(importer: Box<dyn IENMLDocumentImporter>, pool: HandlerPool) -> Self {
      // Java: super(importer, pool);
      SpanHandler { base: InlineElement::new(importer, pool) } }
    // Java: String filterName()
    pub fn filterName(&self) -> String {
      // Java: return "span";
      "span".to_string() }
}
impl ElementHandler for SpanHandler {
    fn onStart(&mut self, var1: &dyn ElementPath) { self.base.onStart_val(var1); }
    fn onCharacters(&mut self, var1: &str) { self.base.onCharacters_val(var1); }
    fn onEnd(&mut self, var1: &dyn ElementPath) { self.base.onEnd_val(var1); }
    fn shouldNotifyChild(&self) -> bool { false }
    fn onStartChild(&mut self, _var1: &dyn ElementPath) {}
    fn onEndChild(&mut self, _var1: &dyn ElementPath) {}
}
