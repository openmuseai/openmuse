// Java: public interface CharacterData extends Node
pub trait CharacterData {
    // Java: abstract void appendText(String var1)
    fn appendText(&mut self, var1: &str);
}
