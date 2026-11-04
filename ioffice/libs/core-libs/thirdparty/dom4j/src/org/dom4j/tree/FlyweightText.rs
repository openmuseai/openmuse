// Java: public class FlyweightText extends AbstractText implements Text
pub struct FlyweightText {
    // Java field: String text
    pub(crate) text: String,
    // Java field: Element parent
}

// Java: public class FlyweightText extends AbstractText implements Text
impl FlyweightText {
    // Java: FlyweightText(String text)
    pub fn new(text: String) -> Self {
        // Java: super();
        // Java: this.text = text;
        FlyweightText { text }
    }

    // Java: String getText()
    pub fn getText(&self) -> &str {
        // Java: return this.text;
        &self.text
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self, _parent: &str) {
        // Java: return new DefaultText(parent, this.getText());
    }
}