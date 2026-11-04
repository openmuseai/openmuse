// Java: import java.util.ArrayList
// Java: import org.xml.sax.Attributes

// Java: public interface Attributes1 extends Attributes
pub trait Attributes1 {
    // Java: abstract void addAttrite(int var1, String var2, String var3, XmlStringBuilder var4)
    fn addAttrite(&mut self, var1: i32, var2: &str, var3: &str, var4: Box<dyn std::any::Any + Send + Sync>);

    // Java: abstract void addAttriteName(XmlStringBuilder var1)
    fn addAttriteName(&mut self, var1: Box<dyn std::any::Any + Send + Sync>);

    // Java: abstract void addAttriteValue(XmlStringBuilder var1)
    fn addAttriteValue(&mut self, var1: Box<dyn std::any::Any + Send + Sync>);

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract Integer getIntValue(String var1, String var2)
    fn getIntValue(&self, var1: &str, var2: &str) -> Option<i32>;

    // Java: abstract int getStringHashCode(String var1, String var2)
    fn getStringHashCode(&self, var1: &str, var2: &str) -> i32;

    // Java: abstract boolean getBooleanValue(String var1, String var2, boolean var3)
    fn getBooleanValue(&self, var1: &str, var2: &str, var3: bool) -> bool;

    // Java: abstract boolean isPreserveSpace()
    fn isPreserveSpace(&self) -> bool;

    // Java: abstract ArrayList<String> getNamespaces()
    fn getNamespaces(&self) -> Vec<String>;

}
