// Java: import cn.wps.io.dom.io.MCEContext
// Java: import cn.wps.io.dom.tree.NamespaceStack
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.xml.sax.Attributes

// Java: public interface Element extends Branch
pub trait Element {
    // Java: abstract void recycle()
    fn recycle(&mut self);

    // Java: abstract String getNamespacePrefix()
    fn getNamespacePrefix(&self) -> String;

    // Java: abstract String getNamespaceURI()
    fn getNamespaceURI(&self) -> String;

    // Java: abstract String getQualifiedName()
    fn getQualifiedName(&self) -> String;

    // Java: abstract List<Namespace> declaredNamespaces()
    fn declaredNamespaces(&self) -> Vec<String>;

    // Java: abstract Element addAttribute(String var1, String var2)
    fn addAttribute(&mut self, var1: &str, var2: &str);

    // Java: abstract Element addAttribute(String var1, String var2, String var3, String var4)
    fn addAttribute_with_ns(&mut self, var1: &str, var2: &str, var3: &str, var4: &str);

    // Java: abstract Element addComment(String var1)
    fn addComment(&mut self, var1: &str);

    // Java: abstract Element addCDATA(String var1)
    fn addCDATA(&mut self, var1: &str);

    // Java: abstract Element addEntity(String var1, String var2)
    fn addEntity(&mut self, var1: &str, var2: &str);

    // Java: abstract Element addNamespace(String var1, String var2)
    fn addNamespace(&mut self, var1: &str, var2: &str);

    // Java: abstract Element addProcessingInstruction(String var1, String var2)
    fn addProcessingInstruction(&mut self, var1: &str, var2: &str);

    // Java: abstract Element addProcessingInstruction(String var1, Map<String, String> var2)
    fn addProcessingInstruction_with_map(&mut self, var1: &str, var2: std::collections::HashMap<String, String>);

    // Java: abstract Element addText(String var1)
    fn addText(&mut self, var1: &str);

    // Java: abstract void add(Attribute var1)
    fn add_attribute(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(CDATA var1)
    fn add_cdata(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(Entity var1)
    fn add_entity(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(Text var1)
    fn add_text(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void add(Namespace var1)
    fn add_namespace(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract boolean remove(Attribute var1)
    fn remove_attribute(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(CDATA var1)
    fn remove_cdata(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(Entity var1)
    fn remove_entity(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(Namespace var1)
    fn remove_namespace(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract boolean remove(Text var1)
    fn remove_text(&mut self, var1: &dyn std::any::Any) -> bool;

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract String getTextTrim()
    fn getTextTrim(&self) -> String;

    // Java: abstract String getStringValue()
    fn getStringValue(&self) -> String;

    // Java: abstract Object getData()
    fn getData(&self) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract void setData(Object var1)
    fn setData(&mut self, var1: Box<dyn std::any::Any>);

    // Java: abstract void setAttributes(Attributes var1, int var2, NamespaceStack var3, boolean var4, MCEContext var5)
    fn setAttributes(&mut self, var1: &dyn std::any::Any, var2: i32, var3: &dyn std::any::Any, var4: bool, var5: &dyn std::any::Any);

    // Java: abstract int attributeCount()
    fn attributeCount(&self) -> i32;

    // Java: abstract Iterator<Attribute> attributeIterator()
    fn attributeIterator(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>>;

    // Java: abstract Attribute attribute(int var1)
    fn attribute_by_index(&self, var1: i32) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Attribute attribute(String var1)
    fn attribute_by_name(&self, var1: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Attribute attribute(String var1, String var2)
    fn attribute_by_name_ns(&self, var1: &str, var2: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract String attributeValue(String var1)
    fn attributeValue(&self, var1: &str) -> Option<String>;

    // Java: abstract String attributeValue(String var1, String var2)
    fn attributeValue_with_ns(&self, var1: &str, var2: &str) -> Option<String>;

    // Java: abstract Element element(String var1)
    fn element_by_name(&self, var1: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Element element(String var1, String var2)
    fn element_by_name_ns(&self, var1: &str, var2: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Element lastElement(String var1)
    fn lastElement_by_name(&self, var1: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Element lastElement(String var1, String var2)
    fn lastElement_by_name_ns(&self, var1: &str, var2: &str) -> Option<Box<dyn std::any::Any>>;

    // Java: abstract Element[] elementsRO()
    fn elementsRO(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract List<Element> elements()
    fn elements(&self) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract List<Element> elements(String var1)
    fn elements_by_name(&self, var1: &str) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract Element[] elementsRO(String var1)
    fn elementsRO_by_name(&self, var1: &str) -> Vec<Box<dyn std::any::Any>>;

    // Java: abstract Iterator<Element> elementIterator()
    fn elementIterator(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>>;

    // Java: abstract Iterator<Element> elementIterator(String var1)
    fn elementIterator_by_name(&self, var1: &str) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>>;

    // Java: abstract boolean isRootElement()
    fn isRootElement(&self) -> bool;

    // Java: abstract boolean hasMixedContent()
    fn hasMixedContent(&self) -> bool;

    // Java: abstract boolean isTextOnly()
    fn isTextOnly(&self) -> bool;

    // Java: abstract void appendAttributes(Element var1)
    fn appendAttributes(&mut self, var1: &dyn std::any::Any);

    // Java: abstract String elementText(String var1)
    fn elementText(&self, var1: &str) -> Option<String>;

    // Java: abstract String elementTextTrim(String var1)
    fn elementTextTrim(&self, var1: &str) -> Option<String>;
}
