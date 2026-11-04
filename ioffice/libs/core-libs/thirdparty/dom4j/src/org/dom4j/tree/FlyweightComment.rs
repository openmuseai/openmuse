// Java: import org.dom4j.Comment
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node

// Java: public class FlyweightComment extends AbstractComment implements Comment
pub struct FlyweightComment {
    // Java field: String text
    pub(crate) text: String,
}

// Java: public class FlyweightComment extends AbstractComment implements Comment
impl FlyweightComment {
    // Java: FlyweightComment(String text)
    pub fn new(text: String) -> Self {
        // Java: super();
        // Java: this.text = text;
        FlyweightComment { text }
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: return this.text;
        self.text.clone()
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultComment(parent, this.getText());
    }
}
