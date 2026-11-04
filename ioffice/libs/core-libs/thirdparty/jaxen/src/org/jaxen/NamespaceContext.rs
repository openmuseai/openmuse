// Java: public interface NamespaceContext
pub trait NamespaceContext {
    // Java: abstract String translateNamespacePrefixToUri(String var1)
    fn translateNamespacePrefixToUri(&self, var1: &str) -> String;
}
