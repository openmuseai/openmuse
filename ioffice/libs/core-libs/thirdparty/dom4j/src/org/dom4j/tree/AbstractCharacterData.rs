// Java: import org.dom4j.CharacterData
// Java: import org.dom4j.Element

// Java: public class AbstractCharacterData extends AbstractNode implements CharacterData
pub struct AbstractCharacterData {
}

// Java: public class AbstractCharacterData extends AbstractNode implements CharacterData
impl AbstractCharacterData {
    // Java: AbstractCharacterData()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: String getPath(Element context)
    pub fn getPath(&self) -> String {
      // Java: Element parent = this.getParent();
      // Java: return parent != null && parent != context ? parent.getPath(context) + "/text()" : "text()";
      String::new()
    }

    // Java: String getUniquePath(Element context)
    pub fn getUniquePath(&self) -> String {
      // Java: Element parent = this.getParent();
      // Java: return parent != null && parent != context ? parent.getUniquePath(context) + "/text()" : "text()";
      String::new()
    }

    // Java: void appendText(String text)
    pub fn appendText(&self, _text: &str) {
      // Java: this.setText(this.getText() + text);
    }

}
