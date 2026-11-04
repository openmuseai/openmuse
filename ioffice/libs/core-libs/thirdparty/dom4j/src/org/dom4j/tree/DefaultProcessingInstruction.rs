// Java: import java.util.Map
// Java: import org.dom4j.Element

use std::collections::HashMap;

// Java: public class DefaultProcessingInstruction extends FlyweightProcessingInstruction
pub struct DefaultProcessingInstruction {
    // Java field: Element parent
    pub(crate) target: String,
    pub(crate) text: String,
    pub(crate) values: HashMap<String, String>,
}

// Java: public class DefaultProcessingInstruction extends FlyweightProcessingInstruction
impl DefaultProcessingInstruction {
    // Java: DefaultProcessingInstruction(String target, Map<String, String> values)
    pub fn new_target_values(target: String, values: HashMap<String, String>) -> Self {
        // Java: super(target, values);
        DefaultProcessingInstruction { target, text: String::new(), values }
    }

    // Java: DefaultProcessingInstruction(String target, String values)
    pub fn new_target_text(target: String, text: String) -> Self {
        // Java: super(target, values);
        DefaultProcessingInstruction { target, text, values: HashMap::new() }
    }

    // Java: DefaultProcessingInstruction(Element parent, String target, String values)
    pub fn new_parent_target_text(target: String, text: String) -> Self {
        // Java: super(target, values);
        // Java: this.parent = parent;
        DefaultProcessingInstruction { target, text, values: HashMap::new() }
    }

    // Java: void setTarget(String target)
    pub fn setTarget(&mut self, target: String) {
        // Java: this.target = target;
        self.target = target;
    }

    // Java: void setText(String text)
    pub fn setText(&mut self, text: String) {
        // Java: this.text = text;
        // Java: this.values = this.parseValues(text);
        self.text = text;
    }

    // Java: void setValues(Map<String, String> values)
    pub fn setValues(&mut self, values: HashMap<String, String>) {
        // Java: this.values = values;
        // Java: this.text = this.toString(values);
        self.values = values;
    }

    // Java: void setValue(String name, String value)
    pub fn setValue(&mut self, name: String, value: String) {
        // Java: this.values.put(name, value);
        self.values.insert(name, value);
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
