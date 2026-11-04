// Java: import cn.wps.kfc.xpxml.util.XmlValueBuilder
// Java: import java.io.IOException

// Java: public interface CompactInnerAccepter
pub trait CompactInnerAccepter {
    // Java: abstract void beginElement(XmlValueBuilder var1)
    fn beginElement(&self, var1: &str);

    // Java: abstract void endElement()
    fn endElement(&self);

    // Java: abstract void addAttribName(XmlValueBuilder var1)
    fn addAttribName(&self, var1: &str);

    // Java: abstract void addAttribValue(String var1)
    fn addAttribValue(&self, var1: &str);

    // Java: abstract void addText(String var1)
    fn addText(&self, var1: &str);

}
