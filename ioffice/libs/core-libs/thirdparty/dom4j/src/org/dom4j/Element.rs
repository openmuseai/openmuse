// Java: public interface Element extends Branch

// Java: public interface Element extends Branch
pub trait Element {
    // Java: QName getQName()
    fn getQName(&self) -> i32;

    // Java: void setQName(QName var1)
    fn setQName(&mut self, _qname: i32);

    // Java: Namespace getNamespace()
    fn getNamespace(&self) -> i32;

    // Java: QName getQName(String var1)
    fn getQName_name(&self, _name: &str) -> i32;

    // Java: Namespace getNamespaceForPrefix(String var1)
    fn getNamespaceForPrefix(&self, _prefix: &str) -> i32;

    // Java: Namespace getNamespaceForURI(String var1)
    fn getNamespaceForURI(&self, _uri: &str) -> i32;

    // Java: List<Namespace> getNamespacesForURI(String var1)
    fn getNamespacesForURI(&self, _uri: &str) -> Vec<i32>;

    // Java: String getNamespacePrefix()
    fn getNamespacePrefix(&self) -> String;

    // Java: String getNamespaceURI()
    fn getNamespaceURI(&self) -> String;

    // Java: String getQualifiedName()
    fn getQualifiedName(&self) -> &str;

    // Java: List<Namespace> additionalNamespaces()
    fn additionalNamespaces(&self) -> Vec<i32>;

    // Java: List<Namespace> declaredNamespaces()
    fn declaredNamespaces(&self) -> Vec<i32>;

    // Java: Element addAttribute(String var1, String var2)
    fn addAttribute(&mut self, _name: &str, _value: &str);

    // Java: Element addAttribute(QName var1, String var2)
    fn addAttribute_qname(&mut self, _qname: i32, _value: &str);

    // Java: Element addComment(String var1)
    fn addComment(&mut self, _comment: &str);

    // Java: Element addCDATA(String var1)
    fn addCDATA(&mut self, _cdata: &str);

    // Java: Element addEntity(String var1, String var2)
    fn addEntity(&mut self, _name: &str, _text: &str);

    // Java: Element addNamespace(String var1, String var2)
    fn addNamespace(&mut self, _prefix: &str, _uri: &str);

    // Java: Element addProcessingInstruction(String var1, String var2)
    fn addProcessingInstruction(&mut self, _target: &str, _text: &str);

    // Java: Element addProcessingInstruction(String var1, Map<String, String> var2)
    fn addProcessingInstruction_map(&mut self, _target: &str, _attrs: &std::collections::HashMap<String, String>);

    // Java: Element addText(String var1)
    fn addText(&mut self, _text: &str);

    // Java: void add(Attribute var1)
    fn add_attribute(&mut self, _attr: i32);

    // Java: void add(CDATA var1)
    fn add_cdata(&mut self, _cdata: i32);

    // Java: void add(Entity var1)
    fn add_entity(&mut self, _entity: i32);

    // Java: void add(Text var1)
    fn add_text(&mut self, _text: i32);

    // Java: void add(Namespace var1)
    fn add_namespace(&mut self, _namespace: i32);

    // Java: boolean remove(Attribute var1)
    fn remove_attribute(&mut self, _attr: i32) -> bool;

    // Java: boolean remove(CDATA var1)
    fn remove_cdata(&mut self, _cdata: i32) -> bool;

    // Java: boolean remove(Entity var1)
    fn remove_entity(&mut self, _entity: i32) -> bool;

    // Java: boolean remove(Namespace var1)
    fn remove_namespace(&mut self, _namespace: i32) -> bool;

    // Java: boolean remove(Text var1)
    fn remove_text(&mut self, _text: i32) -> bool;

    // Java: String getText()
    fn getText(&self) -> String;

    // Java: String getTextTrim()
    fn getTextTrim(&self) -> String;

    // Java: String getStringValue()
    fn getStringValue(&self) -> String;

    // Java: Object getData()
    fn getData(&self) -> i32;

    // Java: void setData(Object var1)
    fn setData(&mut self, _data: i32);

    // Java: List<Attribute> attributes()
    fn attributes(&self) -> Vec<i32>;

    // Java: void setAttributes(Attributes var1, int var2, NamespaceStack var3, boolean var4, MCEContext var5)
    fn setAttributes_sax(&mut self, _attrs: i32, _size: i32, _ns_stack: i32, _no_ns: bool, _mce: i32);

    // Java: void setAttributes(List<Attribute> var1)
    fn setAttributes(&mut self, _attrs: Vec<i32>);

    // Java: int attributeCount()
    fn attributeCount(&self) -> i32;

    // Java: Iterator<Attribute> attributeIterator()
    fn attributeIterator(&self) -> i32;

    // Java: Attribute attribute(int var1)
    fn attribute_index(&self, _index: i32) -> i32;

    // Java: Attribute attribute(String var1)
    fn attribute_name(&self, _name: &str) -> i32;

    // Java: Attribute attribute(QName var1)
    fn attribute_qname(&self, _qname: i32) -> i32;

    // Java: String attributeValue(String var1)
    fn attributeValue(&self, _name: &str) -> Option<String>;

    // Java: String attributeValue(String var1, String var2)
    fn attributeValue_default(&self, _name: &str, _default: &str) -> String;

    // Java: String attributeValue(QName var1)
    fn attributeValue_qname(&self, _qname: i32) -> Option<String>;

    // Java: String attributeValue(QName var1, String var2)
    fn attributeValue_qname_default(&self, _qname: i32, _default: &str) -> String;

    // Java: Element element(String var1)
    fn element_name(&self, _name: &str) -> i32;

    // Java: Element element(QName var1)
    fn element_qname(&self, _qname: i32) -> i32;

    // Java: Element element(String var1, Namespace var2)
    fn element_name_ns(&self, _name: &str, _ns: i32) -> i32;

    // Java: Element lastElement(String var1)
    fn lastElement(&self, _name: &str) -> i32;

    // Java: Element lastElement(QName var1)
    fn lastElement_qname(&self, _qname: i32) -> i32;

    // Java: Element lastElement(String var1, Namespace var2)
    fn lastElement_name_ns(&self, _name: &str, _ns: i32) -> i32;

    // Java: Element[] elementsRO()
    fn elementsRO(&self) -> Vec<i32>;

    // Java: List<Element> elements()
    fn elements(&self) -> Vec<i32>;

    // Java: List<Element> elements(String var1)
    fn elements_name(&self, _name: &str) -> Vec<i32>;

    // Java: Element[] elementsRO(String var1)
    fn elementsRO_name(&self, _name: &str) -> Vec<i32>;

    // Java: List<Element> elements(QName var1)
    fn elements_qname(&self, _qname: i32) -> Vec<i32>;

    // Java: Iterator<Element> elementIterator()
    fn elementIterator(&self) -> i32;

    // Java: Iterator<Element> elementIterator(String var1)
    fn elementIterator_name(&self, _name: &str) -> i32;

    // Java: Iterator<Element> elementIterator(QName var1)
    fn elementIterator_qname(&self, _qname: i32) -> i32;

    // Java: boolean isRootElement()
    fn isRootElement(&self) -> bool;

    // Java: boolean hasMixedContent()
    fn hasMixedContent(&self) -> bool;

    // Java: boolean isTextOnly()
    fn isTextOnly(&self) -> bool;

    // Java: void appendAttributes(Element var1)
    fn appendAttributes(&mut self, _element: i32);

    // Java: Element createCopy()
    fn createCopy(&self) -> i32;

    // Java: Element createCopy(String var1)
    fn createCopy_name(&self, _name: &str) -> i32;

    // Java: Element createCopy(QName var1)
    fn createCopy_qname(&self, _qname: i32) -> i32;

    // Java: String elementText(String var1)
    fn elementText(&self, _name: &str) -> String;

    // Java: String elementText(QName var1)
    fn elementText_qname(&self, _qname: i32) -> String;

    // Java: String elementTextTrim(String var1)
    fn elementTextTrim(&self, _name: &str) -> String;

    // Java: String elementTextTrim(QName var1)
    fn elementTextTrim_qname(&self, _qname: i32) -> String;

    // Java: Node getXPathResult(int var1)
    fn getXPathResult(&self, _index: i32) -> i32;
}
