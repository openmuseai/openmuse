// Java: import org.dom4j.Element

// Java: public class DefaultComment extends FlyweightComment
pub struct DefaultComment {
    // Java field: Element parent
    pub(crate) text: String,
}

// Java: public class DefaultComment extends FlyweightComment
impl DefaultComment {
    // Java: DefaultComment(String text)
    pub fn new(text: String) -> Self {
        // Java: super(text);
        DefaultComment { text }
    }

    // Java: DefaultComment(Element parent, String text)
    pub fn new_with_parent(text: String) -> Self {
        // Java: super(text);
        // Java: this.parent = parent;
        DefaultComment { text }
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.text = text;
        self.text = text;
    }

    // Java: Element getParent()
    pub fn getParent(&self) {
        // Java: return this.parent;
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self) {
        // Java: this.parent = parent;
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isReadOnly()
    pub fn isReadOnly(&self) -> bool {
        // Java: return false;
        false
    }
}
