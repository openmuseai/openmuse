use std::collections::HashMap;

// Java: public interface ProcessingInstruction extends Node
pub trait ProcessingInstruction {
    // Java: abstract String getTarget()
    fn getTarget(&self) -> String;

    // Java: abstract void setTarget(String var1)
    fn setTarget(&mut self, var1: String);

    // Java: abstract String getText()
    fn getText(&self) -> String;

    // Java: abstract String getValue(String var1)
    fn getValue(&self, var1: &str) -> Option<String>;

    // Java: abstract Map<String, String> getValues()
    fn getValues(&self) -> &HashMap<String, String>;

    // Java: abstract void setValue(String var1, String var2)
    fn setValue(&mut self, var1: String, var2: String);

    // Java: abstract void setValues(Map<String, String> var1)
    fn setValues(&mut self, var1: HashMap<String, String>);

    // Java: abstract boolean removeValue(String var1)
    fn removeValue(&mut self, var1: &str) -> bool;
}
