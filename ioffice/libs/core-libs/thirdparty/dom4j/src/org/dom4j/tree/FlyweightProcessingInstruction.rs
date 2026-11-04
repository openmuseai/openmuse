// Java: import java.util.Collections
// Java: import java.util.Map
// Java: import org.dom4j.Element
// Java: import org.dom4j.Node

use std::collections::HashMap;

// Java: public class FlyweightProcessingInstruction extends AbstractProcessingInstruction
pub struct FlyweightProcessingInstruction {
    // Java field: String target
    pub(crate) target: String,
    // Java field: String text
    pub(crate) text: String,
    // Java field: Map<String, String> values
    pub(crate) values: HashMap<String, String>,
}

// Java: public class FlyweightProcessingInstruction extends AbstractProcessingInstruction
impl FlyweightProcessingInstruction {
    // Java: FlyweightProcessingInstruction()
    pub fn new() -> Self {
        // Java: super();
        FlyweightProcessingInstruction {
            target: String::new(),
            text: String::new(),
            values: HashMap::new(),
        }
    }

    // Java: FlyweightProcessingInstruction(String target, Map<String, String> values)
    pub fn new_target_values(target: String, values: HashMap<String, String>) -> Self {
        // Java: super();
        // Java: this.target = target;
        // Java: this.values = values;
        // Java: this.text = this.toString(values);
        let text = String::new();
        FlyweightProcessingInstruction { target, text, values }
    }

    // Java: FlyweightProcessingInstruction(String target, String text)
    pub fn new_target_text(target: String, text: String) -> Self {
        // Java: super();
        // Java: this.target = target;
        // Java: this.text = text;
        // Java: this.values = this.parseValues(text);
        let values = HashMap::new();
        FlyweightProcessingInstruction { target, text, values }
    }

    // Java: String getTarget()
    pub fn getTarget(&self) -> String {
        // Java: return this.target;
        self.target.clone()
    }

    // Java: void setTarget(String target)
    pub fn setTarget(&self, _target: String) {
        // Java: throw new UnsupportedOperationException("This PI is read-only and cannot be modified");
        panic!("This PI is read-only and cannot be modified");
    }

    // Java: String getText()
    pub fn getText(&self) -> String {
        // Java: return this.text;
        self.text.clone()
    }

    // Java: String getValue(String name)
    pub fn getValue(&self, name: &str) -> String {
        // Java: String answer = (String)this.values.get(name);
        // Java: return answer == null ? "" : answer;
        self.values.get(name).cloned().unwrap_or_default()
    }

    // Java: public Map<String, String> getValues()
    pub fn getValues(&self) -> HashMap<String, String> {
        // Java: return Collections.unmodifiableMap(this.values);
        self.values.clone()
    }

    // Java: Node createXPathResult(Element parent)
    pub fn createXPathResult(&self) {
        // Java: return new DefaultProcessingInstruction(parent, this.getTarget(), this.getText());
    }
}
