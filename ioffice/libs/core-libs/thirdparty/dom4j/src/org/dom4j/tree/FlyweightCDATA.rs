// Java: import org.dom4j.CDATA
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node

// Java: public class FlyweightCDATA extends AbstractCDATA implements CDATA
pub struct FlyweightCDATA {
    // Java field: String text
    pub(crate) text: String,
}

// Java: public class FlyweightCDATA extends AbstractCDATA implements CDATA
impl FlyweightCDATA {
    // Java: FlyweightCDATA(String text)
    pub fn new(text: String) -> Self {
        // Java: super();
        // Java: this.text = text;
        FlyweightCDATA { text }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: return this.text;
        self.text.clone()
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultCDATA(parent, this.getText());
    }
}
